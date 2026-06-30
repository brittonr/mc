//! Optional observability hooks for Valence server phases.
//!
//! Core classification and redaction helpers are deterministic over explicit
//! inputs. [`ObservabilityPlugin`] is a thin Bevy adapter that emits events for
//! selected tick and network boundaries only when the plugin is added.
//!
//! Disabled-mode contracts are intentionally per system. Tick-phase systems have
//! no event cursors, so Bevy run conditions skip their bodies when disabled.
//! Packet observation reads [`PacketEvent`] values, so it keeps an explicit
//! in-system drain guard; disabled updates advance that reader and do not replay
//! stale packets after re-enable.
//!
//! [`PacketEvent`]: crate::event_loop::PacketEvent

mod config;
mod events;
mod export;
mod labels;
mod packets;
mod taxonomy;

use bevy_app::prelude::*;
use bevy_ecs::schedule::IntoSystemConfigs;

use crate::event_loop::{EventLoopPostUpdate, EventLoopPreUpdate, EventLoopSet, EventLoopUpdate};

pub use config::ObservabilityConfig;
pub use events::ObservabilityEvent;
use events::{
    emit_event_loop_post_update_phase, emit_event_loop_pre_update_phase,
    emit_event_loop_update_phase, emit_network_packet_records, emit_post_update_phase,
    emit_pre_update_phase, observability_tick_phases_enabled,
};
pub use export::{
    classify_export_result, classify_exporter_failure, export_observability_record,
    plan_observability_export, ObservabilityExportError, ObservabilityExportOutcome,
    ObservabilityExportPlan, ObservabilityExporter, EXPORTER_FAILURE_COUNTER_NAME,
};
pub use labels::{
    redact_sensitive_field, ObservabilityDirection, ObservabilityLabelError,
    ObservabilityLabelValue, ObservabilityLabels, ObservabilityPhase, ObservabilityRedaction,
    ObservabilitySubsystem, PacketIdClass, SensitiveObservabilityField, REDACTED_LABEL_VALUE,
};
pub use packets::{classify_packet_id, classify_serverbound_packet};
pub use taxonomy::{
    classify_tick_phase, ObservabilityContractError, ObservabilityMetricName, ObservabilityRecord,
    ObservabilityRecordKind, SERVERBOUND_PACKET_COUNTER_NAME, TICK_PHASE_SPAN_NAME,
};

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
            .add_systems(
                PreUpdate,
                emit_pre_update_phase.run_if(observability_tick_phases_enabled),
            )
            .add_systems(
                EventLoopPreUpdate,
                emit_event_loop_pre_update_phase
                    .in_set(EventLoopSet::Diagnostics)
                    .run_if(observability_tick_phases_enabled),
            )
            .add_systems(
                EventLoopUpdate,
                emit_event_loop_update_phase
                    .in_set(EventLoopSet::Diagnostics)
                    .run_if(observability_tick_phases_enabled),
            )
            .add_systems(
                EventLoopPostUpdate,
                (
                    emit_event_loop_post_update_phase
                        .in_set(EventLoopSet::Diagnostics)
                        .run_if(observability_tick_phases_enabled),
                    emit_network_packet_records.in_set(EventLoopSet::Diagnostics),
                ),
            )
            .add_systems(
                PostUpdate,
                emit_post_update_phase.run_if(observability_tick_phases_enabled),
            );
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use bevy_ecs::event::Events;
    use bytes::Bytes;

    use super::*;
    use crate::event_loop::{EventLoopPlugin, PacketEvent};

    const TEST_PACKET_ID: i32 = 7;
    const TEST_PACKET_BYTES: &[u8] = b"sensitive packet payload";
    const UNKNOWN_PACKET_ID: i32 = -1;
    const EXPECTED_TICK_PHASE_COUNT: usize = 5;
    const EXPECTED_PACKET_EVENT_COUNT: usize = 1;
    const NO_OBSERVABILITY_EVENT_COUNT: usize = 0;
    const EXPECTED_TOTAL_EVENT_COUNT: usize =
        EXPECTED_TICK_PHASE_COUNT + EXPECTED_PACKET_EVENT_COUNT;
    const INVALID_EMPTY_LABEL: &str = "";
    const INVALID_REDACTION_MARKER_LABEL: &str = REDACTED_LABEL_VALUE;
    const INVALID_DELIMITED_LABEL: &str = "tick,network";
    const INVALID_KEY_VALUE_LABEL: &str = "subsystem=tick";

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
    fn missing_config_without_plugin_has_no_observability_effect() {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin);
        send_packet_event(&mut app, TEST_PACKET_ID);

        app.update();

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
        assert_eq!(
            events.iter_current_update_events().count(),
            NO_OBSERVABILITY_EVENT_COUNT
        );
    }

    #[test]
    fn config_predicates_expose_enabled_and_disabled_contracts() {
        let enabled = ObservabilityConfig::default();
        let disabled = ObservabilityConfig::disabled();
        let tick_only = ObservabilityConfig {
            enabled: true,
            emit_tick_phases: true,
            emit_network_packets: false,
        };

        assert!(enabled.emits_tick_phase_records());
        assert!(enabled.emits_network_packet_records());
        assert!(!disabled.emits_tick_phase_records());
        assert!(!disabled.emits_network_packet_records());
        assert!(tick_only.emits_tick_phase_records());
        assert!(!tick_only.emits_network_packet_records());
    }

    #[test]
    fn tick_phase_run_conditions_follow_runtime_toggle() {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin)
            .insert_resource(ObservabilityConfig {
                enabled: true,
                emit_tick_phases: false,
                emit_network_packets: false,
            })
            .add_plugins(ObservabilityPlugin);

        app.update();
        assert_eq!(
            current_observability_records(&app).len(),
            NO_OBSERVABILITY_EVENT_COUNT
        );

        app.world_mut()
            .resource_mut::<ObservabilityConfig>()
            .emit_tick_phases = true;
        app.update();
        let enabled_records = current_observability_records(&app);
        assert_eq!(enabled_records.len(), EXPECTED_TICK_PHASE_COUNT);
        assert!(enabled_records
            .iter()
            .all(|record| record.name == ObservabilityMetricName::TickPhase));

        app.world_mut()
            .resource_mut::<ObservabilityConfig>()
            .enabled = false;
        app.update();
        assert_eq!(
            current_observability_records(&app).len(),
            NO_OBSERVABILITY_EVENT_COUNT
        );
    }

    #[test]
    fn network_reader_drains_disabled_period_packets_before_reenable() {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin)
            .insert_resource(ObservabilityConfig {
                enabled: true,
                emit_tick_phases: false,
                emit_network_packets: false,
            })
            .add_plugins(ObservabilityPlugin);

        send_packet_event(&mut app, TEST_PACKET_ID);
        app.update();
        assert_eq!(
            current_network_record_count(&app),
            NO_OBSERVABILITY_EVENT_COUNT
        );

        app.world_mut()
            .resource_mut::<ObservabilityConfig>()
            .emit_network_packets = true;
        app.update();
        assert_eq!(
            current_network_record_count(&app),
            NO_OBSERVABILITY_EVENT_COUNT
        );

        send_packet_event(&mut app, TEST_PACKET_ID);
        app.update();
        assert_eq!(
            current_network_record_count(&app),
            EXPECTED_PACKET_EVENT_COUNT
        );
    }

    #[test]
    fn tick_phase_classification_has_bounded_labels() {
        let record = classify_tick_phase(ObservabilityPhase::EventLoopUpdate);

        assert_eq!(record.name.as_str(), TICK_PHASE_SPAN_NAME);
        assert_eq!(record.kind, ObservabilityRecordKind::Span);
        assert_eq!(record.labels.subsystem.as_str(), "tick");
        assert_eq!(record.labels.phase.unwrap().as_str(), "event_loop_update");
        assert_eq!(record.labels.redaction, ObservabilityRedaction::None);
        assert_eq!(record.value, taxonomy::RECORD_INCREMENT);
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
            classify_packet_id(UNKNOWN_PACKET_ID),
            PacketIdClass::Unknown
        );
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
    fn public_static_label_values_accept_bounded_values() {
        let tick =
            ObservabilityLabelValue::public_static(ObservabilitySubsystem::Tick.as_str()).unwrap();
        let phase = ObservabilityLabelValue::public_static(ObservabilityPhase::PostUpdate.as_str())
            .unwrap();

        assert_eq!(tick.as_str(), "tick");
        assert_eq!(phase.as_str(), "post_update");
    }

    #[test]
    fn invalid_label_values_are_rejected() {
        assert_eq!(
            ObservabilityLabelValue::public_static(INVALID_EMPTY_LABEL).unwrap_err(),
            ObservabilityLabelError::Empty
        );
        assert_eq!(
            ObservabilityLabelValue::public_static(INVALID_REDACTION_MARKER_LABEL).unwrap_err(),
            ObservabilityLabelError::ReservedRedactionMarker
        );
        assert_eq!(
            ObservabilityLabelValue::public_static(INVALID_DELIMITED_LABEL).unwrap_err(),
            ObservabilityLabelError::UnsafeDelimiter
        );
        assert_eq!(
            ObservabilityLabelValue::public_static(INVALID_KEY_VALUE_LABEL).unwrap_err(),
            ObservabilityLabelError::UnsafeDelimiter
        );
    }

    #[test]
    fn unknown_metric_names_are_rejected() {
        let error = ObservabilityMetricName::parse("valence.player.username.raw").unwrap_err();

        assert_eq!(error.diagnostic(), "unknown observability metric name");
    }

    #[test]
    fn export_planning_records_metric_names_without_shell_side_effects() {
        let record = classify_tick_phase(ObservabilityPhase::PreUpdate);
        let plan = plan_observability_export(&record);

        assert_eq!(plan.record_name, ObservabilityMetricName::TickPhase);
        assert_eq!(
            classify_export_result(plan, Ok(())),
            ObservabilityExportOutcome::Exported
        );
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
                diagnostic: export::EXPORT_FAILED_DIAGNOSTIC,
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

    fn send_packet_event(app: &mut App, packet_id: i32) {
        let client = app.world_mut().spawn_empty().id();
        app.world_mut()
            .resource_mut::<Events<PacketEvent>>()
            .send(PacketEvent {
                client,
                timestamp: Instant::now(),
                id: packet_id,
                data: Bytes::from_static(TEST_PACKET_BYTES),
            });
    }

    fn current_network_record_count(app: &App) -> usize {
        current_observability_records(app)
            .iter()
            .filter(|record| record.name == ObservabilityMetricName::ServerboundPacket)
            .count()
    }

    fn current_observability_records(app: &App) -> Vec<ObservabilityRecord> {
        app.world()
            .resource::<Events<ObservabilityEvent>>()
            .iter_current_update_events()
            .map(|event| event.record)
            .collect()
    }
}
