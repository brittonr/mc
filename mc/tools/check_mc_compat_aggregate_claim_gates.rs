use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const CURRENT_BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const ACCEPTANCE_MATRIX_PATH: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const CTF_LEDGER_PATH: &str = "docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md";
const PROTOCOL_LEDGER_PATH: &str = "docs/evidence/protocol-763-broad-coverage-ledger-2026-05-28.md";
const PRODUCTION_MATRIX_PATH: &str = "docs/evidence/protocol-763-production-network-safety-matrix-2026-05-28.md";

const CTF_GATE_NAME: &str = "full CTF correctness";
const PROTOCOL_GATE_NAME: &str = "full protocol-763 compatibility";
const PRODUCTION_GATE_NAME: &str = "production readiness";

const CTF_NON_CLAIM: &str = "full ctf correctness remains a non-claim";
const PROTOCOL_NON_CLAIM: &str = "full protocol-763 compatibility remains a non-claim";
const MINECRAFT_NON_CLAIM: &str = "full minecraft compatibility remains a non-claim";
const NON_CLAIM_TOKEN: &str = "non-claim";
const AGGREGATE_GATE_COUNT: usize = 3;
const AGGREGATE_GATE_COUNT_TEXT: &str = "3";
const COVERED_TOKEN: &str = "covered";
const PASSES_TOKEN: &str = "passes";
const PROVEN_TOKEN: &str = "proven";
const CLAIMED_TOKEN: &str = "claimed";

const CTF_REQUIRED_NON_CLAIMS: &[&str] = &[
    "simultaneous pickup/capture race | Non-claim",
    "spawn/team balance/resource reset | Non-claim",
    "full CTF correctness | Non-claim",
];
const CTF_REQUIRED_EVIDENCE_ROWS: &[&str] = &[
    "score_capture_red_blue_bounded",
    "flag_carrier_death_returns_flag_without_score",
    "disconnect_returns_flag_and_reconnect_state_coherent",
    "invalid_pickup_rejected_without_ownership_transfer",
    "invalid_return_drop_rejected_without_state_mutation",
    "score_limit_win_emits_once_without_post_win_mutation",
];
const CTF_CURRENT_BUNDLE_TOKENS: &[&str] = &[
    "CTF rule scope is guarded",
    "full CTF correctness remains a non-claim",
];

const PROTOCOL_REQUIRED_NON_CLAIMS: &[&str] = &[
    "all_packets_all_states | Non-claim",
    "all_entity_metadata_variants | Non-claim",
    "all_inventory_transactions | Non-claim",
    "all_equipment_permutations | Non-claim",
    "all_biomes_chunks_commands_recipes_advancements | Non-claim",
    "full_survival_compatibility | Non-claim",
    "all_vanilla_combat_parity | Non-claim",
];
const PROTOCOL_REQUIRED_LEDGER_TOKENS: &[&str] = &[
    "175 Valence protocol-763 packet rows",
    "four narrow parser-fixture-backed packet rows",
    "full Minecraft compatibility remains a non-claim",
];
const PROTOCOL_CURRENT_BUNDLE_TOKENS: &[&str] = &[
    "Broad coverage is guarded",
    "full protocol-763 compatibility/full Minecraft compatibility",
];

const PRODUCTION_REQUIRED_MATRIX_ROWS: &[&str] = &[
    "owned-local load safety | covered_owned_local_bounded",
    "public-server safety | covered_authorized_fixture_only",
    "WAN tolerance | covered_owned_local_bounded_telemetry",
    "adversarial-network safety | covered_fixture_oracle_only",
];
const PRODUCTION_REQUIRED_NON_CLAIMS: &[&str] = &[
    "No public-server safety",
    "No production readiness",
    "No adversarial safety",
    "no unbounded soak/reconnect safety",
    "no third-party target safety",
];
const PRODUCTION_CURRENT_BUNDLE_TOKENS: &[&str] = &[
    "The production/network matrix promotes bounded owned-local loopback load safety",
    "Broader production readiness",
    "unbounded safety remain non-claims",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct GateInputs {
    current_bundle: String,
    acceptance_matrix: String,
    ctf_ledger: String,
    protocol_ledger: String,
    production_matrix: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GateSummary {
    aggregate_gates: usize,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("aggregate claim gate self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match read_repo_inputs(Path::new(".")).and_then(|inputs| validate_gates(&inputs)) {
        Ok(summary) => {
            println!(
                "aggregate claim gates passed: {} gates checked",
                summary.aggregate_gates
            );
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
        eprintln!("aggregate claim gate failed: {error}");
    }
}

fn read_repo_inputs(root: &Path) -> Result<GateInputs, Vec<String>> {
    Ok(GateInputs {
        current_bundle: read_file(root, CURRENT_BUNDLE_PATH)?,
        acceptance_matrix: read_file(root, ACCEPTANCE_MATRIX_PATH)?,
        ctf_ledger: read_file(root, CTF_LEDGER_PATH)?,
        protocol_ledger: read_file(root, PROTOCOL_LEDGER_PATH)?,
        production_matrix: read_file(root, PRODUCTION_MATRIX_PATH)?,
    })
}

fn read_file(root: &Path, relative_path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(root.join(relative_path))
        .map_err(|error| vec![format!("{relative_path}: {error}")])
}

fn validate_gates(inputs: &GateInputs) -> Result<GateSummary, Vec<String>> {
    let mut errors = Vec::new();
    validate_ctf_gate(inputs, &mut errors);
    validate_protocol_gate(inputs, &mut errors);
    validate_production_gate(inputs, &mut errors);

    if errors.is_empty() {
        Ok(GateSummary {
            aggregate_gates: AGGREGATE_GATE_COUNT,
        })
    } else {
        Err(errors)
    }
}

fn validate_ctf_gate(inputs: &GateInputs, errors: &mut Vec<String>) {
    require_lowercase_token(CTF_GATE_NAME, &inputs.ctf_ledger, CTF_NON_CLAIM, errors);
    require_tokens(CTF_GATE_NAME, &inputs.ctf_ledger, CTF_REQUIRED_NON_CLAIMS, errors);
    require_tokens(CTF_GATE_NAME, &inputs.ctf_ledger, CTF_REQUIRED_EVIDENCE_ROWS, errors);
    require_tokens(
        CTF_GATE_NAME,
        &inputs.current_bundle,
        CTF_CURRENT_BUNDLE_TOKENS,
        errors,
    );
    reject_premature_claim(
        CTF_GATE_NAME,
        &[&inputs.ctf_ledger, &inputs.current_bundle, &inputs.acceptance_matrix],
        errors,
    );
}

fn validate_protocol_gate(inputs: &GateInputs, errors: &mut Vec<String>) {
    require_lowercase_token(
        PROTOCOL_GATE_NAME,
        &inputs.protocol_ledger,
        PROTOCOL_NON_CLAIM,
        errors,
    );
    require_lowercase_token(
        PROTOCOL_GATE_NAME,
        &inputs.protocol_ledger,
        MINECRAFT_NON_CLAIM,
        errors,
    );
    require_tokens(
        PROTOCOL_GATE_NAME,
        &inputs.protocol_ledger,
        PROTOCOL_REQUIRED_NON_CLAIMS,
        errors,
    );
    require_tokens(
        PROTOCOL_GATE_NAME,
        &inputs.protocol_ledger,
        PROTOCOL_REQUIRED_LEDGER_TOKENS,
        errors,
    );
    require_tokens(
        PROTOCOL_GATE_NAME,
        &inputs.current_bundle,
        PROTOCOL_CURRENT_BUNDLE_TOKENS,
        errors,
    );
    reject_premature_claim(
        PROTOCOL_GATE_NAME,
        &[
            &inputs.protocol_ledger,
            &inputs.current_bundle,
            &inputs.acceptance_matrix,
        ],
        errors,
    );
}

fn validate_production_gate(inputs: &GateInputs, errors: &mut Vec<String>) {
    require_tokens(
        PRODUCTION_GATE_NAME,
        &inputs.production_matrix,
        PRODUCTION_REQUIRED_MATRIX_ROWS,
        errors,
    );
    require_tokens(
        PRODUCTION_GATE_NAME,
        &inputs.production_matrix,
        PRODUCTION_REQUIRED_NON_CLAIMS,
        errors,
    );
    require_tokens(
        PRODUCTION_GATE_NAME,
        &inputs.current_bundle,
        PRODUCTION_CURRENT_BUNDLE_TOKENS,
        errors,
    );
    reject_premature_claim(
        PRODUCTION_GATE_NAME,
        &[
            &inputs.production_matrix,
            &inputs.current_bundle,
            &inputs.acceptance_matrix,
        ],
        errors,
    );
}

fn require_tokens(gate: &str, haystack: &str, tokens: &[&str], errors: &mut Vec<String>) {
    for token in tokens {
        if !haystack.contains(token) {
            errors.push(format!("{gate} missing required token: {token}"));
        }
    }
}

fn require_lowercase_token(gate: &str, haystack: &str, token: &str, errors: &mut Vec<String>) {
    if !haystack.to_lowercase().contains(token) {
        errors.push(format!("{gate} missing required token: {token}"));
    }
}

fn reject_premature_claim(gate: &str, documents: &[&str], errors: &mut Vec<String>) {
    let combined = documents.join("\n").to_lowercase();
    let forbidden_phrases = [
        format!("{gate} is {COVERED_TOKEN}"),
        format!("{gate} {PASSES_TOKEN}"),
        format!("{gate} {PROVEN_TOKEN}"),
        format!("{gate} {CLAIMED_TOKEN}"),
    ];
    for phrase in forbidden_phrases {
        let phrase_lower = phrase.to_lowercase();
        if combined.contains(&phrase_lower) {
            errors.push(format!("{gate} has premature aggregate claim: {phrase}"));
        }
    }
    if gate == PRODUCTION_GATE_NAME && !combined.contains(NON_CLAIM_TOKEN) {
        errors.push("production readiness aggregate lacks explicit non-claim wording".to_string());
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let positive = fixture_inputs();
    validate_gates(&positive)?;

    assert_contains(
        &validate_gates(&GateInputs {
            ctf_ledger: positive.ctf_ledger.replace("simultaneous pickup/capture race | Non-claim", "simultaneous pickup/capture race | Covered"),
            ..positive.clone()
        })
        .expect_err("missing CTF gap fixture should fail"),
        CTF_GATE_NAME,
    )?;

    assert_contains(
        &validate_gates(&GateInputs {
            protocol_ledger: positive.protocol_ledger.replace("all_packets_all_states | Non-claim", "all_packets_all_states | Covered"),
            ..positive.clone()
        })
        .expect_err("missing protocol gap fixture should fail"),
        PROTOCOL_GATE_NAME,
    )?;

    assert_contains(
        &validate_gates(&GateInputs {
            production_matrix: positive.production_matrix.replace("No production readiness", "Production readiness is covered"),
            ..positive.clone()
        })
        .expect_err("production overclaim fixture should fail"),
        PRODUCTION_GATE_NAME,
    )?;

    assert_contains(
        &validate_gates(&GateInputs {
            ctf_ledger: positive.ctf_ledger.replace(
                "full CTF correctness remains a non-claim",
                "full CTF correctness is covered",
            ),
            ..positive.clone()
        })
        .expect_err("full CTF overclaim fixture should fail"),
        CTF_GATE_NAME,
    )?;

    assert_contains(
        &validate_gates(&GateInputs {
            protocol_ledger: positive.protocol_ledger.replace(PROTOCOL_NON_CLAIM, "full protocol-763 compatibility is covered"),
            ..positive.clone()
        })
        .expect_err("full protocol overclaim fixture should fail"),
        PROTOCOL_GATE_NAME,
    )?;

    Ok(format!("{AGGREGATE_GATE_COUNT_TEXT} aggregate gates exercised"))
}

fn fixture_inputs() -> GateInputs {
    GateInputs {
        current_bundle: format!(
            "CTF rule scope is guarded and full CTF correctness remains a non-claim.\nBroad coverage is guarded; full protocol-763 compatibility/full Minecraft compatibility remains blocked.\nThe production/network matrix promotes bounded owned-local loopback load safety. Broader production readiness and unbounded safety remain non-claims."
        ),
        acceptance_matrix: "matrix says full CTF correctness remains a non-claim; full protocol-763 compatibility remains a non-claim; production readiness is a non-claim".to_string(),
        ctf_ledger: format!(
            "full CTF correctness remains a non-claim\n| simultaneous pickup/capture race | Non-claim |\n| spawn/team balance/resource reset | Non-claim |\n| full CTF correctness | Non-claim |\n{}\n",
            CTF_REQUIRED_EVIDENCE_ROWS.join("\n")
        ),
        protocol_ledger: format!(
            "full protocol-763 compatibility remains a non-claim. full Minecraft compatibility remains a non-claim. 175 Valence protocol-763 packet rows. four narrow parser-fixture-backed packet rows.\n{}\n",
            PROTOCOL_REQUIRED_NON_CLAIMS.join("\n")
        ),
        production_matrix: format!(
            "{}\n{}\n",
            PRODUCTION_REQUIRED_MATRIX_ROWS.join("\n"),
            PRODUCTION_REQUIRED_NON_CLAIMS.join("\n")
        ),
    }
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
