//! Bevy event and system shells for observability records.

use bevy_ecs::prelude::*;

use super::{
    classify_serverbound_packet, classify_tick_phase, ObservabilityConfig, ObservabilityPhase,
    ObservabilityRecord,
};
use crate::event_loop::PacketEvent;

/// Event emitted by optional observability hooks.
#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityEvent {
    /// Classified span or metric record.
    pub record: ObservabilityRecord,
}

pub(crate) fn observability_tick_phases_enabled(config: Res<ObservabilityConfig>) -> bool {
    config.emits_tick_phase_records()
}

pub(crate) fn emit_pre_update_phase(mut events: EventWriter<ObservabilityEvent>) {
    emit_tick_phase_record(&mut events, ObservabilityPhase::PreUpdate);
}

pub(crate) fn emit_event_loop_pre_update_phase(mut events: EventWriter<ObservabilityEvent>) {
    emit_tick_phase_record(&mut events, ObservabilityPhase::EventLoopPreUpdate);
}

pub(crate) fn emit_event_loop_update_phase(mut events: EventWriter<ObservabilityEvent>) {
    emit_tick_phase_record(&mut events, ObservabilityPhase::EventLoopUpdate);
}

pub(crate) fn emit_event_loop_post_update_phase(mut events: EventWriter<ObservabilityEvent>) {
    emit_tick_phase_record(&mut events, ObservabilityPhase::EventLoopPostUpdate);
}

pub(crate) fn emit_post_update_phase(mut events: EventWriter<ObservabilityEvent>) {
    emit_tick_phase_record(&mut events, ObservabilityPhase::PostUpdate);
}

fn emit_tick_phase_record(events: &mut EventWriter<ObservabilityEvent>, phase: ObservabilityPhase) {
    events.send(ObservabilityEvent {
        record: classify_tick_phase(phase),
    });
}

pub(crate) fn emit_network_packet_records(
    config: Res<ObservabilityConfig>,
    mut packet_events: EventReader<PacketEvent>,
    mut observability_events: EventWriter<ObservabilityEvent>,
) {
    if !config.emits_network_packet_records() {
        packet_events.clear();
        return;
    }

    for packet in packet_events.read() {
        observability_events.send(ObservabilityEvent {
            record: classify_serverbound_packet(packet.id, &packet.data),
        });
    }
}
