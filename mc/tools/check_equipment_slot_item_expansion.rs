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
    "docs/evidence/protocol-763-equipment-slot-item-expansion-contract-2026-05-29.md";
const EXISTING_ROW_ID: &str = "remote_main_hand_slot4_item829_count1_non_empty";
const FUTURE_CHEST_ROW_ID: &str = "remote_chest_slot6_diamond_chestplate_count1_non_empty";
const FUTURE_OFFHAND_ROW_ID: &str = "remote_offhand_slot5_shield_count1_non_empty";
const FUTURE_CLEAR_ROW_ID: &str = "remote_main_hand_slot4_empty_count0_clear";
const EXPECTED_ACTOR: &str = "compatbotb";
const EXPECTED_OBSERVER: &str = "compatbota";
const EXPECTED_REMOTE_ENTITY_ID: &str = "4";
const EXPECTED_SEMANTIC_SLOT: &str = "main_hand_remote_entity";
const EXPECTED_WIRE_SLOT: &str = "slot4";
const EXPECTED_ITEM_ID: &str = "829";
const EXPECTED_ITEM_COUNT: &str = "1";
const EXPECTED_TRANSITION: &str = "non_empty_update";
const EXPECTED_UPDATE_ORDER: &str = "after_remote_spawn";
const EXPECTED_REFERENCE_NONE: &str = "none";
const KEY_VALUE_SEPARATOR: char = '=';
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const CONTRACT_TOKENS: &[&str] = &[
    EXISTING_ROW_ID,
    FUTURE_CHEST_ROW_ID,
    FUTURE_OFFHAND_ROW_ID,
    FUTURE_CLEAR_ROW_ID,
    "missing_equipment_row_field",
    "wrong_slot_mapping",
    "missing_observer_update",
    "item_count_mismatch",
    "stale_entity_id",
    "duplicate_update_order",
    "unpaired_equipment_reference",
    "all_equipment_overclaim",
    "all equipment slots",
];

const REQUIRED_FIELDS: &[&str] = &[
    "row.id",
    "row.promotion_label",
    "actor.username",
    "observer.username",
    "entity.remote_id",
    "slot.semantic",
    "slot.wire",
    "item.id",
    "item.count",
    "transition.kind",
    "update.order",
    "client.milestone.remote_spawn",
    "client.milestone.entity_equipment_update",
    "server.milestone.equipment_update_state",
    "valence.receipt",
    "reference.required",
    "reference.receipt",
    "claims.all_equipment_slots",
    "claims.all_item_types",
    "claims.packet_permutations",
    "claims.armor_mitigation",
    "claims.enchantment_status_effects",
    "claims.production_readiness",
    "claims.full_equipment_semantics",
];

const FORBIDDEN_TRUE_CLAIMS: &[&str] = &[
    "claims.all_equipment_slots",
    "claims.all_item_types",
    "claims.packet_permutations",
    "claims.armor_mitigation",
    "claims.enchantment_status_effects",
    "claims.production_readiness",
    "claims.full_equipment_semantics",
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
                println!("equipment slot/item expansion checker self-test passed: {summary}");
                SUCCESS
            }
            Err(errors) => exit_with_errors(&errors),
        };
    }

    match run_repo_check(&args) {
        Ok(summary) => {
            println!("equipment slot/item expansion checker passed: {summary}");
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
    require_observer_update(&mut errors, record);
    require_reference_pairing(&mut errors, record);
    for claim in FORBIDDEN_TRUE_CLAIMS {
        require_false(&mut errors, record, claim, "all_equipment_overclaim");
    }
    errors
}

fn require_all_fields(errors: &mut Vec<String>, record: &MatrixRecord) {
    for field in REQUIRED_FIELDS {
        if !record.values.contains_key(*field) {
            errors.push(format!("missing_equipment_row_field: {field}"));
        }
    }
}

fn require_allowed_row(errors: &mut Vec<String>, record: &MatrixRecord) {
    let allowed_rows = split_csv(&format!(
        "{EXISTING_ROW_ID},{FUTURE_CHEST_ROW_ID},{FUTURE_OFFHAND_ROW_ID},{FUTURE_CLEAR_ROW_ID}"
    ));
    let row_id = value(record, "row.id");
    if !allowed_rows.contains(row_id) {
        errors.push(format!(
            "missing_equipment_row_field: unknown row.id {row_id}"
        ));
    }
    if row_id == EXISTING_ROW_ID {
        require_text(
            errors,
            record,
            "actor.username",
            EXPECTED_ACTOR,
            "missing_equipment_row_field",
        );
        require_text(
            errors,
            record,
            "observer.username",
            EXPECTED_OBSERVER,
            "missing_equipment_row_field",
        );
        require_text(
            errors,
            record,
            "entity.remote_id",
            EXPECTED_REMOTE_ENTITY_ID,
            "stale_entity_id",
        );
        require_text(
            errors,
            record,
            "slot.semantic",
            EXPECTED_SEMANTIC_SLOT,
            "wrong_slot_mapping",
        );
        require_text(
            errors,
            record,
            "slot.wire",
            EXPECTED_WIRE_SLOT,
            "wrong_slot_mapping",
        );
        require_text(
            errors,
            record,
            "item.id",
            EXPECTED_ITEM_ID,
            "item_count_mismatch",
        );
        require_text(
            errors,
            record,
            "item.count",
            EXPECTED_ITEM_COUNT,
            "item_count_mismatch",
        );
        require_text(
            errors,
            record,
            "transition.kind",
            EXPECTED_TRANSITION,
            "missing_equipment_row_field",
        );
        require_text(
            errors,
            record,
            "update.order",
            EXPECTED_UPDATE_ORDER,
            "duplicate_update_order",
        );
    }
}

fn require_observer_update(errors: &mut Vec<String>, record: &MatrixRecord) {
    let remote_id = value(record, "entity.remote_id");
    let wire_slot = value(record, "slot.wire");
    let item_id = value(record, "item.id");
    let count = value(record, "item.count");
    let spawn = value(record, "client.milestone.remote_spawn");
    let update = value(record, "client.milestone.entity_equipment_update");
    require_present(errors, record, "valence.receipt", "missing_observer_update");
    require_present(
        errors,
        record,
        "server.milestone.equipment_update_state",
        "missing_observer_update",
    );
    if !spawn.contains(remote_id) || remote_id.is_empty() {
        errors.push(format!(
            "stale_entity_id: remote spawn does not bind entity {remote_id}"
        ));
    }
    if update.is_empty() {
        errors.push("missing_observer_update: equipment update missing".to_string());
        return;
    }
    if !update.contains(remote_id) || !update.contains(wire_slot) {
        errors.push(
            "stale_entity_id: equipment update is not bound to remote spawn slot".to_string(),
        );
    }
    let expected_slot_item = format!("{wire_slot}:id={item_id}:count={count}");
    if !update.contains(&expected_slot_item) {
        errors.push(format!(
            "item_count_mismatch: expected {expected_slot_item}"
        ));
    }
    if update.contains("entries=2")
        || update.contains("duplicate")
        || update.contains("out_of_order")
    {
        errors.push("duplicate_update_order: expected exactly one ordered update".to_string());
    }
}

fn require_reference_pairing(errors: &mut Vec<String>, record: &MatrixRecord) {
    match value(record, "reference.required") {
        "true" => require_present(
            errors,
            record,
            "reference.receipt",
            "unpaired_equipment_reference",
        ),
        "false" => require_text(
            errors,
            record,
            "reference.receipt",
            EXPECTED_REFERENCE_NONE,
            "unpaired_equipment_reference",
        ),
        other => errors.push(format!(
            "unpaired_equipment_reference: invalid reference.required {other}"
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
        "row.id={EXISTING_ROW_ID}\n\
         row.promotion_label=existing_bounded_valence_only_containment\n\
         actor.username={EXPECTED_ACTOR}\n\
         observer.username={EXPECTED_OBSERVER}\n\
         entity.remote_id={EXPECTED_REMOTE_ENTITY_ID}\n\
         slot.semantic={EXPECTED_SEMANTIC_SLOT}\n\
         slot.wire={EXPECTED_WIRE_SLOT}\n\
         item.id={EXPECTED_ITEM_ID}\n\
         item.count={EXPECTED_ITEM_COUNT}\n\
         transition.kind={EXPECTED_TRANSITION}\n\
         update.order={EXPECTED_UPDATE_ORDER}\n\
         client.milestone.remote_spawn=remote_player_spawn entity_id={EXPECTED_REMOTE_ENTITY_ID}\n\
         client.milestone.entity_equipment_update=equipment_probe_entity_equipment entity_id={EXPECTED_REMOTE_ENTITY_ID} entries=1 slots={EXPECTED_WIRE_SLOT}:id={EXPECTED_ITEM_ID}:count={EXPECTED_ITEM_COUNT}\n\
         server.milestone.equipment_update_state=equipment_update_state username={EXPECTED_ACTOR} slot={EXPECTED_WIRE_SLOT} item_id={EXPECTED_ITEM_ID} count={EXPECTED_ITEM_COUNT}\n\
         valence.receipt=docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json\n\
         reference.required=false\n\
         reference.receipt={EXPECTED_REFERENCE_NONE}\n\
         claims.all_equipment_slots=false\n\
         claims.all_item_types=false\n\
         claims.packet_permutations=false\n\
         claims.armor_mitigation=false\n\
         claims.enchantment_status_effects=false\n\
         claims.production_readiness=false\n\
         claims.full_equipment_semantics=false\n"
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
        &format!("row.id={EXISTING_ROW_ID}\n"),
        "",
        "missing_equipment_row_field",
    ));
    errors.extend(expect_record_error(
        "wrong slot",
        &format!("slot.wire={EXPECTED_WIRE_SLOT}"),
        "slot.wire=slot5",
        "wrong_slot_mapping",
    ));
    errors.extend(expect_record_error(
        "missing observer update",
        &format!(
            "client.milestone.entity_equipment_update=equipment_probe_entity_equipment entity_id={EXPECTED_REMOTE_ENTITY_ID} entries=1 slots={EXPECTED_WIRE_SLOT}:id={EXPECTED_ITEM_ID}:count={EXPECTED_ITEM_COUNT}"
        ),
        "client.milestone.entity_equipment_update=",
        "missing_observer_update",
    ));
    errors.extend(expect_record_error(
        "item mismatch",
        &format!("item.id={EXPECTED_ITEM_ID}"),
        "item.id=999",
        "item_count_mismatch",
    ));
    errors.extend(expect_record_error(
        "stale entity",
        "client.milestone.remote_spawn=remote_player_spawn entity_id=4",
        "client.milestone.remote_spawn=remote_player_spawn entity_id=5",
        "stale_entity_id",
    ));
    errors.extend(expect_record_error(
        "duplicate update",
        "entries=1",
        "entries=2 duplicate",
        "duplicate_update_order",
    ));
    errors.extend(expect_record_error(
        "unpaired reference",
        "reference.required=false",
        "reference.required=true",
        "unpaired_equipment_reference",
    ));
    errors.extend(expect_record_error(
        "all-equipment overclaim",
        "claims.all_equipment_slots=false",
        "claims.all_equipment_slots=true",
        "all_equipment_overclaim",
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
        eprintln!("equipment slot/item expansion checker failed: {error}");
    }
    FAILURE
}
