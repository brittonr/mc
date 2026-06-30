//! Packet classification cores for observability records.

use bytes::Bytes;

use super::{
    ObservabilityLabels, ObservabilityMetricName, ObservabilityRecord, ObservabilityRecordKind,
    PacketIdClass,
};

const MIN_VALID_PACKET_ID: i32 = 0;

/// Purely classifies a serverbound packet ID into a bounded label class.
pub const fn classify_packet_id(packet_id: i32) -> PacketIdClass {
    if packet_id >= MIN_VALID_PACKET_ID {
        PacketIdClass::Known
    } else {
        PacketIdClass::Unknown
    }
}

/// Purely classifies a redacted serverbound packet counter record.
pub fn classify_serverbound_packet(packet_id: i32, _packet_payload: &Bytes) -> ObservabilityRecord {
    ObservabilityRecord {
        name: ObservabilityMetricName::ServerboundPacket,
        kind: ObservabilityRecordKind::Counter,
        labels: ObservabilityLabels::network(classify_packet_id(packet_id)),
        value: super::taxonomy::RECORD_INCREMENT,
    }
}
