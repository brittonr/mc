type TokenStream = proc_macro2::TokenStream;

const MAX_VIEW_DIST: u8 = 32;
const EXTRA_VIEW_RADIUS: i32 = 2;
const MAX_POSITION_COUNT: usize = 4_761;

pub(crate) fn build() -> TokenStream {
    debug_assert!(
        MAX_VIEW_DIST > 0,
        "chunk-view table includes positive distances"
    );
    let entries = (0..=MAX_VIEW_DIST).map(entry_tokens);
    let array_len = usize::from(MAX_VIEW_DIST).saturating_add(1);

    quote::quote! {
        #[doc = "The maximum view distance for a `ChunkView`."]
        pub const MAX_VIEW_DIST: u8 = #MAX_VIEW_DIST;

        pub const EXTRA_VIEW_RADIUS: i32 = #EXTRA_VIEW_RADIUS;

        pub static CHUNK_VIEW_LUT: [&[(i8, i8)]; #array_len] = [ #(#entries),* ];
    }
}

fn entry_tokens(view_dist: u8) -> TokenStream {
    debug_assert!(
        view_dist <= MAX_VIEW_DIST,
        "view distance stays within table bounds"
    );
    let Some(dist) = i32::from(view_dist).checked_add(EXTRA_VIEW_RADIUS) else {
        return TokenStream::new();
    };
    let mut positions = Vec::with_capacity(MAX_POSITION_COUNT);

    for z in -dist..=dist {
        for x in -dist..=dist {
            let position = ChunkOffset { x, z };
            if within_distance(position, dist) {
                if let Some(position) = position_pair(position) {
                    positions.push(position);
                }
            }
        }
    }

    positions.sort_by_key(|position| distance_key(*position));
    let array_elems = positions.into_iter().map(|(x, z)| quote::quote!((#x, #z)));

    quote::quote! {
        &[ #(#array_elems),* ]
    }
}

#[derive(Clone, Copy)]
struct ChunkOffset {
    x: i32,
    z: i32,
}

fn within_distance(position: ChunkOffset, dist: i32) -> bool {
    let Some(x_sq) = square(position.x) else {
        return false;
    };
    let Some(z_sq) = square(position.z) else {
        return false;
    };
    let Some(dist_sq) = square(dist) else {
        return false;
    };
    x_sq.checked_add(z_sq).is_some_and(|sum| sum <= dist_sq)
}

fn square(value: i32) -> Option<i32> {
    value.checked_mul(value)
}

fn position_pair(position: ChunkOffset) -> Option<(i8, i8)> {
    Some((
        i8::try_from(position.x).ok()?,
        i8::try_from(position.z).ok()?,
    ))
}

fn distance_key(position: (i8, i8)) -> Option<i32> {
    let (x, z) = position;
    let x_sq = square(i32::from(x))?;
    let z_sq = square(i32::from(z))?;
    x_sq.checked_add(z_sq)
}
