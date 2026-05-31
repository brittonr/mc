use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const RECORD_FLAG: &str = "--record";
const KEY_VALUE_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const BLAKE3_HEX_LENGTH: usize = 64;
const FLAG_VALUE_WINDOW: usize = 2;

const CONTRACT_DOC: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-contract-2026-05-30.md";
const ROW_DOC: &str = "docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.md";
const SUMMARY_RECEIPT: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.receipt.json";
const RED_RECEIPT: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.receipt.json";
const RED_RUN_LOG: &str = "docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.run.log";
const RED_CLIENT_LOG: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.client.log";
const RED_SERVER_LOG: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.server.log";
const BLUE_RECEIPT: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.receipt.json";
const BLUE_RUN_LOG: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.run.log";
const BLUE_CLIENT_LOG: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.client.log";
const BLUE_SERVER_LOG: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.server.log";
const ROW_RECORD: &str =
    "docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.record";
const ACCEPTANCE_MATRIX_DOC: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const CURRENT_BUNDLE_DOC: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const CTF_RULE_LEDGER_DOC: &str = "docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md";

const EXPECTED_SEAM: &str = "RED/BLUE scoring soak";
const EXPECTED_REFRESH: &str = "RED/BLUE scoring soak freshness";
const EXPECTED_RED_SCENARIO: &str = "multi-client-load-score";
const EXPECTED_BLUE_SCENARIO: &str = "blue-flag-score";
const EXPECTED_RED_COMMAND: &str = "nix run .#mc-compat-valence-ctf-600s-soak";
const EXPECTED_BLUE_COMMAND: &str = "nix run .#mc-compat-valence-ctf-blue-600s-soak";
const EXPECTED_SCHEMA: &str = "mc.compat.scenario.receipt.v2";
const EXPECTED_PROTOCOL: &str = "\"protocol\": 763";
const EXPECTED_DURATION: &str = "\"duration_secs\": 600";
const EXPECTED_TIMEOUT: &str = "\"timeout_secs\": 600";
const EXPECTED_DIGEST_PLACEHOLDER: &str = "{{DIGEST}}";

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_REFRESH,
    EXPECTED_RED_SCENARIO,
    EXPECTED_BLUE_SCENARIO,
    "fresh live rerun",
    "copied receipts",
    "BLAKE3 manifests",
    "duration_secs=600",
    "timeout_secs=600",
    "full CTF correctness remains a non-claim",
    "production load remains a non-claim",
    "unbounded soak remains a non-claim",
];

const SUMMARY_RECEIPT_TOKENS: &[&str] = &[
    "mc.compat.red_blue_scoring_soak_live_refresh.v1",
    "\"status\": \"pass\"",
    "\"red_scenario\": \"multi-client-load-score\"",
    "\"blue_scenario\": \"blue-flag-score\"",
    RED_RECEIPT,
    BLUE_RECEIPT,
    "\"claims_full_ctf_correctness\": false",
    "\"claims_production_load\": false",
    "\"claims_unbounded_soak\": false",
];

const RED_RECEIPT_TOKENS: &[&str] = &[
    EXPECTED_SCHEMA,
    "\"status\": \"pass\"",
    "\"mode\": \"run\"",
    "\"dry_run\": false",
    "\"name\": \"multi-client-load-score\"",
    "\"multi_client_count\"",
    "\"team_red\"",
    "\"flag_pickup\"",
    "\"flag_capture\"",
    "\"score_red_1\"",
    "\"missing_milestones\": []",
    "\"forbidden_matches\": []",
    "\"server_client_a_seen\"",
    "\"server_client_b_seen\"",
    "\"server_flag_or_score\"",
    EXPECTED_PROTOCOL,
    EXPECTED_DURATION,
    EXPECTED_TIMEOUT,
    "\"claims_correctness\": false",
    "\"claims_semantic_equivalence\": false",
    "\"git_status\": \"clean\"",
];

const BLUE_RECEIPT_TOKENS: &[&str] = &[
    EXPECTED_SCHEMA,
    "\"status\": \"pass\"",
    "\"mode\": \"run\"",
    "\"dry_run\": false",
    "\"name\": \"blue-flag-score\"",
    "\"team_blue\"",
    "\"flag_pickup\"",
    "\"flag_capture\"",
    "\"score_blue_1\"",
    "\"missing_milestones\": []",
    "\"forbidden_matches\": []",
    "\"server_username_seen\"",
    "\"server_flag_or_score\"",
    EXPECTED_PROTOCOL,
    EXPECTED_DURATION,
    EXPECTED_TIMEOUT,
    "\"claims_correctness\": false",
    "\"claims_semantic_equivalence\": false",
    "\"git_status\": \"clean\"",
];

const RECEIPT_FORBIDDEN_TOKENS: &[&str] = &[
    "\"dry_run\": true",
    "\"git_rev\": \"dry-run\"",
    "\"git_status\": \"dry-run\"",
    "\"passed\": false",
    "\"missing_milestones\": [\"",
    "target/mc-compat-blue-soak/blue-flag-score-600s.json",
];

const ROW_DOC_TOKENS: &[&str] = &[
    EXPECTED_REFRESH,
    EXPECTED_RED_COMMAND,
    EXPECTED_BLUE_COMMAND,
    SUMMARY_RECEIPT,
    RED_RECEIPT,
    RED_RUN_LOG,
    RED_CLIENT_LOG,
    RED_SERVER_LOG,
    BLUE_RECEIPT,
    BLUE_RUN_LOG,
    BLUE_CLIENT_LOG,
    BLUE_SERVER_LOG,
    ROW_RECORD,
    "Scenario `multi-client-load-score`",
    "Scenario `blue-flag-score`",
    "score_red_1",
    "score_blue_1",
    "server_flag_or_score",
    "No full CTF correctness",
    "No production load",
    "No unbounded soak",
    "No broad Minecraft compatibility",
];

const ROW_DOC_FORBIDDEN_TOKENS: &[&str] = &[
    "target/mc-compat-blue-soak/blue-flag-score-600s.json",
    "claims full CTF correctness",
    "claims production load",
    "claims unbounded soak",
    "claims broad Minecraft compatibility",
];

const RED_RUN_LOG_TOKENS: &[&str] = &[
    "mc-compat-valence-ctf-600s-soak",
    "status=pass",
    "exit_status=0",
    "scenario=multi-client-load-score",
];

const BLUE_RUN_LOG_TOKENS: &[&str] = &[
    "mc-compat-valence-ctf-blue-600s-soak",
    "status=pass",
    "exit_status=0",
    "scenario=blue-flag-score",
];

const RED_CLIENT_LOG_TOKENS: &[&str] = &[
    "MC-COMPAT-MILESTONE",
    "team_probe_enter_red_portal",
    "flag_probe_score_chat",
];

const BLUE_CLIENT_LOG_TOKENS: &[&str] = &[
    "MC-COMPAT-MILESTONE",
    "team_probe_enter_blue_portal",
    "flag_probe_score_chat",
];

const RED_SERVER_LOG_TOKENS: &[&str] = &[
    "MC-COMPAT-MILESTONE",
    "flag_pickup",
    "carrier_team=Red",
    "flag_team=Blue",
];

const BLUE_SERVER_LOG_TOKENS: &[&str] = &[
    "MC-COMPAT-MILESTONE",
    "flag_pickup",
    "carrier_team=Blue",
    "flag_team=Red",
];

const MATRIX_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    EXPECTED_RED_COMMAND,
    EXPECTED_BLUE_COMMAND,
    SUMMARY_RECEIPT,
    ROW_DOC,
    "fresh live RED/BLUE",
    "No full CTF correctness",
];

const BUNDLE_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    EXPECTED_RED_COMMAND,
    EXPECTED_BLUE_COMMAND,
    SUMMARY_RECEIPT,
    RED_RECEIPT,
    BLUE_RECEIPT,
    "fresh live RED/BLUE scoring soak refresh",
    "historical exception removed",
    "full CTF correctness remains a non-claim",
];

const LEDGER_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    EXPECTED_RED_SCENARIO,
    EXPECTED_BLUE_SCENARIO,
    RED_RECEIPT,
    BLUE_RECEIPT,
    "full CTF correctness remains a non-claim",
];

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("red blue scoring soak live refresh self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    let record_path = flag_value(&args, RECORD_FLAG);
    match run_repo_check(Path::new("."), record_path.as_deref()) {
        Ok(summary) => {
            println!("red blue scoring soak live refresh check passed: {summary}");
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
        eprintln!("red blue scoring soak live refresh check failed: {error}");
    }
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(FLAG_VALUE_WINDOW)
        .find(|window| window.first().is_some_and(|value| value == flag))
        .and_then(|window| window.get(1))
        .cloned()
}

fn run_repo_check(root: &Path, record_path: Option<&str>) -> Result<String, Vec<String>> {
    let evidence = load_repo_evidence(root)?;
    let require_existing_record = record_path.is_none();
    let errors = validate_evidence(&evidence, require_existing_record);
    if !errors.is_empty() {
        return Err(errors);
    }
    if let Some(path) = record_path {
        write_record(root.join(path).as_path(), &evidence)?;
    }
    Ok("fresh RED and BLUE receipts, logs, matrix, bundle, and ledger validated".to_string())
}

#[derive(Debug, Clone)]
struct EvidenceTexts {
    contract: String,
    row_doc: String,
    summary_receipt: String,
    red_receipt: String,
    red_run_log: String,
    red_client_log: String,
    red_server_log: String,
    blue_receipt: String,
    blue_run_log: String,
    blue_client_log: String,
    blue_server_log: String,
    record: String,
    matrix: String,
    bundle: String,
    ledger: String,
}

fn load_repo_evidence(root: &Path) -> Result<EvidenceTexts, Vec<String>> {
    Ok(EvidenceTexts {
        contract: read(root, CONTRACT_DOC)?,
        row_doc: read(root, ROW_DOC)?,
        summary_receipt: read(root, SUMMARY_RECEIPT)?,
        red_receipt: read(root, RED_RECEIPT)?,
        red_run_log: read(root, RED_RUN_LOG)?,
        red_client_log: read(root, RED_CLIENT_LOG)?,
        red_server_log: read(root, RED_SERVER_LOG)?,
        blue_receipt: read(root, BLUE_RECEIPT)?,
        blue_run_log: read(root, BLUE_RUN_LOG)?,
        blue_client_log: read(root, BLUE_CLIENT_LOG)?,
        blue_server_log: read(root, BLUE_SERVER_LOG)?,
        record: read_optional(root, ROW_RECORD)?,
        matrix: read(root, ACCEPTANCE_MATRIX_DOC)?,
        bundle: read(root, CURRENT_BUNDLE_DOC)?,
        ledger: read(root, CTF_RULE_LEDGER_DOC)?,
    })
}

fn read(root: &Path, path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(root.join(path)).map_err(|error| vec![format!("{path}: {error}")])
}

fn read_optional(root: &Path, path: &str) -> Result<String, Vec<String>> {
    match fs::read_to_string(root.join(path)) {
        Ok(text) => Ok(text),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(vec![format!("{path}: {error}")]),
    }
}

fn validate_evidence(evidence: &EvidenceTexts, require_existing_record: bool) -> Vec<String> {
    let mut issues = Vec::new();
    require_tokens(&mut issues, "contract", &evidence.contract, CONTRACT_TOKENS);
    require_tokens(&mut issues, "row_doc", &evidence.row_doc, ROW_DOC_TOKENS);
    reject_tokens(
        &mut issues,
        "row_doc",
        &evidence.row_doc,
        ROW_DOC_FORBIDDEN_TOKENS,
    );
    require_tokens(
        &mut issues,
        "summary_receipt",
        &evidence.summary_receipt,
        SUMMARY_RECEIPT_TOKENS,
    );
    require_tokens(
        &mut issues,
        "red_receipt",
        &evidence.red_receipt,
        RED_RECEIPT_TOKENS,
    );
    require_tokens(
        &mut issues,
        "blue_receipt",
        &evidence.blue_receipt,
        BLUE_RECEIPT_TOKENS,
    );
    reject_tokens(
        &mut issues,
        "red_receipt",
        &evidence.red_receipt,
        RECEIPT_FORBIDDEN_TOKENS,
    );
    reject_tokens(
        &mut issues,
        "blue_receipt",
        &evidence.blue_receipt,
        RECEIPT_FORBIDDEN_TOKENS,
    );
    require_tokens(
        &mut issues,
        "red_run_log",
        &evidence.red_run_log,
        RED_RUN_LOG_TOKENS,
    );
    require_tokens(
        &mut issues,
        "blue_run_log",
        &evidence.blue_run_log,
        BLUE_RUN_LOG_TOKENS,
    );
    require_tokens(
        &mut issues,
        "red_client_log",
        &evidence.red_client_log,
        RED_CLIENT_LOG_TOKENS,
    );
    require_tokens(
        &mut issues,
        "blue_client_log",
        &evidence.blue_client_log,
        BLUE_CLIENT_LOG_TOKENS,
    );
    require_tokens(
        &mut issues,
        "red_server_log",
        &evidence.red_server_log,
        RED_SERVER_LOG_TOKENS,
    );
    require_tokens(
        &mut issues,
        "blue_server_log",
        &evidence.blue_server_log,
        BLUE_SERVER_LOG_TOKENS,
    );
    require_tokens(&mut issues, "matrix", &evidence.matrix, MATRIX_TOKENS);
    require_tokens(&mut issues, "bundle", &evidence.bundle, BUNDLE_TOKENS);
    require_tokens(&mut issues, "ledger", &evidence.ledger, LEDGER_TOKENS);
    if require_existing_record {
        require_record(&mut issues, &evidence.record);
    }
    issues
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
            issues.push(format!("{label} contains forbidden token {token:?}"));
        }
    }
}

fn require_record(issues: &mut Vec<String>, record: &str) {
    let fields = parse_record(record);
    for (key, expected) in [
        ("scenario", "red-blue-scoring-soak-live-refresh"),
        ("red_scenario", EXPECTED_RED_SCENARIO),
        ("blue_scenario", EXPECTED_BLUE_SCENARIO),
        ("summary_receipt", SUMMARY_RECEIPT),
        ("red_receipt", RED_RECEIPT),
        ("blue_receipt", BLUE_RECEIPT),
    ] {
        match fields.iter().find(|(candidate, _)| candidate == key) {
            Some((_, actual)) if actual == expected => {}
            Some((_, actual)) => issues.push(format!(
                "record {key} expected {expected:?}, found {actual:?}"
            )),
            None => issues.push(format!("record missing key {key}")),
        }
    }
}

fn parse_record(record: &str) -> Vec<(String, String)> {
    record
        .lines()
        .filter_map(|line| line.split_once(KEY_VALUE_SEPARATOR))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
}

fn write_record(path: &Path, evidence: &EvidenceTexts) -> Result<(), Vec<String>> {
    let digest =
        extract_digest(&evidence.matrix).unwrap_or_else(|| EXPECTED_DIGEST_PLACEHOLDER.to_string());
    let record = format!(
        "scenario=red-blue-scoring-soak-live-refresh\nred_scenario={EXPECTED_RED_SCENARIO}\nblue_scenario={EXPECTED_BLUE_SCENARIO}\nsummary_receipt={SUMMARY_RECEIPT}\nred_receipt={RED_RECEIPT}\nblue_receipt={BLUE_RECEIPT}\nrow_digest={digest}\n"
    );
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| vec![format!("{}: {error}", parent.display())])?;
    }
    fs::write(path, record).map_err(|error| vec![format!("{}: {error}", path.display())])
}

fn extract_digest(text: &str) -> Option<String> {
    text.as_bytes()
        .windows(BLAKE3_HEX_LENGTH)
        .find(|candidate| candidate.iter().all(|byte| byte.is_ascii_hexdigit()))
        .and_then(|candidate| String::from_utf8(candidate.to_vec()).ok())
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let evidence = fixture_evidence();
    let errors = validate_evidence(&evidence, true);
    if !errors.is_empty() {
        return Err(errors);
    }

    let mut dry_run = evidence.clone();
    dry_run.red_receipt = dry_run
        .red_receipt
        .replace("\"dry_run\": false", "\"dry_run\": true");
    assert_contains(&validate_evidence(&dry_run, true), "forbidden token")?;

    let mut target_only = evidence.clone();
    target_only
        .row_doc
        .push_str("\ntarget/mc-compat-blue-soak/blue-flag-score-600s.json\n");
    assert_contains(&validate_evidence(&target_only, true), "forbidden token")?;

    let mut missing_blue = evidence.clone();
    missing_blue.blue_receipt = missing_blue.blue_receipt.replace("\"score_blue_1\"\n", "");
    assert_contains(
        &validate_evidence(&missing_blue, true),
        "blue_receipt missing",
    )?;

    let mut missing_server = evidence.clone();
    missing_server.red_server_log = missing_server
        .red_server_log
        .replace("flag_team=Blue\n", "");
    assert_contains(
        &validate_evidence(&missing_server, true),
        "red_server_log missing",
    )?;

    let mut overclaim = evidence.clone();
    overclaim
        .row_doc
        .push_str("\nclaims full CTF correctness\n");
    assert_contains(&validate_evidence(&overclaim, true), "forbidden token")?;

    let mut missing_record = evidence;
    missing_record.record = missing_record
        .record
        .replace("blue_receipt=", "blue_receipt_missing=");
    assert_contains(
        &validate_evidence(&missing_record, true),
        "record missing key",
    )?;

    Ok("positive and negative fixtures exercised".to_string())
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
        contract: format!(
            "{EXPECTED_REFRESH}\n{EXPECTED_RED_SCENARIO}\n{EXPECTED_BLUE_SCENARIO}\nfresh live rerun\ncopied receipts\nBLAKE3 manifests\nduration_secs=600\ntimeout_secs=600\nfull CTF correctness remains a non-claim\nproduction load remains a non-claim\nunbounded soak remains a non-claim\n"
        ),
        row_doc: format!(
            "{EXPECTED_REFRESH}\n{EXPECTED_RED_COMMAND}\n{EXPECTED_BLUE_COMMAND}\n{SUMMARY_RECEIPT}\n{RED_RECEIPT}\n{RED_RUN_LOG}\n{RED_CLIENT_LOG}\n{RED_SERVER_LOG}\n{BLUE_RECEIPT}\n{BLUE_RUN_LOG}\n{BLUE_CLIENT_LOG}\n{BLUE_SERVER_LOG}\n{ROW_RECORD}\nScenario `multi-client-load-score`\nScenario `blue-flag-score`\nscore_red_1\nscore_blue_1\nserver_flag_or_score\nNo full CTF correctness\nNo production load\nNo unbounded soak\nNo broad Minecraft compatibility\n"
        ),
        summary_receipt: format!(
            "mc.compat.red_blue_scoring_soak_live_refresh.v1\n\"status\": \"pass\"\n\"red_scenario\": \"multi-client-load-score\"\n\"blue_scenario\": \"blue-flag-score\"\n{RED_RECEIPT}\n{BLUE_RECEIPT}\n\"claims_full_ctf_correctness\": false\n\"claims_production_load\": false\n\"claims_unbounded_soak\": false\n"
        ),
        red_receipt: format!(
            "{EXPECTED_SCHEMA}\n\"status\": \"pass\"\n\"mode\": \"run\"\n\"dry_run\": false\n\"name\": \"multi-client-load-score\"\n\"multi_client_count\"\n\"team_red\"\n\"flag_pickup\"\n\"flag_capture\"\n\"score_red_1\"\n\"missing_milestones\": []\n\"forbidden_matches\": []\n\"server_client_a_seen\"\n\"server_client_b_seen\"\n\"server_flag_or_score\"\n{EXPECTED_PROTOCOL}\n{EXPECTED_DURATION}\n{EXPECTED_TIMEOUT}\n\"claims_correctness\": false\n\"claims_semantic_equivalence\": false\n\"git_status\": \"clean\"\n"
        ),
        red_run_log: format!(
            "mc-compat-valence-ctf-600s-soak\nstatus=pass\nexit_status=0\nscenario=multi-client-load-score\n"
        ),
        red_client_log: "MC-COMPAT-MILESTONE\nteam_probe_enter_red_portal\nflag_probe_score_chat\n"
            .to_string(),
        red_server_log: "MC-COMPAT-MILESTONE\nflag_pickup\ncarrier_team=Red\nflag_team=Blue\n".to_string(),
        blue_receipt: format!(
            "{EXPECTED_SCHEMA}\n\"status\": \"pass\"\n\"mode\": \"run\"\n\"dry_run\": false\n\"name\": \"blue-flag-score\"\n\"team_blue\"\n\"flag_pickup\"\n\"flag_capture\"\n\"score_blue_1\"\n\"missing_milestones\": []\n\"forbidden_matches\": []\n\"server_username_seen\"\n\"server_flag_or_score\"\n{EXPECTED_PROTOCOL}\n{EXPECTED_DURATION}\n{EXPECTED_TIMEOUT}\n\"claims_correctness\": false\n\"claims_semantic_equivalence\": false\n\"git_status\": \"clean\"\n"
        ),
        blue_run_log: format!(
            "mc-compat-valence-ctf-blue-600s-soak\nstatus=pass\nexit_status=0\nscenario=blue-flag-score\n"
        ),
        blue_client_log: "MC-COMPAT-MILESTONE\nteam_probe_enter_blue_portal\nflag_probe_score_chat\n"
            .to_string(),
        blue_server_log: "MC-COMPAT-MILESTONE\nflag_pickup\ncarrier_team=Blue\nflag_team=Red\n"
            .to_string(),
        record: format!(
            "scenario=red-blue-scoring-soak-live-refresh\nred_scenario={EXPECTED_RED_SCENARIO}\nblue_scenario={EXPECTED_BLUE_SCENARIO}\nsummary_receipt={SUMMARY_RECEIPT}\nred_receipt={RED_RECEIPT}\nblue_receipt={BLUE_RECEIPT}\n"
        ),
        matrix: format!(
            "{EXPECTED_SEAM}\n{EXPECTED_RED_COMMAND}\n{EXPECTED_BLUE_COMMAND}\n{SUMMARY_RECEIPT}\n{ROW_DOC}\n{digest}\nfresh live RED/BLUE\nNo full CTF correctness\n"
        ),
        bundle: format!(
            "{EXPECTED_SEAM}\n{EXPECTED_RED_COMMAND}\n{EXPECTED_BLUE_COMMAND}\n{SUMMARY_RECEIPT}\n{RED_RECEIPT}\n{BLUE_RECEIPT}\nfresh live RED/BLUE scoring soak refresh\nhistorical exception removed\nfull CTF correctness remains a non-claim\n"
        ),
        ledger: format!(
            "{EXPECTED_SEAM}\n{EXPECTED_RED_SCENARIO}\n{EXPECTED_BLUE_SCENARIO}\n{RED_RECEIPT}\n{BLUE_RECEIPT}\nfull CTF correctness remains a non-claim\n"
        ),
    }
}
