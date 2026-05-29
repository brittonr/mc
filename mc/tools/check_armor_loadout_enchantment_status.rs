#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const CONTRACT_FLAG: &str = "--contract";
const RECORD_FLAG: &str = "--record";
const CONTRACT_DOC: &str =
    "docs/evidence/protocol-763-armor-loadout-enchantment-status-contract-2026-05-29.md";
const EXPECTED_EXISTING_ROW_ID: &str = "chest_diamond_none_none_melee";
const EXPECTED_FUTURE_ROW_ID: &str = "full_diamond_none_none_melee";
const EXPECTED_PROTECTION_ROW_ID: &str = "chest_diamond_protection_i_none_melee";
const EXPECTED_RESISTANCE_ROW_ID: &str = "chest_diamond_none_resistance_i_melee";
const EXPECTED_EXISTING_LOADOUT: &str = "armor_loadout_chest_only";
const EXPECTED_FULL_LOADOUT: &str = "armor_loadout_full_diamond";
const EXPECTED_CHEST_SLOT: &str = "chest=DiamondChestplate";
const EXPECTED_FULL_SLOTS: &str =
    "head=DiamondHelmet;chest=DiamondChestplate;legs=DiamondLeggings;feet=DiamondBoots";
const EXPECTED_NO_ENCHANTMENT: &str = "enchantment_none";
const EXPECTED_NO_STATUS_EFFECT: &str = "status_effect_none";
const EXPECTED_MELEE_ATTACK: &str = "melee";
const EXPECTED_REFERENCE_NONE: &str = "none";
const EXISTING_BASE_DAMAGE_MILLI: i32 = 4000;
const EXISTING_FINAL_DAMAGE_MILLI: i32 = 2000;
const EXISTING_FINAL_DAMAGE_MILLI_PLUS_MISMATCH: i32 = 3000;
const EXISTING_MITIGATION_DELTA_MILLI: i32 = 2000;
const EXISTING_HEALTH_PRE_MILLI: i32 = 20000;
const EXISTING_HEALTH_POST_MILLI: i32 = 18000;
const DEFAULT_TOLERANCE_MILLI: i32 = 1;
const ZERO_VALUE: i32 = 0;
const KEY_VALUE_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_EXISTING_ROW_ID,
    EXPECTED_FUTURE_ROW_ID,
    EXPECTED_PROTECTION_ROW_ID,
    EXPECTED_RESISTANCE_ROW_ID,
    "missing_matrix_row_field",
    "missing_equipment_evidence",
    "mismatched_damage_delta",
    "absent_tolerance",
    "unpaired_vanilla_parity",
    "all_loadout_overclaim",
    "all armor permutations",
];

const REQUIRED_FIELDS: &[&str] = &[
    "row.id",
    "row.promotion_label",
    "victim.loadout_id",
    "victim.equipment_slots",
    "victim.enchantments",
    "victim.status_effects",
    "attack.type",
    "health.pre_milli",
    "health.post_milli",
    "damage.base_milli",
    "damage.final_milli",
    "mitigation.delta_milli",
    "tolerance.absolute_milli",
    "reference.required",
    "reference.receipt",
    "valence.receipt",
    "server.milestone.equipment_state",
    "server.milestone.armor_mitigation",
    "client.milestone.health_update",
    "claims.all_loadouts",
    "claims.all_enchantments",
    "claims.all_status_effects",
    "claims.exact_vanilla_parity",
    "claims.full_combat_correctness",
];

const FORBIDDEN_TRUE_CLAIMS: &[&str] = &[
    "claims.all_loadouts",
    "claims.all_enchantments",
    "claims.all_status_effects",
    "claims.exact_vanilla_parity",
    "claims.full_combat_correctness",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    self_test: bool,
    contract_path: String,
    record_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatrixRecord {
    values: BTreeMap<String, String>,
}

fn main() -> ExitCode {
    let args = match parse_args(env::args().skip(1)) {
        Ok(parsed) => parsed,
        Err(errors) => return exit_with_errors(&errors),
    };

    if args.self_test {
        return match run_self_tests() {
            Ok(summary) => {
                println!("armor loadout/enchantment/status checker self-test passed: {summary}");
                SUCCESS
            }
            Err(errors) => exit_with_errors(&errors),
        };
    }

    match run_repo_check(&args) {
        Ok(summary) => {
            println!("armor loadout/enchantment/status checker passed: {summary}");
            SUCCESS
        }
        Err(errors) => exit_with_errors(&errors),
    }
}

fn parse_args<I>(args: I) -> Result<Args, Vec<String>>
where
    I: IntoIterator<Item = String>,
{
    let mut parsed = Args {
        self_test: false,
        contract_path: CONTRACT_DOC.to_string(),
        record_path: None,
    };
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        if arg == SELF_TEST_FLAG {
            parsed.self_test = true;
        } else if arg == CONTRACT_FLAG {
            parsed.contract_path = next_value(&mut iter, CONTRACT_FLAG)?;
        } else if arg == RECORD_FLAG {
            parsed.record_path = Some(next_value(&mut iter, RECORD_FLAG)?);
        } else {
            return Err(vec![format!("unknown argument: {arg}")]);
        }
    }
    Ok(parsed)
}

fn next_value<I>(iter: &mut I, flag: &str) -> Result<String, Vec<String>>
where
    I: Iterator<Item = String>,
{
    match iter.next() {
        Some(value) => Ok(value),
        None => Err(vec![format!("missing value for {flag}")]),
    }
}

fn run_repo_check(args: &Args) -> Result<String, Vec<String>> {
    let contract_text = read_text(&args.contract_path)?;
    let mut errors = validate_contract_text(&contract_text);
    if let Some(record_path) = &args.record_path {
        let record_text = read_text(record_path)?;
        let record = parse_record(&record_text)?;
        errors.extend(validate_matrix_record(&record));
    }
    if errors.is_empty() {
        Ok(format!(
            "contract={} record={}",
            args.contract_path,
            args.record_path.is_some()
        ))
    } else {
        Err(errors)
    }
}

fn read_text(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|err| vec![format!("{path}: {err}")])
}

fn validate_contract_text(text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for token in CONTRACT_TOKENS {
        if !text.contains(token) {
            errors.push(format!("contract missing token: {token}"));
        }
    }
    errors
}

fn parse_record(text: &str) -> Result<MatrixRecord, Vec<String>> {
    let mut values = BTreeMap::new();
    let mut errors = Vec::new();
    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line.split_once(KEY_VALUE_SEPARATOR) {
            Some((key, value)) => insert_record_value(&mut values, &mut errors, key, value),
            None => errors.push(format!(
                "line {} missing key=value separator",
                line_index + 1
            )),
        }
    }
    if errors.is_empty() {
        Ok(MatrixRecord { values })
    } else {
        Err(errors)
    }
}

fn insert_record_value(
    values: &mut BTreeMap<String, String>,
    errors: &mut Vec<String>,
    key: &str,
    value: &str,
) {
    let normalized_key = key.trim().to_string();
    let normalized_value = value.trim().to_string();
    if normalized_key.is_empty() {
        errors.push("empty key".to_string());
        return;
    }
    if values
        .insert(normalized_key.clone(), normalized_value)
        .is_some()
    {
        errors.push(format!("duplicate key: {normalized_key}"));
    }
}

fn validate_matrix_record(record: &MatrixRecord) -> Vec<String> {
    let mut errors = Vec::new();
    require_all_fields(&mut errors, record);
    require_allowed_row(&mut errors, record);
    require_equipment_evidence(&mut errors, record);
    require_damage_equations(&mut errors, record);
    require_reference_pairing(&mut errors, record);
    for claim in FORBIDDEN_TRUE_CLAIMS {
        require_false(&mut errors, record, claim, "all_loadout_overclaim");
    }
    errors
}

fn require_all_fields(errors: &mut Vec<String>, record: &MatrixRecord) {
    for field in REQUIRED_FIELDS {
        if !record.values.contains_key(*field) {
            errors.push(format!("missing_matrix_row_field: {field}"));
        }
    }
}

fn require_allowed_row(errors: &mut Vec<String>, record: &MatrixRecord) {
    let row_id = value(record, "row.id");
    let allowed_rows = split_csv(&format!(
        "{EXPECTED_EXISTING_ROW_ID},{EXPECTED_FUTURE_ROW_ID},{EXPECTED_PROTECTION_ROW_ID},{EXPECTED_RESISTANCE_ROW_ID}"
    ));
    if !allowed_rows.contains(row_id) {
        errors.push(format!("missing_matrix_row_field: unknown row.id {row_id}"));
    }
    if row_id == EXPECTED_EXISTING_ROW_ID {
        require_text(
            errors,
            record,
            "victim.loadout_id",
            EXPECTED_EXISTING_LOADOUT,
            "missing_matrix_row_field",
        );
        require_text(
            errors,
            record,
            "victim.equipment_slots",
            EXPECTED_CHEST_SLOT,
            "missing_equipment_evidence",
        );
    }
    if row_id == EXPECTED_FUTURE_ROW_ID {
        require_text(
            errors,
            record,
            "victim.loadout_id",
            EXPECTED_FULL_LOADOUT,
            "missing_matrix_row_field",
        );
        require_text(
            errors,
            record,
            "victim.equipment_slots",
            EXPECTED_FULL_SLOTS,
            "missing_equipment_evidence",
        );
    }
    require_text(
        errors,
        record,
        "attack.type",
        EXPECTED_MELEE_ATTACK,
        "missing_matrix_row_field",
    );
}

fn require_equipment_evidence(errors: &mut Vec<String>, record: &MatrixRecord) {
    require_present(
        errors,
        record,
        "server.milestone.equipment_state",
        "missing_equipment_evidence",
    );
    require_present(
        errors,
        record,
        "server.milestone.armor_mitigation",
        "missing_equipment_evidence",
    );
    require_present(
        errors,
        record,
        "client.milestone.health_update",
        "missing_equipment_evidence",
    );
    require_present(
        errors,
        record,
        "valence.receipt",
        "missing_equipment_evidence",
    );
    require_text(
        errors,
        record,
        "victim.enchantments",
        EXPECTED_NO_ENCHANTMENT,
        "missing_matrix_row_field",
    );
    require_text(
        errors,
        record,
        "victim.status_effects",
        EXPECTED_NO_STATUS_EFFECT,
        "missing_matrix_row_field",
    );
}

fn require_damage_equations(errors: &mut Vec<String>, record: &MatrixRecord) {
    let tolerance = parse_i32(
        errors,
        record,
        "tolerance.absolute_milli",
        "absent_tolerance",
    );
    if tolerance <= ZERO_VALUE {
        errors.push("absent_tolerance: tolerance.absolute_milli must be positive".to_string());
    }
    let base = parse_i32(
        errors,
        record,
        "damage.base_milli",
        "missing_matrix_row_field",
    );
    let final_damage = parse_i32(
        errors,
        record,
        "damage.final_milli",
        "missing_matrix_row_field",
    );
    let mitigation = parse_i32(
        errors,
        record,
        "mitigation.delta_milli",
        "missing_matrix_row_field",
    );
    let pre_health = parse_i32(
        errors,
        record,
        "health.pre_milli",
        "missing_matrix_row_field",
    );
    let post_health = parse_i32(
        errors,
        record,
        "health.post_milli",
        "missing_matrix_row_field",
    );
    if (base - final_damage - mitigation).abs() > tolerance {
        errors.push("mismatched_damage_delta: base - final != mitigation".to_string());
    }
    if (pre_health - final_damage - post_health).abs() > tolerance {
        errors.push("mismatched_damage_delta: pre_health - final != post_health".to_string());
    }
}

fn require_reference_pairing(errors: &mut Vec<String>, record: &MatrixRecord) {
    match value(record, "reference.required") {
        "true" => require_present(
            errors,
            record,
            "reference.receipt",
            "unpaired_vanilla_parity",
        ),
        "false" => require_text(
            errors,
            record,
            "reference.receipt",
            EXPECTED_REFERENCE_NONE,
            "unpaired_vanilla_parity",
        ),
        other => errors.push(format!(
            "unpaired_vanilla_parity: invalid reference.required {other}"
        )),
    }
}

fn require_text(
    errors: &mut Vec<String>,
    record: &MatrixRecord,
    key: &str,
    expected: &str,
    code: &str,
) {
    let actual = value(record, key);
    if actual != expected {
        errors.push(format!("{code}: {key} expected {expected}, found {actual}"));
    }
}

fn require_present(errors: &mut Vec<String>, record: &MatrixRecord, key: &str, code: &str) {
    let actual = value(record, key);
    if actual.is_empty() || actual == EXPECTED_REFERENCE_NONE {
        errors.push(format!("{code}: {key} missing"));
    }
}

fn require_false(errors: &mut Vec<String>, record: &MatrixRecord, key: &str, code: &str) {
    match value(record, key) {
        "false" => {}
        actual => errors.push(format!("{code}: {key} expected false, found {actual}")),
    }
}

fn parse_i32(errors: &mut Vec<String>, record: &MatrixRecord, key: &str, code: &str) -> i32 {
    match value(record, key).parse::<i32>() {
        Ok(parsed) => parsed,
        Err(_) => {
            errors.push(format!("{code}: {key} is not an integer"));
            ZERO_VALUE
        }
    }
}

fn value<'a>(record: &'a MatrixRecord, key: &str) -> &'a str {
    match record.values.get(key) {
        Some(value) => value.as_str(),
        None => "",
    }
}

fn split_csv(value: &str) -> BTreeSet<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn positive_record_text() -> String {
    format!(
        "row.id={EXPECTED_EXISTING_ROW_ID}\n\
         row.promotion_label=existing_bounded_valence_only_containment\n\
         victim.loadout_id={EXPECTED_EXISTING_LOADOUT}\n\
         victim.equipment_slots={EXPECTED_CHEST_SLOT}\n\
         victim.enchantments={EXPECTED_NO_ENCHANTMENT}\n\
         victim.status_effects={EXPECTED_NO_STATUS_EFFECT}\n\
         attack.type={EXPECTED_MELEE_ATTACK}\n\
         health.pre_milli={EXISTING_HEALTH_PRE_MILLI}\n\
         health.post_milli={EXISTING_HEALTH_POST_MILLI}\n\
         damage.base_milli={EXISTING_BASE_DAMAGE_MILLI}\n\
         damage.final_milli={EXISTING_FINAL_DAMAGE_MILLI}\n\
         mitigation.delta_milli={EXISTING_MITIGATION_DELTA_MILLI}\n\
         tolerance.absolute_milli={DEFAULT_TOLERANCE_MILLI}\n\
         reference.required=false\n\
         reference.receipt={EXPECTED_REFERENCE_NONE}\n\
         valence.receipt=docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.receipt.json\n\
         server.milestone.equipment_state=server_equipment_state\n\
         server.milestone.armor_mitigation=server_armor_mitigation\n\
         client.milestone.health_update=combat_health_update\n\
         claims.all_loadouts=false\n\
         claims.all_enchantments=false\n\
         claims.all_status_effects=false\n\
         claims.exact_vanilla_parity=false\n\
         claims.full_combat_correctness=false\n"
    )
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut errors = Vec::new();
    errors.extend(expect_record_ok(
        "positive fixture",
        &positive_record_text(),
    ));
    errors.extend(expect_record_error(
        "missing field",
        &format!("row.id={EXPECTED_EXISTING_ROW_ID}\n"),
        "",
        "missing_matrix_row_field",
    ));
    errors.extend(expect_record_error(
        "missing equipment",
        EXPECTED_CHEST_SLOT,
        "",
        "missing_equipment_evidence",
    ));
    errors.extend(expect_record_error(
        "damage mismatch",
        &format!("damage.final_milli={EXISTING_FINAL_DAMAGE_MILLI}"),
        &format!("damage.final_milli={EXISTING_FINAL_DAMAGE_MILLI_PLUS_MISMATCH}"),
        "mismatched_damage_delta",
    ));
    errors.extend(expect_record_error(
        "absent tolerance",
        &format!("tolerance.absolute_milli={DEFAULT_TOLERANCE_MILLI}"),
        &format!("tolerance.absolute_milli={ZERO_VALUE}"),
        "absent_tolerance",
    ));
    errors.extend(expect_record_error(
        "unpaired parity",
        "reference.required=false",
        "reference.required=true",
        "unpaired_vanilla_parity",
    ));
    errors.extend(expect_record_error(
        "all-loadout overclaim",
        "claims.all_loadouts=false",
        "claims.all_loadouts=true",
        "all_loadout_overclaim",
    ));
    if errors.is_empty() {
        Ok("positive fixture and fail-closed mutations passed".to_string())
    } else {
        Err(errors)
    }
}

fn expect_record_ok(name: &str, text: &str) -> Vec<String> {
    match parse_record(text).map(|record| validate_matrix_record(&record)) {
        Ok(errors) if errors.is_empty() => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected ok, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn expect_record_error(name: &str, old_text: &str, new_text: &str, needle: &str) -> Vec<String> {
    let text = positive_record_text().replace(old_text, new_text);
    match parse_record(&text).map(|record| validate_matrix_record(&record)) {
        Ok(errors) if errors.iter().any(|error| error.contains(needle)) => Vec::new(),
        Ok(errors) => vec![format!("{name}: expected {needle:?}, got {errors:?}")],
        Err(errors) => vec![format!("{name}: parse failed {errors:?}")],
    }
}

fn exit_with_errors(errors: &[String]) -> ExitCode {
    for error in errors {
        eprintln!("armor loadout/enchantment/status checker failed: {error}");
    }
    FAILURE
}
