#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuredReceiptSummary {
    schema: String,
    status: String,
    dry_run: bool,
    contract_claims_correctness: bool,
    contract_claims_semantic_equivalence: bool,
    scenario_name: String,
    backend: String,
    client_classification: Option<String>,
    matched_success_pattern: Option<String>,
    client_git_rev: Option<String>,
    client_git_status: String,
    client_git_dirty: bool,
    valence_git_rev_requested: Option<String>,
    valence_git_rev_resolved: Option<String>,
    valence_git_status: String,
    valence_git_dirty: bool,
    wayland_socket_inherited: bool,
    gameplay_non_claims: Vec<String>,
    typed_event: StructuredTypedEventReceipt,
    mcp_control: StructuredMcpControlReceipt,
    frame_artifacts: StructuredFrameArtifactReceipt,
    armor_matrix: StructuredReferenceMatrixReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuredTypedEventReceipt {
    selected: bool,
    migration_status: String,
    event_log_path: Option<String>,
    timeline_blake3: Option<String>,
    event_count: u32,
    contributes_to_pass_fail: bool,
    raw_payloads_recorded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuredMcpControlReceipt {
    selected: bool,
    endpoint_mode: String,
    handshake_success: bool,
    stdout_clean: bool,
    command_outcome_ids: Vec<String>,
    revision_status: String,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuredFrameArtifactReceipt {
    selected: bool,
    artifact_count: u32,
    path: Option<String>,
    blake3: Option<String>,
    path_containment_checked: bool,
    promotion_ready: bool,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuredReferenceMatrixReceipt {
    selected: bool,
    reference_required: bool,
    reference_receipt: String,
    live_receipt: bool,
    promotion_ready: bool,
    non_claims: Vec<String>,
}

const RECEIPT_SCHEMA_V2: &str = "mc.compat.scenario.receipt.v2";
const RECEIPT_OVERCLAIM_TRUE_PATTERNS: &[&str] = &[
    "\"claims_correctness\": true",
    "\"claims_semantic_equivalence\": true",
    "\"claims_broad_minecraft_compatibility\": true",
    "\"claims_production_readiness\": true",
];
const RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM: &str = "broad_minecraft_compatibility";
const RECEIPT_REQUIRED_ARMOR_NON_CLAIM: &str = "all_armor_permutations";
const RECEIPT_REQUIRED_MCP_NON_CLAIM: &str = "semantic_equivalence";
const RECEIPT_REQUIRED_FRAME_NON_CLAIM: &str = "semantic_equivalence";
const RECEIPT_BLAKE3_HEX_CHARS: usize = 64;
const RECEIPT_PARSE_ERROR_PREVIEW_CHARS: usize = 240;
const RECEIPT_VALID_BLAKE3_HEX: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const RECEIPT_TEST_CLIENT_REV: &str = "test-client-revision";
const RECEIPT_TEST_VALENCE_REV: &str = "test-valence-revision";

fn parse_structured_receipt_summary(text: &str) -> Result<StructuredReceiptSummary, String> {
    ensure_unique_receipt_field(text, "schema", "receipt")?;
    ensure_unique_receipt_field(text, "status", "receipt")?;
    ensure_unique_receipt_field(text, "dry_run", "receipt")?;
    for pattern in RECEIPT_OVERCLAIM_TRUE_PATTERNS {
        if text.contains(pattern) {
            return Err(format!("receipt contains overclaim field {pattern}"));
        }
    }

    let contract = json_object_slice(text, "contract")?;
    ensure_unique_receipt_field(contract, "claims_correctness", "contract")?;
    ensure_unique_receipt_field(contract, "claims_semantic_equivalence", "contract")?;
    let scenario = json_object_slice(text, "scenario")?;
    let server = json_object_slice(text, "server")?;
    let client = json_object_slice(text, "client")?;
    let headless = json_object_slice(client, "headless_isolation")?;
    let valence = json_object_slice(text, "valence")?;
    let gameplay = json_object_slice(text, "gameplay_oracles")?;
    let typed_event = json_object_slice(text, "typed_event_oracle")?;
    let mcp_control = json_object_slice(text, "mcp_control")?;
    let frame_artifacts = json_object_slice(text, "frame_artifacts")?;
    let armor_matrix = json_object_slice(text, "armor_loadout_enchantment_status_matrix")?;

    Ok(StructuredReceiptSummary {
        schema: json_string_field(text, "schema")?,
        status: json_string_field(text, "status")?,
        dry_run: json_bool_field(text, "dry_run")?,
        contract_claims_correctness: json_bool_field(contract, "claims_correctness")?,
        contract_claims_semantic_equivalence: json_bool_field(
            contract,
            "claims_semantic_equivalence",
        )?,
        scenario_name: json_string_field(scenario, "name")?,
        backend: json_string_field(server, "backend")?,
        client_classification: json_optional_string_field(client, "classification")?,
        matched_success_pattern: json_optional_string_field(client, "matched_success_pattern")?,
        client_git_rev: json_optional_string_field(client, "git_rev")?,
        client_git_status: json_string_field(client, "git_status")
            .map_err(|err| receipt_parse_context("client", client, err))?,
        client_git_dirty: json_bool_field(client, "git_dirty")
            .map_err(|err| receipt_parse_context("client", client, err))?,
        valence_git_rev_requested: json_optional_string_field(valence, "git_rev_requested")?,
        valence_git_rev_resolved: json_optional_string_field(valence, "git_rev_resolved")?,
        valence_git_status: json_string_field(valence, "git_status")
            .map_err(|err| receipt_parse_context("valence", valence, err))?,
        valence_git_dirty: json_bool_field(valence, "git_dirty")
            .map_err(|err| receipt_parse_context("valence", valence, err))?,
        wayland_socket_inherited: json_bool_field(headless, "wayland_socket_inherited")?,
        gameplay_non_claims: json_optional_string_array_field(gameplay, "non_claims")?
            .unwrap_or_default(),
        typed_event: parse_structured_typed_event_receipt(typed_event)?,
        mcp_control: parse_structured_mcp_control_receipt(mcp_control)?,
        frame_artifacts: parse_structured_frame_artifact_receipt(frame_artifacts)?,
        armor_matrix: parse_structured_reference_matrix_receipt(armor_matrix)?,
    })
}

fn parse_structured_typed_event_receipt(text: &str) -> Result<StructuredTypedEventReceipt, String> {
    ensure_unique_receipt_field(text, "selected", "typed_event_oracle")?;
    ensure_unique_receipt_field(text, "migration_status", "typed_event_oracle")?;
    ensure_unique_receipt_field(text, "event_count", "typed_event_oracle")?;
    Ok(StructuredTypedEventReceipt {
        selected: json_bool_field(text, "selected")?,
        migration_status: json_string_field(text, "migration_status")?,
        event_log_path: json_optional_string_field(text, "event_log_path")?,
        timeline_blake3: json_optional_string_field(text, "timeline_blake3")?,
        event_count: json_u32_field(text, "event_count")?,
        contributes_to_pass_fail: json_bool_field(text, "contributes_to_pass_fail")?,
        raw_payloads_recorded: json_bool_field(text, "raw_payloads_recorded")?,
    })
}

fn parse_structured_mcp_control_receipt(text: &str) -> Result<StructuredMcpControlReceipt, String> {
    ensure_unique_receipt_field(text, "selected", "mcp_control")?;
    ensure_unique_receipt_field(text, "endpoint_mode", "mcp_control")?;
    Ok(StructuredMcpControlReceipt {
        selected: json_bool_field(text, "selected")?,
        endpoint_mode: json_string_field(text, "endpoint_mode")?,
        handshake_success: json_bool_field(text, "handshake_success")?,
        stdout_clean: json_bool_field(text, "stdout_clean")?,
        command_outcome_ids: json_optional_string_array_field(text, "command_outcome_ids")?
            .unwrap_or_default(),
        revision_status: json_string_field(text, "revision_status")?,
        non_claims: json_optional_string_array_field(text, "non_claims")?.unwrap_or_default(),
    })
}

fn parse_structured_frame_artifact_receipt(
    text: &str,
) -> Result<StructuredFrameArtifactReceipt, String> {
    ensure_unique_receipt_field(text, "selected", "frame_artifacts")?;
    ensure_unique_receipt_field(text, "artifact_count", "frame_artifacts")?;
    Ok(StructuredFrameArtifactReceipt {
        selected: json_bool_field(text, "selected")?,
        artifact_count: json_u32_field(text, "artifact_count")?,
        path: json_optional_string_field(text, "path")?,
        blake3: json_optional_string_field(text, "blake3")?,
        path_containment_checked: json_bool_field(text, "path_containment_checked")?,
        promotion_ready: json_bool_field(text, "promotion_ready")?,
        non_claims: json_optional_string_array_field(text, "non_claims")?.unwrap_or_default(),
    })
}

fn parse_structured_reference_matrix_receipt(
    text: &str,
) -> Result<StructuredReferenceMatrixReceipt, String> {
    ensure_unique_receipt_field(text, "selected", "armor_loadout_enchantment_status_matrix")?;
    ensure_unique_receipt_field(
        text,
        "reference_required",
        "armor_loadout_enchantment_status_matrix",
    )?;
    Ok(StructuredReferenceMatrixReceipt {
        selected: json_bool_field(text, "selected")?,
        reference_required: json_bool_field(text, "reference_required")?,
        reference_receipt: json_string_field(text, "reference_receipt")?,
        live_receipt: json_bool_field(text, "live_receipt")?,
        promotion_ready: json_bool_field(text, "promotion_ready")?,
        non_claims: json_optional_string_array_field(text, "non_claims")?.unwrap_or_default(),
    })
}

fn receipt_parse_context(scope: &str, text: &str, err: String) -> String {
    let preview: String = text
        .chars()
        .take(RECEIPT_PARSE_ERROR_PREVIEW_CHARS)
        .collect();
    format!("{err}; {scope} preview={preview:?}")
}

fn validate_structured_receipt_summary(receipt: &StructuredReceiptSummary) -> Result<(), String> {
    if receipt.schema != RECEIPT_SCHEMA_V2 {
        return Err(format!("unexpected receipt schema {}", receipt.schema));
    }
    if receipt.contract_claims_correctness || receipt.contract_claims_semantic_equivalence {
        return Err("receipt contract overclaims compatibility".to_string());
    }
    if !matches!(receipt.backend.as_str(), "paper" | "valence") {
        return Err(format!("unsupported receipt backend {}", receipt.backend));
    }
    if receipt.wayland_socket_inherited {
        return Err("receipt does not prove headless Wayland isolation".to_string());
    }
    if receipt.status == "pass" && !receipt.dry_run && receipt.client_classification.is_none() {
        return Err("passing live receipt missing client classification".to_string());
    }
    if receipt.status == "pass" && !receipt.dry_run && receipt.matched_success_pattern.is_none() {
        return Err("passing live receipt missing matched success pattern".to_string());
    }
    validate_structured_child_revision(
        "client",
        receipt.dry_run,
        &receipt.client_git_status,
        receipt.client_git_dirty,
    )?;
    validate_structured_child_revision(
        "valence",
        receipt.dry_run,
        &receipt.valence_git_status,
        receipt.valence_git_dirty,
    )?;
    if !receipt
        .gameplay_non_claims
        .iter()
        .any(|claim| claim == RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM)
    {
        return Err(format!(
            "receipt missing non_claim {RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM}"
        ));
    }
    validate_structured_typed_event_receipt(&receipt.typed_event)?;
    validate_structured_mcp_control_receipt(&receipt.mcp_control)?;
    validate_structured_frame_artifact_receipt(&receipt.frame_artifacts)?;
    validate_structured_reference_matrix_receipt(&receipt.armor_matrix)?;
    Ok(())
}

fn validate_structured_child_revision(
    label: &str,
    dry_run: bool,
    status: &str,
    dirty: bool,
) -> Result<(), String> {
    if dry_run {
        if status != GIT_STATUS_DRY_RUN || dirty {
            return Err(format!(
                "{label} dry-run child revision is not deterministic"
            ));
        }
        return Ok(());
    }
    if status != GIT_STATUS_CLEAN || dirty {
        return Err(format!("{label} child revision is not clean"));
    }
    Ok(())
}

fn validate_structured_typed_event_receipt(
    typed_event: &StructuredTypedEventReceipt,
) -> Result<(), String> {
    if typed_event.raw_payloads_recorded {
        return Err("typed event oracle records raw payloads".to_string());
    }
    if typed_event.selected {
        if typed_event.migration_status != TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES {
            return Err("typed event oracle selected without derived migration status".to_string());
        }
        let path = typed_event
            .event_log_path
            .as_deref()
            .ok_or_else(|| "typed event oracle missing event_log_path".to_string())?;
        validate_structured_artifact_path("typed event oracle", path)?;
        let digest = typed_event
            .timeline_blake3
            .as_deref()
            .ok_or_else(|| "typed event oracle missing timeline_blake3".to_string())?;
        validate_blake3_hex("typed event oracle", digest)?;
        if typed_event.event_count == 0 {
            return Err("typed event oracle selected with zero events".to_string());
        }
    } else {
        if typed_event.migration_status != TYPED_EVENT_MIGRATION_FALLBACK {
            return Err("typed event oracle fallback has wrong migration status".to_string());
        }
        if typed_event.event_log_path.is_some() || typed_event.timeline_blake3.is_some() {
            return Err("typed event fallback records artifact paths".to_string());
        }
        if typed_event.event_count != 0 || typed_event.contributes_to_pass_fail {
            return Err("typed event fallback records pass/fail event evidence".to_string());
        }
    }
    Ok(())
}

fn validate_structured_mcp_control_receipt(
    mcp: &StructuredMcpControlReceipt,
) -> Result<(), String> {
    if !mcp.selected {
        return Ok(());
    }
    if mcp.endpoint_mode != "stdio" {
        return Err(format!(
            "mcp_control wrong endpoint mode {}",
            mcp.endpoint_mode
        ));
    }
    if !mcp.handshake_success || !mcp.stdout_clean {
        return Err("mcp_control missing handshake/stdout proof".to_string());
    }
    if !matches!(
        mcp.revision_status.as_str(),
        GIT_STATUS_CLEAN | GIT_STATUS_DRY_RUN
    ) {
        return Err(format!(
            "mcp_control has unacceptable child revision status {}",
            mcp.revision_status
        ));
    }
    if !mcp
        .command_outcome_ids
        .iter()
        .any(|outcome| outcome == "status.applied")
    {
        return Err("mcp_control missing status.applied outcome".to_string());
    }
    if !mcp
        .non_claims
        .iter()
        .any(|claim| claim == RECEIPT_REQUIRED_MCP_NON_CLAIM)
    {
        return Err(format!(
            "mcp_control missing non_claim {RECEIPT_REQUIRED_MCP_NON_CLAIM}"
        ));
    }
    Ok(())
}

fn validate_structured_frame_artifact_receipt(
    frame: &StructuredFrameArtifactReceipt,
) -> Result<(), String> {
    if frame.selected {
        if !frame.path_containment_checked {
            return Err("frame artifacts missing path containment check".to_string());
        }
        let path = frame
            .path
            .as_deref()
            .ok_or_else(|| "frame artifacts missing path".to_string())?;
        validate_structured_artifact_path("frame artifacts", path)?;
        let digest = frame
            .blake3
            .as_deref()
            .ok_or_else(|| "frame artifacts missing blake3".to_string())?;
        validate_blake3_hex("frame artifacts", digest)?;
        if frame.artifact_count == 0 {
            return Err("frame artifacts selected with zero artifacts".to_string());
        }
    }
    if !frame
        .non_claims
        .iter()
        .any(|claim| claim == RECEIPT_REQUIRED_FRAME_NON_CLAIM)
    {
        return Err(format!(
            "frame artifacts missing non_claim {RECEIPT_REQUIRED_FRAME_NON_CLAIM}"
        ));
    }
    Ok(())
}

fn validate_structured_reference_matrix_receipt(
    matrix: &StructuredReferenceMatrixReceipt,
) -> Result<(), String> {
    if matrix.selected {
        if matrix.reference_required
            && matrix.reference_receipt == ARMOR_MATRIX_REFERENCE_RECEIPT_NONE
        {
            return Err("selected armor matrix requires missing reference receipt".to_string());
        }
        if matrix.promotion_ready && !matrix.live_receipt {
            return Err("selected armor matrix promotes without live receipt".to_string());
        }
    }
    if !matrix
        .non_claims
        .iter()
        .any(|claim| claim == RECEIPT_REQUIRED_ARMOR_NON_CLAIM)
    {
        return Err(format!(
            "armor matrix missing non_claim {RECEIPT_REQUIRED_ARMOR_NON_CLAIM}"
        ));
    }
    Ok(())
}

fn validate_structured_artifact_path(label: &str, value: &str) -> Result<(), String> {
    if value.is_empty() || value.contains('\0') {
        return Err(format!("{label} artifact path is empty or contains NUL"));
    }
    if Path::new(value)
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(format!("{label} artifact path escapes evidence root"));
    }
    Ok(())
}

fn validate_blake3_hex(label: &str, value: &str) -> Result<(), String> {
    if value.len() != RECEIPT_BLAKE3_HEX_CHARS || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!("{label} has malformed BLAKE3 digest"));
    }
    Ok(())
}

fn ensure_unique_receipt_field(text: &str, key: &str, scope: &str) -> Result<(), String> {
    let expected_count = 1;
    let count = receipt_key_occurrence_count(text, key);
    if count == expected_count {
        return Ok(());
    }
    Err(format!("{scope} field {key} expected once, found {count}"))
}

fn receipt_key_occurrence_count(text: &str, key: &str) -> usize {
    let needle = format!("\"{key}\"");
    text.match_indices(&needle).count()
}

fn structured_receipt_from_text(text: &str) -> Result<StructuredReceiptSummary, String> {
    let summary = parse_structured_receipt_summary(text)?;
    validate_structured_receipt_summary(&summary)?;
    Ok(summary)
}

fn dry_run_child_revisions(cfg: &Config) -> ChildRevisionEvidence {
    ChildRevisionEvidence {
        client: GitRevisionEvidence::dry_run(None),
        valence: GitRevisionEvidence::dry_run(Some(cfg.valence_rev.clone())),
    }
}

fn clean_child_revisions(cfg: &Config) -> ChildRevisionEvidence {
    ChildRevisionEvidence {
        client: GitRevisionEvidence {
            requested_rev: None,
            resolved_rev: Some(RECEIPT_TEST_CLIENT_REV.to_string()),
            status: GIT_STATUS_CLEAN,
            dirty: false,
            diagnostics: Vec::new(),
        },
        valence: GitRevisionEvidence {
            requested_rev: Some(cfg.valence_rev.clone()),
            resolved_rev: Some(RECEIPT_TEST_VALENCE_REV.to_string()),
            status: GIT_STATUS_CLEAN,
            dirty: false,
            diagnostics: Vec::new(),
        },
    }
}

fn receipt_model_for_test(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &str>,
    typed_event_oracle: Option<&TypedEventOracleArtifact>,
    child_revisions: &ChildRevisionEvidence,
) -> ScenarioReceiptModel {
    build_scenario_receipt_model(ScenarioReceiptInput {
        cfg,
        result,
        typed_event_oracle,
        child_revisions,
    })
}

fn assert_receipt_model_renders(model: &ScenarioReceiptModel, expected_scenario: &str) -> String {
    validate_scenario_receipt_model(model).expect("typed receipt model validates");
    let json = render_scenario_receipt_model_json(model);
    validate_rendered_scenario_receipt_json(&json).expect("rendered receipt validates");
    assert!(json.contains(expected_scenario), "{json}");
    json
}

#[test]
fn scenario_receipt_model_captures_structural_inputs_before_rendering() {
    let cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
        .expect("dry-run config parses");
    let child_revisions = dry_run_child_revisions(&cfg);
    let model = receipt_model_for_test(&cfg, Ok(&None), None, &child_revisions);

    assert_eq!(model.schema, RECEIPT_SCHEMA_V2);
    assert_eq!(model.legacy_schema, "mc.compat.smoke.receipt.v1");
    assert_eq!(model.status, ScenarioReceiptStatus::Pass);
    assert_eq!(model.scenario.name, "survival-break-place-pickup");
    assert!(!model.scenario.client.passed);
    assert!(!model.scenario.server.passed);
    assert!(!model.selected_sections.typed_event_oracle);
    assert!(!model.selected_sections.mcp_control);
    assert!(!model.selected_sections.frame_artifacts);
    assert!(!model.selected_sections.projectile_damage_causality);
    assert!(model
        .gameplay_non_claims
        .iter()
        .any(|claim| *claim == RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM));

    let json = assert_receipt_model_renders(&model, "survival-break-place-pickup");
    let receipt = structured_receipt_from_text(&json).expect("rendered dry receipt validates");
    assert_eq!(receipt.schema, RECEIPT_SCHEMA_V2);
    assert!(receipt.dry_run);
}

#[test]
fn scenario_receipt_model_positive_paths_cover_required_shapes() {
    let dry_cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
        .expect("dry-run config parses");
    let dry_revisions = dry_run_child_revisions(&dry_cfg);
    let dry_model = receipt_model_for_test(&dry_cfg, Ok(&None), None, &dry_revisions);
    let dry_json = assert_receipt_model_renders(&dry_model, "survival-break-place-pickup");
    assert!(dry_json.contains("\"dry_run\": true"), "{dry_json}");

    let fail_cfg =
        test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("failure receipt config parses");
    let fail_revisions = dry_run_child_revisions(&fail_cfg);
    let fail_model = receipt_model_for_test(&fail_cfg, Err("preflight"), None, &fail_revisions);
    let fail_json = assert_receipt_model_renders(&fail_model, "smoke");
    assert!(fail_json.contains("\"status\": \"fail\""), "{fail_json}");

    let multi_cfg = test_config(&["--run", "--scenario", "multi-client-load-score"], &[])
        .expect("multi-client config parses");
    let multi_client = Some(ClientRunEvidence {
        log_path: Some(PathBuf::from("/tmp/multi-client.log")),
        log_paths: vec![
            PathBuf::from("/tmp/multi-client-a.log"),
            PathBuf::from("/tmp/multi-client-b.log"),
        ],
        usernames: vec!["compatbota".to_string(), "compatbotb".to_string()],
        exit_code: Some(124),
        classification: "multi-client-load-evidence",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(evaluate_scenario(
            Scenario::MultiClientLoadScore,
            "mc_compat_multi_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\n",
        )),
        server_scenario: Some(evaluate_server_scenario(
            Scenario::MultiClientLoadScore,
            "compatbota joined\ncompatbotb joined\nred flag captured\n",
            "compatbot",
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    });
    let multi_revisions = clean_child_revisions(&multi_cfg);
    let multi_model = receipt_model_for_test(&multi_cfg, Ok(&multi_client), None, &multi_revisions);
    let multi_json = assert_receipt_model_renders(&multi_model, "multi-client-load-score");
    let multi_receipt =
        structured_receipt_from_text(&multi_json).expect("multi-client rendered receipt validates");
    assert_eq!(
        multi_receipt.client_classification.as_deref(),
        Some("multi-client-load-evidence")
    );

    let projectile_cfg = test_config(&["--scenario", "projectile-damage-attribution"], &[])
        .expect("projectile config parses");
    let projectile_client = Some(projectile_damage_dry_run_evidence(&projectile_cfg));
    let projectile_revisions = dry_run_child_revisions(&projectile_cfg);
    let projectile_model = receipt_model_for_test(
        &projectile_cfg,
        Ok(&projectile_client),
        None,
        &projectile_revisions,
    );
    assert!(
        projectile_model
            .selected_sections
            .projectile_damage_causality
    );
    let projectile_json =
        assert_receipt_model_renders(&projectile_model, "projectile-damage-attribution");
    assert!(
        projectile_json.contains("\"projectile_damage_causality\""),
        "{projectile_json}"
    );

    let mcp_cfg = test_config(&["--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp-controlled config parses");
    let mcp_client = Some(mcp_controlled_dry_run_evidence(&mcp_cfg));
    let mcp_revisions = dry_run_child_revisions(&mcp_cfg);
    let mcp_model = receipt_model_for_test(&mcp_cfg, Ok(&mcp_client), None, &mcp_revisions);
    assert!(mcp_model.selected_sections.mcp_control);
    let mcp_json = assert_receipt_model_renders(&mcp_model, MCP_CONTROLLED_SMOKE_SCENARIO);
    assert!(mcp_json.contains("\"mcp_control\""), "{mcp_json}");

    let events = typed_event_fixture();
    let timeline = normalize_typed_event_timeline(&events);
    let typed_artifact = TypedEventOracleArtifact {
        event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
        timeline_blake3: typed_event_timeline_blake3(&timeline),
        event_count: events.len(),
        contributes_to_pass_fail: true,
    };
    let typed_model =
        receipt_model_for_test(&dry_cfg, Ok(&None), Some(&typed_artifact), &dry_revisions);
    assert!(typed_model.selected_sections.typed_event_oracle);
    let typed_json = assert_receipt_model_renders(&typed_model, "survival-break-place-pickup");
    assert!(
        typed_json.contains("\"typed_event_oracle\""),
        "{typed_json}"
    );
    assert!(typed_json.contains("\"event_count\": 3"), "{typed_json}");
}

#[test]
fn failure_bundle_model_rendering_path_remains_diagnostic_only() {
    let cfg = test_config(&["--scenario", "smoke"], &[]).expect("failure bundle config parses");
    let artifact = FailureBundleArtifact {
        kind: FAILURE_BUNDLE_ARTIFACT_RECEIPT.to_string(),
        path: "docs/evidence/failure-receipt.json".to_string(),
        blake3: RECEIPT_VALID_BLAKE3_HEX.to_string(),
    };
    let bundle =
        failure_bundle_from_config(&cfg, "scenario missing required milestone", vec![artifact]);

    validate_failure_evidence_bundle(&bundle).expect("failure bundle model validates");
    let json = render_failure_evidence_bundle_json(&bundle);

    assert!(json.contains("\"diagnostic_only\": true"), "{json}");
    assert!(
        json.contains("\"claims_scenario_success\": false"),
        "{json}"
    );
    assert!(
        json.contains(FAILURE_BUNDLE_NON_CLAIM_SEMANTIC_EQUIVALENCE),
        "{json}"
    );
}

#[test]
fn scenario_receipt_model_negative_inputs_fail_closed() {
    let live_cfg =
        test_config(&["--run", "--scenario", "smoke"], &[]).expect("live smoke config parses");
    let clean_revisions = clean_child_revisions(&live_cfg);
    let missing_client = receipt_model_for_test(&live_cfg, Ok(&None), None, &clean_revisions);
    let err = validate_scenario_receipt_model(&missing_client)
        .expect_err("passing live receipt without client evidence fails closed");
    assert!(err.contains("client evidence"), "{err}");

    let dry_cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
        .expect("dry-run config parses");
    let dry_revisions = dry_run_child_revisions(&dry_cfg);
    let valid_model = receipt_model_for_test(&dry_cfg, Ok(&None), None, &dry_revisions);
    let json = render_scenario_receipt_model_json(&valid_model);
    let duplicate_status = json.replacen(
        "\"status\": \"pass\",",
        "\"status\": \"pass\",\n  \"status\": \"pass\",",
        1,
    );
    let err = validate_rendered_scenario_receipt_json(&duplicate_status)
        .expect_err("duplicate rendered fields fail closed");
    assert!(err.contains("duplicate field status"), "{err}");

    let malformed_path_artifact = TypedEventOracleArtifact {
        event_log_path: PathBuf::from("../escape.log"),
        timeline_blake3: RECEIPT_VALID_BLAKE3_HEX.to_string(),
        event_count: typed_event_fixture().len(),
        contributes_to_pass_fail: true,
    };
    let malformed_path_model = receipt_model_for_test(
        &dry_cfg,
        Ok(&None),
        Some(&malformed_path_artifact),
        &dry_revisions,
    );
    let err = validate_scenario_receipt_model(&malformed_path_model)
        .expect_err("malformed artifact path fails closed");
    assert!(err.contains("escapes"), "{err}");

    let invalid_digest_artifact = TypedEventOracleArtifact {
        event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
        timeline_blake3: "not-a-blake3-digest".to_string(),
        event_count: typed_event_fixture().len(),
        contributes_to_pass_fail: true,
    };
    let invalid_digest_model = receipt_model_for_test(
        &dry_cfg,
        Ok(&None),
        Some(&invalid_digest_artifact),
        &dry_revisions,
    );
    let err = validate_scenario_receipt_model(&invalid_digest_model)
        .expect_err("invalid artifact digest fails closed");
    assert!(err.contains("BLAKE3"), "{err}");

    let mut unsupported_selected = valid_model.clone();
    unsupported_selected.frame_artifacts = FrameArtifactsReceiptEvidence {
        selected: true,
        capture_requested: true,
        artifact_count: 0,
        artifacts: Vec::new(),
        missing_digests: Vec::new(),
        path_containment_checked: true,
        promotion_ready: false,
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    };
    unsupported_selected.selected_sections.frame_artifacts = true;
    let err = validate_scenario_receipt_model(&unsupported_selected)
        .expect_err("selected frame artifact without evidence fails closed");
    assert!(err.contains("without artifact evidence"), "{err}");
}

#[test]
fn dry_run_receipt_records_deterministic_child_revision_placeholders() {
    let cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
        .expect("dry-run config parses");
    let json = smoke_receipt_json(&cfg, Ok(&None));

    let receipt = structured_receipt_from_text(&json).expect("dry-run receipt validates");
    assert_eq!(
        receipt.client_git_rev.as_deref(),
        Some(GIT_REV_DRY_RUN_PLACEHOLDER)
    );
    assert_eq!(receipt.client_git_status, GIT_STATUS_DRY_RUN);
    assert!(!receipt.client_git_dirty);
    assert_eq!(
        receipt.valence_git_rev_requested.as_deref(),
        Some(DEFAULT_VALENCE_REV)
    );
    assert_eq!(
        receipt.valence_git_rev_resolved.as_deref(),
        Some(GIT_REV_DRY_RUN_PLACEHOLDER)
    );
    assert_eq!(receipt.valence_git_status, GIT_STATUS_DRY_RUN);
    assert!(!receipt.valence_git_dirty);
}

#[test]
fn paired_reference_dry_run_receipts_record_shape_nonclaims() {
    const PAIRED_REFERENCE_SCENARIOS: &[&str] = &[
        "vanilla-combat-reference-parity",
        "vanilla-combat-armor-reference-parity",
    ];

    for scenario in PAIRED_REFERENCE_SCENARIOS {
        let scenario_arg = format!("--scenario={scenario}");
        let cfg = test_config(&[scenario_arg.as_str()], &[])
            .expect("paired-reference dry-run config parses");
        let json = smoke_receipt_json(&cfg, Ok(&None));
        let shape = json_object_slice(&json, "paired_reference_dry_run_shape")
            .expect("paired reference shape block exists");

        assert!(json_bool_field(shape, "selected").expect("selected parses"));
        assert_eq!(
            json_string_field(shape, "scenario").expect("scenario parses"),
            *scenario
        );
        assert_eq!(
            json_string_field(shape, "reference_backend").expect("reference backend parses"),
            "paper-reference"
        );
        assert_eq!(
            json_string_field(shape, "valence_backend").expect("valence backend parses"),
            "valence"
        );
        assert_eq!(
            json_string_field(shape, "reference_revision").expect("reference revision parses"),
            GIT_REV_DRY_RUN_PLACEHOLDER
        );
        assert_eq!(
            json_string_field(shape, "comparison_status").expect("status parses"),
            "dry-run-shape-not-compared"
        );
        assert!(!json_bool_field(shape, "live_comparator_evidence").expect("live flag parses"));
        assert!(!json_bool_field(shape, "claims_live_parity").expect("claim flag parses"));
        assert!(!json_bool_field(shape, "claims_exact_vanilla_parity")
            .expect("exact parity flag parses"));
        assert!(shape.contains("damage_tolerance"), "{shape}");
        assert!(shape.contains("knockback_tolerance"), "{shape}");
        assert!(shape.contains("not_live_paper_valence_evidence"), "{shape}");
        assert!(shape.contains("not_exact_mojang_vanilla_parity"), "{shape}");
    }

    let live_cfg = test_config(
        &["--run", "--scenario=vanilla-combat-reference-parity"],
        &[],
    )
    .expect("paired-reference live config parses");
    let live_json = smoke_receipt_json(&live_cfg, Ok(&None));
    let live_shape = json_object_slice(&live_json, "paired_reference_dry_run_shape")
        .expect("paired reference shape block exists");
    assert!(!json_bool_field(live_shape, "selected").expect("selected parses"));
}

#[test]
fn mcp_controlled_dry_run_receipt_records_control_contract() {
    let cfg = test_config(&["--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp-controlled config parses");
    let evidence = mcp_controlled_dry_run_evidence(&cfg);
    let json = smoke_receipt_json(&cfg, Ok(&Some(evidence)));

    let receipt = structured_receipt_from_text(&json).expect("mcp dry-run receipt validates");
    assert_eq!(receipt.scenario_name, MCP_CONTROLLED_SMOKE_SCENARIO);
    assert!(receipt.mcp_control.selected);
    assert_eq!(receipt.mcp_control.endpoint_mode, "stdio");
    assert!(receipt.mcp_control.handshake_success);
    assert!(receipt.mcp_control.stdout_clean);
    assert!(receipt
        .mcp_control
        .command_outcome_ids
        .iter()
        .any(|outcome| outcome == "status.applied"));
    assert_eq!(receipt.mcp_control.revision_status, GIT_STATUS_DRY_RUN);
    assert!(!receipt.frame_artifacts.promotion_ready);
    assert!(receipt
        .mcp_control
        .non_claims
        .iter()
        .any(|claim| claim == RECEIPT_REQUIRED_MCP_NON_CLAIM));
}

#[test]
fn biome_dimension_join_state_receipt_records_typed_correlation() {
    let cfg = test_config(
        &[
            "--scenario",
            "survival-biome-dimension-state",
            "--client-dir",
            "/tmp/stevenarella",
        ],
        &[],
    )
    .expect("biome/dimension receipt config parses");
    let client_scenario = evaluate_scenario(
        Scenario::SurvivalBiomeDimensionState,
        &format!(
            "Detected server protocol version {DEFAULT_SERVER_PROTOCOL}\njoin_game\nrender_tick_with_player\n{SURVIVAL_BIOME_DIMENSION_CLIENT_STATE_NEEDLE}\n"
        ),
    );
    let server_scenario = evaluate_server_scenario(
        Scenario::SurvivalBiomeDimensionState,
        SURVIVAL_BIOME_DIMENSION_SERVER_STATE_NEEDLE,
        TEST_USERNAME,
    );
    let client = Some(ClientRunEvidence {
        log_path: Some(PathBuf::from("/tmp/client.log")),
        log_paths: vec![PathBuf::from("/tmp/client.log")],
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: Some(0),
        classification: "client-exited-success",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(client_scenario),
        server_scenario: Some(server_scenario),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    });

    let json = smoke_receipt_json(&cfg, Ok(&client));
    assert!(json.contains("\"biome_dimension_join_state\""), "{json}");
    assert!(json.contains("\"client_observed_state\""), "{json}");
    assert!(json.contains("\"server_configured_state\""), "{json}");
    assert!(json.contains("\"not_dimension_travel\""), "{json}");
    assert!(
        json.contains("\"not_full_survival_compatibility\""),
        "{json}"
    );
    assert!(json.contains("\"diagnostics\": []"), "{json}");
}

#[test]
fn typed_event_receipt_artifact_records_reviewable_timeline_hash() {
    let events = typed_event_fixture();
    let timeline = normalize_typed_event_timeline(&events);
    let timeline_blake3 = typed_event_timeline_blake3(&timeline);
    let artifact = TypedEventOracleArtifact {
        event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
        timeline_blake3: timeline_blake3.clone(),
        event_count: events.len(),
        contributes_to_pass_fail: false,
    };

    let json = typed_event_oracle_receipt_json(Some(&artifact));
    let receipt = parse_structured_typed_event_receipt(&json)
        .expect("typed event receipt parses structurally");
    validate_structured_typed_event_receipt(&receipt)
        .expect("typed event receipt validates structurally");

    assert!(receipt.selected);
    assert_eq!(
        receipt.migration_status,
        TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES
    );
    assert_eq!(
        receipt.event_log_path.as_deref(),
        Some("/tmp/mc-compat.typed-events.log")
    );
    assert_eq!(
        receipt.timeline_blake3.as_deref(),
        Some(timeline_blake3.as_str())
    );
    assert_eq!(receipt.event_count as usize, events.len());
    assert!(!receipt.contributes_to_pass_fail);
    assert!(!receipt.raw_payloads_recorded);
}

#[test]
fn typed_event_oracle_receipt_records_migration_fallback() {
    let cfg = test_config(
        &[
            "--server-backend=paper",
            "--receipt",
            "/tmp/receipt.json",
            "--client-dir",
            "/tmp/stevenarella",
        ],
        &[],
    )
    .expect("receipt config parses");
    let json = smoke_receipt_json(&cfg, Err("preflight"));
    let typed_event = parse_structured_typed_event_receipt(
        json_object_slice(&json, "typed_event_oracle").expect("typed event object"),
    )
    .expect("typed event fallback parses");

    validate_structured_typed_event_receipt(&typed_event).expect("typed event fallback validates");
    assert!(!typed_event.selected);
    assert_eq!(typed_event.migration_status, TYPED_EVENT_MIGRATION_FALLBACK);
    assert!(!typed_event.raw_payloads_recorded);
}

#[test]
fn status_packet_proxy_and_gameplay_receipt_blocks_are_recorded() {
    let mut cfg = test_config(
        &[
            "--server-backend=valence",
            "--scenario=reconnect-flag-score",
            "--expect-status-description=compat fixture",
            "--expect-status-version=compat-version",
            "--expect-status-sample=compatbot,observer",
            "--packet-capture-summary",
            "--proxy-route=velocity-local",
            "--proxy-forwarding-mode=modern",
            "--client-dir=/tmp/stevenarella",
        ],
        &[],
    )
    .expect("extended receipt config parses");
    cfg.server_port = 25565;
    let scenario = evaluate_scenario(
        Scenario::ReconnectFlagScore,
        "Detected server protocol version 763
join_game
render_tick_with_player
You are on team RED!
You have the flag!
You captured the flag!
RED: 1
mc_compat_reconnect_session=2
",
    );
    assert!(scenario.passed, "{scenario:?}");
    let client = Some(ClientRunEvidence {
        log_path: Some(PathBuf::from("/tmp/client.log")),
        log_paths: vec![PathBuf::from("/tmp/client.log")],
        usernames: vec!["compatbot".to_string()],
        exit_code: Some(124),
        classification: "timeout-success-evidence",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(scenario),
        server_scenario: Some(evaluate_server_scenario(
            Scenario::ReconnectFlagScore,
            "compatbot joined
red flag captured
",
            "compatbot",
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    });

    let json = smoke_receipt_json(&cfg, Ok(&client));

    assert!(
        json.contains("\"name\": \"reconnect-flag-score\""),
        "{json}"
    );
    assert!(json.contains("\"status_response_resource\""), "{json}");
    assert!(json.contains("\"configured\": true"), "{json}");
    assert!(json.contains("compat fixture"), "{json}");
    assert!(json.contains("compat-version"), "{json}");
    assert!(json.contains("compatbot"), "{json}");
    assert!(json.contains("\"packet_capture_oracle\""), "{json}");
    assert!(json.contains("\"selected\": true"), "{json}");
    assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
    assert!(json.contains("\"proxy_compat_seam\""), "{json}");
    assert!(json.contains("\"route\": \"velocity-local\""), "{json}");
    assert!(json.contains("\"forwarding_mode\": \"modern\""), "{json}");
    assert!(json.contains("\"mtls_ported\": false"), "{json}");
    assert!(json.contains("\"gameplay_oracles\""), "{json}");
    assert!(json.contains("reconnect_session"), "{json}");
    assert!(json.contains("full_ctf_correctness"), "{json}");
}

#[test]
fn compat_bot_probe_scenario_is_bounded_and_receipted() {
    let pass = evaluate_scenario(
        Scenario::CompatBotProbe,
        "Detected server protocol version 763\njoin_game\nrender_tick_with_player\n",
    );
    assert!(pass.passed, "{pass:?}");
    assert_eq!(pass.missing_milestones, Vec::<&str>::new());

    let fail = evaluate_scenario(
        Scenario::CompatBotProbe,
        "Detected server protocol version 763\n",
    );
    assert!(!fail.passed, "{fail:?}");
    assert!(fail.missing_milestones.contains(&"join_game"));
    assert!(fail.missing_milestones.contains(&"render_tick"));

    let mut cfg = test_config(
        &[
            "--server-backend=valence",
            "--scenario=valence-compat-bot-probe",
            "--receipt=/tmp/receipt.json",
            "--client-dir=/tmp/stevenarella",
        ],
        &[],
    )
    .expect("receipt config parses");
    cfg.server_port = 25565;
    let client = Some(ClientRunEvidence {
        log_path: Some(PathBuf::from("/tmp/client.log")),
        log_paths: vec![PathBuf::from("/tmp/client.log")],
        usernames: vec!["compatbot".to_string()],
        exit_code: Some(124),
        classification: "timeout-success-evidence",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(pass),
        server_scenario: Some(evaluate_server_scenario(
            Scenario::CompatBotProbe,
            "compatbot joined\n",
            "compatbot",
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    });

    let json = smoke_receipt_json(&cfg, Ok(&client));

    assert!(
        json.contains("\"name\": \"valence-compat-bot-probe\""),
        "{json}"
    );
    assert!(json.contains("\"compat_bot_probe\""), "{json}");
    assert!(json.contains("\"selected\": true"), "{json}");
    assert!(json.contains("\"safe_bounded_probe\": true"), "{json}");
    assert!(
        json.contains("\"external_server_load_authorized\": false"),
        "{json}"
    );
    assert!(json.contains("\"public_stress_tool\": false"), "{json}");
    assert!(json.contains("\"planned_clients\": 1"), "{json}");
    assert!(json.contains("\"max_clients\": 1"), "{json}");
    assert!(
        json.contains("\"target_address\": \"127.0.0.1:25565\""),
        "{json}"
    );
    assert!(json.contains("\"load_network_safety\""), "{json}");
    assert!(
        json.contains("\"target_scope\": \"owned-local-loopback\""),
        "{json}"
    );
    assert!(
        json.contains("\"claims_public_server_safety\": false"),
        "{json}"
    );
    assert!(json.contains("\"claims_unbounded_soak\": false"), "{json}");
}

#[test]
fn negative_live_rail_receipt_records_observed_containment_outcome() {
    let cfg = test_config(&["--run", "--scenario", "negative-custom-payload"], &[])
        .expect("negative live rail config parses");
    let scenario = evaluate_scenario(
        Scenario::NegativeCustomPayload,
        "Detected server protocol version 763
join_game
render_tick_with_player
negative_custom_payload_sent
negative_custom_payload_contained
",
    );
    assert!(scenario.passed, "{scenario:?}");
    let client = Some(ClientRunEvidence {
        log_path: Some(PathBuf::from("/tmp/client.log")),
        log_paths: vec![PathBuf::from("/tmp/client.log")],
        usernames: vec!["compatbot".to_string()],
        exit_code: Some(124),
        classification: "timeout-success-evidence",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(scenario),
        server_scenario: Some(evaluate_server_scenario(
            Scenario::NegativeCustomPayload,
            "compatbot joined\n",
            "compatbot",
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    });

    let json = smoke_receipt_json(&cfg, Ok(&client));
    assert!(
        json.contains("\"observed_outcome\": \"containment_observed\""),
        "{json}"
    );
    assert!(
        json.contains("client_milestone:negative_custom_payload_contained"),
        "{json}"
    );
    assert!(json.contains("\"telemetry_present\": true"), "{json}");
    assert!(json.contains("\"preflight_passed\": true"), "{json}");
}

#[test]
fn negative_live_rail_envelope_records_expected_outcome_and_non_claims() {
    let cfg = test_config(
        &["--dry-run", "--scenario", "negative-inventory-stale-state"],
        &[],
    )
    .expect("negative rail config parses");
    let evidence = evaluate_negative_live_rail_safety(&cfg);
    assert!(evidence.selected, "{evidence:?}");
    assert_eq!(evidence.rail, Some("negative-inventory-stale-state"));
    assert_eq!(evidence.invalid_action, Some("stale_inventory_state_id"));
    assert_eq!(
        evidence.expected_outcome,
        Some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME)
    );
    assert!(evidence.owned_local_target, "{evidence:?}");
    assert!(evidence.preflight_passed, "{evidence:?}");

    let json = smoke_receipt_json(&cfg, Ok(&None));
    assert!(json.contains("\"negative_live_rail\""), "{json}");
    assert!(json.contains("\"selected\": true"), "{json}");
    assert!(json.contains("stale_inventory_state_id"), "{json}");
    assert!(json.contains(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME), "{json}");
    assert!(json.contains("broad_invalid_input_coverage"), "{json}");
    assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
}

#[test]
fn armor_loadout_enchantment_status_matrix_receipt_keeps_nonclaims() {
    let cfg = test_config(
        &["--scenario", "armor-loadout-enchantment-status-matrix"],
        &[],
    )
    .expect("armor matrix config parses");
    let scenario = evaluate_scenario(
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ninventory_probe_set_slot\ncombat_probe_attack_sent\nupdate_health health=18.0\n",
    );
    let server = evaluate_server_scenario(
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate source=team_inventory_setup\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
        "compatbot",
    );
    let matrix = evaluate_armor_loadout_enchantment_status_matrix(&cfg, &scenario, &server);
    assert!(matrix.selected, "{matrix:?}");
    assert!(!matrix.live_receipt, "{matrix:?}");
    assert!(!matrix.promotion_ready, "{matrix:?}");
    assert_eq!(matrix.row_id, ARMOR_MATRIX_ROW_ID);
    let json = render_armor_loadout_enchantment_status_matrix_json(&matrix);
    assert!(
        json.contains("\"row_id\": \"chest_diamond_none_none_melee\""),
        "{json}"
    );
    assert!(
        json.contains("\"loadout_id\": \"armor_loadout_chest_only\""),
        "{json}"
    );
    assert!(json.contains("\"reference_required\": false"), "{json}");
    assert!(json.contains("\"promotion_ready\": false"), "{json}");
    assert!(json.contains("\"all_enchantments\""), "{json}");
    assert!(json.contains("\"full_combat_correctness\""), "{json}");
}

#[test]
fn equipment_slot_item_matrix_expansion_receipt_keeps_nonclaims() {
    let cfg = test_config(&["--scenario", "equipment-slot-item-matrix-expansion"], &[])
        .expect("equipment matrix config parses");
    let scenario = evaluate_scenario(
        Scenario::EquipmentSlotItemMatrixExpansion,
        "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn entity_id=4\nequipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1\n",
    );
    let server = evaluate_server_scenario(
        Scenario::EquipmentSlotItemMatrixExpansion,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE equipment_update_state username=compatbotb slot=main_hand item_id=829 count=1\n",
        "compatbot",
    );
    let matrix = evaluate_equipment_slot_item_matrix_expansion(&cfg, &scenario, &server);
    assert!(matrix.selected, "{matrix:?}");
    assert!(!matrix.live_receipt, "{matrix:?}");
    assert!(!matrix.promotion_ready, "{matrix:?}");
    assert_eq!(matrix.row_id, EQUIPMENT_MATRIX_ROW_ID);
    let json = render_equipment_slot_item_matrix_expansion_json(&matrix);
    assert!(
        json.contains("\"row_id\": \"remote_main_hand_slot4_item829_count1_non_empty\""),
        "{json}"
    );
    assert!(json.contains("\"wire_slot\": \"slot4\""), "{json}");
    assert!(json.contains("\"item_id\": \"829\""), "{json}");
    assert!(json.contains("\"promotion_ready\": false"), "{json}");
    assert!(json.contains("\"all_equipment_slots\""), "{json}");
    assert!(json.contains("\"full_equipment_semantics\""), "{json}");
}

#[test]
fn smoke_receipt_records_cairn_contract_and_octet_surface() {
    let mut cfg = test_config(
        &[
            "--server-backend=paper",
            "--receipt",
            "/tmp/receipt.json",
            "--client-dir",
            "/tmp/stevenarella",
        ],
        &[],
    )
    .expect("receipt config parses");
    cfg.server_port = 25566;
    let client = Some(ClientRunEvidence {
        log_path: Some(PathBuf::from("/tmp/client.log")),
        log_paths: vec![PathBuf::from("/tmp/client.log")],
        usernames: vec!["compatbot".to_string()],
        exit_code: Some(124),
        classification: "timeout-success-evidence",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(evaluate_scenario(
            Scenario::Smoke,
            "Detected server protocol version",
        )),
        server_scenario: Some(evaluate_server_scenario(Scenario::Smoke, "", "compatbot")),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    });

    let json = smoke_receipt_json(&cfg, Ok(&client));
    let receipt = structured_receipt_from_text(&json).expect("smoke receipt validates");
    let contract = json_object_slice(&json, "contract").expect("contract object");
    let scenario = json_object_slice(&json, "scenario").expect("scenario object");
    let server = json_object_slice(&json, "server").expect("server object");
    let correlation =
        json_object_slice(server, "client_server_correlation").expect("correlation object");
    let client_object = json_object_slice(&json, "client").expect("client object");
    let triage = json_object_slice(&json, "triage").expect("triage object");

    assert_eq!(receipt.schema, RECEIPT_SCHEMA_V2);
    assert_eq!(
        json_string_field(contract, "cairn_contract").expect("cairn contract"),
        RECEIPT_SCHEMA_V2
    );
    assert_eq!(
        json_string_field(contract, "octet_producer_surface").expect("octet surface"),
        "compat/runner/src/main.rs"
    );
    assert_eq!(
        receipt.client_classification.as_deref(),
        Some("timeout-success-evidence")
    );
    assert_eq!(
        receipt.matched_success_pattern.as_deref(),
        Some("Detected server protocol version")
    );
    assert_eq!(receipt.scenario_name, "smoke");
    assert_eq!(
        json_optional_string_array_field(scenario, "observed_milestones")
            .expect("observed milestones"),
        Some(vec!["protocol_detected".to_string()])
    );
    assert!(json_bool_field(scenario, "passed").expect("scenario passed"));
    assert!(json_bool_field(correlation, "passed").expect("correlation passed"));
    assert_eq!(
        json_optional_string_array_field(client_object, "usernames").expect("usernames"),
        Some(vec!["compatbot".to_string()])
    );
    assert_eq!(
        json_optional_string_array_field(client_object, "log_paths").expect("log paths"),
        Some(vec!["/tmp/client.log".to_string()])
    );
    assert_eq!(
        json_optional_string_field(triage, "suggested_boundary").expect("triage boundary"),
        Some("none".to_string())
    );
    assert!(!receipt.wayland_socket_inherited);
}

#[test]
fn enriched_triage_receipt_preserves_existing_fields_and_adds_context() {
    let cfg = test_config(
        &[
            "--server-backend=valence",
            "--scenario=flag-score-repeat",
            "--receipt=/tmp/receipt.json",
            "--client-dir=/tmp/stevenarella",
        ],
        &[],
    )
    .expect("receipt config parses");
    let err = "server status probe failed with token=secret /tmp/private".to_string();

    let json = smoke_receipt_json(&cfg, Err(&err));

    assert!(json.contains("\"suggested_boundary\""), "{json}");
    assert!(json.contains("\"enriched\""), "{json}");
    assert!(json.contains("\"boundary_confidence\""), "{json}");
    assert!(json.contains(TRIAGE_REDACTED), "{json}");
}

#[test]
fn scenario_receipt_records_actionable_failure_triage() {
    let mut cfg = test_config(
        &[
            "--server-backend=valence",
            "--scenario=flag-score-repeat",
            "--receipt=/tmp/receipt.json",
            "--client-dir=/tmp/stevenarella",
        ],
        &[],
    )
    .expect("receipt config parses");
    cfg.valence_log = PathBuf::from("/tmp/valence.log");
    let client = Some(ClientRunEvidence {
        log_path: Some(PathBuf::from("/tmp/client.log")),
        log_paths: vec![PathBuf::from("/tmp/client.log")],
        usernames: vec!["compatbot".to_string()],
        exit_code: Some(124),
        classification: "failure-missing-scenario-evidence",
        matched_success_pattern: None,
        scenario: Some(evaluate_scenario(
            Scenario::FlagScoreRepeat,
            "Detected server protocol version 763\njoin_game\nUnexpectedEof\n",
        )),
        server_scenario: Some(evaluate_server_scenario(
            Scenario::FlagScoreRepeat,
            "compatbot joined\n",
            "compatbot",
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    });

    let json = smoke_receipt_json(&cfg, Ok(&client));

    assert!(
        json.contains("\"first_missing_client_milestone\": \"render_tick\""),
        "{json}"
    );
    assert!(
        json.contains("\"first_missing_server_milestone\": \"server_flag_or_score\""),
        "{json}"
    );
    assert!(
        json.contains("\"first_forbidden_pattern\": \"unexpected_eof\""),
        "{json}"
    );
    assert!(
        json.contains("\"first_forbidden_source\": \"client\""),
        "{json}"
    );
    assert!(
        json.contains("\"suggested_boundary\": \"protocol-runtime\""),
        "{json}"
    );
    assert!(
        json.contains("\"client_log_paths\": [\"/tmp/client.log\"]"),
        "{json}"
    );
    assert!(
        json.contains("\"server_log_path\": \"/tmp/valence.log\""),
        "{json}"
    );
}

#[test]
fn failed_preflight_receipt_triages_before_client_evidence() {
    let cfg = test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("receipt config parses");
    let err = "server status probe failed".to_string();

    let json = smoke_receipt_json(&cfg, Err(&err));

    assert!(
        json.contains("\"first_missing_client_milestone\": \"protocol_detected\""),
        "{json}"
    );
    assert!(
        json.contains("\"suggested_boundary\": \"preflight-or-server-startup\""),
        "{json}"
    );
}

#[test]
fn structured_receipt_schema_parses_representative_shapes() {
    let dry_cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
        .expect("dry-run config parses");
    let dry_json = smoke_receipt_json(&dry_cfg, Ok(&None));
    let dry_receipt = structured_receipt_from_text(&dry_json).expect("dry receipt validates");
    assert_eq!(dry_receipt.scenario_name, "survival-break-place-pickup");
    assert!(!dry_receipt.typed_event.selected);

    let events = typed_event_fixture();
    let timeline = normalize_typed_event_timeline(&events);
    let typed_artifact = TypedEventOracleArtifact {
        event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
        timeline_blake3: typed_event_timeline_blake3(&timeline),
        event_count: events.len(),
        contributes_to_pass_fail: true,
    };
    let typed_json = typed_event_oracle_receipt_json(Some(&typed_artifact));
    let typed_receipt =
        parse_structured_typed_event_receipt(&typed_json).expect("typed-event receipt parses");
    validate_structured_typed_event_receipt(&typed_receipt).expect("typed-event receipt validates");
    assert!(typed_receipt.selected);

    let mcp_cfg = test_config(&["--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp config parses");
    let mcp_evidence = mcp_controlled_dry_run_evidence(&mcp_cfg);
    let mcp_json = smoke_receipt_json(&mcp_cfg, Ok(&Some(mcp_evidence)));
    let mcp_receipt = structured_receipt_from_text(&mcp_json).expect("mcp receipt validates");
    assert!(mcp_receipt.mcp_control.selected);

    let armor_cfg = test_config(
        &["--scenario", "armor-loadout-enchantment-status-matrix"],
        &[],
    )
    .expect("armor matrix config parses");
    let scenario = evaluate_scenario(
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ninventory_probe_set_slot\ncombat_probe_attack_sent\nupdate_health health=18.0\n",
    );
    let server = evaluate_server_scenario(
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate source=team_inventory_setup\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
        "compatbot",
    );
    let matrix = evaluate_armor_loadout_enchantment_status_matrix(&armor_cfg, &scenario, &server);
    let matrix_json = render_armor_loadout_enchantment_status_matrix_json(&matrix);
    let matrix_receipt = parse_structured_reference_matrix_receipt(&matrix_json)
        .expect("reference matrix receipt parses");
    validate_structured_reference_matrix_receipt(&matrix_receipt)
        .expect("reference matrix receipt validates");
    assert!(matrix_receipt.selected);
}

#[test]
fn structured_receipt_schema_negative_fixtures_fail_closed() {
    let cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
        .expect("dry-run config parses");
    let json = smoke_receipt_json(&cfg, Ok(&None));

    let missing_nonclaim = json.replace(RECEIPT_REQUIRED_GAMEPLAY_NON_CLAIM, "removed_claim");
    let err =
        structured_receipt_from_text(&missing_nonclaim).expect_err("missing nonclaim fails closed");
    assert!(err.contains("non_claim"), "{err}");

    let dirty_child = json.replacen(
        "\"git_status\": \"dry-run\",\n    \"git_dirty\": false",
        "\"git_status\": \"dirty\",\n    \"git_dirty\": true",
        1,
    );
    let err =
        structured_receipt_from_text(&dirty_child).expect_err("dirty child revision fails closed");
    assert!(err.contains("child revision"), "{err}");

    let wrong_backend = json.replacen("\"backend\": \"valence\"", "\"backend\": \"spigot\"", 1);
    let err = structured_receipt_from_text(&wrong_backend).expect_err("wrong backend fails closed");
    assert!(err.contains("backend"), "{err}");

    let duplicate_status = json.replacen(
        "\"status\": \"pass\",",
        "\"status\": \"pass\",\n  \"status\": \"pass\",",
        1,
    );
    let err = parse_structured_receipt_summary(&duplicate_status)
        .expect_err("duplicate top-level field fails closed");
    assert!(err.contains("expected once"), "{err}");

    let wrong_typed_field = json.replacen("\"dry_run\": true", "\"dry_run\": \"true\"", 1);
    let err = parse_structured_receipt_summary(&wrong_typed_field)
        .expect_err("wrong typed field fails closed");
    assert!(err.contains("dry_run") || err.contains("bool"), "{err}");

    let overclaim = json.replacen(
        "\"claims_correctness\": false",
        "\"claims_correctness\": true",
        1,
    );
    let err =
        parse_structured_receipt_summary(&overclaim).expect_err("overclaim field fails closed");
    assert!(err.contains("overclaim"), "{err}");

    let events = typed_event_fixture();
    let timeline = normalize_typed_event_timeline(&events);
    let typed_artifact = TypedEventOracleArtifact {
        event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
        timeline_blake3: typed_event_timeline_blake3(&timeline),
        event_count: events.len(),
        contributes_to_pass_fail: true,
    };
    let typed_json = typed_event_oracle_receipt_json(Some(&typed_artifact));
    let zero_count = typed_json.replacen(
        &format!("\"event_count\": {}", events.len()),
        "\"event_count\": 0",
        1,
    );
    let typed_receipt = parse_structured_typed_event_receipt(&zero_count)
        .expect("zero-count typed-event receipt parses");
    let err = validate_structured_typed_event_receipt(&typed_receipt)
        .expect_err("missing typed events fail closed");
    assert!(err.contains("zero events"), "{err}");

    let frame = FrameArtifactsReceiptEvidence {
        selected: true,
        capture_requested: true,
        artifact_count: 1,
        artifacts: vec![FrameArtifactReceiptItem {
            path: "docs/evidence/mcp-controlled-smoke-frames/latest-frame.png".to_string(),
            relative_path: MCP_CONTROL_LIVE_CAPTURE_RELATIVE_PATH.to_string(),
            format: "png".to_string(),
            width_px: 1280,
            height_px: 720,
            frame_id: 1,
            sequence_id: 1,
            byte_len: 16,
            blake3: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
            redaction: "not_reviewed".to_string(),
            includes_ui: true,
        }],
        missing_digests: Vec::new(),
        path_containment_checked: true,
        promotion_ready: true,
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    };
    let frame_json = render_frame_artifacts_receipt_json(&frame);
    let escaped_frame_path = frame_json.replace(
        "docs/evidence/mcp-controlled-smoke-frames/latest-frame.png",
        "../escape.png",
    );
    let frame_receipt = parse_structured_frame_artifact_receipt(&escaped_frame_path)
        .expect("escaped frame receipt parses");
    let err = validate_structured_frame_artifact_receipt(&frame_receipt)
        .expect_err("escaped artifact path fails closed");
    assert!(err.contains("escapes"), "{err}");
}

#[test]
fn smoke_receipt_records_failures_without_success_claims() {
    let cfg = test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("receipt config parses");
    let err = "server status probe failed".to_string();

    let json = smoke_receipt_json(&cfg, Err(&err));

    assert!(json.contains("\"status\": \"fail\""), "{json}");
    assert!(json.contains("\"classification\": null"), "{json}");
    assert!(
        json.contains("\"error\": \"server status probe failed\""),
        "{json}"
    );
    assert!(json.contains("\"claims_correctness\": false"), "{json}");
    assert!(
        json.contains("\"claims_semantic_equivalence\": false"),
        "{json}"
    );
}
