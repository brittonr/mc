use std::io::Read;

type BigEndian = byteorder::BigEndian;
type File = std::fs::File;
type Error = crate::RegionError;

mod chunk;
pub(crate) mod write;

const EXTERNAL_STREAM_CHUNK_FLAG: u8 = 0x80;
const HEADER_SECTORS: usize = 2;
const HEADER_SIZE: usize = 8192;
const LOCATION_ENTRY_BYTES: usize = 4;
const LOCATION_ENTRY_BYTES_U64: u64 = 4;
const REGION_CHUNK_AXIS: i32 = 32;
const REGION_CHUNK_AXIS_USIZE: usize = 32;
const REGION_CHUNK_COUNT: usize = 1024;
const SEARCH_LIMIT_EXTRA_ATTEMPT: usize = 1;
const SECTOR_SIZE: usize = 4096;
const SECTOR_SIZE_U64: u64 = 4096;

pub(crate) type FilePos = (i32, i32);

#[derive(Debug)]
pub(crate) enum CacheEntry {
    /// There is a region file loaded here.
    Occupied(Box<Storage>),
    /// There is no region file at this position. Don't try to read it from the
    /// filesystem again.
    Vacant,
}

#[bitfield_struct::bitfield(u32)]
struct Location {
    count: u8,
    #[bits(24)]
    offset: u32,
}

impl Location {
    fn is_none(self) -> bool {
        self.0 == 0
    }

    fn offset_and_count(self) -> Result<(usize, usize), Error> {
        let Ok(offset) = usize::try_from(self.offset()) else {
            return Err(Error::InvalidChunkSectorOffset);
        };
        Ok((offset, usize::from(self.count())))
    }
}

#[derive(Debug)]
pub(crate) struct Storage {
    file: File,
    locations: [Location; REGION_CHUNK_COUNT],
    timestamps: [u32; REGION_CHUNK_COUNT],
    used_sectors: bitvec::vec::BitVec,
}

impl Storage {
    pub(crate) fn create(mut file: File) -> Result<Self, Error> {
        let header = [0; HEADER_SIZE];
        std::io::Write::write_all(&mut file, &header)?;

        Ok(Self {
            file,
            locations: [Location::default(); REGION_CHUNK_COUNT],
            timestamps: [0; REGION_CHUNK_COUNT],
            used_sectors: bitvec::vec::BitVec::repeat(true, HEADER_SECTORS),
        })
    }

    pub(crate) fn open(mut file: File) -> Result<Self, Error> {
        let mut header = [0; HEADER_SIZE];
        file.read_exact(&mut header)?;

        let locations = std::array::from_fn(|index| {
            Location(u32::from_be_bytes(header_entry_bytes(
                &header,
                HeaderEntry {
                    index,
                    base_offset: 0,
                },
            )))
        });
        let timestamps = std::array::from_fn(|index| {
            u32::from_be_bytes(header_entry_bytes(
                &header,
                HeaderEntry {
                    index,
                    base_offset: SECTOR_SIZE,
                },
            ))
        });

        let mut used_sectors = bitvec::vec::BitVec::repeat(true, HEADER_SECTORS);
        for location in locations {
            if location.is_none() {
                // No chunk exists at this position.
                continue;
            }

            let Ok((sector_offset, sector_count)) = location.offset_and_count() else {
                continue;
            };
            if sector_offset < HEADER_SECTORS {
                // skip locations pointing inside the header
                continue;
            }
            if sector_count == 0 {
                continue;
            }
            if sector_byte_offset(sector_offset)? > file.metadata()?.len() {
                // this would go past the end of the file, which is impossible
                continue;
            }

            reserve_sectors(
                &mut used_sectors,
                SectorSpan {
                    offset: sector_offset,
                    count: sector_count,
                },
            )?;
        }

        Ok(Self {
            file,
            locations,
            timestamps,
            used_sectors,
        })
    }

    fn chunk_idx(pos_x: i32, pos_z: i32) -> usize {
        let x = local_chunk_coord(pos_x);
        let z = local_chunk_coord(pos_z);
        let Some(row_start) = z.checked_mul(REGION_CHUNK_AXIS_USIZE) else {
            unreachable!();
        };
        let Some(index) = row_start.checked_add(x) else {
            unreachable!();
        };
        index
    }

    fn chunk_coords_from_index(index: usize) -> Result<(usize, usize), Error> {
        if index >= REGION_CHUNK_COUNT {
            return Err(Error::InvalidChunkPosition);
        }
        Ok((
            index % REGION_CHUNK_AXIS_USIZE,
            index / REGION_CHUNK_AXIS_USIZE,
        ))
    }

    fn global_chunk_coord(region_coord: i32, local_coord: usize) -> Result<i32, Error> {
        let Ok(local_coord) = i32::try_from(local_coord) else {
            return Err(Error::InvalidChunkPosition);
        };
        let Some(region_start) = region_coord.checked_mul(REGION_CHUNK_AXIS) else {
            return Err(Error::InvalidChunkPosition);
        };
        region_start
            .checked_add(local_coord)
            .ok_or(Error::InvalidChunkPosition)
    }
}

struct SectorSpan {
    offset: usize,
    count: usize,
}

fn reserve_sectors(used_sectors: &mut bitvec::vec::BitVec, span: SectorSpan) -> Result<(), Error> {
    let Some(end_index) = span.offset.checked_add(span.count) else {
        return Err(Error::InvalidChunkSectorOffset);
    };
    if used_sectors.len() < end_index {
        used_sectors.resize(end_index, false);
    }
    used_sectors[span.offset..end_index].fill(true);
    Ok(())
}

fn allocate_sectors(
    used_sectors: &mut bitvec::vec::BitVec,
    num_sectors: usize,
) -> Result<usize, Error> {
    // Find the first set of consecutive free sectors of length `num_sectors`.
    let mut index = 0;
    debug_assert!(num_sectors > 0);
    let search_attempt_cap = used_sectors
        .len()
        .saturating_add(SEARCH_LIMIT_EXTRA_ATTEMPT);
    let mut free_space_start = used_sectors.len();

    for _attempt in 0..search_attempt_cap {
        let Some(mut candidate_start) = used_sectors[index..].first_zero() else {
            free_space_start = used_sectors.len();
            break;
        };
        candidate_start = candidate_start.saturating_add(index);

        let Some(mut free_space_end) = used_sectors[candidate_start..].first_one() else {
            free_space_start = candidate_start;
            break;
        };
        free_space_end = free_space_end.saturating_add(candidate_start);

        if free_space_end.saturating_sub(candidate_start) >= num_sectors {
            free_space_start = candidate_start;
            break;
        }

        index = free_space_end;
    }

    reserve_sectors(
        used_sectors,
        SectorSpan {
            offset: free_space_start,
            count: num_sectors,
        },
    )?;
    Ok(free_space_start)
}

fn header_location_offset(chunk_idx: usize) -> Result<u64, Error> {
    let Ok(chunk_idx) = u64::try_from(chunk_idx) else {
        return Err(Error::InvalidChunkPosition);
    };
    chunk_idx
        .checked_mul(LOCATION_ENTRY_BYTES_U64)
        .ok_or(Error::InvalidChunkPosition)
}

fn header_timestamp_offset(chunk_idx: usize) -> Result<u64, Error> {
    header_location_offset(chunk_idx)?
        .checked_add(SECTOR_SIZE_U64)
        .ok_or(Error::InvalidChunkPosition)
}

struct HeaderEntry {
    index: usize,
    base_offset: usize,
}

fn header_entry_bytes(
    header: &[u8; HEADER_SIZE],
    entry: HeaderEntry,
) -> [u8; LOCATION_ENTRY_BYTES] {
    let Some(offset) = entry.index.checked_mul(LOCATION_ENTRY_BYTES) else {
        unreachable!();
    };
    let Some(start) = entry.base_offset.checked_add(offset) else {
        unreachable!();
    };
    let Some(end) = start.checked_add(LOCATION_ENTRY_BYTES) else {
        unreachable!();
    };

    let mut bytes = [0; LOCATION_ENTRY_BYTES];
    bytes.copy_from_slice(&header[start..end]);
    bytes
}

fn local_chunk_coord(pos: i32) -> usize {
    let coord = pos.rem_euclid(REGION_CHUNK_AXIS);
    let Ok(coord) = usize::try_from(coord) else {
        unreachable!();
    };
    coord
}

fn sector_byte_offset(sector_offset: usize) -> Result<u64, Error> {
    let Ok(sector_offset) = u64::try_from(sector_offset) else {
        return Err(Error::InvalidChunkSectorOffset);
    };
    sector_offset
        .checked_mul(SECTOR_SIZE_U64)
        .ok_or(Error::InvalidChunkSectorOffset)
}
