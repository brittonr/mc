use std::array;
use std::io::Write;

use arrayvec::ArrayVec;
use valence_protocol::{Encode, VarInt};

use super::chunk::bit_width;

const PACKED_INDICES_PER_BYTE: usize = 2;
const NIBBLE_BITS: usize = 4;
const NIBBLE_MASK: u8 = 0b1111;
const INDIRECT_PALETTE_CAPACITY: usize = 16;
const U64_BITS: usize = u64::BITS as usize;

/// `HALF_LEN` must be equal to `ceil(LEN / 2)`.
#[derive(Clone, Debug)]
pub(super) enum PalettedContainer<T, const LEN: usize, const HALF_LEN: usize> {
    Single(T),
    Indirect(Box<Indirect<T, LEN, HALF_LEN>>),
    Direct(Box<[T; LEN]>),
}

#[derive(Clone, Debug)]
pub(super) struct Indirect<T, const LEN: usize, const HALF_LEN: usize> {
    /// Each element is a unique instance of `T`. The length of the palette is
    /// always ≥2.
    palette: ArrayVec<T, INDIRECT_PALETTE_CAPACITY>,
    /// Each half-byte is an index into `palette`.
    indices: [u8; HALF_LEN],
}

impl<T: Copy + Eq + Default, const LEN: usize, const HALF_LEN: usize>
    PalettedContainer<T, LEN, HALF_LEN>
{
    pub(super) fn new() -> Self {
        assert_eq!(LEN.div_ceil(PACKED_INDICES_PER_BYTE), HALF_LEN);
        assert_ne!(LEN, 0);

        Self::Single(T::default())
    }

    pub(super) fn fill(&mut self, val: T) {
        *self = Self::Single(val)
    }

    #[track_caller]
    pub(super) fn get(&self, idx: usize) -> T {
        debug_assert!(idx < LEN);

        match self {
            Self::Single(elem) => *elem,
            Self::Indirect(ind) => ind.get(idx),
            Self::Direct(elems) => elems[idx],
        }
    }

    /// Counts logical entries whose value satisfies `predicate`.
    ///
    /// For indirect storage, the predicate is evaluated once per palette value
    /// and then applied to the packed indices. This keeps the helper cache-free
    /// while avoiding a full logical section scan for palette-backed sections.
    pub(super) fn count_matching<F>(&self, mut predicate: F) -> usize
    where
        F: FnMut(T) -> bool,
    {
        match self {
            Self::Single(elem) => count_single_match(*elem, LEN, &mut predicate),
            Self::Indirect(ind) => ind.count_matching(predicate),
            Self::Direct(vals) => count_direct_matches(vals.as_ref(), predicate),
        }
    }

    #[track_caller]
    pub(super) fn set(&mut self, idx: usize, val: T) -> T {
        debug_assert!(idx < LEN);

        match self {
            Self::Single(old_val) => {
                if *old_val == val {
                    *old_val
                } else {
                    // Upgrade to indirect.
                    let old = *old_val;
                    let mut ind = Box::new(Indirect {
                        palette: ArrayVec::from_iter([old, val]),
                        // All indices are initialized to index 0 (the old element).
                        indices: [0; HALF_LEN],
                    });

                    ind.indices[idx / PACKED_INDICES_PER_BYTE] =
                        1 << (idx % PACKED_INDICES_PER_BYTE * NIBBLE_BITS);
                    *self = Self::Indirect(ind);
                    old
                }
            }
            Self::Indirect(ind) => {
                if let Some(old) = ind.set(idx, val) {
                    old
                } else {
                    // Upgrade to direct.
                    *self = Self::Direct(Box::new(array::from_fn(|i| ind.get(i))));
                    self.set(idx, val)
                }
            }
            Self::Direct(vals) => {
                let old = vals[idx];
                vals[idx] = val;
                old
            }
        }
    }

    pub(super) fn shrink_to_fit(&mut self) {
        match self {
            Self::Single(_) => {}
            Self::Indirect(ind) => {
                let mut new_ind = Indirect {
                    palette: ArrayVec::new(),
                    indices: [0; HALF_LEN],
                };

                for i in 0..LEN {
                    new_ind.set(i, ind.get(i));
                }

                if new_ind.palette.len() == 1 {
                    *self = Self::Single(new_ind.palette[0]);
                } else {
                    **ind = new_ind;
                }
            }
            Self::Direct(dir) => {
                let mut ind = Indirect {
                    palette: ArrayVec::new(),
                    indices: [0; HALF_LEN],
                };

                for (i, val) in dir.iter().copied().enumerate() {
                    if ind.set(i, val).is_none() {
                        return;
                    }
                }

                *self = if ind.palette.len() == 1 {
                    Self::Single(ind.palette[0])
                } else {
                    Self::Indirect(Box::new(ind))
                };
            }
        }
    }

    /// Encodes the paletted container in the format that Minecraft expects.
    ///
    /// - **`writer`**: The [`Write`] instance to write the paletted container
    ///   to.
    /// - **`to_bits`**: A function to convert the element type to bits. The
    ///   output must be less than two to the power of `direct_bits`.
    /// - **`min_indirect_bits`**: The minimum number of bits used to represent
    ///   the element type in the indirect representation. If the bits per index
    ///   is lower, it will be rounded up to this.
    /// - **`max_indirect_bits`**: The maximum number of bits per element
    ///   allowed in the indirect representation. Any higher than this will
    ///   force conversion to the direct representation while encoding.
    /// - **`direct_bits`**: The minimum number of bits required to represent
    ///   all instances of the element type. If `N` is the total number of
    ///   possible values, then `DIRECT_BITS` is `floor(log2(N - 1)) + 1`.
    pub(super) fn encode_mc_format<W, F>(
        &self,
        mut writer: W,
        mut to_bits: F,
        min_indirect_bits: usize,
        max_indirect_bits: usize,
        direct_bits: usize,
    ) -> anyhow::Result<()>
    where
        W: Write,
        F: FnMut(T) -> u64,
    {
        debug_assert!(min_indirect_bits <= NIBBLE_BITS);
        debug_assert!(min_indirect_bits <= max_indirect_bits);
        debug_assert!(max_indirect_bits <= U64_BITS);
        debug_assert!(direct_bits <= U64_BITS);

        match self {
            Self::Single(val) => {
                // Bits per entry
                0_u8.encode(&mut writer)?;

                // Palette
                VarInt(to_bits(*val) as i32).encode(&mut writer)?;

                // Number of longs
                VarInt(0).encode(writer)?;
            }
            Self::Indirect(ind) => {
                let bits_per_entry = min_indirect_bits.max(bit_width(ind.palette.len() - 1));

                // Encode as direct if necessary.
                if bits_per_entry > max_indirect_bits {
                    // Bits per entry
                    (direct_bits as u8).encode(&mut writer)?;

                    // Number of longs in data array.
                    VarInt(compact_u64s_len(LEN, direct_bits) as i32).encode(&mut writer)?;
                    // Data array
                    encode_compact_u64s(
                        writer,
                        (0..LEN).map(|i| to_bits(ind.get(i))),
                        direct_bits,
                    )?;
                } else {
                    // Bits per entry
                    (bits_per_entry as u8).encode(&mut writer)?;

                    // Palette len
                    VarInt(ind.palette.len() as i32).encode(&mut writer)?;
                    // Palette
                    for val in &ind.palette {
                        VarInt(to_bits(*val) as i32).encode(&mut writer)?;
                    }

                    // Number of longs in data array.
                    VarInt(compact_u64s_len(LEN, bits_per_entry) as i32).encode(&mut writer)?;
                    // Data array
                    encode_compact_u64s(
                        writer,
                        ind.indices
                            .iter()
                            .copied()
                            .flat_map(|byte| [byte & NIBBLE_MASK, byte >> NIBBLE_BITS])
                            .map(u64::from)
                            .take(LEN),
                        bits_per_entry,
                    )?;
                }
            }
            Self::Direct(dir) => {
                // Bits per entry
                (direct_bits as u8).encode(&mut writer)?;

                // Number of longs in data array.
                VarInt(compact_u64s_len(LEN, direct_bits) as i32).encode(&mut writer)?;
                // Data array
                encode_compact_u64s(writer, dir.iter().copied().map(to_bits), direct_bits)?;
            }
        }

        Ok(())
    }
}

impl<T: Copy + Eq + Default, const LEN: usize, const HALF_LEN: usize> Default
    for PalettedContainer<T, LEN, HALF_LEN>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Eq + Default, const LEN: usize, const HALF_LEN: usize> Indirect<T, LEN, HALF_LEN> {
    pub(super) fn get(&self, idx: usize) -> T {
        let palette_idx = (self.indices[idx / PACKED_INDICES_PER_BYTE]
            >> (idx % PACKED_INDICES_PER_BYTE * NIBBLE_BITS))
            & NIBBLE_MASK;
        self.palette[palette_idx as usize]
    }

    pub(super) fn count_matching<F>(&self, mut predicate: F) -> usize
    where
        F: FnMut(T) -> bool,
    {
        let matching_mask = self.matching_palette_mask(&mut predicate);
        if matching_mask == 0 {
            return 0;
        }

        if matching_mask == active_palette_mask(self.palette.len()) {
            return LEN;
        }

        let pair_count = LEN / PACKED_INDICES_PER_BYTE;
        let mut count = 0;

        for byte in self.indices.iter().copied().take(pair_count) {
            let low_idx = byte & NIBBLE_MASK;
            let high_idx = byte >> NIBBLE_BITS;
            count += usize::from(palette_mask_contains(matching_mask, low_idx));
            count += usize::from(palette_mask_contains(matching_mask, high_idx));
        }

        if !LEN.is_multiple_of(PACKED_INDICES_PER_BYTE) {
            let final_idx = self.indices[pair_count] & NIBBLE_MASK;
            count += usize::from(palette_mask_contains(matching_mask, final_idx));
        }

        count
    }

    fn matching_palette_mask<F>(&self, predicate: &mut F) -> u16
    where
        F: FnMut(T) -> bool,
    {
        let mut mask = 0;
        for (idx, val) in self.palette.iter().copied().enumerate() {
            if predicate(val) {
                mask |= 1_u16 << idx;
            }
        }
        mask
    }

    pub(super) fn set(&mut self, idx: usize, val: T) -> Option<T> {
        let palette_idx = if let Some(i) = self.palette.iter().position(|v| *v == val) {
            i
        } else {
            self.palette.try_push(val).ok()?;
            self.palette.len() - 1
        };

        let old_val = self.get(idx);
        let u8 = &mut self.indices[idx / PACKED_INDICES_PER_BYTE];
        let shift = idx % PACKED_INDICES_PER_BYTE * NIBBLE_BITS;
        *u8 = (*u8 & !(NIBBLE_MASK << shift)) | ((palette_idx as u8) << shift);
        Some(old_val)
    }
}

#[inline]
fn compact_u64s_len(vals_count: usize, bits_per_val: usize) -> usize {
    let vals_per_u64 = U64_BITS / bits_per_val;
    vals_count.div_ceil(vals_per_u64)
}

fn count_single_match<T: Copy>(val: T, len: usize, mut predicate: impl FnMut(T) -> bool) -> usize {
    if predicate(val) {
        len
    } else {
        0
    }
}

fn count_direct_matches<T: Copy>(vals: &[T], mut predicate: impl FnMut(T) -> bool) -> usize {
    let mut count = 0;
    for val in vals.iter().copied() {
        count += usize::from(predicate(val));
    }
    count
}

fn active_palette_mask(len: usize) -> u16 {
    debug_assert!(len <= INDIRECT_PALETTE_CAPACITY);
    if len == INDIRECT_PALETTE_CAPACITY {
        u16::MAX
    } else {
        (1_u16 << len) - 1
    }
}

fn palette_mask_contains(mask: u16, palette_idx: u8) -> bool {
    (mask & (1_u16 << u32::from(palette_idx))) != 0
}

#[inline]
fn encode_compact_u64s(
    mut w: impl Write,
    mut vals: impl Iterator<Item = u64>,
    bits_per_val: usize,
) -> anyhow::Result<()> {
    debug_assert!(bits_per_val <= U64_BITS);

    let vals_per_u64 = U64_BITS / bits_per_val;

    loop {
        let mut n = 0;
        for i in 0..vals_per_u64 {
            match vals.next() {
                Some(val) => {
                    debug_assert!(val < 2_u128.pow(bits_per_val as u32) as u64);
                    n |= val << (i * bits_per_val);
                }
                None if i > 0 => return n.encode(&mut w),
                None => return Ok(()),
            }
        }
        n.encode(&mut w)?;
    }
}

#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use rand::Rng;

    use super::*;

    const TEST_LEN: usize = 32;
    const TEST_HALF_LEN: usize = TEST_LEN / 2;
    const INDIRECT_PALETTE_CAPACITY: usize = 16;
    const FIRST_DIRECT_ONLY_VALUE: u32 = INDIRECT_PALETTE_CAPACITY as u32;
    const COMPACTED_FIRST_VALUE: u32 = 7;
    const COMPACTED_SECOND_VALUE: u32 = 11;
    const ENCODE_MIN_INDIRECT_BITS: usize = 0;
    const ENCODE_MAX_INDIRECT_BITS_FOR_DIRECT_FALLBACK: usize = 3;
    const ENCODE_DIRECT_BITS: usize = 5;
    const ALTERNATING_INDEX_STEP: usize = 2;
    const EXISTING_AND_REPLACED_MATCH_COUNT: usize = 2;

    type TestContainer = PalettedContainer<u32, TEST_LEN, TEST_HALF_LEN>;

    fn check<T: Copy + Eq + Default, const LEN: usize, const HALF_LEN: usize>(
        p: &PalettedContainer<T, LEN, HALF_LEN>,
        s: &[T],
    ) -> bool {
        assert_eq!(s.len(), LEN);
        (0..LEN).all(|i| p.get(i) == s[i])
    }

    #[test]
    fn transitions_preserve_values() {
        let mut p = TestContainer::new();
        let mut expected = [u32::default(); TEST_LEN];

        assert!(matches!(p, PalettedContainer::Single(_)));

        for idx in 1..INDIRECT_PALETTE_CAPACITY {
            let value = idx as u32;
            assert_eq!(expected[idx], p.set(idx, value));
            expected[idx] = value;
            assert!(check(&p, &expected));
        }

        assert!(matches!(p, PalettedContainer::Indirect(_)));

        let direct_idx = INDIRECT_PALETTE_CAPACITY;
        assert_eq!(
            expected[direct_idx],
            p.set(direct_idx, FIRST_DIRECT_ONLY_VALUE)
        );
        expected[direct_idx] = FIRST_DIRECT_ONLY_VALUE;

        assert!(matches!(p, PalettedContainer::Direct(_)));
        assert!(check(&p, &expected));
    }

    #[test]
    fn fill_and_shrink_drop_stale_representation_data() {
        let mut p = direct_container_with_values();

        p.fill(COMPACTED_FIRST_VALUE);
        assert!(matches!(
            p,
            PalettedContainer::Single(COMPACTED_FIRST_VALUE)
        ));
        assert!((0..TEST_LEN).all(|idx| p.get(idx) == COMPACTED_FIRST_VALUE));

        let mut p = direct_container_with_values();
        let mut expected = direct_container_values();
        for idx in (0..TEST_LEN).step_by(ALTERNATING_INDEX_STEP) {
            assert_eq!(expected[idx], p.set(idx, COMPACTED_SECOND_VALUE));
            expected[idx] = COMPACTED_SECOND_VALUE;
        }

        p.shrink_to_fit();

        assert!(matches!(p, PalettedContainer::Indirect(_)));
        assert!(check(&p, &expected));
    }

    #[test]
    fn indirect_direct_encode_parity_when_indirect_uses_direct_path() {
        let indirect = indirect_container_with_full_palette();
        let mut direct = direct_container_with_values();
        assert!(matches!(direct, PalettedContainer::Direct(_)));

        assert_eq!(
            direct.set(INDIRECT_PALETTE_CAPACITY, u32::default()),
            FIRST_DIRECT_ONLY_VALUE
        );
        assert!(matches!(direct, PalettedContainer::Direct(_)));
        assert!(check_same_values(&indirect, &direct));

        assert_eq!(
            encode_test_container(&indirect),
            encode_test_container(&direct)
        );
    }

    #[test]
    fn invalid_index_panics_without_corrupting_in_range_values() {
        let mut p = TestContainer::new();
        assert_eq!(u32::default(), p.set(0, COMPACTED_FIRST_VALUE));

        let get_result = catch_unwind(AssertUnwindSafe(|| p.get(TEST_LEN)));
        assert!(get_result.is_err());
        assert_eq!(COMPACTED_FIRST_VALUE, p.get(0));

        let set_result = catch_unwind(AssertUnwindSafe(|| p.set(TEST_LEN, COMPACTED_SECOND_VALUE)));
        assert!(set_result.is_err());
        assert_eq!(COMPACTED_FIRST_VALUE, p.get(0));
    }

    #[test]
    fn count_matching_tracks_mutations_without_cached_unique_data() {
        let mut p = TestContainer::new();
        assert_eq!(TEST_LEN, p.count_matching(|val| val == u32::default()));
        assert_eq!(0, p.count_matching(|val| val == COMPACTED_FIRST_VALUE));

        assert_eq!(u32::default(), p.set(0, COMPACTED_FIRST_VALUE));
        assert_eq!(1, p.count_matching(|val| val == COMPACTED_FIRST_VALUE));
        assert_eq!(TEST_LEN - 1, p.count_matching(|val| val == u32::default()));

        let p = indirect_container_with_full_palette();
        assert_eq!(
            INDIRECT_PALETTE_CAPACITY - 1,
            p.count_matching(|val| val != u32::default())
        );

        let mut p = direct_container_with_values();
        assert_eq!(1, p.count_matching(|val| val == FIRST_DIRECT_ONLY_VALUE));
        assert_eq!(
            FIRST_DIRECT_ONLY_VALUE,
            p.set(INDIRECT_PALETTE_CAPACITY, COMPACTED_FIRST_VALUE)
        );
        assert_eq!(0, p.count_matching(|val| val == FIRST_DIRECT_ONLY_VALUE));
        assert_eq!(
            EXISTING_AND_REPLACED_MATCH_COUNT,
            p.count_matching(|val| val == COMPACTED_FIRST_VALUE)
        );

        p.fill(COMPACTED_SECOND_VALUE);
        assert_eq!(
            TEST_LEN,
            p.count_matching(|val| val == COMPACTED_SECOND_VALUE)
        );
        assert_eq!(0, p.count_matching(|val| val == COMPACTED_FIRST_VALUE));
    }

    fn indirect_container_with_full_palette() -> TestContainer {
        let mut p = TestContainer::new();
        for idx in 1..INDIRECT_PALETTE_CAPACITY {
            assert_eq!(u32::default(), p.set(idx, idx as u32));
        }
        assert!(matches!(p, PalettedContainer::Indirect(_)));
        p
    }

    fn direct_container_with_values() -> TestContainer {
        let mut p = indirect_container_with_full_palette();
        assert_eq!(
            u32::default(),
            p.set(INDIRECT_PALETTE_CAPACITY, FIRST_DIRECT_ONLY_VALUE)
        );
        assert!(matches!(p, PalettedContainer::Direct(_)));
        p
    }

    fn direct_container_values() -> [u32; TEST_LEN] {
        let mut values = [u32::default(); TEST_LEN];
        for (idx, value) in values
            .iter_mut()
            .enumerate()
            .take(INDIRECT_PALETTE_CAPACITY)
            .skip(1)
        {
            *value = idx as u32;
        }
        values[INDIRECT_PALETTE_CAPACITY] = FIRST_DIRECT_ONLY_VALUE;
        values
    }

    fn encode_test_container(p: &TestContainer) -> Vec<u8> {
        let mut bytes = Vec::new();
        p.encode_mc_format(
            &mut bytes,
            u64::from,
            ENCODE_MIN_INDIRECT_BITS,
            ENCODE_MAX_INDIRECT_BITS_FOR_DIRECT_FALLBACK,
            ENCODE_DIRECT_BITS,
        )
        .unwrap();
        bytes
    }

    fn check_same_values(left: &TestContainer, right: &TestContainer) -> bool {
        (0..TEST_LEN).all(|idx| left.get(idx) == right.get(idx))
    }

    #[test]
    fn random_assignments() {
        const LEN: usize = 100;
        let range = 0..64;

        let mut rng = rand::thread_rng();

        for _ in 0..20 {
            let mut p = PalettedContainer::<u32, LEN, { LEN / 2 }>::new();

            let init = rng.gen_range(range.clone());

            p.fill(init);
            let mut a = [init; LEN];

            assert!(check(&p, &a));

            let mut rng = rand::thread_rng();

            for _ in 0..LEN * 10 {
                let idx = rng.gen_range(0..LEN);
                let val = rng.gen_range(range.clone());

                assert_eq!(p.get(idx), p.set(idx, val));
                assert_eq!(val, p.get(idx));
                a[idx] = val;

                p.shrink_to_fit();

                assert!(check(&p, &a));
            }
        }
    }
}
