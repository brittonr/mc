//! Stable metric taxonomy and record classification cores.

use super::{ObservabilityLabels, ObservabilityPhase};

/// Stable span name emitted for tick phase observations.
pub const TICK_PHASE_SPAN_NAME: &str = "valence.tick.phase";
/// Stable counter name emitted for serverbound packet observations.
pub const SERVERBOUND_PACKET_COUNTER_NAME: &str = "valence.network.packet.serverbound";

pub(crate) const RECORD_INCREMENT: u64 = 1;

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
            super::EXPORTER_FAILURE_COUNTER_NAME => Ok(Self::ExporterFailure),
            _ => Err(ObservabilityContractError::UnknownMetricName),
        }
    }

    /// Returns the stable string name for this record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TickPhase => TICK_PHASE_SPAN_NAME,
            Self::ServerboundPacket => SERVERBOUND_PACKET_COUNTER_NAME,
            Self::ExporterFailure => super::EXPORTER_FAILURE_COUNTER_NAME,
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

/// Purely classifies a tick phase span record.
pub const fn classify_tick_phase(phase: ObservabilityPhase) -> ObservabilityRecord {
    ObservabilityRecord {
        name: ObservabilityMetricName::TickPhase,
        kind: ObservabilityRecordKind::Span,
        labels: ObservabilityLabels::tick(phase),
        value: RECORD_INCREMENT,
    }
}
