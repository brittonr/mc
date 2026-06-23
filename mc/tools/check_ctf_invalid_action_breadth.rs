use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const RECORD_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const CHECKER_NAME: &str = "ctf invalid-action live breadth";
const BLAKE3_HEX_LENGTH: usize = 64;

const MATRIX_DOC: &str =
    "docs/evidence/protocol-763-ctf-invalid-action-breadth-matrix-2026-06-19.md";
const ROW_DOC: &str = "docs/evidence/protocol-763-ctf-invalid-action-live-breadth-2026-06-22.md";
const ROW_RECEIPT: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.receipt.json";
const ROW_RECORD: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.record";
const LIVE_RUN_LOG: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.run.log";
const CHECKER_RUN_LOG: &str =
    "docs/evidence/protocol-763-ctf-invalid-action-live-breadth-checker-2026-06-22.run.log";
const CLIENT_LOG: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.client.log";
const SERVER_LOG: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.server.log";
const TYPED_EVENT_LOG: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.typed-events.log";
const BLAKE3_MANIFEST: &str =
    "docs/evidence/protocol-763-ctf-invalid-action-live-breadth-2026-06-22.b3";
const ACCEPTANCE_MATRIX_DOC: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const CURRENT_BUNDLE_DOC: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const CTF_RULE_LEDGER_DOC: &str = "docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md";

const ROW_ID: &str = "opponent-base-return-drop-without-carrier";
const SCENARIO_NAME: &str = "ctf-invalid-opponent-base-return-drop";
const SEAM_NAME: &str = "CTF invalid-action breadth live";
const CHECK_COMMAND: &str = "tools/check_ctf_invalid_action_breadth.rs";
const WRAPPER_COMMAND: &str = "mc-compat-valence-ctf-invalid-opponent-base-return-drop";
const EVIDENCE_MODE: &str = "bounded-owned-local-live";
const ACTION_FAMILY: &str = "return/drop";
const ACTOR_IDENTITY: &str = "compatbot";
const ACTOR_TEAM: &str = "red";
const FLAG_TEAM: &str = "blue";
const BASE_STATE: &str = "opponent_base";
const PRE_STATE: &str = "at_base";
const POST_STATE: &str = "at_base";
const EXPECTED_REJECTION: &str = "no_flag_state_mutation_no_score";
const INVALID_ACTION: &str = "opponent_base_return_drop_without_carrier";
const POSTCONDITION: &str = "ctf_invalid_opponent_base_return_drop_contained";
const CLIENT_ATTEMPT: &str = "ctf_invalid_opponent_base_return_drop_attempted actor_team=red flag_team=blue pre_state=at_base base=opponent_base action=opponent_base_return_drop_without_carrier expected=no_flag_state_mutation_no_score";
const CLIENT_CONTAINED: &str = "ctf_invalid_opponent_base_return_drop_contained actor_team=red flag_team=blue post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score";
const SERVER_REJECTION: &str = "invalid_opponent_base_return_drop_rejected username=compatbot actor_team=Red flag_team=Blue pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score";
const SERVER_MILESTONE: &str = "server_invalid_opponent_base_return_drop_rejected";
const TARGET_SCOPE: &str = "owned-local-loopback";
const OBSERVED_OUTCOME: &str = "containment_observed";
const RULE_ID: &str = "invalid_opponent_base_return_drop_live_without_state_mutation";

const FORBIDDEN_TRANSITIONS: &[&str] = &[
    "unexpected_flag_pickup",
    "unexpected_server_flag_pickup",
    "unexpected_flag_return",
    "unexpected_flag_capture",
    "unexpected_red_score",
    "unexpected_blue_score",
    "flag_pickup username=compatbot",
    "You have the flag!",
    "RED: 1",
    "BLUE: 1",
];

const CANONICAL_NON_CLAIMS: &[&str] = &[
    "all invalid actions",
    "all flag permutations",
    "full CTF correctness",
    "adversarial security",
    "public-server safety",
    "production readiness",
    "broad Minecraft compatibility",
    "live CTF semantics breadth",
    "vanilla/reference parity",
];

const MATRIX_TOKENS: &[&str] = &[
    ROW_ID,
    SEAM_NAME,
    SCENARIO_NAME,
    ACTION_FAMILY,
    ACTOR_IDENTITY,
    ACTOR_TEAM,
    FLAG_TEAM,
    BASE_STATE,
    PRE_STATE,
    EXPECTED_REJECTION,
    POSTCONDITION,
    CLIENT_ATTEMPT,
    SERVER_MILESTONE,
    EVIDENCE_MODE,
    "selected bounded live row",
    "own-flag-pickup-without-ownership-transfer",
    "own-base-return-drop-without-carrier",
    "all invalid actions remain non-claims",
];

const ROW_DOC_TOKENS: &[&str] = &[
    ROW_ID,
    SEAM_NAME,
    ROW_RECEIPT,
    ROW_RECORD,
    LIVE_RUN_LOG,
    CHECKER_RUN_LOG,
    CLIENT_LOG,
    SERVER_LOG,
    TYPED_EVENT_LOG,
    BLAKE3_MANIFEST,
    CLIENT_ATTEMPT,
    CLIENT_CONTAINED,
    SERVER_REJECTION,
    EVIDENCE_MODE,
    "No all invalid actions",
    "No all flag permutations",
    "No full CTF correctness",
    "No adversarial security",
    "No public-server safety",
    "No production readiness",
    "No broad Minecraft compatibility",
    "No live CTF semantics breadth",
    "No vanilla/reference parity",
];

const RECEIPT_TOKENS: &[&str] = &[
    "mc.compat.scenario.receipt.v2",
    "\"status\": \"pass\"",
    "\"mode\": \"run\"",
    "\"dry_run\": false",
    SCENARIO_NAME,
    POSTCONDITION,
    SERVER_MILESTONE,
    "\"missing_milestones\": []",
    "\"forbidden_matches\": []",
    "\"passed\": true",
    "\"protocol\": 763",
    "\"live_receipt\": true",
    "\"rail\": \"ctf-invalid-opponent-base-return-drop\"",
    "\"invalid_action\": \"opponent_base_return_drop_without_carrier\"",
    "\"observed_outcome\": \"containment_observed\"",
    "\"postcondition_milestone\": \"ctf_invalid_opponent_base_return_drop_contained\"",
    "\"telemetry_present\": true",
    "\"owned_local_target\": true",
    "\"preflight_passed\": true",
    "\"typed_event_oracle\"",
    "\"selected\": true",
    "\"contributes_to_pass_fail\": true",
    "\"claims_correctness\": false",
    "\"claims_semantic_equivalence\": false",
];

const ACCEPTANCE_MATRIX_TOKENS: &[&str] = &[
    SEAM_NAME,
    ROW_RECEIPT,
    ROW_DOC,
    CHECK_COMMAND,
    "bounded owned-local live opponent-base return/drop rejection row",
    "No all invalid actions",
];

const CURRENT_BUNDLE_TOKENS: &[&str] = &[
    SEAM_NAME,
    ROW_ID,
    ROW_RECEIPT,
    CHECK_COMMAND,
    "bounded owned-local live invalid-action breadth row",
    "full CTF correctness remains a non-claim",
];

const CTF_RULE_LEDGER_TOKENS: &[&str] = &[
    RULE_ID,
    SEAM_NAME,
    ROW_ID,
    POSTCONDITION,
    SERVER_MILESTONE,
    ROW_RECEIPT,
    "all invalid actions",
    "full CTF correctness remains a non-claim",
];

const MANIFEST_TOKENS: &[&str] = &[
    MATRIX_DOC,
    ROW_DOC,
    ROW_RECEIPT,
    ROW_RECORD,
    LIVE_RUN_LOG,
    CHECKER_RUN_LOG,
    CLIENT_LOG,
    SERVER_LOG,
    TYPED_EVENT_LOG,
    ACCEPTANCE_MATRIX_DOC,
    CURRENT_BUNDLE_DOC,
    CTF_RULE_LEDGER_DOC,
    "tools/check_ctf_invalid_action_breadth.rs",
];

const FORBIDDEN_OVERCLAIM_TOKENS: &[&str] = &[
    "claims all invalid actions",
    "claims all flag permutations",
    "claims full CTF correctness",
    "claims adversarial security",
    "claims public-server safety",
    "claims production readiness",
    "claims broad Minecraft compatibility",
    "claims live CTF semantics breadth",
    "claims vanilla/reference parity",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatrixExpectation {
    row_id: &'static str,
    scenario_name: &'static str,
    seam_name: &'static str,
    action_family: &'static str,
    actor_identity: &'static str,
    actor_team: &'static str,
    flag_team: &'static str,
    base_state: &'static str,
    pre_state: &'static str,
    post_state: &'static str,
    invalid_action: &'static str,
    expected_rejection: &'static str,
    postcondition: &'static str,
    client_attempt: &'static str,
    client_contained: &'static str,
    server_rejection: &'static str,
    server_milestone: &'static str,
    target_scope: &'static str,
    observed_outcome: &'static str,
    evidence_mode: &'static str,
}

#[derive(Debug, Clone)]
struct EvidenceTexts {
    matrix_doc: String,
    row_doc: String,
    receipt: String,
    record: String,
    live_run_log: String,
    checker_run_log: String,
    client_log: String,
    server_log: String,
    typed_event_log: String,
    manifest: String,
    acceptance_matrix: String,
    current_bundle: String,
    ctf_rule_ledger: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("{CHECKER_NAME} self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match run_repo_check(Path::new(".")) {
        Ok(summary) => {
            println!("{CHECKER_NAME} check passed: {summary}");
            SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("{CHECKER_NAME} check failed: {error}");
    }
}

fn run_repo_check(root: &Path) -> Result<String, Vec<String>> {
    let evidence = load_repo_evidence(root)?;
    validate_evidence(&evidence, matrix_expectation())?;
    Ok("matrix, live row, typed events, manifests, bundle, and ledger validated".to_string())
}

fn load_repo_evidence(root: &Path) -> Result<EvidenceTexts, Vec<String>> {
    Ok(EvidenceTexts {
        matrix_doc: read(root, MATRIX_DOC)?,
        row_doc: read(root, ROW_DOC)?,
        receipt: read(root, ROW_RECEIPT)?,
        record: read(root, ROW_RECORD)?,
        live_run_log: read(root, LIVE_RUN_LOG)?,
        checker_run_log: read(root, CHECKER_RUN_LOG)?,
        client_log: read(root, CLIENT_LOG)?,
        server_log: read(root, SERVER_LOG)?,
        typed_event_log: read(root, TYPED_EVENT_LOG)?,
        manifest: read(root, BLAKE3_MANIFEST)?,
        acceptance_matrix: read(root, ACCEPTANCE_MATRIX_DOC)?,
        current_bundle: read(root, CURRENT_BUNDLE_DOC)?,
        ctf_rule_ledger: read(root, CTF_RULE_LEDGER_DOC)?,
    })
}

fn read(root: &Path, path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(root.join(path)).map_err(|error| vec![format!("{path}: {error}")])
}

fn matrix_expectation() -> MatrixExpectation {
    MatrixExpectation {
        row_id: ROW_ID,
        scenario_name: SCENARIO_NAME,
        seam_name: SEAM_NAME,
        action_family: ACTION_FAMILY,
        actor_identity: ACTOR_IDENTITY,
        actor_team: ACTOR_TEAM,
        flag_team: FLAG_TEAM,
        base_state: BASE_STATE,
        pre_state: PRE_STATE,
        post_state: POST_STATE,
        invalid_action: INVALID_ACTION,
        expected_rejection: EXPECTED_REJECTION,
        postcondition: POSTCONDITION,
        client_attempt: CLIENT_ATTEMPT,
        client_contained: CLIENT_CONTAINED,
        server_rejection: SERVER_REJECTION,
        server_milestone: SERVER_MILESTONE,
        target_scope: TARGET_SCOPE,
        observed_outcome: OBSERVED_OUTCOME,
        evidence_mode: EVIDENCE_MODE,
    }
}

fn validate_evidence(
    evidence: &EvidenceTexts,
    expectation: MatrixExpectation,
) -> Result<(), Vec<String>> {
    let mut issues = Vec::new();
    require_tokens(
        &mut issues,
        "breadth_matrix",
        &evidence.matrix_doc,
        MATRIX_TOKENS,
    );
    require_tokens(&mut issues, "row_doc", &evidence.row_doc, ROW_DOC_TOKENS);
    reject_tokens(
        &mut issues,
        "row_doc",
        &evidence.row_doc,
        FORBIDDEN_OVERCLAIM_TOKENS,
    );
    require_tokens(&mut issues, "receipt", &evidence.receipt, RECEIPT_TOKENS);
    reject_tokens(
        &mut issues,
        "receipt",
        &evidence.receipt,
        FORBIDDEN_OVERCLAIM_TOKENS,
    );
    require_tokens(
        &mut issues,
        "client_log",
        &evidence.client_log,
        &[expectation.client_attempt, expectation.client_contained],
    );
    require_tokens(
        &mut issues,
        "server_log",
        &evidence.server_log,
        &[expectation.server_rejection],
    );
    require_tokens(
        &mut issues,
        "typed_event_log",
        &evidence.typed_event_log,
        &[
            expectation
                .client_attempt
                .split_whitespace()
                .next()
                .unwrap_or(""),
            expectation
                .client_contained
                .split_whitespace()
                .next()
                .unwrap_or(""),
            expectation.server_milestone,
        ],
    );
    reject_tokens(
        &mut issues,
        "client_log",
        &evidence.client_log,
        FORBIDDEN_TRANSITIONS,
    );
    reject_tokens(
        &mut issues,
        "server_log",
        &evidence.server_log,
        FORBIDDEN_TRANSITIONS,
    );
    reject_tokens(
        &mut issues,
        "typed_event_log",
        &evidence.typed_event_log,
        FORBIDDEN_TRANSITIONS,
    );
    validate_record(&mut issues, &evidence.record, expectation);
    require_tokens(
        &mut issues,
        "live_run_log",
        &evidence.live_run_log,
        &[WRAPPER_COMMAND, "exit_status=0", ROW_ID, SCENARIO_NAME],
    );
    require_tokens(
        &mut issues,
        "checker_run_log",
        &evidence.checker_run_log,
        &[CHECK_COMMAND, SELF_TEST_FLAG, "exit_status=0", ROW_ID],
    );
    require_tokens(&mut issues, "manifest", &evidence.manifest, MANIFEST_TOKENS);
    require_blake3_manifest_entries(&mut issues, &evidence.manifest);
    require_tokens(
        &mut issues,
        "acceptance_matrix",
        &evidence.acceptance_matrix,
        ACCEPTANCE_MATRIX_TOKENS,
    );
    require_tokens(
        &mut issues,
        "current_bundle",
        &evidence.current_bundle,
        CURRENT_BUNDLE_TOKENS,
    );
    require_tokens(
        &mut issues,
        "ctf_rule_ledger",
        &evidence.ctf_rule_ledger,
        CTF_RULE_LEDGER_TOKENS,
    );
    reject_tokens(
        &mut issues,
        "current_bundle",
        &evidence.current_bundle,
        FORBIDDEN_OVERCLAIM_TOKENS,
    );

    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

fn require_tokens(issues: &mut Vec<String>, label: &str, text: &str, tokens: &[&str]) {
    for token in tokens {
        if !text.contains(token) {
            issues.push(format!("{label} missing required token: {token}"));
        }
    }
}

fn reject_tokens(issues: &mut Vec<String>, label: &str, text: &str, tokens: &[&str]) {
    for token in tokens {
        if text.contains(token) {
            issues.push(format!(
                "{label} contains forbidden overclaim/transition: {token}"
            ));
        }
    }
}

fn validate_record(issues: &mut Vec<String>, record: &str, expectation: MatrixExpectation) {
    let fields = parse_record(record);
    require_field(issues, &fields, "row_id", expectation.row_id);
    require_field(issues, &fields, "scenario", expectation.scenario_name);
    require_field(issues, &fields, "seam", expectation.seam_name);
    require_field(issues, &fields, "action_family", expectation.action_family);
    require_field(
        issues,
        &fields,
        "actor_identity",
        expectation.actor_identity,
    );
    require_field(issues, &fields, "actor_team", expectation.actor_team);
    require_field(issues, &fields, "flag_team", expectation.flag_team);
    require_field(issues, &fields, "base_state", expectation.base_state);
    require_field(issues, &fields, "pre_state", expectation.pre_state);
    require_field(issues, &fields, "post_state", expectation.post_state);
    require_field(
        issues,
        &fields,
        "invalid_action",
        expectation.invalid_action,
    );
    require_field(
        issues,
        &fields,
        "expected_rejection",
        expectation.expected_rejection,
    );
    require_field(issues, &fields, "postcondition", expectation.postcondition);
    require_field(
        issues,
        &fields,
        "server_milestone",
        expectation.server_milestone,
    );
    require_field(issues, &fields, "target_scope", expectation.target_scope);
    require_field(
        issues,
        &fields,
        "observed_outcome",
        expectation.observed_outcome,
    );
    require_field(issues, &fields, "evidence_mode", expectation.evidence_mode);
    require_field(issues, &fields, "live_traffic_enabled", "true");
    require_field(issues, &fields, "client_server_correlation", "compatbot");
    require_field(issues, &fields, "claims_correctness", "false");
    require_field(issues, &fields, "claims_semantic_equivalence", "false");
    require_field(
        issues,
        &fields,
        "non_claims",
        &CANONICAL_NON_CLAIMS.join(","),
    );
}

fn parse_record(record: &str) -> BTreeMap<&str, &str> {
    let mut fields = BTreeMap::new();
    for line in record.lines() {
        let Some((key, value)) = line.split_once(RECORD_SEPARATOR) else {
            continue;
        };
        fields.insert(key.trim(), value.trim());
    }
    fields
}

fn require_field(
    issues: &mut Vec<String>,
    fields: &BTreeMap<&str, &str>,
    key: &str,
    expected: &str,
) {
    match fields.get(key) {
        Some(actual) if *actual == expected => {}
        Some(actual) => issues.push(format!(
            "record field {key} mismatch: expected {expected}, got {actual}"
        )),
        None => issues.push(format!("record missing field {key}")),
    }
}

fn require_blake3_manifest_entries(issues: &mut Vec<String>, manifest: &str) {
    for (line_index, line) in manifest.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            issues.push(format!(
                "manifest line {} is not '<b3>  <path>': {line}",
                line_index + 1
            ));
            continue;
        }
        let digest = parts[0];
        if digest.len() != BLAKE3_HEX_LENGTH || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
            issues.push(format!(
                "manifest line {} has invalid BLAKE3 digest: {digest}",
                line_index + 1
            ));
        }
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut failures = Vec::new();
    expect_ok(&mut failures, "fixture evidence passes", fixture_evidence());
    expect_failure_contains(
        &mut failures,
        "missing server rejection fails",
        mutate_fixture(|fixture| {
            fixture.server_log = fixture
                .server_log
                .replace(SERVER_REJECTION, "server only booted")
        }),
        "server_log missing required token",
    );
    expect_failure_contains(
        &mut failures,
        "forbidden state mutation fails",
        mutate_fixture(|fixture| {
            fixture
                .server_log
                .push_str("\nMC-COMPAT-MILESTONE flag_pickup username=compatbot carrier_team=Red flag_team=Blue\n")
        }),
        "forbidden overclaim/transition",
    );
    expect_failure_contains(
        &mut failures,
        "wrong actor team correlation fails",
        mutate_fixture(|fixture| {
            fixture.record = fixture.record.replace("actor_team=red", "actor_team=blue")
        }),
        "record field actor_team mismatch",
    );
    expect_failure_contains(
        &mut failures,
        "missing non-claims fail",
        mutate_fixture(|fixture| {
            fixture.row_doc = fixture.row_doc.replace("No all invalid actions", "")
        }),
        "row_doc missing required token: No all invalid actions",
    );
    expect_failure_contains(
        &mut failures,
        "manifest digest shape fails",
        mutate_fixture(|fixture| {
            fixture.manifest = fixture
                .manifest
                .replace(&"a".repeat(BLAKE3_HEX_LENGTH), "abc")
        }),
        "invalid BLAKE3 digest",
    );

    if failures.is_empty() {
        Ok("6 fixtures (positive + negative) passed".to_string())
    } else {
        Err(failures)
    }
}

fn expect_ok(failures: &mut Vec<String>, name: &str, fixture: EvidenceTexts) {
    if let Err(errors) = validate_evidence(&fixture, matrix_expectation()) {
        failures.push(format!("{name}: expected success, got {errors:?}"));
    }
}

fn expect_failure_contains(
    failures: &mut Vec<String>,
    name: &str,
    fixture: EvidenceTexts,
    expected_fragment: &str,
) {
    match validate_evidence(&fixture, matrix_expectation()) {
        Ok(()) => failures.push(format!(
            "{name}: expected failure containing {expected_fragment}"
        )),
        Err(errors) if errors.iter().any(|error| error.contains(expected_fragment)) => {}
        Err(errors) => failures.push(format!(
            "{name}: expected failure containing {expected_fragment}, got {errors:?}"
        )),
    }
}

fn mutate_fixture(mutator: impl FnOnce(&mut EvidenceTexts)) -> EvidenceTexts {
    let mut fixture = fixture_evidence();
    mutator(&mut fixture);
    fixture
}

fn fixture_evidence() -> EvidenceTexts {
    EvidenceTexts {
        matrix_doc: format!(
            "# Matrix\n{ROW_ID}\n{SEAM_NAME}\n{SCENARIO_NAME}\n{ACTION_FAMILY}\n{ACTOR_IDENTITY}\n{ACTOR_TEAM}\n{FLAG_TEAM}\n{BASE_STATE}\n{PRE_STATE}\n{EXPECTED_REJECTION}\n{POSTCONDITION}\n{CLIENT_ATTEMPT}\n{SERVER_MILESTONE}\n{EVIDENCE_MODE}\nselected bounded live row\nown-flag-pickup-without-ownership-transfer\nown-base-return-drop-without-carrier\nall invalid actions remain non-claims\n"
        ),
        row_doc: format!(
            "# Live row\n{ROW_ID}\n{SEAM_NAME}\n{ROW_RECEIPT}\n{ROW_RECORD}\n{LIVE_RUN_LOG}\n{CHECKER_RUN_LOG}\n{CLIENT_LOG}\n{SERVER_LOG}\n{TYPED_EVENT_LOG}\n{BLAKE3_MANIFEST}\n{CLIENT_ATTEMPT}\n{CLIENT_CONTAINED}\n{SERVER_REJECTION}\n{EVIDENCE_MODE}\nNo all invalid actions\nNo all flag permutations\nNo full CTF correctness\nNo adversarial security\nNo public-server safety\nNo production readiness\nNo broad Minecraft compatibility\nNo live CTF semantics breadth\nNo vanilla/reference parity\n"
        ),
        receipt: format!(
            r#"{{
  "schema": "mc.compat.scenario.receipt.v2",
  "status": "pass",
  "mode": "run",
  "dry_run": false,
  "scenario": {{ "name": "{SCENARIO_NAME}" }},
  "client": {{ "passed": true, "missing_milestones": [], "forbidden_matches": [], "observed_milestones": ["{POSTCONDITION}"] }},
  "server": {{ "passed": true, "missing_milestones": [], "forbidden_matches": [], "observed_milestones": ["{SERVER_MILESTONE}"] }},
  "server_target": {{ "version": "1.20.1", "protocol": 763 }},
  "negative_live_rail": {{ "selected": true, "rail": "ctf-invalid-opponent-base-return-drop", "live_receipt": true, "invalid_action": "opponent_base_return_drop_without_carrier", "observed_outcome": "containment_observed", "postcondition_milestone": "ctf_invalid_opponent_base_return_drop_contained", "telemetry_present": true, "owned_local_target": true, "preflight_passed": true }},
  "typed_event_oracle": {{ "selected": true, "contributes_to_pass_fail": true, "event_log_path": "{TYPED_EVENT_LOG}" }},
  "scope": {{ "claims_correctness": false, "claims_semantic_equivalence": false }}
}}"#
        ),
        record: fixture_record(),
        live_run_log: format!("{WRAPPER_COMMAND}\nrow_id={ROW_ID}\nscenario={SCENARIO_NAME}\nexit_status=0\n"),
        checker_run_log: format!("{CHECK_COMMAND} {SELF_TEST_FLAG}\nrow_id={ROW_ID}\nexit_status=0\n"),
        client_log: format!("{CLIENT_ATTEMPT}\n{CLIENT_CONTAINED}\n"),
        server_log: format!("MC-COMPAT-MILESTONE {SERVER_REJECTION}\n"),
        typed_event_log: format!(
            "client event {POSTCONDITION}\nclient event ctf_invalid_opponent_base_return_drop_attempted\nserver event {SERVER_MILESTONE}\n"
        ),
        manifest: fixture_manifest(),
        acceptance_matrix: format!(
            "{SEAM_NAME}\n{ROW_RECEIPT}\n{ROW_DOC}\n{CHECK_COMMAND}\nbounded owned-local live opponent-base return/drop rejection row\nNo all invalid actions\n"
        ),
        current_bundle: format!(
            "{SEAM_NAME}\n{ROW_ID}\n{ROW_RECEIPT}\n{CHECK_COMMAND}\nbounded owned-local live invalid-action breadth row\nfull CTF correctness remains a non-claim\n"
        ),
        ctf_rule_ledger: format!(
            "{RULE_ID}\n{SEAM_NAME}\n{ROW_ID}\n{POSTCONDITION}\n{SERVER_MILESTONE}\n{ROW_RECEIPT}\nall invalid actions\nfull CTF correctness remains a non-claim\n"
        ),
    }
}

fn fixture_record() -> String {
    format!(
        "row_id={ROW_ID}\nscenario={SCENARIO_NAME}\nseam={SEAM_NAME}\naction_family={ACTION_FAMILY}\nactor_identity={ACTOR_IDENTITY}\nactor_team={ACTOR_TEAM}\nflag_team={FLAG_TEAM}\nbase_state={BASE_STATE}\npre_state={PRE_STATE}\npost_state={POST_STATE}\ninvalid_action={INVALID_ACTION}\nexpected_rejection={EXPECTED_REJECTION}\npostcondition={POSTCONDITION}\nserver_milestone={SERVER_MILESTONE}\ntarget_scope={TARGET_SCOPE}\nobserved_outcome={OBSERVED_OUTCOME}\nevidence_mode={EVIDENCE_MODE}\nlive_traffic_enabled=true\nclient_server_correlation=compatbot\nclaims_correctness=false\nclaims_semantic_equivalence=false\nnon_claims={}\n",
        CANONICAL_NON_CLAIMS.join(",")
    )
}

fn fixture_manifest() -> String {
    MANIFEST_TOKENS
        .iter()
        .map(|path| format!("{}  {path}", "a".repeat(BLAKE3_HEX_LENGTH)))
        .collect::<Vec<_>>()
        .join("\n")
}
