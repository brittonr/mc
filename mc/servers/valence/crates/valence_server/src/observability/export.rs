//! Export planning cores and exporter shell helpers.

use super::{
    ObservabilityLabels, ObservabilityMetricName, ObservabilityRecord, ObservabilityRecordKind,
};

/// Stable counter name emitted when an optional exporter reports failure.
pub const EXPORTER_FAILURE_COUNTER_NAME: &str = "valence.observability.exporter.failure";

pub(crate) const EXPORT_FAILED_DIAGNOSTIC: &str = "observability exporter failed";

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

/// Pure plan for sending one record to an optional exporter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityExportPlan {
    /// Name of the record that should be sent.
    pub record_name: ObservabilityMetricName,
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

/// Purely classifies an optional exporter failure counter record.
pub const fn classify_exporter_failure() -> ObservabilityRecord {
    ObservabilityRecord {
        name: ObservabilityMetricName::ExporterFailure,
        kind: ObservabilityRecordKind::Counter,
        labels: ObservabilityLabels::exporter_failure(),
        value: super::taxonomy::RECORD_INCREMENT,
    }
}

/// Purely plans an exporter attempt for a classified record.
pub const fn plan_observability_export(record: &ObservabilityRecord) -> ObservabilityExportPlan {
    ObservabilityExportPlan {
        record_name: record.name,
    }
}

/// Purely classifies an exporter result into a stable outcome.
pub const fn classify_export_result(
    plan: ObservabilityExportPlan,
    result: Result<(), ObservabilityExportError>,
) -> ObservabilityExportOutcome {
    match result {
        Ok(()) => ObservabilityExportOutcome::Exported,
        Err(error) => ObservabilityExportOutcome::Failed {
            record_name: plan.record_name,
            diagnostic: error.diagnostic(),
        },
    }
}

/// Imperative-shell helper that reports exporter failure without panicking.
pub fn export_observability_record(
    exporter: &mut impl ObservabilityExporter,
    record: &ObservabilityRecord,
) -> ObservabilityExportOutcome {
    let plan = plan_observability_export(record);
    classify_export_result(plan, exporter.export(record))
}
