//! Optional observability hooks for Valence server phases.
//!
//! The classification helpers in this module are deterministic over explicit
//! inputs. [`ObservabilityPlugin`] is a thin Bevy adapter that emits events for
//! selected tick and network boundaries only when the plugin is added.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bytes::Bytes;

use crate::event_loop::{EventLoopPostUpdate, EventLoopPreUpdate, EventLoopUpdate, PacketEvent};

/// Stable span name emitted for tick phase observations.
pub const TICK_PHASE_SPAN_NAME: &str = "valence.tick.phase";
/// Stable counter name emitted for serverbound packet observations.
pub const SERVERBOUND_PACKET_COUNTER_NAME: &str = "valence.network.packet.serverbound";
/// Stable counter name emitted when an optional exporter reports failure.
pub const EXPORTER_FAILURE_COUNTER_NAME: &str = "valence.observability.exporter.failure";

const RECORD_INCREMENT: u64 = 1;
const MIN_VALID_PACKET_ID: i32 = 0;
const REDACTED_LABEL_VALUE: &str = "<redacted>";
const EXPORT_FAILED_DIAGNOSTIC: &str = "observability exporter failed";

/// Optional observability plugin.
///
/// The plugin is not part of Valence default plugins. Add it explicitly when a
/// server wants observability events; otherwise no resources, events, spans, or
/// exporter dependencies are installed by this module.
pub struct ObservabilityPlugin;

impl Plugin for ObservabilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObservabilityConfig>()
            .add_event::<ObservabilityEvent>()
            .add_systems(PreUpdate, emit_pre_update_phase)
            .add_systems(EventLoopPreUpdate, emit_event_loop_pre_update_phase)
            .add_systems(EventLoopUpdate, emit_event_loop_update_phase)
            .add_systems(
                EventLoopPostUpdate,
                (
                    emit_event_loop_post_update_phase,
                    emit_network_packet_records,
                ),
            )
            .add_systems(PostUpdate, emit_post_update_phase);
    }
}

/// Runtime switch for optional observability hooks.
#[derive(Resource, Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityConfig {
    /// Enables all systems in [`ObservabilityPlugin`].
    pub enabled: bool,
    /// Emits bounded tick-phase span records.
    pub emit_tick_phases: bool,
    /// Emits redacted serverbound packet counter records.
    pub emit_network_packets: bool,
}

impl ObservabilityConfig {
    /// Returns a config with all observability hooks disabled.
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            emit_tick_phases: false,
            emit_network_packets: false,
        }
    }
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            emit_tick_phases: true,
            emit_network_packets: true,
        }
    }
}

/// Event emitted by optional observability hooks.
#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityEvent {
    /// Classified span or metric record.
    pub record: ObservabilityRecord,
}

/// Classified observability record with bounded labels.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityRecord {
    /// Stable record name.
    pub name: ObservabilityMetricName,
    /// Whether this record represents a span boundary or counter increment.
    pub kind: ObservabilityRecordKind,
    /// Bounded labels attached to the record.
    pub labels: ObservabilityLabels,
    /// Counter increment or span occurrence count.
    pub value: u64,
}

/// Stable observability record names.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityMetricName {
    /// Tick phase span record.
    TickPhase,
    /// Serverbound packet counter record.
    ServerboundPacket,
    /// Optional exporter failure counter record.
    ExporterFailure,
}

impl ObservabilityMetricName {
    /// Parses a stable metric/span name.
    pub fn parse(name: &str) -> Result<Self, ObservabilityContractError> {
        match name {
            TICK_PHASE_SPAN_NAME => Ok(Self::TickPhase),
            SERVERBOUND_PACKET_COUNTER_NAME => Ok(Self::ServerboundPacket),
            EXPORTER_FAILURE_COUNTER_NAME => Ok(Self::ExporterFailure),
            _ => Err(ObservabilityContractError::UnknownMetricName),
        }
    }

    /// Returns the stable string name for this record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TickPhase => TICK_PHASE_SPAN_NAME,
            Self::ServerboundPacket => SERVERBOUND_PACKET_COUNTER_NAME,
            Self::ExporterFailure => EXPORTER_FAILURE_COUNTER_NAME,
        }
    }
}

/// Contract validation diagnostics.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityContractError {
    /// Metric names must be one of the documented stable names.
    UnknownMetricName,
}

impl ObservabilityContractError {
    /// Stable diagnostic text suitable for fixtures and docs.
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::UnknownMetricName => "unknown observability metric name",
        }
    }
}

/// Record kind for a classified observation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityRecordKind {
    /// Span-style observation for a phase boundary.
    Span,
    /// Counter-style metric increment.
    Counter,
}

/// Bounded subsystem labels.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilitySubsystem {
    /// Tick or schedule phase work.
    Tick,
    /// Network ingress or egress work.
    Network,
    /// Chunk egress or world data work.
    Chunk,
    /// Entity update work.
    Entity,
    /// Plugin adapter work.
    Plugin,
    /// Optional exporter adapter work.
    Exporter,
}

impl ObservabilitySubsystem {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tick => "tick",
            Self::Network => "network",
            Self::Chunk => "chunk",
            Self::Entity => "entity",
            Self::Plugin => "plugin",
            Self::Exporter => "exporter",
        }
    }
}

/// Bounded Valence phase labels selected for initial hooks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityPhase {
    /// Bevy `PreUpdate` phase.
    PreUpdate,
    /// Valence event loop pre-update phase.
    EventLoopPreUpdate,
    /// Valence event loop update phase.
    EventLoopUpdate,
    /// Valence event loop post-update phase.
    EventLoopPostUpdate,
    /// Bevy `PostUpdate` phase.
    PostUpdate,
}

impl ObservabilityPhase {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreUpdate => "pre_update",
            Self::EventLoopPreUpdate => "event_loop_pre_update",
            Self::EventLoopUpdate => "event_loop_update",
            Self::EventLoopPostUpdate => "event_loop_post_update",
            Self::PostUpdate => "post_update",
        }
    }
}

/// Bounded packet direction label.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityDirection {
    /// Client-to-server packet flow.
    Serverbound,
}

impl ObservabilityDirection {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Serverbound => "serverbound",
        }
    }
}

/// Bounded packet ID class.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PacketIdClass {
    /// Packet ID is non-negative and protocol-shaped.
    Known,
    /// Packet ID is outside the protocol-shaped non-negative range.
    Unknown,
}

impl PacketIdClass {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Known => "known",
            Self::Unknown => "unknown",
        }
    }
}

/// Redaction policy applied to a record.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityRedaction {
    /// No sensitive inputs participated in the record.
    None,
    /// Sensitive inputs were present but omitted from labels.
    OmittedSensitiveInput,
    /// A sensitive label value was replaced with the redaction marker.
    RedactedSensitiveInput,
}

impl ObservabilityRedaction {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OmittedSensitiveInput => "omitted_sensitive_input",
            Self::RedactedSensitiveInput => "redacted_sensitive_input",
        }
    }
}

/// Bounded label set for observability records.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityLabels {
    /// Subsystem that produced the record.
    pub subsystem: ObservabilitySubsystem,
    /// Optional phase label for span records.
    pub phase: Option<ObservabilityPhase>,
    /// Optional packet direction label for network records.
    pub direction: Option<ObservabilityDirection>,
    /// Optional packet ID class; raw IDs are intentionally not labels.
    pub packet_id_class: Option<PacketIdClass>,
    /// Redaction policy applied to sensitive inputs.
    pub redaction: ObservabilityRedaction,
}

impl ObservabilityLabels {
    const fn tick(phase: ObservabilityPhase) -> Self {
        Self {
            subsystem: ObservabilitySubsystem::Tick,
            phase: Some(phase),
            direction: None,
            packet_id_class: None,
            redaction: ObservabilityRedaction::None,
        }
    }

    const fn network(packet_id_class: PacketIdClass) -> Self {
        Self {
            subsystem: ObservabilitySubsystem::Network,
            phase: None,
            direction: Some(ObservabilityDirection::Serverbound),
            packet_id_class: Some(packet_id_class),
            redaction: ObservabilityRedaction::OmittedSensitiveInput,
        }
    }

    const fn exporter_failure() -> Self {
        Self {
            subsystem: ObservabilitySubsystem::Exporter,
            phase: None,
            direction: None,
            packet_id_class: None,
            redaction: ObservabilityRedaction::OmittedSensitiveInput,
        }
    }
}

/// Sensitive input categories never copied into public labels.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SensitiveObservabilityField {
    /// Player usernames, UUIDs, or account identifiers.
    PlayerIdentifier,
    /// Socket addresses or connection endpoints.
    SocketAddress,
    /// Raw packet bytes or decoded packet payloads.
    PacketPayload,
    /// Chat, sign, command, or other user-provided text.
    UserText,
}

/// Label value produced after applying redaction policy.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityLabelValue {
    /// Bounded static label value.
    Static(&'static str),
    /// Redacted label value.
    Redacted,
}

impl ObservabilityLabelValue {
    /// Returns the string that may be exported.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static(value) => value,
            Self::Redacted => REDACTED_LABEL_VALUE,
        }
    }
}

/// Purely classifies a tick phase span record.
pub const fn classify_tick_phase(phase: ObservabilityPhase) -> ObservabilityRecord {
    ObservabilityRecord {
        name: ObservabilityMetricName::TickPhase,
        kind: ObservabilityRecordKind::Span,
        labels: ObservabilityLabels::tick(phase),
        value: RECORD_INCREMENT,
    }
}

/// Purely classifies a redacted serverbound packet counter record.
pub fn classify_serverbound_packet(packet_id: i32, _packet_payload: &Bytes) -> ObservabilityRecord {
    let packet_id_class = if packet_id >= MIN_VALID_PACKET_ID {
        PacketIdClass::Known
    } else {
        PacketIdClass::Unknown
    };

    ObservabilityRecord {
        name: ObservabilityMetricName::ServerboundPacket,
        kind: ObservabilityRecordKind::Counter,
        labels: ObservabilityLabels::network(packet_id_class),
        value: RECORD_INCREMENT,
    }
}

/// Purely classifies an optional exporter failure counter record.
pub const fn classify_exporter_failure() -> ObservabilityRecord {
    ObservabilityRecord {
        name: ObservabilityMetricName::ExporterFailure,
        kind: ObservabilityRecordKind::Counter,
        labels: ObservabilityLabels::exporter_failure(),
        value: RECORD_INCREMENT,
    }
}

/// Purely redacts a sensitive input category into an export-safe label value.
pub const fn redact_sensitive_field(
    _field: SensitiveObservabilityField,
) -> ObservabilityLabelValue {
    ObservabilityLabelValue::Redacted
}

/// Optional exporter shell contract.
pub trait ObservabilityExporter {
    /// Exports one classified record.
    fn export(&mut self, record: &ObservabilityRecord) -> Result<(), ObservabilityExportError>;
}

/// Stable exporter failure diagnostic.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityExportError {
    diagnostic: &'static str,
}

impl ObservabilityExportError {
    /// Creates an exporter failure with stable diagnostic text.
    pub const fn failed() -> Self {
        Self {
            diagnostic: EXPORT_FAILED_DIAGNOSTIC,
        }
    }

    /// Returns stable diagnostic text.
    pub const fn diagnostic(self) -> &'static str {
        self.diagnostic
    }
}

/// Outcome of sending a record to an optional exporter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityExportOutcome {
    /// Exporter accepted the record.
    Exported,
    /// Exporter rejected the record; server logic can continue.
    Failed {
        /// Name of the record that failed to export.
        record_name: ObservabilityMetricName,
        /// Stable diagnostic text from the exporter adapter.
        diagnostic: &'static str,
    },
}

/// Imperative-shell helper that reports exporter failure without panicking.
pub fn export_observability_record(
    exporter: &mut impl ObservabilityExporter,
    record: &ObservabilityRecord,
) -> ObservabilityExportOutcome {
    match exporter.export(record) {
        Ok(()) => ObservabilityExportOutcome::Exported,
        Err(error) => ObservabilityExportOutcome::Failed {
            record_name: record.name,
            diagnostic: error.diagnostic(),
        },
    }
}

fn emit_pre_update_phase(
    config: Res<ObservabilityConfig>,
    mut events: EventWriter<ObservabilityEvent>,
) {
    emit_tick_phase_record(&config, &mut events, ObservabilityPhase::PreUpdate);
}

fn emit_event_loop_pre_update_phase(
    config: Res<ObservabilityConfig>,
    mut events: EventWriter<ObservabilityEvent>,
) {
    emit_tick_phase_record(&config, &mut events, ObservabilityPhase::EventLoopPreUpdate);
}

fn emit_event_loop_update_phase(
    config: Res<ObservabilityConfig>,
    mut events: EventWriter<ObservabilityEvent>,
) {
    emit_tick_phase_record(&config, &mut events, ObservabilityPhase::EventLoopUpdate);
}

fn emit_event_loop_post_update_phase(
    config: Res<ObservabilityConfig>,
    mut events: EventWriter<ObservabilityEvent>,
) {
    emit_tick_phase_record(
        &config,
        &mut events,
        ObservabilityPhase::EventLoopPostUpdate,
    );
}

fn emit_post_update_phase(
    config: Res<ObservabilityConfig>,
    mut events: EventWriter<ObservabilityEvent>,
) {
    emit_tick_phase_record(&config, &mut events, ObservabilityPhase::PostUpdate);
}

fn emit_tick_phase_record(
    config: &ObservabilityConfig,
    events: &mut EventWriter<ObservabilityEvent>,
    phase: ObservabilityPhase,
) {
    if config.enabled && config.emit_tick_phases {
        events.send(ObservabilityEvent {
            record: classify_tick_phase(phase),
        });
    }
}

fn emit_network_packet_records(
    config: Res<ObservabilityConfig>,
    mut packet_events: EventReader<PacketEvent>,
    mut observability_events: EventWriter<ObservabilityEvent>,
) {
    if !config.enabled || !config.emit_network_packets {
        packet_events.clear();
        return;
    }

    for packet in packet_events.read() {
        observability_events.send(ObservabilityEvent {
            record: classify_serverbound_packet(packet.id, &packet.data),
        });
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use bevy_ecs::event::Events;

    use super::*;
    use crate::event_loop::EventLoopPlugin;

    const TEST_PACKET_ID: i32 = 7;
    const TEST_PACKET_BYTES: &[u8] = b"sensitive packet payload";
    const UNKNOWN_PACKET_ID: i32 = -1;
    const EXPECTED_TICK_PHASE_COUNT: usize = 5;
    const EXPECTED_PACKET_EVENT_COUNT: usize = 1;
    const EXPECTED_TOTAL_EVENT_COUNT: usize =
        EXPECTED_TICK_PHASE_COUNT + EXPECTED_PACKET_EVENT_COUNT;

    #[test]
    fn disabled_plugin_has_no_observability_effect() {
        let app = App::new();

        assert!(app.world().get_resource::<ObservabilityConfig>().is_none());
        assert!(app
            .world()
            .get_resource::<Events<ObservabilityEvent>>()
            .is_none());
    }

    #[test]
    fn config_disabled_plugin_emits_no_observability_events() {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin)
            .insert_resource(ObservabilityConfig::disabled())
            .add_plugins(ObservabilityPlugin);

        app.update();

        let events = app.world().resource::<Events<ObservabilityEvent>>();
        assert_eq!(events.iter_current_update_events().count(), 0);
    }

    #[test]
    fn tick_phase_classification_has_bounded_labels() {
        let record = classify_tick_phase(ObservabilityPhase::EventLoopUpdate);

        assert_eq!(record.name.as_str(), TICK_PHASE_SPAN_NAME);
        assert_eq!(record.kind, ObservabilityRecordKind::Span);
        assert_eq!(record.labels.subsystem.as_str(), "tick");
        assert_eq!(record.labels.phase.unwrap().as_str(), "event_loop_update");
        assert_eq!(record.labels.redaction, ObservabilityRedaction::None);
        assert_eq!(record.value, RECORD_INCREMENT);
    }

    #[test]
    fn packet_classification_omits_payload_and_bounds_packet_label() {
        let record =
            classify_serverbound_packet(TEST_PACKET_ID, &Bytes::from_static(TEST_PACKET_BYTES));

        assert_eq!(record.name.as_str(), SERVERBOUND_PACKET_COUNTER_NAME);
        assert_eq!(record.kind, ObservabilityRecordKind::Counter);
        assert_eq!(record.labels.subsystem.as_str(), "network");
        assert_eq!(record.labels.direction.unwrap().as_str(), "serverbound");
        assert_eq!(record.labels.packet_id_class.unwrap(), PacketIdClass::Known);
        assert_eq!(
            record.labels.redaction,
            ObservabilityRedaction::OmittedSensitiveInput
        );
    }

    #[test]
    fn unknown_packet_id_is_bounded_to_unknown_class() {
        let record = classify_serverbound_packet(UNKNOWN_PACKET_ID, &Bytes::new());

        assert_eq!(
            record.labels.packet_id_class.unwrap(),
            PacketIdClass::Unknown
        );
    }

    #[test]
    fn sensitive_labels_are_redacted() {
        let fields = [
            SensitiveObservabilityField::PlayerIdentifier,
            SensitiveObservabilityField::SocketAddress,
            SensitiveObservabilityField::PacketPayload,
            SensitiveObservabilityField::UserText,
        ];

        for field in fields {
            assert_eq!(redact_sensitive_field(field).as_str(), REDACTED_LABEL_VALUE);
        }
    }

    #[test]
    fn unknown_metric_names_are_rejected() {
        let error = ObservabilityMetricName::parse("valence.player.username.raw").unwrap_err();

        assert_eq!(error.diagnostic(), "unknown observability metric name");
    }

    #[test]
    fn exporter_failure_reports_outcome_without_panicking() {
        struct FailingExporter;

        impl ObservabilityExporter for FailingExporter {
            fn export(
                &mut self,
                _record: &ObservabilityRecord,
            ) -> Result<(), ObservabilityExportError> {
                Err(ObservabilityExportError::failed())
            }
        }

        let record = classify_tick_phase(ObservabilityPhase::PreUpdate);
        let failure_record = classify_exporter_failure();
        let mut exporter = FailingExporter;

        assert_eq!(
            failure_record.labels.subsystem,
            ObservabilitySubsystem::Exporter
        );
        assert_eq!(
            failure_record.labels.redaction,
            ObservabilityRedaction::OmittedSensitiveInput
        );
        assert_eq!(
            export_observability_record(&mut exporter, &record),
            ObservabilityExportOutcome::Failed {
                record_name: ObservabilityMetricName::TickPhase,
                diagnostic: EXPORT_FAILED_DIAGNOSTIC,
            }
        );
    }

    #[test]
    fn plugin_emits_enabled_tick_and_packet_observations() {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin)
            .add_plugins(ObservabilityPlugin);
        let client = app.world_mut().spawn_empty().id();

        app.world_mut()
            .resource_mut::<Events<PacketEvent>>()
            .send(PacketEvent {
                client,
                timestamp: Instant::now(),
                id: TEST_PACKET_ID,
                data: Bytes::from_static(TEST_PACKET_BYTES),
            });

        app.update();

        let events = app
            .world()
            .resource::<Events<ObservabilityEvent>>()
            .iter_current_update_events()
            .collect::<Vec<_>>();

        assert_eq!(events.len(), EXPECTED_TOTAL_EVENT_COUNT);
        assert!(events.iter().any(|event| {
            event.record.name == ObservabilityMetricName::TickPhase
                && event.record.labels.phase == Some(ObservabilityPhase::EventLoopPreUpdate)
        }));
        assert!(events.iter().any(|event| {
            event.record.name == ObservabilityMetricName::ServerboundPacket
                && event.record.labels.direction == Some(ObservabilityDirection::Serverbound)
        }));
        assert!(app.world().get_entity(client).is_some());
    }
}
