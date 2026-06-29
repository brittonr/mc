#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerDecision {
    WriteMarker,
    MissingMarker,
    ObservePersisted,
}

pub fn evaluate_marker_decision(post_restart: bool, marker_present: bool) -> MarkerDecision {
    if !post_restart {
        return MarkerDecision::WriteMarker;
    }
    if marker_present {
        MarkerDecision::ObservePersisted
    } else {
        MarkerDecision::MissingMarker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_decision_writes_before_restart_observes_present_and_fails_missing() {
        assert_eq!(
            evaluate_marker_decision(false, false),
            MarkerDecision::WriteMarker,
        );
        assert_eq!(
            evaluate_marker_decision(true, true),
            MarkerDecision::ObservePersisted,
        );
        assert_eq!(
            evaluate_marker_decision(true, false),
            MarkerDecision::MissingMarker,
        );
    }
}
