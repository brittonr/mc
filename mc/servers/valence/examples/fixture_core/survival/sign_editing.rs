pub fn live_breadth_milestones(username: &str) -> Vec<String> {
    vec![
        format!(
            "MC-COMPAT-MILESTONE survival_sign_editing_open username={} position=28,64,0 side=front milestone=sign_editor_open_observed",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_sign_editing_update_accepted username={} position=28,64,0 side=front payload=MC|Compat|Sign|Edit milestone=sign_update_accepted_observed",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_sign_editing_state username={} position=28,64,0 side=front payload=MC|Compat|Sign|Edit post_update=text_visible arbitrary_sign_ui=false",
            username,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_USERNAME: &str = "compatbot";

    #[test]
    fn sign_editing_milestones_preserve_bounded_non_claim() {
        let milestones = live_breadth_milestones(TEST_USERNAME);
        assert!(milestones
            .iter()
            .any(|milestone| milestone.contains("sign_update_accepted_observed")));
        assert!(milestones
            .iter()
            .any(|milestone| milestone.contains("arbitrary_sign_ui=false")));
        assert!(!milestones
            .iter()
            .any(|milestone| milestone.contains("full_survival")));
    }
}
