const TEST_TICK_RATE: u32 = 20;
const ONE_SECOND_TICKS: u64 = 20;
const ONE_SECOND_MILLIS: i64 = 1_000;

#[test]
fn remaining_ticks_to_millis_converts_ticks_to_duration() {
    assert_eq!(
        super::systems::remaining_ticks_to_millis(ONE_SECOND_TICKS, nonzero_tick_rate()),
        ONE_SECOND_MILLIS
    );
}

#[test]
fn remaining_ticks_to_millis_saturates_invalid_duration() {
    assert_eq!(
        super::systems::remaining_ticks_to_millis(u64::MAX, nonzero_tick_rate()),
        i64::MAX
    );
}

fn nonzero_tick_rate() -> std::num::NonZeroU32 {
    match std::num::NonZeroU32::new(TEST_TICK_RATE) {
        Some(tick_rate) => tick_rate,
        None => panic!("test tick rate must be non-zero"),
    }
}
