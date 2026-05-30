use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const RECORD_FLAG: &str = "--record";
const KEY_VALUE_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const CONTRACT_DOC: &str =
    "docs/evidence/protocol-763-ctf-score-limit-win-condition-contract-2026-05-30.md";
const ROW_DOC: &str = "docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.md";
const ROW_RECEIPT: &str =
    "docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.receipt.json";
const RUN_LOG: &str =
    "docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.run.log";
const CLIENT_LOG: &str =
    "docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.client.log";
const SERVER_LOG: &str =
    "docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.server.log";
const ROW_RECORD: &str =
    "docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.record";
const ACCEPTANCE_MATRIX_DOC: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const CURRENT_BUNDLE_DOC: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const CTF_RULE_LEDGER_DOC: &str = "docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md";

const EXPECTED_SCENARIO: &str = "ctf-score-limit-win-condition";
const EXPECTED_SEAM: &str = "Score limit / win condition";
const EXPECTED_COMMAND: &str = "nix run .#mc-compat-valence-ctf-score-limit-win-condition";
const EXPECTED_SCHEMA: &str = "mc.compat.scenario.receipt.v2";
const EXPECTED_PROTOCOL: &str = "\"protocol\": 763";
const EXPECTED_SCORE_LIMIT: &str = "2";
const EXPECTED_RED_BEFORE: &str = "1";
const EXPECTED_BLUE_SCORE: &str = "0";
const EXPECTED_RED_AFTER: &str = "2";
const EXPECTED_WIN_TEAM: &str = "Red";
const EXPECTED_END_STATE: &str = "winner_declared";
const EXPECTED_WIN_EMISSIONS: &str = "1";
const EXPECTED_RULE_ID: &str = "score_limit_win_emits_once_without_post_win_mutation";
const EXPECTED_DIGEST_PLACEHOLDER: &str = "{{DIGEST}}";
const BLAKE3_HEX_LENGTH: usize = 64;

const EXPECTED_CLIENT_WIN: &str = "ctf_score_limit_win_seen score_limit=2 winning_team=red red_score=2 blue_score=0 end_state=winner_declared duplicate_win=false";
const EXPECTED_SERVER_PRE_STATE: &str = "score_limit_pre_state score_limit=2 red_score=1 blue_score=0 next_capture_team=Red outcome=one_capture_before_win";
const EXPECTED_SERVER_FINAL_CAPTURE: &str = "score_limit_final_capture username=compatbot capture_team=Red carried_flag=Blue score_limit=2 red_score_before=1 blue_score_before=0 red_score_after=2 blue_score_after=0";
const EXPECTED_SERVER_WIN: &str = "score_limit_win_condition username=compatbot winning_team=Red score_limit=2 red_score=2 blue_score=0 end_state=winner_declared win_emissions=1 duplicate_win=false post_win_score_delta=0";
const EXPECTED_PACKETS: &str = "\"expected_summary_packets\": [\"login_success\", \"play_join_game\", \"flag_pickup\", \"flag_capture\", \"score_limit_win_condition\"]";

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_SCENARIO,
    "score_limit=2",
    "red_score_before=1",
    "blue_score_before=0",
    "red_score_after=2",
    "winning_team=Red",
    "end_state=winner_declared",
    "win_emissions=1",
    "duplicate_win=false",
    "post_win_score_delta=0",
    "full CTF correctness remains a non-claim",
];

const RECEIPT_TOKENS: &[&str] = &[
    EXPECTED_SCHEMA,
    "\"status\": \"pass\"",
    "\"mode\": \"run\"",
    "\"dry_run\": false",
    "\"name\": \"ctf-score-limit-win-condition\"",
    "\"ctf_score_limit_win_seen\"",
    "\"server_score_limit_pre_state\"",
    "\"server_score_limit_final_capture\"",
    "\"server_score_limit_win_condition\"",
    "\"forbidden_matches\": []",
    EXPECTED_PROTOCOL,
    EXPECTED_PACKETS,
    "\"claims_correctness\": false",
    "\"claims_semantic_equivalence\": false",
    "\"git_status\": \"clean\"",
];

const ROW_DOC_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    ROW_RECEIPT,
    RUN_LOG,
    CLIENT_LOG,
    SERVER_LOG,
    ROW_RECORD,
    EXPECTED_CLIENT_WIN,
    EXPECTED_SERVER_PRE_STATE,
    EXPECTED_SERVER_FINAL_CAPTURE,
    EXPECTED_SERVER_WIN,
    "No all score limits",
    "No overtime/tiebreakers",
    "No all scoring races",
    "No scoreboard UI parity",
    "No full CTF correctness",
    "No production readiness",
];

const MATRIX_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    EXPECTED_COMMAND,
    ROW_RECEIPT,
    ROW_DOC,
    "near-limit RED capture",
    "No all score limits",
];

const BUNDLE_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    EXPECTED_COMMAND,
    EXPECTED_SCENARIO,
    "near-limit capture checkpoint",
    "score_limit_win_condition",
    "full CTF correctness remains a non-claim",
];

const LEDGER_TOKENS: &[&str] = &[
    EXPECTED_RULE_ID,
    EXPECTED_SEAM,
    "ctf_score_limit_win_seen",
    "server_score_limit_pre_state",
    "server_score_limit_final_capture",
    "server_score_limit_win_condition",
    "score_limit_duplicate_win",
    "score_limit_post_win_score_mutation",
    ROW_RECEIPT,
    "score_limit_variants broad score-limit breadth | Non-claim",
    "full CTF correctness remains a non-claim",
];

const FORBIDDEN_ROW_DOC_TOKENS: &[&str] = &[
    "claims all score limits",
    "claims all scoring races",
    "claims full CTF correctness",
    "claims production readiness",
];

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("ctf score limit win condition self-test ok: {summary}");
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
            println!("ctf score limit win condition check passed: {summary}");
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
        eprintln!("ctf score limit win condition check failed: {error}");
    }
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
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
    Ok("receipt, logs, matrix, bundle, and ledger validated".to_string())
}

#[derive(Debug, Clone)]
struct EvidenceTexts {
    contract: String,
    row_doc: String,
    receipt: String,
    run_log: String,
    client_log: String,
    server_log: String,
    record: String,
    matrix: String,
    bundle: String,
    ledger: String,
}

fn load_repo_evidence(root: &Path) -> Result<EvidenceTexts, Vec<String>> {
    Ok(EvidenceTexts {
        contract: read(root, CONTRACT_DOC)?,
        row_doc: read(root, ROW_DOC)?,
        receipt: read(root, ROW_RECEIPT)?,
        run_log: read(root, RUN_LOG)?,
        client_log: read(root, CLIENT_LOG)?,
        server_log: read(root, SERVER_LOG)?,
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
        FORBIDDEN_ROW_DOC_TOKENS,
    );
    require_tokens(&mut issues, "receipt", &evidence.receipt, RECEIPT_TOKENS);
    require_tokens(
        &mut issues,
        "client_log",
        &evidence.client_log,
        &[EXPECTED_CLIENT_WIN, "RED: 2", "You captured the flag!"],
    );
    require_single_occurrence(
        &mut issues,
        "client_log",
        &evidence.client_log,
        EXPECTED_CLIENT_WIN,
    );
    require_tokens(
        &mut issues,
        "server_log",
        &evidence.server_log,
        &[
            EXPECTED_SERVER_PRE_STATE,
            EXPECTED_SERVER_FINAL_CAPTURE,
            EXPECTED_SERVER_WIN,
        ],
    );
    reject_tokens(
        &mut issues,
        "server_log",
        &evidence.server_log,
        &["score_limit_duplicate_win", "score_limit_post_win_score_mutation"],
    );
    require_tokens(
        &mut issues,
        "run_log",
        &evidence.run_log,
        &["mc-compat-valence-ctf-score-limit-win-condition", "status=pass", EXPECTED_SCENARIO],
    );
    if require_existing_record {
        require_record(&mut issues, &evidence.record);
    }
    require_tokens(
        &mut issues,
        "acceptance_matrix",
        &evidence.matrix,
        MATRIX_TOKENS,
    );
    require_tokens(
        &mut issues,
        "current_bundle",
        &evidence.bundle,
        BUNDLE_TOKENS,
    );
    require_tokens(
        &mut issues,
        "ctf_rule_ledger",
        &evidence.ledger,
        LEDGER_TOKENS,
    );
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

fn require_single_occurrence(issues: &mut Vec<String>, label: &str, haystack: &str, token: &str) {
    let count = haystack.matches(token).count();
    if count != 1 {
        issues.push(format!(
            "{label} expected one occurrence of {token:?}, found {count}"
        ));
    }
}

fn require_record(issues: &mut Vec<String>, record: &str) {
    let fields = parse_record(record);
    for (key, expected) in [
        ("scenario", EXPECTED_SCENARIO),
        ("score_limit", EXPECTED_SCORE_LIMIT),
        ("red_score_before", EXPECTED_RED_BEFORE),
        ("blue_score", EXPECTED_BLUE_SCORE),
        ("red_score_after", EXPECTED_RED_AFTER),
        ("winning_team", EXPECTED_WIN_TEAM),
        ("end_state", EXPECTED_END_STATE),
        ("win_emissions", EXPECTED_WIN_EMISSIONS),
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
        "scenario={EXPECTED_SCENARIO}\nscore_limit={EXPECTED_SCORE_LIMIT}\nred_score_before={EXPECTED_RED_BEFORE}\nblue_score={EXPECTED_BLUE_SCORE}\nred_score_after={EXPECTED_RED_AFTER}\nwinning_team={EXPECTED_WIN_TEAM}\nend_state={EXPECTED_END_STATE}\nwin_emissions={EXPECTED_WIN_EMISSIONS}\nreceipt={ROW_RECEIPT}\nrow_digest={digest}\n"
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

    let mut duplicate_win = evidence.clone();
    duplicate_win.server_log.push_str(
        "\nMC-COMPAT-MILESTONE score_limit_duplicate_win username=compatbot winning_team=Red score_limit=2 outcome=forbidden_duplicate_win\n",
    );
    assert_contains(&validate_evidence(&duplicate_win, true), "forbidden token")?;

    let mut mutation = evidence.clone();
    mutation
        .server_log
        .push_str("\nMC-COMPAT-MILESTONE score_limit_post_win_score_mutation username=compatbot winning_team=Red score_limit=2 outcome=forbidden_score_after_win\n");
    assert_contains(&validate_evidence(&mutation, true), "forbidden token")?;

    let mut wrong_score = evidence.clone();
    wrong_score.server_log = wrong_score.server_log.replace("red_score_after=2", "red_score_after=3");
    assert_contains(&validate_evidence(&wrong_score, true), "server_log missing")?;

    let mut overclaim = evidence.clone();
    overclaim.row_doc.push_str("\nclaims full CTF correctness\n");
    assert_contains(&validate_evidence(&overclaim, true), "forbidden token")?;

    let mut missing_record = evidence;
    missing_record.record = missing_record.record.replace("winning_team=Red\n", "");
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
            "{EXPECTED_SCENARIO}\nscore_limit=2\nred_score_before=1\nblue_score_before=0\nred_score_after=2\nwinning_team=Red\nend_state=winner_declared\nwin_emissions=1\nduplicate_win=false\npost_win_score_delta=0\nfull CTF correctness remains a non-claim\n"
        ),
        row_doc: format!(
            "{EXPECTED_SEAM}\n{ROW_RECEIPT}\n{RUN_LOG}\n{CLIENT_LOG}\n{SERVER_LOG}\n{ROW_RECORD}\n{EXPECTED_CLIENT_WIN}\n{EXPECTED_SERVER_PRE_STATE}\n{EXPECTED_SERVER_FINAL_CAPTURE}\n{EXPECTED_SERVER_WIN}\nNo all score limits\nNo overtime/tiebreakers\nNo all scoring races\nNo scoreboard UI parity\nNo full CTF correctness\nNo production readiness\n"
        ),
        receipt: format!(
            "{EXPECTED_SCHEMA}\n\"status\": \"pass\"\n\"mode\": \"run\"\n\"dry_run\": false\n\"name\": \"ctf-score-limit-win-condition\"\n\"ctf_score_limit_win_seen\"\n\"server_score_limit_pre_state\"\n\"server_score_limit_final_capture\"\n\"server_score_limit_win_condition\"\n\"forbidden_matches\": []\n{EXPECTED_PROTOCOL}\n{EXPECTED_PACKETS}\n\"claims_correctness\": false\n\"claims_semantic_equivalence\": false\n\"git_status\": \"clean\"\n"
        ),
        run_log: format!("{EXPECTED_COMMAND}\nstatus=pass\n{EXPECTED_SCENARIO}\n"),
        client_log: format!("You captured the flag!\nRED: 2\n{EXPECTED_CLIENT_WIN}\n"),
        server_log: format!(
            "{EXPECTED_SERVER_PRE_STATE}\n{EXPECTED_SERVER_FINAL_CAPTURE}\n{EXPECTED_SERVER_WIN}\n"
        ),
        record: format!(
            "scenario={EXPECTED_SCENARIO}\nscore_limit={EXPECTED_SCORE_LIMIT}\nred_score_before={EXPECTED_RED_BEFORE}\nblue_score={EXPECTED_BLUE_SCORE}\nred_score_after={EXPECTED_RED_AFTER}\nwinning_team={EXPECTED_WIN_TEAM}\nend_state={EXPECTED_END_STATE}\nwin_emissions={EXPECTED_WIN_EMISSIONS}\n"
        ),
        matrix: format!(
            "{EXPECTED_SEAM}\n{EXPECTED_COMMAND}\n{ROW_RECEIPT}\n{ROW_DOC}\n{digest}\nnear-limit RED capture\nNo all score limits\n"
        ),
        bundle: format!(
            "{EXPECTED_SEAM}\n{EXPECTED_COMMAND}\n{EXPECTED_SCENARIO}\nnear-limit capture checkpoint\nscore_limit_win_condition\nfull CTF correctness remains a non-claim\n"
        ),
        ledger: format!(
            "{EXPECTED_RULE_ID}\n{EXPECTED_SEAM}\nctf_score_limit_win_seen\nserver_score_limit_pre_state\nserver_score_limit_final_capture\nserver_score_limit_win_condition\nscore_limit_duplicate_win\nscore_limit_post_win_score_mutation\n{ROW_RECEIPT}\nscore_limit_variants broad score-limit breadth | Non-claim\nfull CTF correctness remains a non-claim\n"
        ),
    }
}
