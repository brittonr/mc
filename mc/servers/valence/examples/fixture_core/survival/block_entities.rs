use super::persistence::{evaluate_marker_decision, MarkerDecision};

pub fn should_place_block_entity_sign(post_restart: bool, marker_present: bool) -> bool {
    matches!(
        evaluate_marker_decision(post_restart, marker_present),
        MarkerDecision::WriteMarker | MarkerDecision::ObservePersisted
    )
}

pub fn validate_block_entity_payload(
    lines: &[&str],
    expected_payload: &str,
    separator: &str,
    expected_line_count: usize,
) -> Result<(), &'static str> {
    if lines.len() != expected_line_count {
        return Err("line_count");
    }
    let payload = lines.join(separator);
    if payload != expected_payload {
        return Err("payload");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIGN_LINE_COUNT: usize = 4;
    const SIGN_PAYLOAD: &str = "MC|Compat|Sign|Persist";
    const SIGN_SEPARATOR: &str = "|";

    #[test]
    fn sign_placement_accepts_initial_or_observed_persisted_and_rejects_missing_restart() {
        assert!(should_place_block_entity_sign(false, false));
        assert!(should_place_block_entity_sign(true, true));
        assert!(!should_place_block_entity_sign(true, false));
    }

    #[test]
    fn sign_payload_requires_exact_line_count_and_payload() {
        assert_eq!(
            validate_block_entity_payload(
                &["MC", "Compat", "Sign", "Persist"],
                SIGN_PAYLOAD,
                SIGN_SEPARATOR,
                SIGN_LINE_COUNT,
            ),
            Ok(()),
        );
        assert_eq!(
            validate_block_entity_payload(
                &["MC", "Compat"],
                SIGN_PAYLOAD,
                SIGN_SEPARATOR,
                SIGN_LINE_COUNT,
            ),
            Err("line_count"),
        );
        assert_eq!(
            validate_block_entity_payload(
                &["MC", "Compat", "Sign", "Edit"],
                SIGN_PAYLOAD,
                SIGN_SEPARATOR,
                SIGN_LINE_COUNT,
            ),
            Err("payload"),
        );
    }
}
