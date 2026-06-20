use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const RECORD_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const CHECKER_NAME: &str = "ctf invalid-action breadth";
const BLAKE3_HEX_LENGTH: usize = 64;

const MATRIX_DOC: &str =
    "docs/evidence/protocol-763-ctf-invalid-action-breadth-matrix-2026-06-19.md";
const ROW_DOC: &str = "docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.md";
const ROW_RECEIPT: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.receipt.json";
const ROW_RECORD: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.record";
const RUN_LOG: &str = "docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.run.log";
const CLIENT_LOG: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.client.log";
const SERVER_LOG: &str =
    "docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.server.log";
const BLAKE3_MANIFEST: &str = "docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.b3";
const ACCEPTANCE_MATRIX_DOC: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const CURRENT_BUNDLE_DOC: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const CTF_RULE_LEDGER_DOC: &str = "docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md";

const ROW_ID: &str = "opponent-base-return-drop-without-carrier";
const SCENARIO_NAME: &str = "ctf-invalid-opponent-base-return-drop";
const SEAM_NAME: &str = "CTF invalid-action breadth fixture";
const CHECK_COMMAND: &str = "tools/check_ctf_invalid_action_breadth.rs";
const EVIDENCE_MODE: &str = "deterministic-fixture";
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
const TARGET_SCOPE: &str = "owned-local-loopback-fixture";
const OBSERVED_OUTCOME: &str = "containment_observed";
const RULE_ID: &str = "invalid_opponent_base_return_drop_fixture_without_state_mutation";

const FORBIDDEN_TRANSITIONS: &[&str] = &[
    "unexpected_flag_pickup",
    "unexpected_flag_return",
    "unexpected_flag_capture",
    "unexpected_red_score",
    "unexpected_blue_score",
];

const CANONICAL_NON_CLAIMS: &[&str] = &[
    "all invalid actions",
    "all flag permutations",
    "full CTF correctness",
    "adversarial security",
    "public-server safety",
    "production readiness",
    "broad Minecraft compatibility",
    "live CTF semantics",
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
    "selected additional row",
    "own-flag-pickup-without-ownership-transfer",
    "own-base-return-drop-without-carrier",
    "all invalid actions remain non-claims",
];

const ROW_DOC_TOKENS: &[&str] = &[
    ROW_ID,
    SEAM_NAME,
    ROW_RECEIPT,
    ROW_RECORD,
    RUN_LOG,
    CLIENT_LOG,
    SERVER_LOG,
    BLAKE3_MANIFEST,
    CLIENT_ATTEMPT,
    CLIENT_CONTAINED,
    SERVER_REJECTION,
    "No all invalid actions",
    "No all flag permutations",
    "No full CTF correctness",
    "No adversarial security",
    "No public-server safety",
    "No production readiness",
    "No broad Minecraft compatibility",
    "No live CTF semantics",
];

const RECEIPT_TOKENS: &[&str] = &[
    "mc.compat.invalid-action-breadth.fixture.v1",
    "\"status\": \"pass\"",
    "\"mode\": \"fixture\"",
    "\"dry_run\": true",
    SCENARIO_NAME,
    ROW_ID,
    INVALID_ACTION,
    POSTCONDITION,
    CLIENT_ATTEMPT,
    CLIENT_CONTAINED,
    SERVER_MILESTONE,
    "\"missing_milestones\": []",
    "\"forbidden_matches\": []",
    "\"passed\": true",
    "\"protocol\": 763",
    EVIDENCE_MODE,
    "\"live_traffic_enabled\": false",
    "\"all_invalid_actions_claimed\": false",
    "\"claims_correctness\": false",
    "\"claims_semantic_equivalence\": false",
];

const ACCEPTANCE_MATRIX_TOKENS: &[&str] = &[
    SEAM_NAME,
    ROW_RECEIPT,
    ROW_DOC,
    CHECK_COMMAND,
    "deterministic fixture-only opponent-base return/drop rejection row",
    "No all invalid actions",
];

const CURRENT_BUNDLE_TOKENS: &[&str] = &[
    SEAM_NAME,
    ROW_ID,
    ROW_RECEIPT,
    CHECK_COMMAND,
    "deterministic fixture-only invalid-action breadth row",
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
    RUN_LOG,
    CLIENT_LOG,
    SERVER_LOG,
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
    "claims live CTF semantics",
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
    run_log: String,
    client_log: String,
    server_log: String,
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
    Ok("matrix, fixture row, manifests, bundle, and ledger validated".to_string())
}

fn load_repo_evidence(root: &Path) -> Result<EvidenceTexts, Vec<String>> {
    Ok(EvidenceTexts {
        matrix_doc: read(root, MATRIX_DOC)?,
        row_doc: read(root, ROW_DOC)?,
        receipt: read(root, ROW_RECEIPT)?,
        record: read(root, ROW_RECORD)?,
        run_log: read(root, RUN_LOG)?,
        client_log: read(root, CLIENT_LOG)?,
        server_log: read(root, SERVER_LOG)?,
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
    validate_record(&mut issues, &evidence.record, expectation);
    require_tokens(
        &mut issues,
        "run_log",
        &evidence.run_log,
        &[CHECK_COMMAND, "--self-test", "exit_status=0", ROW_ID],
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
    for non_claim in CANONICAL_NON_CLAIMS {
        let token = format!("No {non_claim}");
        if !evidence.row_doc.contains(&token) {
            issues.push(format!("row_doc missing canonical non-claim {token:?}"));
        }
    }

    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

fn validate_record(issues: &mut Vec<String>, record: &str, expectation: MatrixExpectation) {
    let fields = parse_record(record);
    for (key, expected) in [
        ("row_id", expectation.row_id),
        ("scenario", expectation.scenario_name),
        ("seam", expectation.seam_name),
        ("action_family", expectation.action_family),
        ("actor_identity", expectation.actor_identity),
        ("actor_team", expectation.actor_team),
        ("flag_team", expectation.flag_team),
        ("base_state", expectation.base_state),
        ("pre_state", expectation.pre_state),
        ("post_state", expectation.post_state),
        ("invalid_action", expectation.invalid_action),
        ("expected_rejection", expectation.expected_rejection),
        ("postcondition", expectation.postcondition),
        ("client_attempt", expectation.client_attempt),
        ("client_contained", expectation.client_contained),
        ("server_rejection", expectation.server_rejection),
        ("server_milestone", expectation.server_milestone),
        ("target_scope", expectation.target_scope),
        ("observed_outcome", expectation.observed_outcome),
        ("evidence_mode", expectation.evidence_mode),
        ("receipt", ROW_RECEIPT),
        ("matrix", MATRIX_DOC),
    ] {
        match fields.get(key) {
            Some(actual) if actual == expected => {}
            Some(actual) => issues.push(format!(
                "record {key} expected {expected:?}, found {actual:?}"
            )),
            None => issues.push(format!("record missing key {key}")),
        }
    }
    let forbidden = fields
        .get("forbidden_absent")
        .map(String::as_str)
        .unwrap_or("");
    for transition in FORBIDDEN_TRANSITIONS {
        if !forbidden.contains(transition) {
            issues.push(format!(
                "record forbidden_absent missing transition {transition:?}"
            ));
        }
    }
}

fn parse_record(record: &str) -> BTreeMap<String, String> {
    record
        .lines()
        .filter_map(|line| line.split_once(RECORD_SEPARATOR))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
}

fn require_tokens(issues: &mut Vec<String>, label: &str, haystack: &str, tokens: &[&str]) {
    for token in tokens {
        if !haystack.contains(token) {
            issues.push(format!("{label} missing token {token:?}"));
        }
    }
}

fn reject_tokens(issues: &mut Vec<String>, label: &str, haystack: &str, tokens: &[&str]) {
    for token in tokens {
        if haystack.contains(token) {
            issues.push(format!(
                "{label} contains forbidden overclaim token {token:?}"
            ));
        }
    }
}

fn require_blake3_manifest_entries(issues: &mut Vec<String>, manifest: &str) {
    for line in manifest.lines().filter(|line| !line.trim().is_empty()) {
        let Some((digest, _path)) = line.split_once("  ") else {
            issues.push(format!("manifest row is not a b3sum row: {line:?}"));
            continue;
        };
        if !is_blake3_digest(digest) {
            issues.push(format!("manifest row has invalid BLAKE3 digest: {line:?}"));
        }
    }
}

fn is_blake3_digest(value: &str) -> bool {
    value.len() == BLAKE3_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let evidence = fixture_evidence();
    validate_evidence(&evidence, matrix_expectation())?;

    let mut missing_row = evidence.clone();
    missing_row.record = missing_row
        .record
        .replace("row_id=opponent-base-return-drop-without-carrier\n", "");
    assert_contains(
        &validate_evidence(&missing_row, matrix_expectation())
            .expect_err("missing row id should fail"),
        "record missing key",
    )?;

    let mut mismatched_team = evidence.clone();
    mismatched_team.record = mismatched_team
        .record
        .replace("actor_team=red", "actor_team=blue");
    assert_contains(
        &validate_evidence(&mismatched_team, matrix_expectation())
            .expect_err("mismatched actor team should fail"),
        "record actor_team",
    )?;

    let mut missing_server = evidence.clone();
    missing_server.server_log = missing_server
        .server_log
        .replace(SERVER_REJECTION, "missing server rejection");
    assert_contains(
        &validate_evidence(&missing_server, matrix_expectation())
            .expect_err("missing server evidence should fail"),
        "server_log missing token",
    )?;

    let mut forbidden_transition = evidence.clone();
    forbidden_transition
        .server_log
        .push_str("\nunexpected_flag_return\n");
    assert_contains(
        &validate_evidence(&forbidden_transition, matrix_expectation())
            .expect_err("forbidden transition should fail"),
        "forbidden overclaim token",
    )?;

    let mut overclaim = evidence.clone();
    overclaim
        .row_doc
        .push_str("\nclaims full CTF correctness\n");
    assert_contains(
        &validate_evidence(&overclaim, matrix_expectation()).expect_err("overclaim should fail"),
        "forbidden overclaim token",
    )?;

    let mut malformed_manifest = evidence;
    malformed_manifest.manifest = malformed_manifest
        .manifest
        .replace(&"0".repeat(BLAKE3_HEX_LENGTH), "not-a-digest");
    assert_contains(
        &validate_evidence(&malformed_manifest, matrix_expectation())
            .expect_err("malformed manifest should fail"),
        "invalid BLAKE3",
    )?;

    Ok(
        "positive, missing, mismatched, forbidden, overclaim, and manifest fixtures exercised"
            .to_string(),
    )
}

fn assert_contains(errors: &[String], needle: &str) -> Result<(), Vec<String>> {
    if errors.iter().any(|error| error.contains(needle)) {
        Ok(())
    } else {
        Err(vec![format!(
            "missing expected diagnostic {needle:?}: {errors:?}"
        )])
    }
}

fn fixture_evidence() -> EvidenceTexts {
    let digest = "0".repeat(BLAKE3_HEX_LENGTH);
    EvidenceTexts {
        matrix_doc: format!(
            "{ROW_ID}\n{SEAM_NAME}\n{SCENARIO_NAME}\n{ACTION_FAMILY}\n{ACTOR_IDENTITY}\n{ACTOR_TEAM}\n{FLAG_TEAM}\n{BASE_STATE}\n{PRE_STATE}\n{EXPECTED_REJECTION}\n{POSTCONDITION}\n{CLIENT_ATTEMPT}\n{SERVER_MILESTONE}\n{EVIDENCE_MODE}\nselected additional row\nown-flag-pickup-without-ownership-transfer\nown-base-return-drop-without-carrier\nall invalid actions remain non-claims\n"
        ),
        row_doc: format!(
            "{ROW_ID}\n{SEAM_NAME}\n{ROW_RECEIPT}\n{ROW_RECORD}\n{RUN_LOG}\n{CLIENT_LOG}\n{SERVER_LOG}\n{BLAKE3_MANIFEST}\n{CLIENT_ATTEMPT}\n{CLIENT_CONTAINED}\n{SERVER_REJECTION}\nNo all invalid actions\nNo all flag permutations\nNo full CTF correctness\nNo adversarial security\nNo public-server safety\nNo production readiness\nNo broad Minecraft compatibility\nNo live CTF semantics\n"
        ),
        receipt: format!(
            "mc.compat.invalid-action-breadth.fixture.v1\n\"status\": \"pass\"\n\"mode\": \"fixture\"\n\"dry_run\": true\n{SCENARIO_NAME}\n{ROW_ID}\n{INVALID_ACTION}\n{POSTCONDITION}\n{CLIENT_ATTEMPT}\n{CLIENT_CONTAINED}\n{SERVER_MILESTONE}\n\"missing_milestones\": []\n\"forbidden_matches\": []\n\"passed\": true\n\"protocol\": 763\n{EVIDENCE_MODE}\n\"live_traffic_enabled\": false\n\"all_invalid_actions_claimed\": false\n\"claims_correctness\": false\n\"claims_semantic_equivalence\": false\n"
        ),
        record: fixture_record(),
        run_log: format!("{CHECK_COMMAND} --self-test\nexit_status=0\n{ROW_ID}\n"),
        client_log: format!("{CLIENT_ATTEMPT}\n{CLIENT_CONTAINED}\n"),
        server_log: format!("{SERVER_REJECTION}\n"),
        manifest: format!(
            "{digest}  {MATRIX_DOC}\n{digest}  {ROW_DOC}\n{digest}  {ROW_RECEIPT}\n{digest}  {ROW_RECORD}\n{digest}  {RUN_LOG}\n{digest}  {CLIENT_LOG}\n{digest}  {SERVER_LOG}\n{digest}  {ACCEPTANCE_MATRIX_DOC}\n{digest}  {CURRENT_BUNDLE_DOC}\n{digest}  {CTF_RULE_LEDGER_DOC}\n{digest}  tools/check_ctf_invalid_action_breadth.rs\n"
        ),
        acceptance_matrix: format!(
            "{SEAM_NAME}\n{ROW_RECEIPT}\n{ROW_DOC}\n{CHECK_COMMAND}\ndeterministic fixture-only opponent-base return/drop rejection row\nNo all invalid actions\n"
        ),
        current_bundle: format!(
            "{SEAM_NAME}\n{ROW_ID}\n{ROW_RECEIPT}\n{CHECK_COMMAND}\ndeterministic fixture-only invalid-action breadth row\nfull CTF correctness remains a non-claim\n"
        ),
        ctf_rule_ledger: format!(
            "{RULE_ID}\n{SEAM_NAME}\n{ROW_ID}\n{POSTCONDITION}\n{SERVER_MILESTONE}\n{ROW_RECEIPT}\nall invalid actions\nfull CTF correctness remains a non-claim\n"
        ),
    }
}

fn fixture_record() -> String {
    format!(
        "row_id={ROW_ID}\nscenario={SCENARIO_NAME}\nseam={SEAM_NAME}\naction_family={ACTION_FAMILY}\nactor_identity={ACTOR_IDENTITY}\nactor_team={ACTOR_TEAM}\nflag_team={FLAG_TEAM}\nbase_state={BASE_STATE}\npre_state={PRE_STATE}\npost_state={POST_STATE}\ninvalid_action={INVALID_ACTION}\nexpected_rejection={EXPECTED_REJECTION}\npostcondition={POSTCONDITION}\nclient_attempt={CLIENT_ATTEMPT}\nclient_contained={CLIENT_CONTAINED}\nserver_rejection={SERVER_REJECTION}\nserver_milestone={SERVER_MILESTONE}\ntarget_scope={TARGET_SCOPE}\nobserved_outcome={OBSERVED_OUTCOME}\nevidence_mode={EVIDENCE_MODE}\nforbidden_absent=unexpected_flag_pickup,unexpected_flag_return,unexpected_flag_capture,unexpected_red_score,unexpected_blue_score\nreceipt={ROW_RECEIPT}\nmatrix={MATRIX_DOC}\n"
    )
}
