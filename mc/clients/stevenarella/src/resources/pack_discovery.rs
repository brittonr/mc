const FIRST_EXTERNAL_PACK_INDEX: usize = 1;

pub(crate) fn downloaded_pack_insert_index(existing_pack_count: usize) -> usize {
    existing_pack_count.min(FIRST_EXTERNAL_PACK_INDEX)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_PACK_COUNT: usize = 0;
    const INTERNAL_ONLY_PACK_COUNT: usize = 1;
    const MULTI_PACK_COUNT: usize = 3;

    #[test]
    fn downloaded_packs_insert_after_internal_pack_when_present() {
        assert_eq!(
            downloaded_pack_insert_index(EMPTY_PACK_COUNT),
            EMPTY_PACK_COUNT
        );
        assert_eq!(
            downloaded_pack_insert_index(INTERNAL_ONLY_PACK_COUNT),
            FIRST_EXTERNAL_PACK_INDEX
        );
        assert_eq!(
            downloaded_pack_insert_index(MULTI_PACK_COUNT),
            FIRST_EXTERNAL_PACK_INDEX
        );
    }
}
