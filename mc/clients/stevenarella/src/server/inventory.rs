pub(crate) fn window_state_is_ready(state_id: i32) -> bool {
    state_id > 0
}

#[cfg(test)]
pub(crate) fn slot_belongs_to_window(slot: i16, slot_count: usize) -> bool {
    let Ok(slot_index) = usize::try_from(slot) else {
        return false;
    };
    slot_index < slot_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const READY_STATE_ID: i32 = 12;
    const EMPTY_STATE_ID: i32 = 0;
    const VALID_SLOT: i16 = 4;
    const INVALID_NEGATIVE_SLOT: i16 = -1;
    const SLOT_COUNT: usize = 5;

    #[test]
    fn ready_window_state_requires_positive_state_id() {
        assert!(window_state_is_ready(READY_STATE_ID));
        assert!(!window_state_is_ready(EMPTY_STATE_ID));
    }

    #[test]
    fn slot_validation_rejects_negative_or_out_of_range_slots() {
        assert!(slot_belongs_to_window(VALID_SLOT, SLOT_COUNT));
        assert!(!slot_belongs_to_window(INVALID_NEGATIVE_SLOT, SLOT_COUNT));
        assert!(!slot_belongs_to_window(VALID_SLOT, VALID_SLOT as usize));
    }
}
