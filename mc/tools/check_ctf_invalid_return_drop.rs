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
    "docs/evidence/protocol-763-ctf-invalid-return-drop-contract-2026-05-30.md";
const ROW_DOC: &str = "docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.md";
const ROW_RECEIPT: &str =
    "docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.receipt.json";
const RUN_LOG: &str = "docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.run.log";
const CLIENT_LOG: &str = "docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.client.log";
const SERVER_LOG: &str = "docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.server.log";
const ROW_RECORD: &str = "docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.record";
const ACCEPTANCE_MATRIX_DOC: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const CURRENT_BUNDLE_DOC: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const CTF_RULE_LEDGER_DOC: &str = "docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md";

const EXPECTED_SCENARIO: &str = "ctf-invalid-return-drop";
const EXPECTED_SEAM: &str = "Invalid flag return/drop";
const EXPECTED_COMMAND: &str = "nix run .#mc-compat-valence-ctf-invalid-return-drop";
const EXPECTED_SCHEMA: &str = "mc.compat.scenario.receipt.v2";
const EXPECTED_PROTOCOL: &str = "\"protocol\": 763";
const EXPECTED_INVALID_ACTION: &str = "own_base_return_without_carrier";
const EXPECTED_POSTCONDITION: &str = "ctf_invalid_return_drop_contained";
const EXPECTED_CLIENT_ATTEMPT: &str = "ctf_invalid_return_drop_attempted player_team=red flag_team=red pre_state=at_base action=own_base_return expected=no_flag_state_mutation_no_score";
const EXPECTED_CLIENT_CONTAINED: &str = "ctf_invalid_return_drop_contained player_team=red flag_team=red post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score";
const EXPECTED_SERVER_REJECTION: &str = "invalid_flag_return_drop_rejected username=compatbot actor_team=Red flag_team=Red pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score";
const EXPECTED_TARGET_SCOPE: &str = "owned-local-loopback";
const EXPECTED_OBSERVED_OUTCOME: &str = "containment_observed";
const EXPECTED_RULE_ID: &str = "invalid_return_drop_rejected_without_state_mutation";
const EXPECTED_DIGEST_PLACEHOLDER: &str = "{{DIGEST}}";

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_SCENARIO,
    "player_team=red",
    "flag_team=red",
    "pre_state=at_base",
    "post_state=at_base",
    "red_score=0",
    "blue_score=0",
    EXPECTED_INVALID_ACTION,
    EXPECTED_POSTCONDITION,
    "unexpected flag return/drop",
    "unexpected score/capture",
    "all invalid return/drop permutations remain non-claims",
];

const RECEIPT_TOKENS: &[&str] = &[
    EXPECTED_SCHEMA,
    "\"status\": \"pass\"",
    "\"mode\": \"run\"",
    "\"dry_run\": false",
    "\"name\": \"ctf-invalid-return-drop\"",
    "\"ctf_invalid_return_drop_attempted\"",
    "\"ctf_invalid_return_drop_contained\"",
    "\"server_invalid_return_drop_rejected\"",
    "\"forbidden_matches\": []",
    EXPECTED_PROTOCOL,
    "\"selected\": true",
    "\"rail\": \"ctf-invalid-return-drop\"",
    "\"invalid_action\": \"own_base_return_without_carrier\"",
    "\"observed_outcome\": \"containment_observed\"",
    "client_milestone:ctf_invalid_return_drop_contained",
    "\"postcondition_milestone\": \"ctf_invalid_return_drop_contained\"",
    "\"telemetry_present\": true",
    "\"target_scope\": \"owned-local-loopback\"",
    "\"preflight_passed\": true",
    "\"git_status\": \"clean\"",
];

const ROW_DOC_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    ROW_RECEIPT,
    RUN_LOG,
    CLIENT_LOG,
    SERVER_LOG,
    ROW_RECORD,
    EXPECTED_CLIENT_ATTEMPT,
    EXPECTED_CLIENT_CONTAINED,
    EXPECTED_SERVER_REJECTION,
    "No all invalid return/drop permutations",
    "No all flag permutations",
    "No full CTF correctness",
    "No adversarial security",
    "No production readiness",
    "No broad Minecraft compatibility",
];

const MATRIX_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    EXPECTED_COMMAND,
    ROW_RECEIPT,
    ROW_DOC,
    "own-base return/drop",
    "No all invalid return/drop permutations",
];

const BUNDLE_TOKENS: &[&str] = &[
    EXPECTED_SEAM,
    EXPECTED_COMMAND,
    "ctf-invalid-return-drop",
    "invalid flag return/drop checkpoint",
    EXPECTED_INVALID_ACTION,
    "full CTF correctness remains a non-claim",
];

const LEDGER_TOKENS: &[&str] = &[
    EXPECTED_RULE_ID,
    EXPECTED_SEAM,
    "ctf_invalid_return_drop_attempted",
    "ctf_invalid_return_drop_contained",
    "server_invalid_return_drop_rejected",
    "unexpected_flag_return",
    "unexpected_server_flag_pickup",
    ROW_RECEIPT,
    "invalid_return_accepted broad invalid return/drop breadth | Non-claim",
    "full CTF correctness remains a non-claim",
];

const FORBIDDEN_ROW_DOC_TOKENS: &[&str] = &[
    "claims all invalid actions",
    "claims full CTF correctness",
    "claims production readiness",
];

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("ctf invalid return drop self-test ok: {summary}");
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
            println!("ctf invalid return drop check passed: {summary}");
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
        eprintln!("ctf invalid return drop check failed: {error}");
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
        &[EXPECTED_CLIENT_ATTEMPT, EXPECTED_CLIENT_CONTAINED],
    );
    require_tokens(
        &mut issues,
        "server_log",
        &evidence.server_log,
        &[EXPECTED_SERVER_REJECTION],
    );
    reject_tokens(
        &mut issues,
        "server_log",
        &evidence.server_log,
        &[
            "MC-COMPAT-MILESTONE flag_pickup username=compatbot",
            "MC-COMPAT-MILESTONE flag_return",
        ],
    );
    require_tokens(
        &mut issues,
        "run_log",
        &evidence.run_log,
        &[EXPECTED_COMMAND, "status=pass", EXPECTED_SCENARIO],
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

fn require_record(issues: &mut Vec<String>, record: &str) {
    let fields = parse_record(record);
    for (key, expected) in [
        ("scenario", EXPECTED_SCENARIO),
        ("invalid_action", EXPECTED_INVALID_ACTION),
        ("target_scope", EXPECTED_TARGET_SCOPE),
        ("observed_outcome", EXPECTED_OBSERVED_OUTCOME),
        ("postcondition", EXPECTED_POSTCONDITION),
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
        "scenario={EXPECTED_SCENARIO}\ninvalid_action={EXPECTED_INVALID_ACTION}\ntarget_scope={EXPECTED_TARGET_SCOPE}\nobserved_outcome={EXPECTED_OBSERVED_OUTCOME}\npostcondition={EXPECTED_POSTCONDITION}\nreceipt={ROW_RECEIPT}\nrow_digest={digest}\n"
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

const BLAKE3_HEX_LENGTH: usize = 64;

fn run_self_tests() -> Result<String, Vec<String>> {
    let evidence = fixture_evidence();
    let errors = validate_evidence(&evidence, true);
    if !errors.is_empty() {
        return Err(errors);
    }

    let mut missing_server = evidence.clone();
    missing_server.server_log = missing_server
        .server_log
        .replace(EXPECTED_SERVER_REJECTION, "missing server rejection");
    assert_contains(
        &validate_evidence(&missing_server, true),
        "server_log missing",
    )?;

    let mut state_mutation = evidence.clone();
    state_mutation
        .server_log
        .push_str("\nMC-COMPAT-MILESTONE flag_return carrier=compatbot flag_team=red\n");
    assert_contains(&validate_evidence(&state_mutation, true), "forbidden token")?;

    let mut overclaim = evidence.clone();
    overclaim
        .row_doc
        .push_str("\nclaims full CTF correctness\n");
    assert_contains(&validate_evidence(&overclaim, true), "forbidden token")?;

    let mut missing_record = evidence;
    missing_record.record = missing_record
        .record
        .replace("observed_outcome=containment_observed\n", "");
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
            "{EXPECTED_SCENARIO}\nplayer_team=red\nflag_team=red\npre_state=at_base\npost_state=at_base\nred_score=0\nblue_score=0\n{EXPECTED_INVALID_ACTION}\n{EXPECTED_POSTCONDITION}\nunexpected flag return/drop\nunexpected score/capture\nall invalid return/drop permutations remain non-claims\n"
        ),
        row_doc: format!(
            "{EXPECTED_SEAM}\n{ROW_RECEIPT}\n{RUN_LOG}\n{CLIENT_LOG}\n{SERVER_LOG}\n{ROW_RECORD}\n{EXPECTED_CLIENT_ATTEMPT}\n{EXPECTED_CLIENT_CONTAINED}\n{EXPECTED_SERVER_REJECTION}\nNo all invalid return/drop permutations\nNo all flag permutations\nNo full CTF correctness\nNo adversarial security\nNo production readiness\nNo broad Minecraft compatibility\n"
        ),
        receipt: format!(
            "{EXPECTED_SCHEMA}\n\"status\": \"pass\"\n\"mode\": \"run\"\n\"dry_run\": false\n\"name\": \"ctf-invalid-return-drop\"\n\"ctf_invalid_return_drop_attempted\"\n\"ctf_invalid_return_drop_contained\"\n\"server_invalid_return_drop_rejected\"\n\"forbidden_matches\": []\n{EXPECTED_PROTOCOL}\n\"selected\": true\n\"rail\": \"ctf-invalid-return-drop\"\n\"invalid_action\": \"own_base_return_without_carrier\"\n\"observed_outcome\": \"containment_observed\"\nclient_milestone:ctf_invalid_return_drop_contained\n\"postcondition_milestone\": \"ctf_invalid_return_drop_contained\"\n\"telemetry_present\": true\n\"target_scope\": \"owned-local-loopback\"\n\"preflight_passed\": true\n\"git_status\": \"clean\"\n"
        ),
        run_log: format!("{EXPECTED_COMMAND}\nstatus=pass\n{EXPECTED_SCENARIO}\n"),
        client_log: format!("{EXPECTED_CLIENT_ATTEMPT}\n{EXPECTED_CLIENT_CONTAINED}\n"),
        server_log: format!("{EXPECTED_SERVER_REJECTION}\n"),
        record: format!(
            "scenario={EXPECTED_SCENARIO}\ninvalid_action={EXPECTED_INVALID_ACTION}\ntarget_scope={EXPECTED_TARGET_SCOPE}\nobserved_outcome={EXPECTED_OBSERVED_OUTCOME}\npostcondition={EXPECTED_POSTCONDITION}\n"
        ),
        matrix: format!(
            "{EXPECTED_SEAM}\n{EXPECTED_COMMAND}\n{ROW_RECEIPT}\n{ROW_DOC}\n{digest}\nown-base return/drop\nNo all invalid return/drop permutations\n"
        ),
        bundle: format!(
            "{EXPECTED_SEAM}\n{EXPECTED_COMMAND}\nctf-invalid-return-drop\ninvalid flag return/drop checkpoint\n{EXPECTED_INVALID_ACTION}\nfull CTF correctness remains a non-claim\n"
        ),
        ledger: format!(
            "{EXPECTED_RULE_ID}\n{EXPECTED_SEAM}\nctf_invalid_return_drop_attempted\nctf_invalid_return_drop_contained\nserver_invalid_return_drop_rejected\nunexpected_flag_return\nunexpected_server_flag_pickup\n{ROW_RECEIPT}\ninvalid_return_accepted broad invalid return/drop breadth | Non-claim\nfull CTF correctness remains a non-claim\n"
        ),
    }
}
