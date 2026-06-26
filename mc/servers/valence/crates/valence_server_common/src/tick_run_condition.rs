use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

use bevy_ecs::prelude::Res;

use crate::Server;

/// Phase alignment for a tick-cadence run condition.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TickPhase {
    offset_ticks: u32,
}

impl TickPhase {
    /// Aligns due ticks to multiples of the cadence interval.
    pub const ZERO: Self = Self { offset_ticks: 0 };

    /// Creates a phase offset measured in ticks.
    pub const fn new(offset_ticks: u32) -> Self {
        Self { offset_ticks }
    }

    /// Returns the phase offset measured in ticks.
    pub const fn offset_ticks(self) -> u32 {
        self.offset_ticks
    }
}

/// A reusable tick cadence with explicit interval and phase alignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TickCadence {
    interval_ticks: NonZeroU32,
    phase: TickPhase,
}

impl TickCadence {
    /// Creates a cadence from a non-zero tick interval aligned to tick zero.
    pub const fn new(interval_ticks: NonZeroU32) -> Self {
        Self::with_phase(interval_ticks, TickPhase::ZERO)
    }

    /// Creates a cadence from a non-zero tick interval and explicit phase.
    pub const fn with_phase(interval_ticks: NonZeroU32, phase: TickPhase) -> Self {
        Self {
            interval_ticks,
            phase,
        }
    }

    /// Creates a cadence whose interval is the current server tick rate.
    pub const fn from_tick_rate(tick_rate: NonZeroU32) -> Self {
        Self::new(tick_rate)
    }

    /// Parses a signed tick interval, returning a typed error for invalid input.
    pub fn try_from_ticks(interval_ticks: i64) -> Result<Self, InvalidTickInterval> {
        Self::try_from_ticks_with_phase(interval_ticks, TickPhase::ZERO)
    }

    /// Parses a signed tick interval with explicit phase alignment.
    pub fn try_from_ticks_with_phase(
        interval_ticks: i64,
        phase: TickPhase,
    ) -> Result<Self, InvalidTickInterval> {
        if interval_ticks < 0 {
            return Err(InvalidTickInterval::Negative);
        }

        if interval_ticks == 0 {
            return Err(InvalidTickInterval::Zero);
        }

        if interval_ticks > i64::from(u32::MAX) {
            return Err(InvalidTickInterval::Overflow);
        }

        let Ok(interval_ticks) = u32::try_from(interval_ticks) else {
            return Err(InvalidTickInterval::Overflow);
        };
        let Some(interval_ticks) = NonZeroU32::new(interval_ticks) else {
            return Err(InvalidTickInterval::Zero);
        };

        Ok(Self::with_phase(interval_ticks, phase))
    }

    /// Returns the non-zero tick interval used by this cadence.
    pub const fn interval_ticks(self) -> NonZeroU32 {
        self.interval_ticks
    }

    /// Returns the phase alignment used by this cadence.
    pub const fn phase(self) -> TickPhase {
        self.phase
    }

    /// Returns whether `current_tick` is due for this cadence.
    pub fn is_due(self, current_tick: i64) -> bool {
        current_tick_is_due(current_tick, self)
    }
}

/// The typed error returned when a tick interval cannot form a cadence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvalidTickInterval {
    /// The interval was zero and would be due on every tick by accident.
    Zero,
    /// The interval was negative and cannot represent elapsed ticks.
    Negative,
    /// The interval was too large for the public non-zero tick representation.
    Overflow,
}

impl fmt::Display for InvalidTickInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => f.write_str("tick cadence interval must be non-zero"),
            Self::Negative => f.write_str("tick cadence interval must not be negative"),
            Self::Overflow => f.write_str("tick cadence interval exceeds u32::MAX"),
        }
    }
}

impl Error for InvalidTickInterval {}

/// Returns whether `current_tick` is due for `cadence`.
pub fn current_tick_is_due(current_tick: i64, cadence: TickCadence) -> bool {
    if current_tick < 0 {
        return false;
    }

    let interval_ticks = i64::from(cadence.interval_ticks.get());
    let phase_ticks = i64::from(cadence.phase.offset_ticks()) % interval_ticks;

    current_tick % interval_ticks == phase_ticks
}

/// Creates a Bevy run condition that is true every `cadence` ticks.
pub fn every_ticks(cadence: TickCadence) -> impl Fn(Option<Res<Server>>) -> bool + Clone {
    move |server: Option<Res<Server>>| match server {
        Some(server) => current_tick_is_due(server.current_tick(), cadence),
        None => false,
    }
}

/// Creates a Bevy run condition that is true once per server tick-rate second.
pub fn once_per_second() -> impl Fn(Option<Res<Server>>) -> bool + Clone {
    once_per_second_with_phase(TickPhase::ZERO)
}

/// Creates a once-per-second Bevy run condition with explicit phase alignment.
pub fn once_per_second_with_phase(
    phase: TickPhase,
) -> impl Fn(Option<Res<Server>>) -> bool + Clone {
    move |server: Option<Res<Server>>| match server {
        Some(server) => {
            let cadence = TickCadence::with_phase(server.tick_rate(), phase);
            current_tick_is_due(server.current_tick(), cadence)
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use bevy_app::{App, Update};
    use bevy_ecs::event::{Event, EventReader, Events};
    use bevy_ecs::prelude::{IntoSystemConfigs, Resource};

    use super::*;
    use crate::ServerPlugin;

    const TWO_TICK_INTERVAL_VALUE: u32 = 2;
    const THREE_TICK_INTERVAL_VALUE: u32 = 3;
    const TEN_TICKS: i64 = 10;
    const ELEVEN_TICKS: i64 = 11;
    const TWELVE_TICKS: i64 = 12;
    const NEGATIVE_INTERVAL_TICKS: i64 = -1;
    const ZERO_INTERVAL_TICKS: i64 = 0;
    const OVERFLOW_INTERVAL_TICKS: i64 = i64::MAX;
    const CHANGED_TICK_RATE_VALUE: u32 = 10;
    const EXPECTED_FIRST_RUN_COUNT: u32 = 1;
    const EXPECTED_SECOND_RUN_COUNT: u32 = 2;
    const EXPECTED_EVENT_DRAIN_COUNT: usize = 1;

    const TWO_TICK_INTERVAL: NonZeroU32 = match NonZeroU32::new(TWO_TICK_INTERVAL_VALUE) {
        Some(interval) => interval,
        None => unreachable!(),
    };
    const THREE_TICK_INTERVAL: NonZeroU32 = match NonZeroU32::new(THREE_TICK_INTERVAL_VALUE) {
        Some(interval) => interval,
        None => unreachable!(),
    };
    const CHANGED_TICK_RATE: NonZeroU32 = match NonZeroU32::new(CHANGED_TICK_RATE_VALUE) {
        Some(tick_rate) => tick_rate,
        None => unreachable!(),
    };
    const TWO_TICK_CADENCE: TickCadence = TickCadence::new(TWO_TICK_INTERVAL);

    #[derive(Resource, Default)]
    struct RunCount(u32);

    #[derive(Resource, Default)]
    struct EventCounts {
        drained_count: usize,
        cadence_runs: u32,
    }

    #[derive(Event)]
    struct SmokeEvent;

    #[test]
    fn cadence_accepts_due_ticks_and_rejects_not_due_ticks() {
        assert!(current_tick_is_due(TEN_TICKS, TWO_TICK_CADENCE));
        assert!(!current_tick_is_due(ELEVEN_TICKS, TWO_TICK_CADENCE));
    }

    #[test]
    fn cadence_phase_aligns_due_ticks() {
        let phase = TickPhase::new(TWO_TICK_INTERVAL_VALUE);
        let cadence = TickCadence::with_phase(THREE_TICK_INTERVAL, phase);

        assert!(current_tick_is_due(ELEVEN_TICKS, cadence));
        assert!(!current_tick_is_due(TWELVE_TICKS, cadence));
    }

    #[test]
    fn invalid_intervals_return_typed_errors() {
        assert_eq!(
            TickCadence::try_from_ticks(ZERO_INTERVAL_TICKS).unwrap_err(),
            InvalidTickInterval::Zero
        );
        assert_eq!(
            TickCadence::try_from_ticks(NEGATIVE_INTERVAL_TICKS).unwrap_err(),
            InvalidTickInterval::Negative
        );
        assert_eq!(
            TickCadence::try_from_ticks(OVERFLOW_INTERVAL_TICKS).unwrap_err(),
            InvalidTickInterval::Overflow
        );
    }

    #[test]
    fn run_condition_runs_system_on_due_ticks_only() {
        let mut app = app_with_server();
        app.add_systems(Update, count_run.run_if(every_ticks(TWO_TICK_CADENCE)));

        app.update();
        assert_eq!(run_count(&app), EXPECTED_FIRST_RUN_COUNT);

        app.update();
        assert_eq!(run_count(&app), EXPECTED_FIRST_RUN_COUNT);

        app.update();
        assert_eq!(run_count(&app), EXPECTED_SECOND_RUN_COUNT);
    }

    #[test]
    fn run_condition_fails_closed_without_server_plugin() {
        let mut app = App::new();
        app.init_resource::<RunCount>();
        app.add_systems(Update, count_run.run_if(every_ticks(TWO_TICK_CADENCE)));

        app.update();

        assert_eq!(run_count(&app), 0);
    }

    #[test]
    fn once_per_second_uses_current_tick_rate_at_evaluation_time() {
        let mut app = app_with_server();
        app.add_systems(Update, count_run.run_if(once_per_second()));

        app.update();
        assert_eq!(run_count(&app), EXPECTED_FIRST_RUN_COUNT);

        {
            let mut server = app.world_mut().resource_mut::<Server>();
            server.current_tick = i64::from(CHANGED_TICK_RATE_VALUE);
            server.tick_rate = CHANGED_TICK_RATE;
        };

        app.update();
        assert_eq!(run_count(&app), EXPECTED_SECOND_RUN_COUNT);
    }

    #[test]
    fn ungated_event_reader_does_not_replay_stale_events_on_due_tick() {
        let mut app = app_with_server();
        app.add_event::<SmokeEvent>();
        app.init_resource::<EventCounts>();
        app.add_systems(
            Update,
            (
                drain_smoke_events,
                count_cadence_run.run_if(every_ticks(TWO_TICK_CADENCE)),
            ),
        );

        app.update();
        app.world_mut()
            .resource_mut::<Events<SmokeEvent>>()
            .send(SmokeEvent);
        app.update();
        app.update();

        let counts = app.world().resource::<EventCounts>();
        assert_eq!(counts.drained_count, EXPECTED_EVENT_DRAIN_COUNT);
        assert_eq!(counts.cadence_runs, EXPECTED_SECOND_RUN_COUNT);
    }

    fn app_with_server() -> App {
        let mut app = App::new();
        app.add_plugins(ServerPlugin);
        app.init_resource::<RunCount>();
        app
    }

    fn count_run(mut count: bevy_ecs::prelude::ResMut<RunCount>) {
        count.0 += 1;
    }

    fn count_cadence_run(mut counts: bevy_ecs::prelude::ResMut<EventCounts>) {
        counts.cadence_runs += 1;
    }

    fn drain_smoke_events(
        mut events: EventReader<SmokeEvent>,
        mut counts: bevy_ecs::prelude::ResMut<EventCounts>,
    ) {
        counts.drained_count += events.read().count();
    }

    fn run_count(app: &App) -> u32 {
        app.world().resource::<RunCount>().0
    }
}
