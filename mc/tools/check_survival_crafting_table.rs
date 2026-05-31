#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! serde_json = "1"
//! ```

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONTRACT_DOC: &str =
    "docs/evidence/protocol-763-survival-crafting-table-contract-2026-05-30.md";
const EXPECTED_PROTOCOL: &str = "763";
const EXPECTED_SCENARIO: &str = "survival-crafting-table";
const REFERENCE_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const EXPECTED_TABLE_POSITION: &str = "4,64,0";
const EXPECTED_WINDOW: &str = "1";
const EXPECTED_INPUT_A_SLOT: &str = "1";
const EXPECTED_INPUT_B_SLOT: &str = "4";
const EXPECTED_RESULT_SLOT: &str = "0";
const EXPECTED_INVENTORY_SLOT: &str = "36";
const EXPECTED_INPUT_ITEM: &str = "OakPlanks";
const EXPECTED_INPUT_COUNT: &str = "1";
const EXPECTED_RESULT_ITEM: &str = "Stick";
const EXPECTED_RESULT_COUNT: &str = "4";
const EXPECTED_RECIPE: &str = "minecraft:stick";
const MISMATCHED_POSITION: &str = "5,64,0";
const MISMATCHED_SLOT: &str = "2";
const MISMATCHED_ITEM: &str = "Stone";
const MISMATCHED_COUNT: &str = "2";
const PRESENT: &str = "present";
const ABSENT: &str = "absent";
const NO_VALUE: &str = "<missing>";
const CLEAN_REVISION_STATUS: &str = "clean";
const DIRTY_REVISION_STATUS: &str = "dirty";
const DRY_RUN_REVISION: &str = "dry-run";
const UNAVAILABLE_REVISION_STATUS: &str = "unavailable";
const ORACLE_NONE: &str = "none";
const ORACLE_CHECKPOINT_DOC: &str =
    "docs/evidence/protocol-763-survival-crafting-table-revision-oracle-2026-05-30.md";
const ARG_VALUE_STRIDE: usize = 2;

const CLIENT_MILESTONES: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_crafting_table_open_seen",
    "survival_crafting_input_a_sent",
    "survival_crafting_input_b_sent",
    "survival_crafting_result_seen",
    "survival_crafting_result_collected",
    "survival_crafting_inventory_updated",
];

const SERVER_MILESTONES: &[&str] = &[
    "server_survival_crafting_table_open",
    "server_survival_crafting_input_a",
    "server_survival_crafting_input_b",
    "server_survival_crafting_result",
    "server_survival_crafting_collect",
];

const REQUIRED_METRICS: &[&str] = &[
    "scenario.name",
    "server.protocol",
    "server.backend",
    "client.username",
    "client.git_rev",
    "client.git_status",
    "client.git_dirty",
    "valence.git_rev_requested",
    "valence.git_rev_resolved",
    "valence.git_status",
    "valence.git_dirty",
    "revision.oracle_checkpoint",
    "client.missing_milestones.empty",
    "client.forbidden_matches.empty",
    "server.missing_milestones.empty",
    "server.forbidden_matches.empty",
    "client.milestone.protocol_detected",
    "client.milestone.join_game",
    "client.milestone.render_tick",
    "client.milestone.survival_crafting_table_open_seen",
    "client.milestone.survival_crafting_input_a_sent",
    "client.milestone.survival_crafting_input_b_sent",
    "client.milestone.survival_crafting_result_seen",
    "client.milestone.survival_crafting_result_collected",
    "client.milestone.survival_crafting_inventory_updated",
    "server.milestone.server_survival_crafting_table_open",
    "server.milestone.server_survival_crafting_input_a",
    "server.milestone.server_survival_crafting_input_b",
    "server.milestone.server_survival_crafting_result",
    "server.milestone.server_survival_crafting_collect",
    "client.crafting.open.window",
    "client.crafting.open.position",
    "client.crafting.input_a.window",
    "client.crafting.input_a.slot",
    "client.crafting.input_a.item",
    "client.crafting.input_a.count",
    "client.crafting.input_b.window",
    "client.crafting.input_b.slot",
    "client.crafting.input_b.item",
    "client.crafting.input_b.count",
    "client.crafting.result.window",
    "client.crafting.result.slot",
    "client.crafting.result.item",
    "client.crafting.result.count",
    "client.crafting.result.recipe",
    "client.crafting.collect.window",
    "client.crafting.collect.slot",
    "client.crafting.collect.item",
    "client.crafting.collect.count",
    "client.crafting.inventory.slot",
    "client.crafting.inventory.item",
    "client.crafting.inventory.count",
    "server.crafting.open.position",
    "server.crafting.open.window",
    "server.crafting.input_a.window",
    "server.crafting.input_a.slot",
    "server.crafting.input_a.item",
    "server.crafting.input_a.count",
    "server.crafting.input_b.window",
    "server.crafting.input_b.slot",
    "server.crafting.input_b.item",
    "server.crafting.input_b.count",
    "server.crafting.result.window",
    "server.crafting.result.slot",
    "server.crafting.result.item",
    "server.crafting.result.count",
    "server.crafting.result.recipe",
    "server.crafting.collect.window",
    "server.crafting.collect.slot",
    "server.crafting.collect.item",
    "server.crafting.collect.count",
    "server.crafting.collect.inventory_slot",
];

const COMPARISON_METRICS: &[&str] = &[
    "scenario.name",
    "server.protocol",
    "client.git_rev",
    "client.crafting.open.position",
    "client.crafting.input_a.slot",
    "client.crafting.input_a.item",
    "client.crafting.input_a.count",
    "client.crafting.input_b.slot",
    "client.crafting.input_b.item",
    "client.crafting.input_b.count",
    "client.crafting.result.slot",
    "client.crafting.result.item",
    "client.crafting.result.count",
    "client.crafting.result.recipe",
    "client.crafting.collect.slot",
    "client.crafting.collect.item",
    "client.crafting.collect.count",
    "client.crafting.inventory.slot",
    "client.crafting.inventory.item",
    "client.crafting.inventory.count",
    "server.crafting.open.position",
    "server.crafting.input_a.slot",
    "server.crafting.input_a.item",
    "server.crafting.input_a.count",
    "server.crafting.input_b.slot",
    "server.crafting.input_b.item",
    "server.crafting.input_b.count",
    "server.crafting.result.slot",
    "server.crafting.result.item",
    "server.crafting.result.count",
    "server.crafting.result.recipe",
    "server.crafting.collect.slot",
    "server.crafting.collect.item",
    "server.crafting.collect.count",
    "server.crafting.collect.inventory_slot",
];

const EXPECTED_VALUE_METRICS: &[(&str, &str)] = &[
    ("client.crafting.open.window", EXPECTED_WINDOW),
    ("client.crafting.open.position", EXPECTED_TABLE_POSITION),
    ("client.crafting.input_a.window", EXPECTED_WINDOW),
    ("client.crafting.input_a.slot", EXPECTED_INPUT_A_SLOT),
    ("client.crafting.input_a.item", EXPECTED_INPUT_ITEM),
    ("client.crafting.input_a.count", EXPECTED_INPUT_COUNT),
    ("client.crafting.input_b.window", EXPECTED_WINDOW),
    ("client.crafting.input_b.slot", EXPECTED_INPUT_B_SLOT),
    ("client.crafting.input_b.item", EXPECTED_INPUT_ITEM),
    ("client.crafting.input_b.count", EXPECTED_INPUT_COUNT),
    ("client.crafting.result.window", EXPECTED_WINDOW),
    ("client.crafting.result.slot", EXPECTED_RESULT_SLOT),
    ("client.crafting.result.item", EXPECTED_RESULT_ITEM),
    ("client.crafting.result.count", EXPECTED_RESULT_COUNT),
    ("client.crafting.result.recipe", EXPECTED_RECIPE),
    ("client.crafting.collect.window", EXPECTED_WINDOW),
    ("client.crafting.collect.slot", EXPECTED_RESULT_SLOT),
    ("client.crafting.collect.item", EXPECTED_RESULT_ITEM),
    ("client.crafting.collect.count", EXPECTED_RESULT_COUNT),
    ("client.crafting.inventory.slot", EXPECTED_INVENTORY_SLOT),
    ("client.crafting.inventory.item", EXPECTED_RESULT_ITEM),
    ("client.crafting.inventory.count", EXPECTED_RESULT_COUNT),
    ("server.crafting.open.window", EXPECTED_WINDOW),
    ("server.crafting.open.position", EXPECTED_TABLE_POSITION),
    ("server.crafting.input_a.window", EXPECTED_WINDOW),
    ("server.crafting.input_a.slot", EXPECTED_INPUT_A_SLOT),
    ("server.crafting.input_a.item", EXPECTED_INPUT_ITEM),
    ("server.crafting.input_a.count", EXPECTED_INPUT_COUNT),
    ("server.crafting.input_b.window", EXPECTED_WINDOW),
    ("server.crafting.input_b.slot", EXPECTED_INPUT_B_SLOT),
    ("server.crafting.input_b.item", EXPECTED_INPUT_ITEM),
    ("server.crafting.input_b.count", EXPECTED_INPUT_COUNT),
    ("server.crafting.result.window", EXPECTED_WINDOW),
    ("server.crafting.result.slot", EXPECTED_RESULT_SLOT),
    ("server.crafting.result.item", EXPECTED_RESULT_ITEM),
    ("server.crafting.result.count", EXPECTED_RESULT_COUNT),
    ("server.crafting.result.recipe", EXPECTED_RECIPE),
    ("server.crafting.collect.window", EXPECTED_WINDOW),
    ("server.crafting.collect.slot", EXPECTED_RESULT_SLOT),
    ("server.crafting.collect.item", EXPECTED_RESULT_ITEM),
    ("server.crafting.collect.count", EXPECTED_RESULT_COUNT),
    (
        "server.crafting.collect.inventory_slot",
        EXPECTED_INVENTORY_SLOT,
    ),
];

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_SCENARIO,
    EXPECTED_TABLE_POSITION,
    EXPECTED_INPUT_A_SLOT,
    EXPECTED_INPUT_B_SLOT,
    EXPECTED_RESULT_SLOT,
    EXPECTED_INVENTORY_SLOT,
    EXPECTED_INPUT_ITEM,
    EXPECTED_INPUT_COUNT,
    EXPECTED_RESULT_ITEM,
    EXPECTED_RESULT_COUNT,
    EXPECTED_RECIPE,
    "one deterministic crafting table",
    "one configured recipe",
    "one configured input stack set",
    "one result stack",
    "full crafting coverage",
    "all recipes",
    "recipe-book behavior",
    "shift-click matrices",
    "all container transaction modes",
    "full survival compatibility",
    "broad vanilla parity",
    "production readiness",
    "missing_reference",
    "valence_only",
    "missing_metric",
    "mismatched_metric:*.slot",
    "mismatched_metric:*.item",
    "mismatched_metric:*.count",
    "wrong_backend",
    "client.git_rev",
    "client.git_status",
    "client.git_dirty",
    "valence.git_rev_requested",
    "valence.git_rev_resolved",
    "valence.git_status",
    "valence.git_dirty",
    "revision.oracle_checkpoint",
    ORACLE_CHECKPOINT_DOC,
    "stale_revision",
    "missing_revision_or_oracle",
    "invalid_oracle_checkpoint",
    "missing_oracle_checkpoint_file",
];

const PAIR_ARGS: &[&str] = &[
    "--reference-receipt",
    "--reference-client-log",
    "--reference-server-log",
    "--valence-receipt",
    "--valence-client-log",
    "--valence-server-log",
];

#[derive(Clone, Debug)]
struct EvidenceInput {
    receipt: Value,
    client_log: String,
    server_log: String,
}

#[derive(Debug)]
struct NormalizedEvidence {
    values: BTreeMap<String, String>,
    diagnostics: Vec<String>,
}

#[derive(Debug)]
struct ComparisonResult {
    passed: bool,
    diagnostics: Vec<String>,
}

#[derive(Debug)]
struct Args {
    self_test: bool,
    contract_doc: PathBuf,
    reference_receipt: Option<PathBuf>,
    reference_client_log: Option<PathBuf>,
    reference_server_log: Option<PathBuf>,
    valence_receipt: Option<PathBuf>,
    valence_client_log: Option<PathBuf>,
    valence_server_log: Option<PathBuf>,
}

fn strip_ansi(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if code == 'm' {
                    break;
                }
            }
            continue;
        }
        output.push(ch);
    }
    output
}

fn parse_key_values(segment: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for token in segment.split_whitespace() {
        if let Some((key, value)) = token.split_once('=') {
            fields.insert(key.to_string(), value.trim_end_matches(',').to_string());
        }
    }
    fields
}

fn find_fields(log_text: &str, marker: &str) -> Option<BTreeMap<String, String>> {
    let clean = strip_ansi(log_text);
    for line in clean.lines() {
        if let Some((_, segment)) = line.split_once(marker) {
            return Some(parse_key_values(segment));
        }
    }
    None
}

fn status_for_presence(value: Option<&Value>, expected: &str) -> &'static str {
    let Some(items) = value.and_then(Value::as_array) else {
        return ABSENT;
    };
    if items.iter().any(|item| item.as_str() == Some(expected)) {
        PRESENT
    } else {
        ABSENT
    }
}

fn empty_status(value: Option<&Value>) -> &'static str {
    let Some(items) = value.and_then(Value::as_array) else {
        return ABSENT;
    };
    if items.is_empty() {
        PRESENT
    } else {
        ABSENT
    }
}

fn json_scalar_to_string(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Number(number)) => number.to_string(),
        Some(Value::Bool(flag)) => flag.to_string(),
        _ => NO_VALUE.to_string(),
    }
}

fn revision_oracle_checkpoint(receipt: &Value) -> String {
    let revision_oracle = receipt
        .get("revision")
        .and_then(|item| item.get("oracle_checkpoint"));
    let evidence_oracle = receipt
        .get("evidence")
        .and_then(|item| item.get("oracle_checkpoint"));
    let value = json_scalar_to_string(revision_oracle.or(evidence_oracle));
    if value == NO_VALUE || value.trim().is_empty() {
        ORACLE_NONE.to_string()
    } else {
        value
    }
}

fn put_metric(
    values: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    metric: &str,
    fields: Option<&BTreeMap<String, String>>,
    key: &str,
) {
    let value = fields.and_then(|map| map.get(key));
    match value {
        Some(found) => {
            values.insert(metric.to_string(), found.clone());
        }
        None => {
            diagnostics.push(format!("missing_metric:{metric}"));
            values.insert(metric.to_string(), NO_VALUE.to_string());
        }
    }
}

fn add_receipt_metrics(values: &mut BTreeMap<String, String>, receipt: &Value) {
    let scenario = receipt.get("scenario");
    let server = receipt.get("server");
    let client = receipt.get("client");
    values.insert(
        "scenario.name".to_string(),
        json_scalar_to_string(scenario.and_then(|item| item.get("name"))),
    );
    values.insert(
        "server.protocol".to_string(),
        json_scalar_to_string(server.and_then(|item| item.get("protocol"))),
    );
    values.insert(
        "server.backend".to_string(),
        json_scalar_to_string(server.and_then(|item| item.get("backend"))),
    );
    values.insert(
        "client.username".to_string(),
        json_scalar_to_string(client.and_then(|item| item.get("username"))),
    );
    add_revision_metrics(values, receipt, client);
    values.insert(
        "client.missing_milestones.empty".to_string(),
        empty_status(scenario.and_then(|item| item.get("missing_milestones"))).to_string(),
    );
    values.insert(
        "client.forbidden_matches.empty".to_string(),
        empty_status(scenario.and_then(|item| item.get("forbidden_matches"))).to_string(),
    );
    values.insert(
        "server.missing_milestones.empty".to_string(),
        empty_status(server.and_then(|item| item.get("missing_milestones"))).to_string(),
    );
    values.insert(
        "server.forbidden_matches.empty".to_string(),
        empty_status(server.and_then(|item| item.get("forbidden_matches"))).to_string(),
    );
    add_milestone_metrics(values, scenario, server);
}

fn add_revision_metrics(
    values: &mut BTreeMap<String, String>,
    receipt: &Value,
    client: Option<&Value>,
) {
    let valence = receipt.get("valence");
    values.insert(
        "client.git_rev".to_string(),
        json_scalar_to_string(client.and_then(|item| item.get("git_rev"))),
    );
    values.insert(
        "client.git_status".to_string(),
        json_scalar_to_string(client.and_then(|item| item.get("git_status"))),
    );
    values.insert(
        "client.git_dirty".to_string(),
        json_scalar_to_string(client.and_then(|item| item.get("git_dirty"))),
    );
    values.insert(
        "valence.git_rev_requested".to_string(),
        json_scalar_to_string(valence.and_then(|item| item.get("git_rev_requested"))),
    );
    values.insert(
        "valence.git_rev_resolved".to_string(),
        json_scalar_to_string(valence.and_then(|item| item.get("git_rev_resolved"))),
    );
    values.insert(
        "valence.git_status".to_string(),
        json_scalar_to_string(valence.and_then(|item| item.get("git_status"))),
    );
    values.insert(
        "valence.git_dirty".to_string(),
        json_scalar_to_string(valence.and_then(|item| item.get("git_dirty"))),
    );
    values.insert(
        "revision.oracle_checkpoint".to_string(),
        revision_oracle_checkpoint(receipt),
    );
}

fn add_milestone_metrics(
    values: &mut BTreeMap<String, String>,
    scenario: Option<&Value>,
    server: Option<&Value>,
) {
    let client_observed = scenario.and_then(|item| item.get("observed_milestones"));
    let server_observed = server.and_then(|item| item.get("observed_milestones"));
    for milestone in CLIENT_MILESTONES {
        values.insert(
            format!("client.milestone.{milestone}"),
            status_for_presence(client_observed, milestone).to_string(),
        );
    }
    for milestone in SERVER_MILESTONES {
        values.insert(
            format!("server.milestone.{milestone}"),
            status_for_presence(server_observed, milestone).to_string(),
        );
    }
}

fn add_field_group(
    values: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    prefix: &str,
    fields: Option<&BTreeMap<String, String>>,
    keys: &[&str],
) {
    for key in keys {
        put_metric(values, diagnostics, &format!("{prefix}.{key}"), fields, key);
    }
}

fn add_client_log_metrics(
    values: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    client_log: &str,
) {
    let open = find_fields(client_log, "survival_crafting_table_open_seen");
    add_field_group(
        values,
        diagnostics,
        "client.crafting.open",
        open.as_ref(),
        &["window", "position"],
    );
    let input_a = find_fields(client_log, "survival_crafting_input_a_sent");
    add_field_group(
        values,
        diagnostics,
        "client.crafting.input_a",
        input_a.as_ref(),
        &["window", "slot", "item", "count"],
    );
    let input_b = find_fields(client_log, "survival_crafting_input_b_sent");
    add_field_group(
        values,
        diagnostics,
        "client.crafting.input_b",
        input_b.as_ref(),
        &["window", "slot", "item", "count"],
    );
    let result = find_fields(client_log, "survival_crafting_result_seen");
    add_field_group(
        values,
        diagnostics,
        "client.crafting.result",
        result.as_ref(),
        &["window", "slot", "item", "count", "recipe"],
    );
    let collect = find_fields(client_log, "survival_crafting_result_collected");
    add_field_group(
        values,
        diagnostics,
        "client.crafting.collect",
        collect.as_ref(),
        &["window", "slot", "item", "count"],
    );
    let inventory = find_fields(client_log, "survival_crafting_inventory_updated");
    add_field_group(
        values,
        diagnostics,
        "client.crafting.inventory",
        inventory.as_ref(),
        &["slot", "item", "count"],
    );
}

fn add_server_log_metrics(
    values: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    server_log: &str,
) {
    let open = find_fields(server_log, "survival_crafting_table_open");
    add_field_group(
        values,
        diagnostics,
        "server.crafting.open",
        open.as_ref(),
        &["position", "window"],
    );
    let input_a = find_fields(server_log, "survival_crafting_input_a");
    add_field_group(
        values,
        diagnostics,
        "server.crafting.input_a",
        input_a.as_ref(),
        &["window", "slot", "item", "count"],
    );
    let input_b = find_fields(server_log, "survival_crafting_input_b");
    add_field_group(
        values,
        diagnostics,
        "server.crafting.input_b",
        input_b.as_ref(),
        &["window", "slot", "item", "count"],
    );
    let result = find_fields(server_log, "survival_crafting_result");
    add_field_group(
        values,
        diagnostics,
        "server.crafting.result",
        result.as_ref(),
        &["window", "slot", "item", "count", "recipe"],
    );
    let collect = find_fields(server_log, "survival_crafting_collect");
    add_field_group(
        values,
        diagnostics,
        "server.crafting.collect",
        collect.as_ref(),
        &["window", "slot", "item", "count", "inventory_slot"],
    );
}

fn add_expected_value_diagnostics(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
) {
    for (metric, expected) in EXPECTED_VALUE_METRICS {
        let actual = values.get(*metric).map(String::as_str).unwrap_or(NO_VALUE);
        if actual == NO_VALUE || actual == ABSENT {
            continue;
        }
        if actual != *expected {
            diagnostics.push(format!(
                "wrong_contract:{metric}: expected {expected} found {actual}"
            ));
        }
    }
}

fn normalize_evidence(evidence: &EvidenceInput, expected_backend: &str) -> NormalizedEvidence {
    let mut values = BTreeMap::new();
    let mut diagnostics = Vec::new();
    add_receipt_metrics(&mut values, &evidence.receipt);
    add_client_log_metrics(&mut values, &mut diagnostics, &evidence.client_log);
    add_server_log_metrics(&mut values, &mut diagnostics, &evidence.server_log);
    add_receipt_diagnostics(&values, &mut diagnostics, expected_backend);
    add_revision_diagnostics(&values, &mut diagnostics, expected_backend);
    add_required_metric_diagnostics(&values, &mut diagnostics, expected_backend);
    add_expected_value_diagnostics(&values, &mut diagnostics);
    diagnostics = unique_sorted(diagnostics);
    NormalizedEvidence {
        values,
        diagnostics,
    }
}

fn add_receipt_diagnostics(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    expected_backend: &str,
) {
    if values.get("server.backend").map(String::as_str) != Some(expected_backend) {
        let found = values
            .get("server.backend")
            .map(String::as_str)
            .unwrap_or(NO_VALUE);
        diagnostics.push(format!("wrong_backend:{found} expected {expected_backend}"));
    }
    if values.get("scenario.name").map(String::as_str) != Some(EXPECTED_SCENARIO) {
        let found = values
            .get("scenario.name")
            .map(String::as_str)
            .unwrap_or(NO_VALUE);
        diagnostics.push(format!("wrong_scenario:{found}"));
    }
    if values.get("server.protocol").map(String::as_str) != Some(EXPECTED_PROTOCOL) {
        let found = values
            .get("server.protocol")
            .map(String::as_str)
            .unwrap_or(NO_VALUE);
        diagnostics.push(format!("wrong_protocol:{found}"));
    }
}

fn add_revision_diagnostics(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    expected_backend: &str,
) {
    if reviewable_revision_oracle_present(values, diagnostics) {
        return;
    }
    add_clean_revision_diagnostic(values, diagnostics, "client.git");
    if expected_backend == VALENCE_BACKEND {
        add_clean_revision_diagnostic(values, diagnostics, "valence.git");
    }
}

fn reviewable_revision_oracle_present(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
) -> bool {
    let value = revision_oracle_value(values);
    if value == ORACLE_NONE || value == NO_VALUE || value.trim().is_empty() {
        return false;
    }
    if value != ORACLE_CHECKPOINT_DOC {
        diagnostics.push(format!(
            "invalid_oracle_checkpoint:{value}: expected {ORACLE_CHECKPOINT_DOC}"
        ));
        return false;
    }
    if !Path::new(ORACLE_CHECKPOINT_DOC).is_file() {
        diagnostics.push(format!(
            "missing_oracle_checkpoint_file:{ORACLE_CHECKPOINT_DOC}"
        ));
        return false;
    }
    true
}

fn revision_oracle_present(values: &BTreeMap<String, String>) -> bool {
    revision_oracle_value(values) == ORACLE_CHECKPOINT_DOC
        && Path::new(ORACLE_CHECKPOINT_DOC).is_file()
}

fn revision_oracle_value(values: &BTreeMap<String, String>) -> &str {
    values
        .get("revision.oracle_checkpoint")
        .map(String::as_str)
        .unwrap_or(ORACLE_NONE)
}

fn add_clean_revision_diagnostic(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    prefix: &str,
) {
    let rev_metric = if prefix == "client.git" {
        "client.git_rev"
    } else {
        "valence.git_rev_resolved"
    };
    let requested_metric = if prefix == "client.git" {
        None
    } else {
        Some("valence.git_rev_requested")
    };
    let status_metric = format!("{prefix}_status");
    let dirty_metric = format!("{prefix}_dirty");
    let rev = values
        .get(rev_metric)
        .map(String::as_str)
        .unwrap_or(NO_VALUE);
    if rev == NO_VALUE || rev == DRY_RUN_REVISION || rev.trim().is_empty() {
        diagnostics.push(format!("stale_revision:{rev_metric}: {rev}"));
    }
    if let Some(metric) = requested_metric {
        let requested = values.get(metric).map(String::as_str).unwrap_or(NO_VALUE);
        if requested == NO_VALUE || requested == DRY_RUN_REVISION || requested.trim().is_empty() {
            diagnostics.push(format!("stale_revision:{metric}: {requested}"));
        } else if rev != NO_VALUE
            && rev != DRY_RUN_REVISION
            && !rev.trim().is_empty()
            && requested != rev
        {
            diagnostics.push(format!(
                "stale_revision:valence.git_rev_mismatch: requested={requested} resolved={rev}"
            ));
        }
    }
    let status = values
        .get(status_metric.as_str())
        .map(String::as_str)
        .unwrap_or(NO_VALUE);
    if status != CLEAN_REVISION_STATUS {
        diagnostics.push(format!("stale_revision:{status_metric}: {status}"));
    }
    let dirty = values
        .get(dirty_metric.as_str())
        .map(String::as_str)
        .unwrap_or(NO_VALUE);
    if dirty != "false" {
        diagnostics.push(format!("stale_revision:{dirty_metric}: {dirty}"));
    }
    if status == UNAVAILABLE_REVISION_STATUS || status == DIRTY_REVISION_STATUS {
        diagnostics.push(format!("missing_revision_or_oracle:{prefix}"));
    }
}

fn add_required_metric_diagnostics(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    expected_backend: &str,
) {
    for metric in REQUIRED_METRICS {
        if !metric_required_for_backend(metric, values, expected_backend) {
            continue;
        }
        let value = values.get(*metric).map(String::as_str).unwrap_or(NO_VALUE);
        if value == NO_VALUE || value == ABSENT {
            diagnostics.push(format!("missing_metric:{metric}"));
        }
    }
}

fn metric_required_for_backend(
    metric: &str,
    values: &BTreeMap<String, String>,
    expected_backend: &str,
) -> bool {
    if is_valence_revision_metric(metric) {
        expected_backend == VALENCE_BACKEND && !revision_oracle_present(values)
    } else {
        true
    }
}

fn is_valence_revision_metric(metric: &str) -> bool {
    matches!(
        metric,
        "valence.git_rev_requested"
            | "valence.git_rev_resolved"
            | "valence.git_status"
            | "valence.git_dirty"
    )
}

fn compare_evidence(
    reference: Option<&EvidenceInput>,
    valence: Option<&EvidenceInput>,
) -> ComparisonResult {
    let mut diagnostics = Vec::new();
    if reference.is_none() {
        diagnostics.push("missing_reference".to_string());
    }
    if valence.is_none() {
        diagnostics.push("missing_valence".to_string());
    }
    if reference.is_none() && valence.is_some() {
        diagnostics.push("valence_only".to_string());
    }
    let (Some(reference), Some(valence)) = (reference, valence) else {
        return ComparisonResult {
            passed: false,
            diagnostics,
        };
    };
    let reference_metrics = normalize_evidence(reference, REFERENCE_BACKEND);
    let valence_metrics = normalize_evidence(valence, VALENCE_BACKEND);
    append_prefixed(
        &mut diagnostics,
        "reference",
        &reference_metrics.diagnostics,
    );
    append_prefixed(&mut diagnostics, "valence", &valence_metrics.diagnostics);
    add_comparison_diagnostics(
        &reference_metrics.values,
        &valence_metrics.values,
        &mut diagnostics,
    );
    diagnostics = unique_sorted(diagnostics);
    ComparisonResult {
        passed: diagnostics.is_empty(),
        diagnostics,
    }
}

fn append_prefixed(diagnostics: &mut Vec<String>, prefix: &str, items: &[String]) {
    for item in items {
        diagnostics.push(format!("{prefix}:{item}"));
    }
}

fn add_comparison_diagnostics(
    reference_values: &BTreeMap<String, String>,
    valence_values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
) {
    for metric in COMPARISON_METRICS {
        let reference = reference_values
            .get(*metric)
            .map(String::as_str)
            .unwrap_or(NO_VALUE);
        let valence = valence_values
            .get(*metric)
            .map(String::as_str)
            .unwrap_or(NO_VALUE);
        if reference != valence {
            diagnostics.push(format!(
                "mismatched_metric:{metric}: reference={reference} valence={valence}"
            ));
        }
    }
}

fn unique_sorted(items: Vec<String>) -> Vec<String> {
    let mut set = BTreeSet::new();
    for item in items {
        set.insert(item);
    }
    set.into_iter().collect()
}

fn validate_contract_doc(text: &str) -> Vec<String> {
    let mut issues = Vec::new();
    for token in CONTRACT_TOKENS {
        if !text.contains(token) {
            issues.push(format!("contract missing token: {token}"));
        }
    }
    for metric in REQUIRED_METRICS {
        if !text.contains(metric) {
            issues.push(format!("contract missing token: {metric}"));
        }
    }
    issues
}

fn good_receipt(backend: &str) -> Value {
    serde_json::json!({
        "scenario": {
            "name": EXPECTED_SCENARIO,
            "observed_milestones": CLIENT_MILESTONES,
            "missing_milestones": [],
            "forbidden_matches": []
        },
        "client": {
            "username": "compatbot",
            "git_rev": "0123456789abcdef0123456789abcdef01234567",
            "git_status": CLEAN_REVISION_STATUS,
            "git_dirty": false
        },
        "valence": {
            "git_rev_requested": "89abcdef0123456789abcdef0123456789abcdef",
            "git_rev_resolved": "89abcdef0123456789abcdef0123456789abcdef",
            "git_status": CLEAN_REVISION_STATUS,
            "git_dirty": false
        },
        "revision": {
            "oracle_checkpoint": ORACLE_NONE
        },
        "server": {
            "backend": backend,
            "protocol": 763,
            "observed_milestones": SERVER_MILESTONES,
            "missing_milestones": [],
            "forbidden_matches": []
        }
    })
}

fn good_client_log() -> String {
    [
        format!("MC-COMPAT-MILESTONE survival_crafting_table_open_seen window={EXPECTED_WINDOW} position={EXPECTED_TABLE_POSITION}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_input_a_sent window={EXPECTED_WINDOW} slot={EXPECTED_INPUT_A_SLOT} item={EXPECTED_INPUT_ITEM} count={EXPECTED_INPUT_COUNT}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_input_b_sent window={EXPECTED_WINDOW} slot={EXPECTED_INPUT_B_SLOT} item={EXPECTED_INPUT_ITEM} count={EXPECTED_INPUT_COUNT}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_result_seen window={EXPECTED_WINDOW} slot={EXPECTED_RESULT_SLOT} item={EXPECTED_RESULT_ITEM} count={EXPECTED_RESULT_COUNT} recipe={EXPECTED_RECIPE}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_result_collected window={EXPECTED_WINDOW} slot={EXPECTED_RESULT_SLOT} item={EXPECTED_RESULT_ITEM} count={EXPECTED_RESULT_COUNT}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_inventory_updated slot={EXPECTED_INVENTORY_SLOT} item={EXPECTED_RESULT_ITEM} count={EXPECTED_RESULT_COUNT}"),
    ]
    .join("\n")
}

fn good_server_log() -> String {
    [
        format!("MC-COMPAT-MILESTONE survival_crafting_table_open username=compatbot position={EXPECTED_TABLE_POSITION} window={EXPECTED_WINDOW}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_input_a username=compatbot window={EXPECTED_WINDOW} slot={EXPECTED_INPUT_A_SLOT} item={EXPECTED_INPUT_ITEM} count={EXPECTED_INPUT_COUNT}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_input_b username=compatbot window={EXPECTED_WINDOW} slot={EXPECTED_INPUT_B_SLOT} item={EXPECTED_INPUT_ITEM} count={EXPECTED_INPUT_COUNT}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_result username=compatbot window={EXPECTED_WINDOW} slot={EXPECTED_RESULT_SLOT} item={EXPECTED_RESULT_ITEM} count={EXPECTED_RESULT_COUNT} recipe={EXPECTED_RECIPE}"),
        format!("MC-COMPAT-MILESTONE survival_crafting_collect username=compatbot window={EXPECTED_WINDOW} slot={EXPECTED_RESULT_SLOT} item={EXPECTED_RESULT_ITEM} count={EXPECTED_RESULT_COUNT} inventory_slot={EXPECTED_INVENTORY_SLOT}"),
    ]
    .join("\n")
}

fn good_evidence(backend: &str) -> EvidenceInput {
    EvidenceInput {
        receipt: good_receipt(backend),
        client_log: good_client_log(),
        server_log: good_server_log(),
    }
}

fn replace_logs(evidence: &EvidenceInput, old: &str, new: &str) -> EvidenceInput {
    EvidenceInput {
        receipt: evidence.receipt.clone(),
        client_log: evidence.client_log.replace(old, new),
        server_log: evidence.server_log.replace(old, new),
    }
}

fn assert_rejected(result: &ComparisonResult, expected_fragment: &str) {
    assert!(!result.passed, "unexpected pass: {result:?}");
    assert!(
        result
            .diagnostics
            .iter()
            .any(|item| item.contains(expected_fragment)),
        "missing {expected_fragment}: {result:?}"
    );
}

fn assert_self_tests(contract_doc: &str) {
    let good = compare_evidence(
        Some(&good_evidence(REFERENCE_BACKEND)),
        Some(&good_evidence(VALENCE_BACKEND)),
    );
    assert!(good.passed, "good fixture failed: {good:?}");
    assert_missing_reference_rejects();
    assert_missing_metric_rejects();
    assert_wrong_contract_rejects();
    assert_mismatched_pair_rejects();
    assert_wrong_backend_rejects();
    assert_stale_revision_rejects();
    assert_valence_revision_mismatch_rejects();
    assert_non_reviewable_oracle_checkpoint_rejects();
    assert_oracle_checkpoint_allows_missing_valence_revision();
    let contract_issues = validate_contract_doc(contract_doc);
    assert!(contract_issues.is_empty(), "{contract_issues:?}");
}

fn assert_missing_reference_rejects() {
    let valence = good_evidence(VALENCE_BACKEND);
    let result = compare_evidence(None, Some(&valence));
    assert_rejected(&result, "missing_reference");
    assert_rejected(&result, "valence_only");
}

fn assert_missing_metric_rejects() {
    assert_missing_client_marker_rejects(
        "survival_crafting_table_open_seen",
        "survival_crafting_missing_open",
        "missing_metric:client.crafting.open.window",
    );
    assert_missing_client_marker_rejects(
        "survival_crafting_input_b_sent",
        "survival_crafting_missing_input_b",
        "missing_metric:client.crafting.input_b.slot",
    );
    assert_missing_client_marker_rejects(
        "survival_crafting_result_seen",
        "survival_crafting_missing_result",
        "missing_metric:client.crafting.result.slot",
    );
    assert_missing_client_marker_rejects(
        "survival_crafting_result_collected",
        "survival_crafting_missing_collect",
        "missing_metric:client.crafting.collect.slot",
    );
    assert_missing_server_marker_rejects(
        "survival_crafting_collect",
        "survival_crafting_missing_collect",
        "missing_metric:server.crafting.collect.slot",
    );
}

fn assert_missing_client_marker_rejects(marker: &str, replacement: &str, expected: &str) {
    let reference = EvidenceInput {
        receipt: good_receipt(REFERENCE_BACKEND),
        client_log: good_client_log().replace(marker, replacement),
        server_log: good_server_log(),
    };
    let valence = good_evidence(VALENCE_BACKEND);
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, expected);
}

fn assert_missing_server_marker_rejects(marker: &str, replacement: &str, expected: &str) {
    let reference = EvidenceInput {
        receipt: good_receipt(REFERENCE_BACKEND),
        client_log: good_client_log(),
        server_log: good_server_log().replace(marker, replacement),
    };
    let valence = good_evidence(VALENCE_BACKEND);
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, expected);
}

fn assert_wrong_contract_rejects() {
    assert_both_wrong_rejects(
        EXPECTED_TABLE_POSITION,
        MISMATCHED_POSITION,
        "wrong_contract:client.crafting.open.position",
    );
    assert_both_wrong_rejects(
        &format!("slot={EXPECTED_INPUT_A_SLOT}"),
        &format!("slot={MISMATCHED_SLOT}"),
        "wrong_contract:client.crafting.input_a.slot",
    );
    assert_both_wrong_rejects(
        &format!("item={EXPECTED_INPUT_ITEM}"),
        &format!("item={MISMATCHED_ITEM}"),
        "wrong_contract:client.crafting.input_a.item",
    );
    assert_both_wrong_rejects(
        &format!("count={EXPECTED_INPUT_COUNT}"),
        &format!("count={MISMATCHED_COUNT}"),
        "wrong_contract:client.crafting.input_a.count",
    );
}

fn assert_both_wrong_rejects(old: &str, new: &str, expected: &str) {
    let reference = replace_logs(&good_evidence(REFERENCE_BACKEND), old, new);
    let valence = replace_logs(&good_evidence(VALENCE_BACKEND), old, new);
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, expected);
}

fn assert_mismatched_pair_rejects() {
    assert_valence_changed_rejects(
        &format!("slot={EXPECTED_RESULT_SLOT}"),
        &format!("slot={MISMATCHED_SLOT}"),
        "mismatched_metric:client.crafting.result.slot",
    );
    assert_valence_changed_rejects(
        &format!("item={EXPECTED_RESULT_ITEM}"),
        &format!("item={MISMATCHED_ITEM}"),
        "mismatched_metric:client.crafting.result.item",
    );
    assert_valence_changed_rejects(
        &format!("count={EXPECTED_RESULT_COUNT}"),
        &format!("count={MISMATCHED_COUNT}"),
        "mismatched_metric:client.crafting.result.count",
    );
}

fn assert_valence_changed_rejects(old: &str, new: &str, expected: &str) {
    let reference = good_evidence(REFERENCE_BACKEND);
    let valence = replace_logs(&good_evidence(VALENCE_BACKEND), old, new);
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, expected);
}

fn assert_wrong_backend_rejects() {
    let reference = good_evidence(VALENCE_BACKEND);
    let valence = good_evidence(VALENCE_BACKEND);
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, "wrong_backend");
}

fn assert_stale_revision_rejects() {
    let mut reference = good_evidence(REFERENCE_BACKEND);
    reference.receipt["client"]["git_status"] = serde_json::json!(DIRTY_REVISION_STATUS);
    let valence = good_evidence(VALENCE_BACKEND);
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, "stale_revision:client.git_status");

    let reference = good_evidence(REFERENCE_BACKEND);
    let mut valence = good_evidence(VALENCE_BACKEND);
    valence.receipt["valence"]["git_rev_resolved"] = Value::Null;
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, "stale_revision:valence.git_rev_resolved");
}

fn assert_valence_revision_mismatch_rejects() {
    let reference = good_evidence(REFERENCE_BACKEND);
    let mut valence = good_evidence(VALENCE_BACKEND);
    valence.receipt["valence"]["git_rev_resolved"] =
        serde_json::json!("fedcba9876543210fedcba9876543210fedcba98");
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, "stale_revision:valence.git_rev_mismatch");
}

fn assert_non_reviewable_oracle_checkpoint_rejects() {
    let reference = good_evidence(REFERENCE_BACKEND);
    let mut valence = good_evidence(VALENCE_BACKEND);
    valence.receipt["valence"]["git_rev_resolved"] = Value::Null;
    valence.receipt["revision"]["oracle_checkpoint"] =
        serde_json::json!("docs/evidence/not-reviewable-oracle.md");
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert_rejected(&result, "invalid_oracle_checkpoint");
}

fn assert_oracle_checkpoint_allows_missing_valence_revision() {
    let reference = good_evidence(REFERENCE_BACKEND);
    let mut valence = good_evidence(VALENCE_BACKEND);
    valence.receipt["valence"]["git_rev_resolved"] = Value::Null;
    valence.receipt["valence"]["git_status"] = serde_json::json!(UNAVAILABLE_REVISION_STATUS);
    valence.receipt["valence"]["git_dirty"] = serde_json::json!(true);
    valence.receipt["revision"]["oracle_checkpoint"] = serde_json::json!(ORACLE_CHECKPOINT_DOC);
    let result = compare_evidence(Some(&reference), Some(&valence));
    assert!(result.passed, "oracle-backed fixture failed: {result:?}");
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args {
        self_test: false,
        contract_doc: PathBuf::from(CONTRACT_DOC),
        reference_receipt: None,
        reference_client_log: None,
        reference_server_log: None,
        valence_receipt: None,
        valence_client_log: None,
        valence_server_log: None,
    };
    let values: Vec<String> = env::args().skip(1).collect();
    let mut index = 0;
    while index < values.len() {
        let flag = values[index].as_str();
        if flag == "--self-test" {
            args.self_test = true;
            index += 1;
            continue;
        }
        let value = values
            .get(index + 1)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        set_path_arg(&mut args, flag, value)?;
        index += ARG_VALUE_STRIDE;
    }
    Ok(args)
}

fn set_path_arg(args: &mut Args, flag: &str, value: &str) -> Result<(), String> {
    let path = PathBuf::from(value);
    match flag {
        "--contract-doc" => args.contract_doc = path,
        "--reference-receipt" => args.reference_receipt = Some(path),
        "--reference-client-log" => args.reference_client_log = Some(path),
        "--reference-server-log" => args.reference_server_log = Some(path),
        "--valence-receipt" => args.valence_receipt = Some(path),
        "--valence-client-log" => args.valence_client_log = Some(path),
        "--valence-server-log" => args.valence_server_log = Some(path),
        _ => return Err(format!("unknown argument: {flag}")),
    }
    Ok(())
}

fn maybe_load_pair(
    args: &Args,
) -> Result<(Option<EvidenceInput>, Option<EvidenceInput>), Vec<String>> {
    let provided = count_pair_args(args);
    if provided == 0 {
        return Ok((None, None));
    }
    if provided != PAIR_ARGS.len() {
        return Err(missing_pair_arg_messages(args));
    }
    let reference = load_evidence(
        args.reference_receipt.as_ref().expect("checked pair args"),
        args.reference_client_log
            .as_ref()
            .expect("checked pair args"),
        args.reference_server_log
            .as_ref()
            .expect("checked pair args"),
    )?;
    let valence = load_evidence(
        args.valence_receipt.as_ref().expect("checked pair args"),
        args.valence_client_log.as_ref().expect("checked pair args"),
        args.valence_server_log.as_ref().expect("checked pair args"),
    )?;
    Ok((Some(reference), Some(valence)))
}

fn count_pair_args(args: &Args) -> usize {
    [
        args.reference_receipt.as_ref(),
        args.reference_client_log.as_ref(),
        args.reference_server_log.as_ref(),
        args.valence_receipt.as_ref(),
        args.valence_client_log.as_ref(),
        args.valence_server_log.as_ref(),
    ]
    .iter()
    .filter(|item| item.is_some())
    .count()
}

fn missing_pair_arg_messages(args: &Args) -> Vec<String> {
    let fields = [
        ("--reference-receipt", args.reference_receipt.as_ref()),
        ("--reference-client-log", args.reference_client_log.as_ref()),
        ("--reference-server-log", args.reference_server_log.as_ref()),
        ("--valence-receipt", args.valence_receipt.as_ref()),
        ("--valence-client-log", args.valence_client_log.as_ref()),
        ("--valence-server-log", args.valence_server_log.as_ref()),
    ];
    fields
        .into_iter()
        .filter_map(|(flag, value)| {
            if value.is_none() {
                Some(format!("missing pair argument: {flag}"))
            } else {
                None
            }
        })
        .collect()
}

fn load_evidence(
    receipt_path: &PathBuf,
    client_log_path: &PathBuf,
    server_log_path: &PathBuf,
) -> Result<EvidenceInput, Vec<String>> {
    let receipt_text = read_text(receipt_path)?;
    let receipt: Value = serde_json::from_str(&receipt_text)
        .map_err(|error| vec![format!("{}: invalid JSON: {error}", receipt_path.display())])?;
    Ok(EvidenceInput {
        receipt,
        client_log: read_text(client_log_path)?,
        server_log: read_text(server_log_path)?,
    })
}

fn read_text(path: &PathBuf) -> Result<String, Vec<String>> {
    fs::read_to_string(path).map_err(|error| vec![format!("{}: {error}", path.display())])
}

fn run(args: Args) -> Result<String, Vec<String>> {
    let contract_text = read_text(&args.contract_doc)?;
    if args.self_test {
        assert_self_tests(&contract_text);
        return Ok("survival crafting table self-test ok".to_string());
    }
    let mut issues = validate_contract_doc(&contract_text);
    let (reference, valence) = maybe_load_pair(&args)?;
    if reference.is_some() || valence.is_some() {
        let comparison = compare_evidence(reference.as_ref(), valence.as_ref());
        issues.extend(comparison.diagnostics);
    }
    if issues.is_empty() {
        Ok(format!(
            "survival crafting table contract ok: {} metrics",
            REQUIRED_METRICS.len()
        ))
    } else {
        Err(issues)
    }
}

fn main() {
    let args = parse_args().unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    match run(args) {
        Ok(message) => println!("{message}"),
        Err(issues) => {
            for issue in issues {
                eprintln!("{issue}");
            }
            std::process::exit(1);
        }
    }
}
