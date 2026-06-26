//! Optional advisory anti-cheat statistics.
//!
//! The core statistics functions in this module are deterministic over explicit
//! samples and sample-window settings. The [`AnticheatStatisticsPlugin`] is only
//! an adapter from Valence event streams to those pure calculations; it emits
//! observations and does not kick, ban, or mutate gameplay state.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::client::Client;
use crate::event_loop::{EventLoopPostUpdate, EventLoopSet, PacketEvent};
use crate::movement::MovementEvent;

/// Default retained samples per metric and client.
pub const DEFAULT_SAMPLE_WINDOW_CAPACITY: usize = 64;
/// Default retained tick span per metric and client.
pub const DEFAULT_SAMPLE_WINDOW_TICKS: u64 = 20;

const COUNT_INCREMENT: u64 = 1;
const EMPTY_SAMPLE_COUNT: usize = 0;
const PACKET_CADENCE_SAMPLE_VALUE: f64 = 1.0;
const SAMPLE_VARIANCE_MIN_COUNT: usize = 2;
const TICK_INCREMENT: u64 = 1;
const FULL_TURN_DEGREES: f32 = 360.0;
const HALF_TURN_DEGREES: f32 = 180.0;

/// Advisory metric families sampled by [`AnticheatStatisticsPlugin`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AnticheatMetric {
    /// Count incoming packets inside the sample window.
    PacketCadence,
    /// Track position deltas from [`MovementEvent`] values.
    MovementDelta,
    /// Track yaw/pitch deltas from [`MovementEvent`] values.
    RotationDelta,
}

/// Validated rolling sample-window settings.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SampleWindowSettings {
    max_samples: usize,
    max_tick_span: u64,
}

impl SampleWindowSettings {
    /// Validates explicit sample-window limits.
    pub fn new(max_samples: usize, max_tick_span: u64) -> Result<Self, SampleWindowError> {
        if max_samples == EMPTY_SAMPLE_COUNT {
            return Err(SampleWindowError::EmptySampleCapacity);
        }

        if max_tick_span == 0 {
            return Err(SampleWindowError::EmptyTickSpan);
        }

        Ok(Self {
            max_samples,
            max_tick_span,
        })
    }

    /// The maximum number of samples retained in one metric window.
    pub const fn max_samples(self) -> usize {
        self.max_samples
    }

    /// The maximum tick span retained in one metric window.
    pub const fn max_tick_span(self) -> u64 {
        self.max_tick_span
    }
}

impl Default for SampleWindowSettings {
    fn default() -> Self {
        Self {
            max_samples: DEFAULT_SAMPLE_WINDOW_CAPACITY,
            max_tick_span: DEFAULT_SAMPLE_WINDOW_TICKS,
        }
    }
}

/// Deterministic diagnostics for invalid sample-window settings.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SampleWindowError {
    /// A rolling window with no capacity cannot produce meaningful samples.
    EmptySampleCapacity,
    /// A rolling window with no tick span cannot describe time-bounded samples.
    EmptyTickSpan,
}

impl SampleWindowError {
    /// Stable diagnostic text suitable for fixtures and logs.
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::EmptySampleCapacity => "sample window capacity must be greater than zero",
            Self::EmptyTickSpan => "sample window tick span must be greater than zero",
        }
    }
}

/// One metric sample tied to an explicit plugin tick.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TimedSample {
    /// The deterministic tick supplied by the imperative shell.
    pub tick: u64,
    /// The sampled metric value.
    pub value: f64,
}

/// Deterministic diagnostics for sample recording failures.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SampleError {
    /// Non-finite values are ignored to avoid misleading scores.
    NonFiniteValue { value: f64 },
    /// Samples for one metric must be processed in non-decreasing tick order.
    NonMonotonicTick {
        previous_tick: u64,
        sample_tick: u64,
    },
    /// The lifetime observed sample counter overflowed.
    CountOverflow { previous_count: u64 },
}

impl SampleError {
    /// Stable diagnostic text suitable for fixtures and logs.
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::NonFiniteValue { .. } => "sample value must be finite",
            Self::NonMonotonicTick { .. } => "sample tick moved backwards",
            Self::CountOverflow { .. } => "sample count overflowed",
        }
    }
}

/// Retained samples and lifetime count for one metric.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetricWindow {
    samples: Vec<TimedSample>,
    observed_sample_count: u64,
}

impl MetricWindow {
    /// Retained samples that are still inside the rolling window.
    pub fn samples(&self) -> &[TimedSample] {
        &self.samples
    }

    /// Total accepted samples seen since the last reset.
    pub const fn observed_sample_count(&self) -> u64 {
        self.observed_sample_count
    }

    /// Computes a snapshot for the currently retained samples.
    pub fn snapshot(&self) -> MetricSnapshot {
        MetricSnapshot::from_samples(&self.samples, self.observed_sample_count)
    }

    /// Returns an empty metric window, dropping all retained and lifetime state.
    pub fn reset(mut self) -> Self {
        self.samples.clear();
        self.observed_sample_count = 0;
        self
    }
}

/// Summary statistics for one retained metric window.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MetricSnapshot {
    /// Samples retained inside the active rolling window.
    pub sample_count: usize,
    /// Total accepted samples seen since the last reset.
    pub observed_sample_count: u64,
    /// Arithmetic mean for retained samples, or `None` for an empty window.
    pub mean: Option<f64>,
    /// Sample variance for retained samples, or `None` until two samples exist.
    pub variance: Option<f64>,
    /// Minimum retained sample, or `None` for an empty window.
    pub min: Option<f64>,
    /// Maximum retained sample, or `None` for an empty window.
    pub max: Option<f64>,
}

impl MetricSnapshot {
    fn empty(observed_sample_count: u64) -> Self {
        Self {
            sample_count: EMPTY_SAMPLE_COUNT,
            observed_sample_count,
            mean: None,
            variance: None,
            min: None,
            max: None,
        }
    }

    fn from_samples(samples: &[TimedSample], observed_sample_count: u64) -> Self {
        if samples.is_empty() {
            return Self::empty(observed_sample_count);
        }

        let mut sample_count = EMPTY_SAMPLE_COUNT;
        let mut mean = 0.0;
        let mut m2 = 0.0;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for sample in samples {
            sample_count += 1;
            min = min.min(sample.value);
            max = max.max(sample.value);

            let sample_count_f64 = sample_count as f64;
            let delta = sample.value - mean;
            mean += delta / sample_count_f64;
            let delta_after_mean = sample.value - mean;
            m2 += delta * delta_after_mean;
        }

        let variance = if sample_count >= SAMPLE_VARIANCE_MIN_COUNT {
            Some(m2 / (sample_count - 1) as f64)
        } else {
            None
        };

        Self {
            sample_count,
            observed_sample_count,
            mean: Some(mean),
            variance,
            min: Some(min),
            max: Some(max),
        }
    }
}

/// Purely records a sample into a new rolling window.
pub fn record_sample(
    settings: SampleWindowSettings,
    window: &MetricWindow,
    sample: TimedSample,
) -> Result<MetricWindow, SampleError> {
    validate_sample_value(sample.value)?;

    if let Some(previous) = window.samples.last() {
        if sample.tick < previous.tick {
            return Err(SampleError::NonMonotonicTick {
                previous_tick: previous.tick,
                sample_tick: sample.tick,
            });
        }
    }

    let observed_sample_count = window
        .observed_sample_count
        .checked_add(COUNT_INCREMENT)
        .ok_or(SampleError::CountOverflow {
            previous_count: window.observed_sample_count,
        })?;

    let mut samples = window.samples.clone();
    samples.push(sample);
    let samples = retain_window_samples(settings, sample.tick, samples);

    Ok(MetricWindow {
        samples,
        observed_sample_count,
    })
}

/// Computes the movement-distance sample for a Valence movement event.
pub fn movement_delta_sample(event: &MovementEvent) -> Result<f64, SampleError> {
    validate_sample_value(event.position.distance(event.old_position))
}

/// Computes the yaw/pitch delta sample for a Valence movement event.
pub fn rotation_delta_sample(event: &MovementEvent) -> Result<f64, SampleError> {
    let yaw_delta = shortest_degrees_delta(event.look.yaw, event.old_look.yaw);
    let pitch_delta = event.look.pitch - event.old_look.pitch;

    validate_sample_value(f64::from(yaw_delta.hypot(pitch_delta)))
}

fn validate_sample_value(value: f64) -> Result<f64, SampleError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SampleError::NonFiniteValue { value })
    }
}

fn shortest_degrees_delta(current: f32, previous: f32) -> f32 {
    let raw_delta = current - previous;
    (raw_delta + HALF_TURN_DEGREES).rem_euclid(FULL_TURN_DEGREES) - HALF_TURN_DEGREES
}

fn retain_window_samples(
    settings: SampleWindowSettings,
    current_tick: u64,
    samples: Vec<TimedSample>,
) -> Vec<TimedSample> {
    let oldest_tick = current_tick.saturating_sub(settings.max_tick_span);
    let mut retained = samples
        .into_iter()
        .filter(|sample| sample.tick >= oldest_tick)
        .collect::<Vec<_>>();

    let excess_count = retained.len().saturating_sub(settings.max_samples);
    if excess_count > EMPTY_SAMPLE_COUNT {
        retained.drain(..excess_count);
    }

    retained
}

/// Plugin configuration for advisory anti-cheat statistics.
#[derive(Resource, Clone, Debug, Default, Eq, PartialEq)]
pub struct AnticheatStatisticsConfig {
    /// Shared rolling window used by every metric and player.
    pub sample_window: SampleWindowSettings,
}

/// Retained metric windows for one live client entity.
///
/// The optional plugin attaches this component to entities that receive a live
/// [`Client`] component. Query this component directly for retained per-client
/// statistics; [`AnticheatStatisticsState`] only owns plugin-global state.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct PlayerAnticheatStatistics {
    /// Packet cadence observations for this player.
    pub packet_cadence: MetricWindow,
    /// Movement delta observations for this player.
    pub movement_delta: MetricWindow,
    /// Rotation delta observations for this player.
    pub rotation_delta: MetricWindow,
}

impl PlayerAnticheatStatistics {
    fn window(&self, metric: AnticheatMetric) -> &MetricWindow {
        match metric {
            AnticheatMetric::PacketCadence => &self.packet_cadence,
            AnticheatMetric::MovementDelta => &self.movement_delta,
            AnticheatMetric::RotationDelta => &self.rotation_delta,
        }
    }

    fn set_window(&mut self, metric: AnticheatMetric, window: MetricWindow) {
        match metric {
            AnticheatMetric::PacketCadence => self.packet_cadence = window,
            AnticheatMetric::MovementDelta => self.movement_delta = window,
            AnticheatMetric::RotationDelta => self.rotation_delta = window,
        }
    }
}

/// In-memory advisory statistics retained by the optional plugin.
#[derive(Resource, Clone, Debug, Default)]
pub struct AnticheatStatisticsState {
    current_tick: u64,
}

impl AnticheatStatisticsState {
    /// Returns the plugin-local tick used to stamp samples.
    pub const fn current_tick(&self) -> u64 {
        self.current_tick
    }

    fn advance_tick(&mut self) {
        self.current_tick = self.current_tick.saturating_add(TICK_INCREMENT);
    }
}

/// Advisory anti-cheat statistics plugin.
///
/// Add this plugin explicitly when you want observations. It is not part of
/// Valence default plugins, and it performs no enforcement by itself.
pub struct AnticheatStatisticsPlugin;

impl Plugin for AnticheatStatisticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnticheatStatisticsConfig>()
            .init_resource::<AnticheatStatisticsState>()
            .add_event::<AnticheatStatisticsEvent>()
            .add_systems(
                EventLoopPostUpdate,
                (
                    remove_disconnected_anticheat_statistics.in_set(EventLoopSet::Diagnostics),
                    initialize_anticheat_statistics.in_set(EventLoopSet::Diagnostics),
                    sample_anticheat_statistics.in_set(EventLoopSet::Diagnostics),
                )
                    .chain(),
            );
    }
}

/// Observation emitted after one metric sample is accepted.
#[derive(Event, Clone, Debug, PartialEq)]
pub struct AnticheatStatisticsEvent {
    /// Player entity that produced the sample.
    pub client: Entity,
    /// Metric family that was sampled.
    pub metric: AnticheatMetric,
    /// Plugin-local tick attached to the sample.
    pub tick: u64,
    /// The raw accepted sample value.
    pub sample: f64,
    /// Snapshot after this sample was accepted.
    pub snapshot: MetricSnapshot,
}

fn remove_disconnected_anticheat_statistics(
    mut commands: Commands,
    mut removed_clients: RemovedComponents<Client>,
    statistics: Query<(), With<PlayerAnticheatStatistics>>,
) {
    for client in removed_clients.read() {
        if statistics.contains(client) {
            commands
                .entity(client)
                .remove::<PlayerAnticheatStatistics>();
        }
    }
}

fn initialize_anticheat_statistics(
    mut commands: Commands,
    clients: Query<Entity, (Added<Client>, Without<PlayerAnticheatStatistics>)>,
) {
    for client in &clients {
        commands
            .entity(client)
            .insert(PlayerAnticheatStatistics::default());
    }
}

fn sample_anticheat_statistics(
    config: Res<AnticheatStatisticsConfig>,
    mut state: ResMut<AnticheatStatisticsState>,
    mut packet_events: EventReader<PacketEvent>,
    mut movement_events: EventReader<MovementEvent>,
    mut client_statistics: Query<&mut PlayerAnticheatStatistics, With<Client>>,
    mut observations: EventWriter<AnticheatStatisticsEvent>,
) {
    let settings = config.sample_window;
    let tick = state.current_tick();

    for packet in packet_events.read() {
        let Ok(mut player_statistics) = client_statistics.get_mut(packet.client) else {
            continue;
        };
        if let Some(observation) = record_player_metric(
            &mut player_statistics,
            settings,
            tick,
            packet.client,
            AnticheatMetric::PacketCadence,
            PACKET_CADENCE_SAMPLE_VALUE,
        ) {
            observations.send(observation);
        }
    }

    for movement in movement_events.read() {
        let Ok(movement_delta) = movement_delta_sample(movement) else {
            continue;
        };
        let Ok(mut player_statistics) = client_statistics.get_mut(movement.client) else {
            continue;
        };
        if let Some(observation) = record_player_metric(
            &mut player_statistics,
            settings,
            tick,
            movement.client,
            AnticheatMetric::MovementDelta,
            movement_delta,
        ) {
            observations.send(observation);
        }

        let Ok(rotation_delta) = rotation_delta_sample(movement) else {
            continue;
        };
        if let Some(observation) = record_player_metric(
            &mut player_statistics,
            settings,
            tick,
            movement.client,
            AnticheatMetric::RotationDelta,
            rotation_delta,
        ) {
            observations.send(observation);
        }
    }

    state.advance_tick();
}

fn record_player_metric(
    player_statistics: &mut PlayerAnticheatStatistics,
    settings: SampleWindowSettings,
    tick: u64,
    client: Entity,
    metric: AnticheatMetric,
    value: f64,
) -> Option<AnticheatStatisticsEvent> {
    let sample = TimedSample { tick, value };
    let window = record_sample(settings, player_statistics.window(metric), sample).ok()?;
    let snapshot = window.snapshot();
    player_statistics.set_window(metric, window);

    Some(AnticheatStatisticsEvent {
        client,
        metric,
        tick,
        sample: value,
        snapshot,
    })
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::Instant;

    use bevy_ecs::event::Events;
    use bytes::{Bytes, BytesMut};
    use uuid::Uuid;
    use valence_entity::Look;
    use valence_math::DVec3;
    use valence_protocol::encode::PacketEncoder;

    use super::*;
    use crate::client::{Client, ClientBundle, ClientBundleArgs, ClientConnection, ReceivedPacket};
    use crate::event_loop::EventLoopPlugin;
    use crate::movement::MovementPlugin;

    const EPSILON: f64 = 0.000_001;
    const NORMAL_WINDOW_CAPACITY: usize = 4;
    const NORMAL_WINDOW_TICKS: u64 = 10;
    const BURST_WINDOW_CAPACITY: usize = 3;
    const BURST_WINDOW_TICKS: u64 = 20;
    const FIRST_TICK: u64 = 1;
    const SECOND_TICK: u64 = 2;
    const THIRD_TICK: u64 = 3;
    const FOURTH_TICK: u64 = 4;
    const FIRST_VALUE: f64 = 2.0;
    const SECOND_VALUE: f64 = 4.0;
    const THIRD_VALUE: f64 = 6.0;
    const FOURTH_VALUE: f64 = 8.0;
    const EXPECTED_EMPTY_OBSERVED_COUNT: u64 = 0;
    const EXPECTED_NORMAL_MEAN: f64 = 3.0;
    const EXPECTED_NORMAL_VARIANCE: f64 = 2.0;
    const EXPECTED_NORMAL_OBSERVED_COUNT: u64 = 2;
    const EXPECTED_BURST_MEAN: f64 = 6.0;
    const EXPECTED_BURST_MIN: f64 = 4.0;
    const EXPECTED_BURST_MAX: f64 = 8.0;
    const EXPECTED_RETAINED_BURST_COUNT: usize = 3;
    const EXPECTED_OBSERVED_BURST_COUNT: u64 = 4;
    const EXPECTED_SINGLE_SAMPLE_COUNT: usize = 1;
    const EXPECTED_NO_EVENTS: usize = 0;
    const MOVEMENT_X: f64 = 3.0;
    const MOVEMENT_Y: f64 = 4.0;
    const MOVEMENT_Z: f64 = 0.0;
    const EXPECTED_MOVEMENT_DELTA: f64 = 5.0;
    const CURRENT_YAW: f32 = 10.0;
    const CURRENT_PITCH: f32 = 20.0;
    const EXPECTED_MOVEMENT_OBSERVATION_COUNT: usize = 2;
    const TEST_PACKET_ID: i32 = 0;
    const TEST_USERNAME: &str = "anticheat-test-client";

    #[test]
    fn normal_samples_compute_rolling_statistics() {
        let settings =
            SampleWindowSettings::new(NORMAL_WINDOW_CAPACITY, NORMAL_WINDOW_TICKS).unwrap();
        let window = MetricWindow::default();
        let window = record_sample(
            settings,
            &window,
            TimedSample {
                tick: FIRST_TICK,
                value: FIRST_VALUE,
            },
        )
        .unwrap();
        let window = record_sample(
            settings,
            &window,
            TimedSample {
                tick: SECOND_TICK,
                value: SECOND_VALUE,
            },
        )
        .unwrap();

        let snapshot = window.snapshot();

        assert_eq!(snapshot.sample_count, SAMPLE_VARIANCE_MIN_COUNT);
        assert_eq!(
            snapshot.observed_sample_count,
            EXPECTED_NORMAL_OBSERVED_COUNT
        );
        assert_close(snapshot.mean.unwrap(), EXPECTED_NORMAL_MEAN);
        assert_close(snapshot.variance.unwrap(), EXPECTED_NORMAL_VARIANCE);
        assert_eq!(snapshot.min, Some(FIRST_VALUE));
        assert_eq!(snapshot.max, Some(SECOND_VALUE));
    }

    #[test]
    fn burst_samples_trim_to_sample_capacity() {
        let settings =
            SampleWindowSettings::new(BURST_WINDOW_CAPACITY, BURST_WINDOW_TICKS).unwrap();
        let window = MetricWindow::default();
        let window = record_sample(
            settings,
            &window,
            TimedSample {
                tick: FIRST_TICK,
                value: FIRST_VALUE,
            },
        )
        .unwrap();
        let window = record_sample(
            settings,
            &window,
            TimedSample {
                tick: SECOND_TICK,
                value: SECOND_VALUE,
            },
        )
        .unwrap();
        let window = record_sample(
            settings,
            &window,
            TimedSample {
                tick: THIRD_TICK,
                value: THIRD_VALUE,
            },
        )
        .unwrap();
        let window = record_sample(
            settings,
            &window,
            TimedSample {
                tick: FOURTH_TICK,
                value: FOURTH_VALUE,
            },
        )
        .unwrap();

        let snapshot = window.snapshot();

        assert_eq!(snapshot.sample_count, EXPECTED_RETAINED_BURST_COUNT);
        assert_eq!(
            snapshot.observed_sample_count,
            EXPECTED_OBSERVED_BURST_COUNT
        );
        assert_close(snapshot.mean.unwrap(), EXPECTED_BURST_MEAN);
        assert_eq!(snapshot.min, Some(EXPECTED_BURST_MIN));
        assert_eq!(snapshot.max, Some(EXPECTED_BURST_MAX));
    }

    #[test]
    fn empty_window_returns_documented_empty_snapshot() {
        let snapshot = MetricWindow::default().snapshot();

        assert_eq!(snapshot.sample_count, EMPTY_SAMPLE_COUNT);
        assert_eq!(
            snapshot.observed_sample_count,
            EXPECTED_EMPTY_OBSERVED_COUNT
        );
        assert_eq!(snapshot.mean, None);
        assert_eq!(snapshot.variance, None);
        assert_eq!(snapshot.min, None);
        assert_eq!(snapshot.max, None);
    }

    #[test]
    fn invalid_window_and_non_finite_sample_fail_closed() {
        assert_eq!(
            SampleWindowSettings::new(EMPTY_SAMPLE_COUNT, NORMAL_WINDOW_TICKS)
                .unwrap_err()
                .diagnostic(),
            "sample window capacity must be greater than zero"
        );
        assert_eq!(
            SampleWindowSettings::new(NORMAL_WINDOW_CAPACITY, EMPTY_SAMPLE_COUNT as u64)
                .unwrap_err()
                .diagnostic(),
            "sample window tick span must be greater than zero"
        );

        let settings =
            SampleWindowSettings::new(NORMAL_WINDOW_CAPACITY, NORMAL_WINDOW_TICKS).unwrap();
        let window = MetricWindow::default();
        let error = record_sample(
            settings,
            &window,
            TimedSample {
                tick: FIRST_TICK,
                value: f64::NAN,
            },
        )
        .unwrap_err();

        assert_eq!(error.diagnostic(), "sample value must be finite");
        assert_eq!(window.snapshot().sample_count, EMPTY_SAMPLE_COUNT);
    }

    #[test]
    fn overflow_boundary_fails_closed() {
        let settings =
            SampleWindowSettings::new(NORMAL_WINDOW_CAPACITY, NORMAL_WINDOW_TICKS).unwrap();
        let window = MetricWindow {
            samples: Vec::new(),
            observed_sample_count: u64::MAX,
        };
        let error = record_sample(
            settings,
            &window,
            TimedSample {
                tick: FIRST_TICK,
                value: FIRST_VALUE,
            },
        )
        .unwrap_err();

        assert_eq!(
            error,
            SampleError::CountOverflow {
                previous_count: u64::MAX
            }
        );
    }

    #[test]
    fn reset_behavior_clears_window() {
        let settings =
            SampleWindowSettings::new(NORMAL_WINDOW_CAPACITY, NORMAL_WINDOW_TICKS).unwrap();
        let window = record_sample(
            settings,
            &MetricWindow::default(),
            TimedSample {
                tick: FIRST_TICK,
                value: FIRST_VALUE,
            },
        )
        .unwrap();

        let snapshot = window.reset().snapshot();

        assert_eq!(snapshot.sample_count, EMPTY_SAMPLE_COUNT);
        assert_eq!(
            snapshot.observed_sample_count,
            EXPECTED_EMPTY_OBSERVED_COUNT
        );
    }

    #[test]
    fn plugin_disabled_has_no_anticheat_effect() {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin).add_plugins(MovementPlugin);
        let client = spawn_live_client(&mut app);

        send_packet_event(&mut app, client);
        app.update();

        assert!(app
            .world()
            .get_resource::<AnticheatStatisticsState>()
            .is_none());
        assert!(app
            .world()
            .get_resource::<Events<AnticheatStatisticsEvent>>()
            .is_none());
        assert!(app
            .world()
            .get::<PlayerAnticheatStatistics>(client)
            .is_none());
    }

    #[test]
    fn plugin_samples_movement_without_default_enforcement() {
        let mut app = anticheat_app();
        let client = spawn_live_client(&mut app);

        send_movement_event(&mut app, client);
        app.update();

        let events = anticheat_events(&app);

        assert_eq!(events.len(), EXPECTED_MOVEMENT_OBSERVATION_COUNT);
        assert!(events
            .iter()
            .any(|event| event.metric == AnticheatMetric::MovementDelta));
        assert!(events
            .iter()
            .any(|event| event.metric == AnticheatMetric::RotationDelta));
        assert!(app.world().get_entity(client).is_some());

        let player = app
            .world()
            .get::<PlayerAnticheatStatistics>(client)
            .unwrap();
        let movement_snapshot = player.movement_delta.snapshot();
        assert_close(movement_snapshot.mean.unwrap(), EXPECTED_MOVEMENT_DELTA);
    }

    #[test]
    fn plugin_samples_packet_cadence() {
        let mut app = anticheat_app();
        let client = spawn_live_client(&mut app);

        send_packet_event(&mut app, client);
        app.update();

        let player = app
            .world()
            .get::<PlayerAnticheatStatistics>(client)
            .unwrap();
        let packet_snapshot = player.packet_cadence.snapshot();

        assert_eq!(packet_snapshot.sample_count, EXPECTED_SINGLE_SAMPLE_COUNT);
        assert_close(packet_snapshot.mean.unwrap(), PACKET_CADENCE_SAMPLE_VALUE);
    }

    #[test]
    fn stale_despawned_client_sample_is_ignored() {
        let mut app = anticheat_app();
        let stale_client = spawn_live_client(&mut app);
        app.world_mut().despawn(stale_client);

        send_packet_event(&mut app, stale_client);
        app.update();

        assert_eq!(anticheat_events(&app).len(), EXPECTED_NO_EVENTS);
        assert!(app.world().get_entity(stale_client).is_none());
    }

    #[test]
    fn disconnected_client_loses_statistics_and_samples_are_ignored() {
        let mut app = anticheat_app();
        let client = spawn_live_client(&mut app);
        app.update();
        assert!(app
            .world()
            .get::<PlayerAnticheatStatistics>(client)
            .is_some());

        app.world_mut().entity_mut(client).remove::<Client>();
        send_packet_event(&mut app, client);
        app.update();

        assert_eq!(anticheat_events(&app).len(), EXPECTED_NO_EVENTS);
        assert!(app.world().get::<Client>(client).is_none());
        assert!(app
            .world()
            .get::<PlayerAnticheatStatistics>(client)
            .is_none());
    }

    #[test]
    fn missing_statistics_component_sample_is_ignored() {
        let mut app = anticheat_app();
        let client = spawn_live_client(&mut app);
        app.update();
        app.world_mut()
            .entity_mut(client)
            .remove::<PlayerAnticheatStatistics>();

        send_packet_event(&mut app, client);
        app.update();

        assert_eq!(anticheat_events(&app).len(), EXPECTED_NO_EVENTS);
        assert!(app
            .world()
            .get::<PlayerAnticheatStatistics>(client)
            .is_none());
    }

    #[test]
    fn reconnect_like_new_entity_starts_with_fresh_statistics() {
        let mut app = anticheat_app();
        let first_client = spawn_live_client(&mut app);
        send_packet_event(&mut app, first_client);
        app.update();
        assert_eq!(
            packet_sample_count(&app, first_client),
            EXPECTED_SINGLE_SAMPLE_COUNT
        );

        app.world_mut().despawn(first_client);
        let second_client = spawn_live_client(&mut app);
        send_packet_event(&mut app, second_client);
        app.update();

        assert!(app.world().get_entity(first_client).is_none());
        assert_eq!(
            packet_sample_count(&app, second_client),
            EXPECTED_SINGLE_SAMPLE_COUNT
        );
    }

    #[test]
    fn preexisting_statistics_component_is_not_overwritten_on_client_initialization() {
        let mut app = anticheat_app();
        let initial_statistics = PlayerAnticheatStatistics {
            packet_cadence: seeded_metric_window(),
            ..Default::default()
        };
        let client = app
            .world_mut()
            .spawn((test_client_bundle(), initial_statistics))
            .id();

        app.update();

        assert_eq!(
            packet_sample_count(&app, client),
            EXPECTED_SINGLE_SAMPLE_COUNT
        );
    }

    fn anticheat_app() -> App {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(AnticheatStatisticsPlugin);
        app
    }

    fn spawn_live_client(app: &mut App) -> Entity {
        app.world_mut().spawn(test_client_bundle()).id()
    }

    fn test_client_bundle() -> ClientBundle {
        ClientBundle::new(ClientBundleArgs {
            username: TEST_USERNAME.to_owned(),
            uuid: Uuid::nil(),
            ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
            properties: Vec::new(),
            conn: Box::new(TestConnection),
            enc: PacketEncoder::new(),
        })
    }

    fn send_packet_event(app: &mut App, client: Entity) {
        app.world_mut()
            .resource_mut::<Events<PacketEvent>>()
            .send(PacketEvent {
                client,
                timestamp: Instant::now(),
                id: TEST_PACKET_ID,
                data: Bytes::new(),
            });
    }

    fn send_movement_event(app: &mut App, client: Entity) {
        app.world_mut()
            .resource_mut::<Events<MovementEvent>>()
            .send(MovementEvent {
                client,
                position: DVec3::new(MOVEMENT_X, MOVEMENT_Y, MOVEMENT_Z),
                old_position: DVec3::ZERO,
                look: Look {
                    yaw: CURRENT_YAW,
                    pitch: CURRENT_PITCH,
                },
                old_look: Look::default(),
                on_ground: true,
                old_on_ground: true,
            });
    }

    fn anticheat_events(app: &App) -> Vec<AnticheatStatisticsEvent> {
        app.world()
            .resource::<Events<AnticheatStatisticsEvent>>()
            .iter_current_update_events()
            .cloned()
            .collect::<Vec<_>>()
    }

    fn packet_sample_count(app: &App, client: Entity) -> usize {
        app.world()
            .get::<PlayerAnticheatStatistics>(client)
            .unwrap()
            .packet_cadence
            .snapshot()
            .sample_count
    }

    fn seeded_metric_window() -> MetricWindow {
        let settings =
            SampleWindowSettings::new(NORMAL_WINDOW_CAPACITY, NORMAL_WINDOW_TICKS).unwrap();
        record_sample(
            settings,
            &MetricWindow::default(),
            TimedSample {
                tick: FIRST_TICK,
                value: PACKET_CADENCE_SAMPLE_VALUE,
            },
        )
        .unwrap()
    }

    fn assert_close(actual: f64, expected: f64) {
        let difference = (actual - expected).abs();
        assert!(
            difference <= EPSILON,
            "expected {expected}, got {actual}, difference {difference} exceeds {EPSILON}"
        );
    }

    struct TestConnection;

    impl ClientConnection for TestConnection {
        fn try_send(&mut self, _bytes: BytesMut) -> anyhow::Result<()> {
            Ok(())
        }

        fn try_recv(&mut self) -> anyhow::Result<Option<ReceivedPacket>> {
            Ok(None)
        }

        fn len(&self) -> usize {
            EMPTY_SAMPLE_COUNT
        }
    }
}
