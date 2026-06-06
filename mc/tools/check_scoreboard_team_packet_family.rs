use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process;

const ROW_ID_KEY: &str = "row.id";
const ROW_ID: &str = "scoreboard-team-packet-family";
const SCENARIO_KEY: &str = "scenario.context";
const SCENARIO: &str = "ctf-spawn-team-balance-reset";
const RECEIPT_KEY: &str = "evidence.receipt";
const RECEIPT: &str = "docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json";
const PACKET_ROW_KEY: &str = "packet.row";
const PACKET_ROW: &str = "play/clientbound/0x5a TeamS2CPacket -> Teams_VarInt";
const PACKET_MAPPING_KEY: &str = "packet.inventory.mapping_status";
const PACKET_MAPPING: &str = "reviewed_override_no_shape_claim";
const PACKET_PARSER_KEY: &str = "packet.inventory.parser_shape_status";
const PACKET_PARSER: &str = "shape_review_missing";
const PACKET_COVERAGE_KEY: &str = "packet.inventory.coverage_status";
const PACKET_COVERAGE: &str = "scenario_bounded";
const PACKET_EVIDENCE_KEY: &str = "packet.inventory.scenario_evidence";
const PACKET_EVIDENCE: &str = "survival_reference_packet_acceptance";
const CLIENT_REV_KEY: &str = "child.stevenarella.rev";
const CLIENT_STATUS_KEY: &str = "child.stevenarella.status";
const VALENCE_REV_KEY: &str = "child.valence.rev";
const VALENCE_STATUS_KEY: &str = "child.valence.status";
const CLEAN_STATUS: &str = "clean";
const UNKNOWN_REV: &str = "unknown";
const DRY_RUN_REV: &str = "dry-run";
const PENDING_REVIEW_REV: &str = "pending-review";
const RED_USER_KEY: &str = "team.red.user";
const RED_USER: &str = "compatbota";
const BLUE_USER_KEY: &str = "team.blue.user";
const BLUE_USER: &str = "compatbotb";
const TEAM_COUNTS_KEY: &str = "team.counts";
const TEAM_COUNTS: &str = "red=1,blue=1";
const OK_VALUE: &str = "ok";
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

#[derive(Debug, Clone)]
struct Evidence {
    values: BTreeMap<String, String>,
}

impl Evidence {
    fn parse(text: &str) -> Result<Self, String> {
        let mut values = BTreeMap::new();
        for (index, raw_line) in text.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(format!("line {} is not key=value", index + 1));
            };
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() {
                return Err(format!("line {} has empty key", index + 1));
            }
            if values.insert(key.to_string(), value.to_string()).is_some() {
                return Err(format!("duplicate key {key}"));
            }
        }
        Ok(Self { values })
    }

    fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

fn require_exact(evidence: &Evidence, diagnostics: &mut Vec<String>, key: &str, expected: &str) {
    match evidence.value(key) {
        Some(actual) if actual == expected => {}
        Some(actual) => diagnostics.push(format!("{key} expected {expected}, got {actual}")),
        None => diagnostics.push(format!("missing {key}")),
    }
}

fn require_ok(evidence: &Evidence, diagnostics: &mut Vec<String>, key: &str) {
    require_exact(evidence, diagnostics, key, OK_VALUE);
}

fn require_clean_child_revision(
    evidence: &Evidence,
    diagnostics: &mut Vec<String>,
    rev_key: &str,
    status_key: &str,
) {
    match evidence.value(rev_key) {
        Some(rev)
            if !rev.is_empty()
                && rev != UNKNOWN_REV
                && rev != DRY_RUN_REV
                && rev != PENDING_REVIEW_REV => {}
        Some(rev) => diagnostics.push(format!("{rev_key} must be concrete, got {rev}")),
        None => diagnostics.push(format!("missing {rev_key}")),
    }
    require_exact(evidence, diagnostics, status_key, CLEAN_STATUS);
}

fn reject_broad_overclaims(evidence: &Evidence, diagnostics: &mut Vec<String>) {
    for key in BROAD_OVERCLAIM_KEYS {
        if let Some(value) = evidence.value(key) {
            if TRUTHY_OVERCLAIM_VALUES
                .iter()
                .any(|truthy| value.eq_ignore_ascii_case(truthy))
            {
                diagnostics.push(format!("broad overclaim {key}={value}"));
            }
        }
    }
}

fn validate_evidence(evidence: &Evidence) -> Result<(), Vec<String>> {
    let mut diagnostics = Vec::new();
    require_exact(evidence, &mut diagnostics, ROW_ID_KEY, ROW_ID);
    require_exact(evidence, &mut diagnostics, SCENARIO_KEY, SCENARIO);
    require_exact(evidence, &mut diagnostics, RECEIPT_KEY, RECEIPT);
    require_exact(evidence, &mut diagnostics, PACKET_ROW_KEY, PACKET_ROW);
    require_exact(evidence, &mut diagnostics, PACKET_MAPPING_KEY, PACKET_MAPPING);
    require_exact(evidence, &mut diagnostics, PACKET_PARSER_KEY, PACKET_PARSER);
    require_exact(evidence, &mut diagnostics, PACKET_COVERAGE_KEY, PACKET_COVERAGE);
    require_exact(evidence, &mut diagnostics, PACKET_EVIDENCE_KEY, PACKET_EVIDENCE);
    require_clean_child_revision(evidence, &mut diagnostics, CLIENT_REV_KEY, CLIENT_STATUS_KEY);
    require_clean_child_revision(evidence, &mut diagnostics, VALENCE_REV_KEY, VALENCE_STATUS_KEY);
    require_exact(evidence, &mut diagnostics, RED_USER_KEY, RED_USER);
    require_exact(evidence, &mut diagnostics, BLUE_USER_KEY, BLUE_USER);
    require_exact(evidence, &mut diagnostics, TEAM_COUNTS_KEY, TEAM_COUNTS);
    for key in REQUIRED_OK_METRICS {
        require_ok(evidence, &mut diagnostics, key);
    }
    for key in REQUIRED_NONCLAIMS {
        require_exact(evidence, &mut diagnostics, key, "true");
    }
    reject_broad_overclaims(evidence, &mut diagnostics);

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

fn run_self_test() -> Result<(), String> {
    let valid = Evidence::parse(&valid_fixture())?;
    validate_evidence(&valid).map_err(|diagnostics| diagnostics.join("; "))?;

    let negative_fixtures: &[(&str, String, &str)] = &[
        (
            "missing row id",
            fixture_with_replacement("row.id=scoreboard-team-packet-family\n", ""),
            "missing row.id",
        ),
        (
            "unsupported packet row",
            fixture_with_replacement("TeamS2CPacket -> Teams_VarInt", "ScoreboardObjectiveUpdateS2CPacket -> ScoreboardObjective"),
            "packet.row expected play/clientbound/0x5a TeamS2CPacket",
        ),
        (
            "missing client team observation",
            fixture_with_replacement("metric.client.team_red_observed=ok\n", ""),
            "missing metric.client.team_red_observed",
        ),
        (
            "missing server correlation",
            fixture_with_replacement("metric.server.team_balance=ok", "metric.server.team_balance=missing"),
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

    for (name, text, expected) in negative_fixtures {
        let evidence = Evidence::parse(text).map_err(|err| format!("{name}: parse: {err}"))?;
        match validate_evidence(&evidence) {
            Ok(()) => return Err(format!("{name}: unexpectedly passed")),
            Err(diagnostics) => {
                let rendered = diagnostics.join("; ");
                if !rendered.contains(expected) {
                    return Err(format!(
                        "{name}: expected diagnostic containing {expected:?}, got {rendered}"
                    ));
                }
            }
        }
    }

    Ok(())
}

fn run_path(path: &str) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|err| format!("read {path}: {err}"))?;
    let evidence = Evidence::parse(&text)?;
    validate_evidence(&evidence).map_err(|diagnostics| diagnostics.join("\n"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = if args.iter().any(|arg| arg == "--self-test") {
        run_self_test().map(|()| "self-test ok".to_string())
    } else if let Some(path) = args.first() {
        run_path(path).map(|()| format!("{path}: ok"))
    } else {
        Err("usage: check_scoreboard_team_packet_family (--self-test | <evidence.kv>)".to_string())
    };

    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    }
}
