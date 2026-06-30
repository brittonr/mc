use crate::key_value::{
    assert_self_test_fixtures, reject_truthy_overclaims, require_clean_child_revision,
    require_exact_fields, require_ok_fields, require_true_fields, ExpectedField, KeyValueChecker,
    KeyValueRecord, ValidationResult,
};

const CLIENT_REV_KEY: &str = "child.stevenarella.rev";
const CLIENT_STATUS_KEY: &str = "child.stevenarella.status";
const VALENCE_REV_KEY: &str = "child.valence.rev";
const VALENCE_STATUS_KEY: &str = "child.valence.status";
const REQUIRED_EXACT_FIELDS: &[ExpectedField] = &[
    ExpectedField::new("row.id", "scoreboard-team-packet-family"),
    ExpectedField::new("scenario.context", "ctf-spawn-team-balance-reset"),
    ExpectedField::new(
        "evidence.receipt",
        "docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json",
    ),
    ExpectedField::new(
        "packet.row",
        "play/clientbound/0x5a TeamS2CPacket -> Teams_VarInt",
    ),
    ExpectedField::new(
        "packet.inventory.mapping_status",
        "reviewed_override_no_shape_claim",
    ),
    ExpectedField::new(
        "packet.inventory.parser_shape_status",
        "shape_review_missing",
    ),
    ExpectedField::new("packet.inventory.coverage_status", "scenario_bounded"),
    ExpectedField::new(
        "packet.inventory.scenario_evidence",
        "survival_reference_packet_acceptance",
    ),
    ExpectedField::new("team.red.user", "compatbota"),
    ExpectedField::new("team.blue.user", "compatbotb"),
    ExpectedField::new("team.counts", "red=1,blue=1"),
];
const REQUIRED_OK_METRICS: &[&str] = &[
    "metric.client.team_red_observed",
    "metric.client.team_blue_observed",
    "metric.server.red_assignment",
    "metric.server.blue_assignment",
    "metric.server.team_balance",
    "metric.packet.team_update_row_bound",
];
const REQUIRED_NONCLAIMS: &[&str] = &[
    "nonclaim.scoreboard_ui_parity",
    "nonclaim.all_scoreboards",
    "nonclaim.all_team_rules",
    "nonclaim.objective_display_score_variants",
    "nonclaim.full_ctf_correctness",
    "nonclaim.full_protocol_compatibility",
    "nonclaim.public_server_safety",
    "nonclaim.production_readiness",
];
const BROAD_OVERCLAIM_KEYS: &[&str] = &[
    "claim.scoreboard_ui_parity",
    "claim.all_scoreboards",
    "claim.all_team_rules",
    "claim.objective_display_score_variants",
    "claim.full_ctf_correctness",
    "claim.full_protocol_compatibility",
    "claim.public_server_safety",
    "claim.production_readiness",
    "claim.production_ready",
];
const TRUTHY_OVERCLAIM_VALUES: &[&str] = &["true", "yes", "ok", "claimed", "1"];

pub struct ScoreboardTeamPacketFamilyChecker;

type Evidence = KeyValueRecord;

pub fn validate_evidence(evidence: &Evidence) -> ValidationResult {
    let mut diagnostics = Vec::new();
    require_exact_fields(evidence, &mut diagnostics, REQUIRED_EXACT_FIELDS);
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        CLIENT_REV_KEY,
        CLIENT_STATUS_KEY,
    );
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        VALENCE_REV_KEY,
        VALENCE_STATUS_KEY,
    );
    require_ok_fields(evidence, &mut diagnostics, REQUIRED_OK_METRICS);
    require_true_fields(evidence, &mut diagnostics, REQUIRED_NONCLAIMS);
    reject_truthy_overclaims(
        evidence,
        &mut diagnostics,
        BROAD_OVERCLAIM_KEYS,
        TRUTHY_OVERCLAIM_VALUES,
    );

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn valid_fixture() -> String {
    [
        "row.id=scoreboard-team-packet-family",
        "scenario.context=ctf-spawn-team-balance-reset",
        "evidence.receipt=docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json",
        "packet.row=play/clientbound/0x5a TeamS2CPacket -> Teams_VarInt",
        "packet.inventory.mapping_status=reviewed_override_no_shape_claim",
        "packet.inventory.parser_shape_status=shape_review_missing",
        "packet.inventory.coverage_status=scenario_bounded",
        "packet.inventory.scenario_evidence=survival_reference_packet_acceptance",
        "child.stevenarella.rev=d9caec597041b3443d894701591752d23772e5ae",
        "child.stevenarella.status=clean",
        "child.valence.rev=f40d6d6d5aeee300bdaeb406c475e2607ac3b6a7",
        "child.valence.status=clean",
        "team.red.user=compatbota",
        "team.blue.user=compatbotb",
        "team.counts=red=1,blue=1",
        "metric.client.team_red_observed=ok",
        "metric.client.team_blue_observed=ok",
        "metric.server.red_assignment=ok",
        "metric.server.blue_assignment=ok",
        "metric.server.team_balance=ok",
        "metric.packet.team_update_row_bound=ok",
        "nonclaim.scoreboard_ui_parity=true",
        "nonclaim.all_scoreboards=true",
        "nonclaim.all_team_rules=true",
        "nonclaim.objective_display_score_variants=true",
        "nonclaim.full_ctf_correctness=true",
        "nonclaim.full_protocol_compatibility=true",
        "nonclaim.public_server_safety=true",
        "nonclaim.production_readiness=true",
    ]
    .join("\n")
}

fn fixture_with_replacement(old: &str, new: &str) -> String {
    valid_fixture().replace(old, new)
}

pub fn run_self_test() -> Result<(), String> {
    let negative_fixtures: &[(&str, String, &str)] = &[
        (
            "missing row id",
            fixture_with_replacement("row.id=scoreboard-team-packet-family\n", ""),
            "missing row.id",
        ),
        (
            "unsupported packet row",
            fixture_with_replacement(
                "TeamS2CPacket -> Teams_VarInt",
                "ScoreboardObjectiveUpdateS2CPacket -> ScoreboardObjective",
            ),
            "packet.row expected play/clientbound/0x5a TeamS2CPacket",
        ),
        (
            "missing client team observation",
            fixture_with_replacement("metric.client.team_red_observed=ok\n", ""),
            "missing metric.client.team_red_observed",
        ),
        (
            "missing server correlation",
            fixture_with_replacement(
                "metric.server.team_balance=ok",
                "metric.server.team_balance=missing",
            ),
            "metric.server.team_balance expected ok",
        ),
        (
            "stale child revision",
            fixture_with_replacement(
                "child.stevenarella.rev=d9caec597041b3443d894701591752d23772e5ae",
                "child.stevenarella.rev=unknown",
            ),
            "child.stevenarella.rev must be concrete",
        ),
        (
            "pending review child revision",
            fixture_with_replacement(
                "child.stevenarella.rev=d9caec597041b3443d894701591752d23772e5ae",
                "child.stevenarella.rev=pending-review",
            ),
            "child.stevenarella.rev must be concrete",
        ),
        (
            "wrong team count",
            fixture_with_replacement("team.counts=red=1,blue=1", "team.counts=red=2,blue=0"),
            "team.counts expected red=1,blue=1",
        ),
        (
            "missing nonclaim",
            fixture_with_replacement("nonclaim.scoreboard_ui_parity=true\n", ""),
            "missing nonclaim.scoreboard_ui_parity",
        ),
        (
            "broad ui overclaim",
            format!("{}\nclaim.scoreboard_ui_parity=true", valid_fixture()),
            "broad overclaim claim.scoreboard_ui_parity=true",
        ),
        (
            "full ctf overclaim",
            format!("{}\nclaim.full_ctf_correctness=claimed", valid_fixture()),
            "broad overclaim claim.full_ctf_correctness=claimed",
        ),
    ];

    assert_self_test_fixtures(&valid_fixture(), validate_evidence, negative_fixtures)
}

impl KeyValueChecker for ScoreboardTeamPacketFamilyChecker {
    fn usage(&self) -> &'static str {
        "usage: check_scoreboard_team_packet_family (--self-test | <evidence.kv>)"
    }

    fn validate(&self, evidence: &Evidence) -> ValidationResult {
        validate_evidence(evidence)
    }

    fn self_test(&self) -> Result<(), String> {
        run_self_test()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_fixture_passes() {
        let evidence = Evidence::parse(&valid_fixture()).expect("valid fixture parses");
        validate_evidence(&evidence).expect("valid fixture passes");
    }

    #[test]
    fn negative_fixture_rejects_overclaim() {
        let text = format!("{}\nclaim.public_server_safety=yes", valid_fixture());
        let evidence = Evidence::parse(&text).expect("overclaim fixture parses");
        let diagnostics = validate_evidence(&evidence).expect_err("overclaim fails");
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "broad overclaim claim.public_server_safety=yes"),
            "{diagnostics:?}"
        );
    }

    #[test]
    fn self_test_runs_positive_and_negative_fixtures() {
        run_self_test().expect("self-test fixtures pass");
    }
}
