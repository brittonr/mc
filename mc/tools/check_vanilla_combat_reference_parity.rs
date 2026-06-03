use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const ROW_FLAG: &str = "--row";
const REFERENCE_FLAG: &str = "--reference";
const VALENCE_FLAG: &str = "--valence";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const PROGRAM_ARGUMENT_COUNT: usize = 1;
const REQUIRED_ARGUMENT_COUNT: usize = 7;
const FLAG_VALUE_STRIDE: usize = 2;
const KEY_VALUE_SEPARATOR: char = '=';
const FLOAT_EPSILON: f64 = 0.000_001;

const NO_ARMOR_ROW_ID: &str = "vanilla-combat-reference-parity";
const ARMOR_ROW_ID: &str = "vanilla-combat-armor-reference-parity";
const ROW_ID: &str = NO_ARMOR_ROW_ID;
const REFERENCE_BACKEND: &str = "paper-reference";
const VALENCE_BACKEND: &str = "valence";
const REFERENCE_ORACLE: &str = "paper-1.20.1-reference-harness";
const REFERENCE_VERSION: &str = "minecraft-1.20.1-protocol-763";
const CLEAN_REVISION_STATUS: &str = "clean";
const UNKNOWN_REVISION: &str = "unknown";
const DIRTY_REVISION: &str = "dirty";

const ROLE_REFERENCE: &str = "reference";
const ROLE_VALENCE: &str = "valence";

const FIELD_ROW: &str = "row";
const FIELD_BACKEND: &str = "backend";
const FIELD_REVISION_STATUS: &str = "revision_status";
const FIELD_CHILD_REVISION: &str = "child_revision";
const FIELD_REFERENCE_ORACLE: &str = "reference_oracle";
const FIELD_REFERENCE_VERSION: &str = "reference_version";
const FIELD_ATTACKER_IDENTITY: &str = "attacker_identity";
const FIELD_VICTIM_IDENTITY: &str = "victim_identity";
const FIELD_WEAPON: &str = "weapon";
const FIELD_ARMOR_STATE: &str = "armor_state";
const FIELD_PRE_HEALTH: &str = "pre_health";
const FIELD_POST_HEALTH: &str = "post_health";
const FIELD_DAMAGE_DELTA: &str = "damage_delta";
const FIELD_KNOCKBACK_METRIC: &str = "knockback_metric";
const FIELD_DAMAGE_TOLERANCE: &str = "damage_tolerance";
const FIELD_KNOCKBACK_TOLERANCE: &str = "knockback_tolerance";

const FIXTURE_CHILD_REVISION: &str = "abc1234";
const FIXTURE_ATTACKER: &str = "compatbota";
const FIXTURE_VICTIM: &str = "compatbotb";
const FIXTURE_WEAPON: &str = "iron_sword";
const FIXTURE_ARMOR_STATE: &str = "none";
const FIXTURE_ARMOR_REFERENCE_ARMOR_STATE: &str = "diamond_chestplate";
const FIXTURE_PRE_HEALTH: &str = "20.0";
const FIXTURE_POST_HEALTH: &str = "14.0";
const FIXTURE_DAMAGE_DELTA: &str = "6.0";
const FIXTURE_ARMOR_REFERENCE_POST_HEALTH: &str = "15.3";
const FIXTURE_ARMOR_REFERENCE_DAMAGE_DELTA: &str = "4.7";
const FIXTURE_KNOCKBACK_METRIC: &str = "0.40";
const FIXTURE_DAMAGE_TOLERANCE: &str = "0.0";
const FIXTURE_KNOCKBACK_TOLERANCE: &str = "0.05";
const FIXTURE_WITHIN_TOLERANCE_KNOCKBACK: &str = "0.44";
const FIXTURE_OUT_OF_TOLERANCE_DAMAGE: &str = "7.0";
const FIXTURE_OUT_OF_TOLERANCE_KNOCKBACK: &str = "0.48";
const FIXTURE_MISMATCHED_WEAPON: &str = "diamond_sword";
const FIXTURE_MISMATCHED_ARMOR_STATE: &str = "diamond_chestplate";
const WRONG_REFERENCE_VERSION: &str = "minecraft-1.18.2-protocol-758";

#[derive(Debug, Clone, Copy, PartialEq)]
struct CombatParityContract {
    row_id: &'static str,
    reference_backend: &'static str,
    valence_backend: &'static str,
    reference_oracle: &'static str,
    reference_version: &'static str,
    expected_weapon: &'static str,
    expected_armor_state: &'static str,
    expected_pre_health: f64,
    expected_post_health: f64,
    expected_damage_delta: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CombatParityRecord {
    fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
struct NormalizedCombatRecord {
    row: String,
    backend: String,
    revision_status: String,
    child_revision: String,
    reference_oracle: String,
    reference_version: String,
    attacker_identity: String,
    victim_identity: String,
    weapon: String,
    armor_state: String,
    pre_health: f64,
    post_health: f64,
    damage_delta: f64,
    knockback_metric: f64,
    damage_tolerance: f64,
    knockback_tolerance: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CombatParityPair {
    reference: Option<CombatParityRecord>,
    valence: Option<CombatParityRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CombatParityDecision {
    passed: bool,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliConfig {
    row: String,
    reference_path: String,
    valence_path: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("vanilla combat reference parity self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match parse_cli(&args).and_then(run_config) {
        Ok(summary) => {
            println!("{summary}");
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
        eprintln!("vanilla combat reference parity check failed: {error}");
    }
}

fn parse_cli(args: &[String]) -> Result<CliConfig, Vec<String>> {
    if args.len() != REQUIRED_ARGUMENT_COUNT {
        return Err(vec![usage()]);
    }

    let mut row = None;
    let mut reference_path = None;
    let mut valence_path = None;
    let mut index = PROGRAM_ARGUMENT_COUNT;
    while index < args.len() {
        let flag = args[index].as_str();
        let Some(value) = args.get(index + PROGRAM_ARGUMENT_COUNT) else {
            return Err(vec![usage()]);
        };
        match flag {
            ROW_FLAG => row = Some(value.clone()),
            REFERENCE_FLAG => reference_path = Some(value.clone()),
            VALENCE_FLAG => valence_path = Some(value.clone()),
            _ => return Err(vec![format!("unknown argument: {flag}")]),
        }
        index += FLAG_VALUE_STRIDE;
    }

    Ok(CliConfig {
        row: row.ok_or_else(|| vec![usage()])?,
        reference_path: reference_path.ok_or_else(|| vec![usage()])?,
        valence_path: valence_path.ok_or_else(|| vec![usage()])?,
    })
}

fn usage() -> String {
    format!(
        "usage: check_vanilla_combat_reference_parity {ROW_FLAG} <row-id> {REFERENCE_FLAG} <reference-evidence> {VALENCE_FLAG} <valence-evidence>"
    )
}

fn run_config(config: CliConfig) -> Result<String, Vec<String>> {
    let contract = contract_for_row(&config.row)?;

    let reference_text = read_file(&config.reference_path)?;
    let valence_text = read_file(&config.valence_path)?;
    let reference = parse_evidence(&reference_text)?;
    let valence = parse_evidence(&valence_text)?;
    let pair = CombatParityPair {
        reference: Some(reference),
        valence: Some(valence),
    };
    let decision = compare_combat_parity(&pair, &contract);
    if decision.passed {
        Ok("vanilla combat reference parity row check passed".to_string())
    } else {
        Err(decision.diagnostics)
    }
}

fn read_file(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|error| vec![format!("{path}: {error}")])
}

fn default_contract() -> CombatParityContract {
    CombatParityContract {
        row_id: ROW_ID,
        reference_backend: REFERENCE_BACKEND,
        valence_backend: VALENCE_BACKEND,
        reference_oracle: REFERENCE_ORACLE,
        reference_version: REFERENCE_VERSION,
        expected_weapon: FIXTURE_WEAPON,
        expected_armor_state: FIXTURE_ARMOR_STATE,
        expected_pre_health: parse_contract_number(FIXTURE_PRE_HEALTH),
        expected_post_health: parse_contract_number(FIXTURE_POST_HEALTH),
        expected_damage_delta: parse_contract_number(FIXTURE_DAMAGE_DELTA),
    }
}

fn armor_contract() -> CombatParityContract {
    CombatParityContract {
        row_id: ARMOR_ROW_ID,
        reference_backend: REFERENCE_BACKEND,
        valence_backend: VALENCE_BACKEND,
        reference_oracle: REFERENCE_ORACLE,
        reference_version: REFERENCE_VERSION,
        expected_weapon: FIXTURE_WEAPON,
        expected_armor_state: FIXTURE_ARMOR_REFERENCE_ARMOR_STATE,
        expected_pre_health: parse_contract_number(FIXTURE_PRE_HEALTH),
        expected_post_health: parse_contract_number(FIXTURE_ARMOR_REFERENCE_POST_HEALTH),
        expected_damage_delta: parse_contract_number(FIXTURE_ARMOR_REFERENCE_DAMAGE_DELTA),
    }
}

fn contract_for_row(row: &str) -> Result<CombatParityContract, Vec<String>> {
    match row {
        NO_ARMOR_ROW_ID => Ok(default_contract()),
        ARMOR_ROW_ID => Ok(armor_contract()),
        _ => Err(vec![format!("unknown row: {row}")]),
    }
}

fn parse_contract_number(value: &str) -> f64 {
    value
        .parse::<f64>()
        .expect("contract fixture number is valid")
}

fn parse_evidence(text: &str) -> Result<CombatParityRecord, Vec<String>> {
    let mut fields = BTreeMap::new();
    let mut errors = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once(KEY_VALUE_SEPARATOR) else {
            errors.push(format!("expected key=value line: {trimmed}"));
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() {
            errors.push(format!("empty key in line: {trimmed}"));
            continue;
        }
        fields.insert(key.to_string(), value.to_string());
    }

    if errors.is_empty() {
        Ok(CombatParityRecord { fields })
    } else {
        Err(errors)
    }
}

fn compare_combat_parity(
    pair: &CombatParityPair,
    contract: &CombatParityContract,
) -> CombatParityDecision {
    let mut diagnostics = Vec::new();

    if pair.reference.is_none() {
        diagnostics.push("missing_reference".to_string());
    }
    if pair.valence.is_none() {
        diagnostics.push("missing_valence".to_string());
    }

    let reference = pair
        .reference
        .as_ref()
        .and_then(|record| normalize_record(ROLE_REFERENCE, record, &mut diagnostics));
    let valence = pair
        .valence
        .as_ref()
        .and_then(|record| normalize_record(ROLE_VALENCE, record, &mut diagnostics));

    if let Some(record) = &reference {
        validate_record_contract(
            ROLE_REFERENCE,
            record,
            contract.reference_backend,
            contract,
            &mut diagnostics,
        );
    }
    if let Some(record) = &valence {
        validate_record_contract(
            ROLE_VALENCE,
            record,
            contract.valence_backend,
            contract,
            &mut diagnostics,
        );
    }

    if !raw_backend_is(pair.reference.as_ref(), contract.reference_backend)
        && raw_backend_present(
            pair.reference.as_ref(),
            pair.valence.as_ref(),
            contract.valence_backend,
        )
    {
        diagnostics.push("valence_only_evidence".to_string());
    }

    if let (Some(reference), Some(valence)) = (&reference, &valence) {
        compare_shared_metrics(reference, valence, &mut diagnostics);
        compare_numeric_metrics(reference, valence, &mut diagnostics);
    }

    CombatParityDecision {
        passed: diagnostics.is_empty(),
        diagnostics,
    }
}

fn normalize_record(
    role: &str,
    record: &CombatParityRecord,
    diagnostics: &mut Vec<String>,
) -> Option<NormalizedCombatRecord> {
    let row = required_string(role, &record.fields, FIELD_ROW, diagnostics);
    let backend = required_string(role, &record.fields, FIELD_BACKEND, diagnostics);
    let revision_status = required_string(role, &record.fields, FIELD_REVISION_STATUS, diagnostics);
    let child_revision = required_string(role, &record.fields, FIELD_CHILD_REVISION, diagnostics);
    let reference_oracle =
        required_string(role, &record.fields, FIELD_REFERENCE_ORACLE, diagnostics);
    let reference_version =
        required_string(role, &record.fields, FIELD_REFERENCE_VERSION, diagnostics);
    let attacker_identity =
        required_string(role, &record.fields, FIELD_ATTACKER_IDENTITY, diagnostics);
    let victim_identity = required_string(role, &record.fields, FIELD_VICTIM_IDENTITY, diagnostics);
    let weapon = required_string(role, &record.fields, FIELD_WEAPON, diagnostics);
    let armor_state = required_string(role, &record.fields, FIELD_ARMOR_STATE, diagnostics);
    let pre_health = required_number(role, &record.fields, FIELD_PRE_HEALTH, diagnostics);
    let post_health = required_number(role, &record.fields, FIELD_POST_HEALTH, diagnostics);
    let damage_delta = required_number(role, &record.fields, FIELD_DAMAGE_DELTA, diagnostics);
    let knockback_metric =
        required_number(role, &record.fields, FIELD_KNOCKBACK_METRIC, diagnostics);
    let damage_tolerance =
        required_tolerance(role, &record.fields, FIELD_DAMAGE_TOLERANCE, diagnostics);
    let knockback_tolerance =
        required_tolerance(role, &record.fields, FIELD_KNOCKBACK_TOLERANCE, diagnostics);

    let Some(row) = row else { return None };
    let Some(backend) = backend else { return None };
    let Some(revision_status) = revision_status else {
        return None;
    };
    let Some(child_revision) = child_revision else {
        return None;
    };
    let Some(reference_oracle) = reference_oracle else {
        return None;
    };
    let Some(reference_version) = reference_version else {
        return None;
    };
    let Some(attacker_identity) = attacker_identity else {
        return None;
    };
    let Some(victim_identity) = victim_identity else {
        return None;
    };
    let Some(weapon) = weapon else { return None };
    let Some(armor_state) = armor_state else {
        return None;
    };
    let Some(pre_health) = pre_health else {
        return None;
    };
    let Some(post_health) = post_health else {
        return None;
    };
    let Some(damage_delta) = damage_delta else {
        return None;
    };
    let Some(knockback_metric) = knockback_metric else {
        return None;
    };
    let Some(damage_tolerance) = damage_tolerance else {
        return None;
    };
    let Some(knockback_tolerance) = knockback_tolerance else {
        return None;
    };

    Some(NormalizedCombatRecord {
        row,
        backend,
        revision_status,
        child_revision,
        reference_oracle,
        reference_version,
        attacker_identity,
        victim_identity,
        weapon,
        armor_state,
        pre_health,
        post_health,
        damage_delta,
        knockback_metric,
        damage_tolerance,
        knockback_tolerance,
    })
}

fn required_string(
    role: &str,
    fields: &BTreeMap<String, String>,
    field: &str,
    diagnostics: &mut Vec<String>,
) -> Option<String> {
    match fields.get(field) {
        Some(value) if !value.is_empty() => Some(value.clone()),
        _ => {
            diagnostics.push(format!("missing_{field}:{role}"));
            None
        }
    }
}

fn required_number(
    role: &str,
    fields: &BTreeMap<String, String>,
    field: &str,
    diagnostics: &mut Vec<String>,
) -> Option<f64> {
    let value = required_string(role, fields, field, diagnostics)?;
    match value.parse::<f64>() {
        Ok(number) => Some(number),
        Err(error) => {
            diagnostics.push(format!("invalid_{field}:{role}:{error}"));
            None
        }
    }
}

fn required_tolerance(
    role: &str,
    fields: &BTreeMap<String, String>,
    field: &str,
    diagnostics: &mut Vec<String>,
) -> Option<f64> {
    let tolerance = required_number(role, fields, field, diagnostics)?;
    if tolerance < 0.0 {
        diagnostics.push(format!("negative_{field}:{role}"));
        None
    } else {
        Some(tolerance)
    }
}

fn validate_record_contract(
    role: &str,
    record: &NormalizedCombatRecord,
    expected_backend: &str,
    contract: &CombatParityContract,
    diagnostics: &mut Vec<String>,
) {
    if record.row != contract.row_id {
        diagnostics.push(format!("wrong_row:{role}:{}", record.row));
    }
    if record.backend != expected_backend {
        diagnostics.push(format!("expected_{role}_backend:{}", record.backend));
    }
    if record.revision_status != CLEAN_REVISION_STATUS {
        diagnostics.push(format!("stale_revision:{role}:{}", record.revision_status));
    }
    if record.child_revision == UNKNOWN_REVISION || record.child_revision == DIRTY_REVISION {
        diagnostics.push(format!("missing_child_revision:{role}"));
    }
    if record.reference_oracle != contract.reference_oracle {
        diagnostics.push(format!(
            "wrong_reference_oracle:{role}:{}",
            record.reference_oracle
        ));
    }
    if record.reference_version != contract.reference_version {
        diagnostics.push(format!(
            "wrong_reference_version:{role}:{}",
            record.reference_version
        ));
    }
    validate_expected_string(
        role,
        FIELD_WEAPON,
        &record.weapon,
        contract.expected_weapon,
        diagnostics,
    );
    validate_expected_string(
        role,
        FIELD_ARMOR_STATE,
        &record.armor_state,
        contract.expected_armor_state,
        diagnostics,
    );
    validate_expected_number(
        role,
        FIELD_PRE_HEALTH,
        record.pre_health,
        contract.expected_pre_health,
        record.damage_tolerance,
        diagnostics,
    );
    validate_expected_number(
        role,
        FIELD_POST_HEALTH,
        record.post_health,
        contract.expected_post_health,
        record.damage_tolerance,
        diagnostics,
    );
    validate_expected_number(
        role,
        FIELD_DAMAGE_DELTA,
        record.damage_delta,
        contract.expected_damage_delta,
        record.damage_tolerance,
        diagnostics,
    );
}

fn validate_expected_string(
    role: &str,
    field: &str,
    actual: &str,
    expected: &str,
    diagnostics: &mut Vec<String>,
) {
    if actual != expected {
        diagnostics.push(format!(
            "unexpected_{field}:{role}:expected={expected}:actual={actual}"
        ));
    }
}

fn validate_expected_number(
    role: &str,
    field: &str,
    actual: f64,
    expected: f64,
    tolerance: f64,
    diagnostics: &mut Vec<String>,
) {
    if exceeds_tolerance(expected, actual, tolerance) {
        diagnostics.push(format!(
            "unexpected_{field}:{role}:expected={expected}:actual={actual}:tolerance={tolerance}"
        ));
    }
}

fn raw_backend_is(record: Option<&CombatParityRecord>, expected_backend: &str) -> bool {
    record
        .and_then(|record| record.fields.get(FIELD_BACKEND))
        .map(|backend| backend == expected_backend)
        .unwrap_or(false)
}

fn raw_backend_present(
    reference: Option<&CombatParityRecord>,
    valence: Option<&CombatParityRecord>,
    expected_backend: &str,
) -> bool {
    raw_backend_is(reference, expected_backend) || raw_backend_is(valence, expected_backend)
}

fn compare_shared_metrics(
    reference: &NormalizedCombatRecord,
    valence: &NormalizedCombatRecord,
    diagnostics: &mut Vec<String>,
) {
    compare_string_metric(
        FIELD_ATTACKER_IDENTITY,
        &reference.attacker_identity,
        &valence.attacker_identity,
        diagnostics,
    );
    compare_string_metric(
        FIELD_VICTIM_IDENTITY,
        &reference.victim_identity,
        &valence.victim_identity,
        diagnostics,
    );
    compare_string_metric(
        FIELD_WEAPON,
        &reference.weapon,
        &valence.weapon,
        diagnostics,
    );
    compare_string_metric(
        FIELD_ARMOR_STATE,
        &reference.armor_state,
        &valence.armor_state,
        diagnostics,
    );
}

fn compare_string_metric(
    field: &str,
    reference: &str,
    valence: &str,
    diagnostics: &mut Vec<String>,
) {
    if reference != valence {
        diagnostics.push(format!(
            "mismatched_{field}:reference={reference}:valence={valence}"
        ));
    }
}

fn compare_numeric_metrics(
    reference: &NormalizedCombatRecord,
    valence: &NormalizedCombatRecord,
    diagnostics: &mut Vec<String>,
) {
    if exceeds_tolerance(
        reference.damage_tolerance,
        valence.damage_tolerance,
        FLOAT_EPSILON,
    ) {
        diagnostics.push(format!(
            "mismatched_damage_tolerance:reference={}:valence={}",
            reference.damage_tolerance, valence.damage_tolerance
        ));
    }
    if exceeds_tolerance(
        reference.knockback_tolerance,
        valence.knockback_tolerance,
        FLOAT_EPSILON,
    ) {
        diagnostics.push(format!(
            "mismatched_knockback_tolerance:reference={}:valence={}",
            reference.knockback_tolerance, valence.knockback_tolerance
        ));
    }

    compare_number_metric(
        FIELD_PRE_HEALTH,
        reference.pre_health,
        valence.pre_health,
        reference.damage_tolerance,
        diagnostics,
    );
    compare_number_metric(
        FIELD_POST_HEALTH,
        reference.post_health,
        valence.post_health,
        reference.damage_tolerance,
        diagnostics,
    );
    compare_number_metric(
        FIELD_DAMAGE_DELTA,
        reference.damage_delta,
        valence.damage_delta,
        reference.damage_tolerance,
        diagnostics,
    );
    compare_number_metric(
        FIELD_KNOCKBACK_METRIC,
        reference.knockback_metric,
        valence.knockback_metric,
        reference.knockback_tolerance,
        diagnostics,
    );
}

fn compare_number_metric(
    field: &str,
    reference: f64,
    valence: f64,
    tolerance: f64,
    diagnostics: &mut Vec<String>,
) {
    if exceeds_tolerance(reference, valence, tolerance) {
        diagnostics.push(format!(
            "{field}_out_of_tolerance:reference={reference}:valence={valence}:tolerance={tolerance}"
        ));
    }
}

fn exceeds_tolerance(reference: f64, valence: f64, tolerance: f64) -> bool {
    (reference - valence).abs() - tolerance > FLOAT_EPSILON
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let contract = default_contract();
    let armor_contract = armor_contract();
    assert!(contract_for_row(NO_ARMOR_ROW_ID).is_ok());
    assert!(contract_for_row(ARMOR_ROW_ID).is_ok());
    assert_contains_error(contract_for_row("unknown-combat-row"), "unknown row")?;

    let reference = parse_evidence(&fixture_evidence(REFERENCE_BACKEND, &contract))?;
    let valence = parse_evidence(&fixture_evidence(VALENCE_BACKEND, &contract))?;
    let armor_reference = parse_evidence(&fixture_evidence(REFERENCE_BACKEND, &armor_contract))?;
    let armor_valence = parse_evidence(&fixture_evidence(VALENCE_BACKEND, &armor_contract))?;
    assert_passes(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(reference.clone()),
                valence: Some(valence.clone()),
            },
            &contract,
        ),
        "valid paired evidence",
    )?;

    assert_passes(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(armor_reference.clone()),
                valence: Some(armor_valence.clone()),
            },
            &armor_contract,
        ),
        "valid paired armor evidence",
    )?;

    let within_tolerance = parse_evidence(&fixture_evidence(VALENCE_BACKEND, &contract).replace(
        &format!("{FIELD_KNOCKBACK_METRIC}={FIXTURE_KNOCKBACK_METRIC}"),
        &format!("{FIELD_KNOCKBACK_METRIC}={FIXTURE_WITHIN_TOLERANCE_KNOCKBACK}"),
    ))?;
    assert_passes(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(reference.clone()),
                valence: Some(within_tolerance),
            },
            &contract,
        ),
        "within-tolerance knockback evidence",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: None,
                valence: Some(valence.clone()),
            },
            &contract,
        ),
        "missing_reference",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(valence.clone()),
                valence: Some(valence.clone()),
            },
            &contract,
        ),
        "valence_only_evidence",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(replace_field(
                    &reference,
                    FIELD_REFERENCE_VERSION,
                    WRONG_REFERENCE_VERSION,
                )),
                valence: Some(valence.clone()),
            },
            &contract,
        ),
        "wrong_reference_version",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(remove_field(&reference, FIELD_DAMAGE_TOLERANCE)),
                valence: Some(valence.clone()),
            },
            &contract,
        ),
        "missing_damage_tolerance",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(reference.clone()),
                valence: Some(replace_field(
                    &valence,
                    FIELD_DAMAGE_DELTA,
                    FIXTURE_OUT_OF_TOLERANCE_DAMAGE,
                )),
            },
            &contract,
        ),
        "damage_delta_out_of_tolerance",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(reference.clone()),
                valence: Some(replace_field(
                    &valence,
                    FIELD_KNOCKBACK_METRIC,
                    FIXTURE_OUT_OF_TOLERANCE_KNOCKBACK,
                )),
            },
            &contract,
        ),
        "knockback_metric_out_of_tolerance",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(reference.clone()),
                valence: Some(replace_field(
                    &valence,
                    FIELD_REVISION_STATUS,
                    DIRTY_REVISION,
                )),
            },
            &contract,
        ),
        "stale_revision",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(reference.clone()),
                valence: Some(replace_field(
                    &valence,
                    FIELD_WEAPON,
                    FIXTURE_MISMATCHED_WEAPON,
                )),
            },
            &contract,
        ),
        "mismatched_weapon",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(reference.clone()),
                valence: Some(replace_field(
                    &valence,
                    FIELD_ARMOR_STATE,
                    FIXTURE_MISMATCHED_ARMOR_STATE,
                )),
            },
            &contract,
        ),
        "mismatched_armor_state",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(armor_reference.clone()),
                valence: Some(replace_field(
                    &armor_valence,
                    FIELD_ARMOR_STATE,
                    FIXTURE_ARMOR_STATE,
                )),
            },
            &armor_contract,
        ),
        "unexpected_armor_state",
    )?;

    assert_contains(
        compare_combat_parity(
            &CombatParityPair {
                reference: Some(armor_reference),
                valence: Some(replace_field(
                    &armor_valence,
                    FIELD_DAMAGE_DELTA,
                    FIXTURE_DAMAGE_DELTA,
                )),
            },
            &armor_contract,
        ),
        "unexpected_damage_delta",
    )?;

    Ok("positive and negative fixtures exercised".to_string())
}

fn fixture_evidence(backend: &str, contract: &CombatParityContract) -> String {
    format!(
        "{FIELD_ROW}={}\n{FIELD_BACKEND}={backend}\n{FIELD_REVISION_STATUS}={CLEAN_REVISION_STATUS}\n{FIELD_CHILD_REVISION}={FIXTURE_CHILD_REVISION}\n{FIELD_REFERENCE_ORACLE}={REFERENCE_ORACLE}\n{FIELD_REFERENCE_VERSION}={REFERENCE_VERSION}\n{FIELD_ATTACKER_IDENTITY}={FIXTURE_ATTACKER}\n{FIELD_VICTIM_IDENTITY}={FIXTURE_VICTIM}\n{FIELD_WEAPON}={}\n{FIELD_ARMOR_STATE}={}\n{FIELD_PRE_HEALTH}={:.1}\n{FIELD_POST_HEALTH}={:.1}\n{FIELD_DAMAGE_DELTA}={:.1}\n{FIELD_KNOCKBACK_METRIC}={FIXTURE_KNOCKBACK_METRIC}\n{FIELD_DAMAGE_TOLERANCE}={FIXTURE_DAMAGE_TOLERANCE}\n{FIELD_KNOCKBACK_TOLERANCE}={FIXTURE_KNOCKBACK_TOLERANCE}\n",
        contract.row_id,
        contract.expected_weapon,
        contract.expected_armor_state,
        contract.expected_pre_health,
        contract.expected_post_health,
        contract.expected_damage_delta,
    )
}

fn replace_field(record: &CombatParityRecord, field: &str, value: &str) -> CombatParityRecord {
    let mut fields = record.fields.clone();
    fields.insert(field.to_string(), value.to_string());
    CombatParityRecord { fields }
}

fn remove_field(record: &CombatParityRecord, field: &str) -> CombatParityRecord {
    let mut fields = record.fields.clone();
    fields.remove(field);
    CombatParityRecord { fields }
}

fn assert_passes(decision: CombatParityDecision, fixture_name: &str) -> Result<(), Vec<String>> {
    if decision.passed {
        Ok(())
    } else {
        Err(vec![format!(
            "{fixture_name} should pass: {:?}",
            decision.diagnostics
        )])
    }
}

fn assert_contains(decision: CombatParityDecision, needle: &str) -> Result<(), Vec<String>> {
    if !decision.passed
        && decision
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains(needle))
    {
        Ok(())
    } else {
        Err(vec![format!(
            "missing expected diagnostic {needle:?}: {:?}",
            decision.diagnostics
        )])
    }
}

fn assert_contains_error<T>(
    result: Result<T, Vec<String>>,
    needle: &str,
) -> Result<(), Vec<String>> {
    match result {
        Ok(_) => Err(vec![format!("expected error containing {needle:?}")]),
        Err(errors) if errors.iter().any(|error| error.contains(needle)) => Ok(()),
        Err(errors) => Err(vec![format!(
            "missing expected error {needle:?}: {errors:?}"
        )]),
    }
}
