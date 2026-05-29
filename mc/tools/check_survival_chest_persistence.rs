#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! serde_json = "1"
//! ```

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;

const CONTRACT_DOC: &str =
    "docs/evidence/protocol-763-survival-chest-persistence-contract-2026-05-28.md";
const EXPECTED_PROTOCOL: &str = "763";
const EXPECTED_SCENARIO: &str = "survival-chest-persistence";
const REFERENCE_BACKEND: &str = "paper";
const VALENCE_BACKEND: &str = "valence";
const EXPECTED_CHEST_POSITION: &str = "8,64,0";
const EXPECTED_CHEST_SLOT: &str = "0";
const EXPECTED_STORED_ITEM: &str = "Dirt";
const EXPECTED_STORED_COUNT: &str = "1";
const EXPECTED_RECONNECT_SESSION: &str = "1";
const FIRST_CHEST_WINDOW: &str = "1";
const REOPENED_CHEST_WINDOW: &str = "1";
const MISMATCHED_CHEST_POSITION: &str = "9,64,0";
const MISMATCHED_CHEST_SLOT: &str = "1";
const MISMATCHED_STORED_ITEM: &str = "Stone";
const MISMATCHED_STORED_COUNT: &str = "2";
const MISMATCHED_RECONNECT_SESSION: &str = "2";
const PRESENT: &str = "present";
const ABSENT: &str = "absent";
const NO_VALUE: &str = "<missing>";
const ARG_VALUE_STRIDE: usize = 2;

const CLIENT_MILESTONES: &[&str] = &[
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_chest_open_seen",
    "survival_chest_store_sent",
    "survival_chest_close_sent",
    "survival_chest_reconnect_sent",
    "survival_chest_reopen_seen",
    "survival_chest_persisted_seen",
];

const SERVER_MILESTONES: &[&str] = &[
    "server_survival_chest_open",
    "server_survival_chest_store",
    "server_survival_chest_close",
    "server_survival_chest_reopen",
    "server_survival_chest_persisted",
];

const REQUIRED_METRICS: &[&str] = &[
    "scenario.name",
    "server.protocol",
    "server.backend",
    "client.username",
    "client.missing_milestones.empty",
    "client.forbidden_matches.empty",
    "server.missing_milestones.empty",
    "server.forbidden_matches.empty",
    "client.milestone.protocol_detected",
    "client.milestone.join_game",
    "client.milestone.render_tick",
    "client.milestone.survival_chest_open_seen",
    "client.milestone.survival_chest_store_sent",
    "client.milestone.survival_chest_close_sent",
    "client.milestone.survival_chest_reconnect_sent",
    "client.milestone.survival_chest_reopen_seen",
    "client.milestone.survival_chest_persisted_seen",
    "server.milestone.server_survival_chest_open",
    "server.milestone.server_survival_chest_store",
    "server.milestone.server_survival_chest_close",
    "server.milestone.server_survival_chest_reopen",
    "server.milestone.server_survival_chest_persisted",
    "client.chest.open.window",
    "client.chest.open.position",
    "client.chest.store.window",
    "client.chest.store.slot",
    "client.chest.store.item",
    "client.chest.store.count",
    "client.chest.close.window",
    "client.chest.reconnect.session",
    "client.chest.reopen.window",
    "client.chest.reopen.position",
    "client.chest.persisted.window",
    "client.chest.persisted.slot",
    "client.chest.persisted.item",
    "client.chest.persisted.count",
    "server.chest.open.position",
    "server.chest.open.window",
    "server.chest.store.window",
    "server.chest.store.slot",
    "server.chest.store.item",
    "server.chest.store.count",
    "server.chest.close.window",
    "server.chest.reopen.position",
    "server.chest.reopen.window",
    "server.chest.persisted.slot",
    "server.chest.persisted.item",
    "server.chest.persisted.count",
];

const COMPARISON_METRICS: &[&str] = &[
    "scenario.name",
    "server.protocol",
    "client.chest.open.position",
    "client.chest.store.slot",
    "client.chest.store.item",
    "client.chest.store.count",
    "client.chest.reconnect.session",
    "client.chest.reopen.position",
    "client.chest.persisted.slot",
    "client.chest.persisted.item",
    "client.chest.persisted.count",
    "server.chest.open.position",
    "server.chest.store.slot",
    "server.chest.store.item",
    "server.chest.store.count",
    "server.chest.reopen.position",
    "server.chest.persisted.slot",
    "server.chest.persisted.item",
    "server.chest.persisted.count",
];

const EXPECTED_VALUE_METRICS: &[(&str, &str)] = &[
    ("client.chest.open.position", EXPECTED_CHEST_POSITION),
    ("client.chest.store.slot", EXPECTED_CHEST_SLOT),
    ("client.chest.store.item", EXPECTED_STORED_ITEM),
    ("client.chest.store.count", EXPECTED_STORED_COUNT),
    ("client.chest.reconnect.session", EXPECTED_RECONNECT_SESSION),
    ("client.chest.reopen.position", EXPECTED_CHEST_POSITION),
    ("client.chest.persisted.slot", EXPECTED_CHEST_SLOT),
    ("client.chest.persisted.item", EXPECTED_STORED_ITEM),
    ("client.chest.persisted.count", EXPECTED_STORED_COUNT),
    ("server.chest.open.position", EXPECTED_CHEST_POSITION),
    ("server.chest.store.slot", EXPECTED_CHEST_SLOT),
    ("server.chest.store.item", EXPECTED_STORED_ITEM),
    ("server.chest.store.count", EXPECTED_STORED_COUNT),
    ("server.chest.reopen.position", EXPECTED_CHEST_POSITION),
    ("server.chest.persisted.slot", EXPECTED_CHEST_SLOT),
    ("server.chest.persisted.item", EXPECTED_STORED_ITEM),
    ("server.chest.persisted.count", EXPECTED_STORED_COUNT),
];

const CONTRACT_TOKENS: &[&str] = &[
    EXPECTED_SCENARIO,
    EXPECTED_CHEST_POSITION,
    EXPECTED_CHEST_SLOT,
    EXPECTED_STORED_ITEM,
    EXPECTED_STORED_COUNT,
    EXPECTED_RECONNECT_SESSION,
    "one chest block",
    "one item stack",
    "one chest slot",
    "close, disconnect/reconnect once, reopen",
    "full survival compatibility",
    "all-container behavior",
    "server restart persistence",
    "world persistence",
    "broader vanilla parity",
    "missing_reference",
    "missing_metric",
    "mismatched_metric:*.slot",
    "mismatched_metric:*.item",
    "mismatched_metric:*.count",
    "wrong_backend",
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

fn add_client_log_metrics(
    values: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    client_log: &str,
) {
    let open = find_fields(client_log, "survival_chest_open_seen");
    put_metric(
        values,
        diagnostics,
        "client.chest.open.window",
        open.as_ref(),
        "window",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.open.position",
        open.as_ref(),
        "position",
    );
    let store = find_fields(client_log, "survival_chest_store_sent");
    put_metric(
        values,
        diagnostics,
        "client.chest.store.window",
        store.as_ref(),
        "window",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.store.slot",
        store.as_ref(),
        "slot",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.store.item",
        store.as_ref(),
        "item",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.store.count",
        store.as_ref(),
        "count",
    );
    let close = find_fields(client_log, "survival_chest_close_sent");
    put_metric(
        values,
        diagnostics,
        "client.chest.close.window",
        close.as_ref(),
        "window",
    );
    let reconnect = find_fields(client_log, "survival_chest_reconnect_sent");
    put_metric(
        values,
        diagnostics,
        "client.chest.reconnect.session",
        reconnect.as_ref(),
        "session",
    );
    let reopen = find_fields(client_log, "survival_chest_reopen_seen");
    put_metric(
        values,
        diagnostics,
        "client.chest.reopen.window",
        reopen.as_ref(),
        "window",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.reopen.position",
        reopen.as_ref(),
        "position",
    );
    let persisted = find_fields(client_log, "survival_chest_persisted_seen");
    put_metric(
        values,
        diagnostics,
        "client.chest.persisted.window",
        persisted.as_ref(),
        "window",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.persisted.slot",
        persisted.as_ref(),
        "slot",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.persisted.item",
        persisted.as_ref(),
        "item",
    );
    put_metric(
        values,
        diagnostics,
        "client.chest.persisted.count",
        persisted.as_ref(),
        "count",
    );
}

fn add_server_log_metrics(
    values: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    server_log: &str,
) {
    let open = find_fields(server_log, "survival_chest_open");
    put_metric(
        values,
        diagnostics,
        "server.chest.open.position",
        open.as_ref(),
        "position",
    );
    put_metric(
        values,
        diagnostics,
        "server.chest.open.window",
        open.as_ref(),
        "window",
    );
    let store = find_fields(server_log, "survival_chest_store");
    put_metric(
        values,
        diagnostics,
        "server.chest.store.window",
        store.as_ref(),
        "window",
    );
    put_metric(
        values,
        diagnostics,
        "server.chest.store.slot",
        store.as_ref(),
        "slot",
    );
    put_metric(
        values,
        diagnostics,
        "server.chest.store.item",
        store.as_ref(),
        "item",
    );
    put_metric(
        values,
        diagnostics,
        "server.chest.store.count",
        store.as_ref(),
        "count",
    );
    let close = find_fields(server_log, "survival_chest_close");
    put_metric(
        values,
        diagnostics,
        "server.chest.close.window",
        close.as_ref(),
        "window",
    );
    let reopen = find_fields(server_log, "survival_chest_reopen");
    put_metric(
        values,
        diagnostics,
        "server.chest.reopen.position",
        reopen.as_ref(),
        "position",
    );
    put_metric(
        values,
        diagnostics,
        "server.chest.reopen.window",
        reopen.as_ref(),
        "window",
    );
    let persisted = find_fields(server_log, "survival_chest_persisted");
    put_metric(
        values,
        diagnostics,
        "server.chest.persisted.slot",
        persisted.as_ref(),
        "slot",
    );
    put_metric(
        values,
        diagnostics,
        "server.chest.persisted.item",
        persisted.as_ref(),
        "item",
    );
    put_metric(
        values,
        diagnostics,
        "server.chest.persisted.count",
        persisted.as_ref(),
        "count",
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
    add_required_metric_diagnostics(&values, &mut diagnostics);
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

fn add_required_metric_diagnostics(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
) {
    for metric in REQUIRED_METRICS {
        let value = values.get(*metric).map(String::as_str).unwrap_or(NO_VALUE);
        if value == NO_VALUE || value == ABSENT {
            diagnostics.push(format!("missing_metric:{metric}"));
        }
    }
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
        "client": { "username": "compatbot" },
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
        format!("MC-COMPAT-MILESTONE survival_chest_open_seen window={FIRST_CHEST_WINDOW} position={EXPECTED_CHEST_POSITION}"),
        format!("MC-COMPAT-MILESTONE survival_chest_store_sent window={FIRST_CHEST_WINDOW} slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}"),
        format!("MC-COMPAT-MILESTONE survival_chest_close_sent window={FIRST_CHEST_WINDOW}"),
        format!("MC-COMPAT-MILESTONE survival_chest_reconnect_sent session={EXPECTED_RECONNECT_SESSION}"),
        format!("MC-COMPAT-MILESTONE survival_chest_reopen_seen window={REOPENED_CHEST_WINDOW} position={EXPECTED_CHEST_POSITION}"),
        format!("MC-COMPAT-MILESTONE survival_chest_persisted_seen window={REOPENED_CHEST_WINDOW} slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}"),
    ]
    .join("\n")
}

fn good_server_log() -> String {
    [
        format!("MC-COMPAT-MILESTONE survival_chest_open username=compatbot position={EXPECTED_CHEST_POSITION} window={FIRST_CHEST_WINDOW}"),
        format!("MC-COMPAT-MILESTONE survival_chest_store username=compatbot window={FIRST_CHEST_WINDOW} slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}"),
        format!("MC-COMPAT-MILESTONE survival_chest_close username=compatbot window={FIRST_CHEST_WINDOW}"),
        format!("MC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position={EXPECTED_CHEST_POSITION} window={REOPENED_CHEST_WINDOW}"),
        format!("MC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}"),
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
        "survival_chest_open_seen",
        "survival_chest_missing_open",
        "missing_metric:client.chest.open.window",
    );
    assert_missing_client_marker_rejects(
        "survival_chest_store_sent",
        "survival_chest_missing_store",
        "missing_metric:client.chest.store.slot",
    );
    assert_missing_client_marker_rejects(
        "survival_chest_close_sent",
        "survival_chest_missing_close",
        "missing_metric:client.chest.close.window",
    );
    assert_missing_client_marker_rejects(
        "survival_chest_reconnect_sent",
        "survival_chest_missing_reconnect",
        "missing_metric:client.chest.reconnect.session",
    );
    assert_missing_client_marker_rejects(
        "survival_chest_persisted_seen",
        "survival_chest_missing_persisted",
        "missing_metric:client.chest.persisted.slot",
    );
    assert_missing_server_marker_rejects(
        "survival_chest_persisted",
        "survival_chest_missing_persisted",
        "missing_metric:server.chest.persisted.slot",
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
        EXPECTED_CHEST_POSITION,
        MISMATCHED_CHEST_POSITION,
        "wrong_contract:client.chest.open.position",
    );
    assert_both_wrong_rejects(
        &format!("slot={EXPECTED_CHEST_SLOT}"),
        &format!("slot={MISMATCHED_CHEST_SLOT}"),
        "wrong_contract:client.chest.store.slot",
    );
    assert_both_wrong_rejects(
        &format!("item={EXPECTED_STORED_ITEM}"),
        &format!("item={MISMATCHED_STORED_ITEM}"),
        "wrong_contract:client.chest.store.item",
    );
    assert_both_wrong_rejects(
        &format!("count={EXPECTED_STORED_COUNT}"),
        &format!("count={MISMATCHED_STORED_COUNT}"),
        "wrong_contract:client.chest.store.count",
    );
    assert_both_wrong_rejects(
        &format!("session={EXPECTED_RECONNECT_SESSION}"),
        &format!("session={MISMATCHED_RECONNECT_SESSION}"),
        "wrong_contract:client.chest.reconnect.session",
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
        &format!("slot={EXPECTED_CHEST_SLOT}"),
        &format!("slot={MISMATCHED_CHEST_SLOT}"),
        "mismatched_metric:client.chest.store.slot",
    );
    assert_valence_changed_rejects(
        &format!("item={EXPECTED_STORED_ITEM}"),
        &format!("item={MISMATCHED_STORED_ITEM}"),
        "mismatched_metric:client.chest.store.item",
    );
    assert_valence_changed_rejects(
        &format!("count={EXPECTED_STORED_COUNT}"),
        &format!("count={MISMATCHED_STORED_COUNT}"),
        "mismatched_metric:client.chest.store.count",
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
        return Ok("survival chest persistence self-test ok".to_string());
    }
    let mut issues = validate_contract_doc(&contract_text);
    let (reference, valence) = maybe_load_pair(&args)?;
    if reference.is_some() || valence.is_some() {
        let comparison = compare_evidence(reference.as_ref(), valence.as_ref());
        issues.extend(comparison.diagnostics);
    }
    if issues.is_empty() {
        Ok(format!(
            "survival chest persistence contract ok: {} metrics",
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
