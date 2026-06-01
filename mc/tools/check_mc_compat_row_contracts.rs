use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const SELF_TEST_FLAG: &str = "--self-test";
const ROW_FLAG: &str = "--row";
const EVIDENCE_FLAG: &str = "--evidence";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const KEY_VALUE_SEPARATOR: char = '=';
const REQUIRED_ARGUMENT_COUNT: usize = 5;
const PROGRAM_ARGUMENT_COUNT: usize = 1;
const CLEAN_REVISION_STATUS: &str = "clean";
const UNKNOWN_REVISION: &str = "unknown";
const DIRTY_REVISION: &str = "dirty";
const EXPECTED_METRIC_VALUE: &str = "ok";
const EXPECTED_STANDARD_VALUE: &str = "present";
const EXPECTED_NON_CLAIM_VALUE: &str = "explicit";
const COMMON_STANDARDS: &[&str] = &[
    "bounded_contract",
    "positive_fixture",
    "negative_fixture",
    "fail_closed_promotion_gate",
];
const COMMON_NON_CLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "production_readiness",
    "unbounded_vanilla_parity",
];

const ROWS: &[RowContract] = &[
    RowContract {
        id: "ctf-simultaneous-pickup-capture-race",
        label: "simultaneous pickup/capture race",
        metrics: &[
            "client_identities",
            "team_roles",
            "action_timestamps",
            "ordered_milestones",
            "accepted_transition",
            "rejected_transition",
            "final_flag_state",
            "final_score",
            "race_window_bounds",
        ],
        non_claims: &["all_ctf_concurrency", "latency_tolerance", "production_readiness"],
    },
    RowContract {
        id: "ctf-spawn-team-balance-reset",
        label: "spawn/team balance/resource reset",
        metrics: &[
            "team_counts",
            "selected_teams",
            "spawn_coordinates",
            "initial_resources",
            "post_score_or_post_death_reset_state",
            "inventory_resource_ids",
            "server_correlation_ids",
        ],
        non_claims: &["all_spawn_rules", "matchmaking_balance", "production_readiness"],
    },
    RowContract {
        id: "death-respawn-invalid-respawn-timing",
        label: "invalid respawn timing",
        metrics: &[
            "pre_death_state",
            "invalid_respawn_attempt_timing",
            "containment_result",
            "death_state_retained",
            "valid_respawn_request",
            "restored_health",
            "duplicate_respawn_guard",
            "server_correlation",
        ],
        non_claims: &["all_death_states", "anti_cheat", "production_readiness"],
    },
    RowContract {
        id: "death-respawn-inventory-reset",
        label: "death inventory reset",
        metrics: &[
            "pre_death_inventory_slots",
            "death_cause",
            "drop_reset_policy",
            "dropped_item_ids_counts",
            "respawn_inventory_slots",
            "server_correlation_milestones",
        ],
        non_claims: &["all_inventory_rules", "economy_safety", "production_readiness"],
    },
    RowContract {
        id: "death-respawn-ordinary-death",
        label: "ordinary death/respawn",
        metrics: &[
            "death_cause",
            "pre_death_health",
            "death_milestone",
            "respawn_request",
            "post_respawn_health",
            "post_respawn_position",
            "flag_state_absence",
            "inventory_policy",
            "server_correlation",
        ],
        non_claims: &["all_death_causes", "bed_spawn_rules", "production_readiness"],
    },
    RowContract {
        id: "death-respawn-reconnect-during-death",
        label: "reconnect during death",
        metrics: &[
            "death_milestone",
            "disconnect_point",
            "reconnect_username_session",
            "server_retained_death_state",
            "client_post_reconnect_state",
            "respawn_action",
            "final_health_playable_state",
        ],
        non_claims: &["all_reconnect_races", "session_security", "production_readiness"],
    },
    RowContract {
        id: "death-respawn-repeated-death-safety",
        label: "repeated death safety",
        metrics: &[
            "cycle_index",
            "death_cause",
            "respawn_request",
            "restored_health",
            "entity_session_id",
            "inventory_policy_state",
            "forbidden_duplicate_deaths",
            "final_playable_state",
        ],
        non_claims: &["load_resilience", "adversarial_reconnects", "production_readiness"],
    },
    RowContract {
        id: "full-ctf-correctness-gate",
        label: "full CTF correctness aggregate",
        metrics: &[
            "rule_family",
            "status",
            "receipt_path",
            "run_log_path",
            "blake3_manifest",
            "forbidden_transition_checks",
            "negative_fixture_coverage",
            "current_bundle_label",
        ],
        non_claims: &["full_protocol_763_compatibility", "production_readiness", "uncited_ctf_rules"],
    },
    RowContract {
        id: "full-protocol-763-compatibility-gate",
        label: "full protocol-763 compatibility aggregate",
        metrics: &[
            "packet_row_count",
            "family_status",
            "mapping_status",
            "parser_fixture_id",
            "malformed_fixture_status",
            "live_receipt_path",
            "owner",
            "next_action",
            "digest",
        ],
        non_claims: &["full_minecraft_compatibility", "production_readiness", "uncited_packet_families"],
    },
    RowContract {
        id: "inventory-creative-mode",
        label: "creative-mode inventory",
        metrics: &[
            "game_mode",
            "permission_state",
            "creative_action_type",
            "item_id",
            "item_count",
            "target_slot",
            "client_observation",
            "server_inventory_state",
            "forbidden_survival_only_assumptions",
        ],
        non_claims: &["all_creative_permissions", "public_server_safety", "production_readiness"],
    },
    RowContract {
        id: "inventory-drag-transactions",
        label: "inventory drag transactions",
        metrics: &[
            "window_id",
            "state_id",
            "drag_phase_sequence",
            "source_stack",
            "target_slots",
            "per_slot_final_counts",
            "carried_remainder",
            "server_transaction_correlation",
        ],
        non_claims: &["all_inventory_guis", "all_drag_modes", "production_readiness"],
    },
    RowContract {
        id: "inventory-extra-window-types",
        label: "extra inventory window types",
        metrics: &[
            "window_type",
            "window_id",
            "opened_title_type",
            "slot_mapping",
            "action_item_count",
            "final_window_slot_state",
            "final_player_inventory_state",
            "server_correlation",
        ],
        non_claims: &["all_window_types", "modded_guis", "production_readiness"],
    },
    RowContract {
        id: "inventory-stack-split-merge",
        label: "inventory stack split/merge",
        metrics: &[
            "initial_slot_item_count",
            "split_action",
            "carried_stack_count",
            "destination_slot_count",
            "merge_action",
            "final_slot_counts",
            "state_id",
            "server_click_slot_correlation",
        ],
        non_claims: &["all_inventory_transactions", "crafting_rules", "production_readiness"],
    },
    RowContract {
        id: "production-readiness-envelope",
        label: "production readiness aggregate",
        metrics: &[
            "target_scope",
            "authorization",
            "owner",
            "client_count",
            "duration",
            "perturbation_settings",
            "adversarial_model",
            "telemetry",
            "abort_criteria",
            "redaction_status",
            "row_evidence_paths",
        ],
        non_claims: &["public_server_safety", "unbounded_load", "wan_adversarial_readiness"],
    },
    RowContract {
        id: "projectile-travel-collision-parity",
        label: "projectile travel/collision",
        metrics: &[
            "spawn_position",
            "launch_vector",
            "travel_samples",
            "collision_target",
            "impact_position",
            "hit_entity_or_block",
            "damage_attribution",
            "tolerance_bounds",
        ],
        non_claims: &["all_projectiles", "anti_cheat", "production_readiness"],
    },
    RowContract {
        id: "projectile-weapon-variants",
        label: "projectile weapon variants",
        metrics: &[
            "weapon_id",
            "ammunition_item_state",
            "use_action",
            "projectile_spawn",
            "target_identity",
            "hit_miss_outcome",
            "damage_delta_when_applicable",
            "server_correlation",
        ],
        non_claims: &["all_weapon_variants", "combat_balance", "production_readiness"],
    },
    RowContract {
        id: "protocol-chunk-biome-family-coverage",
        label: "chunk/biome packet family",
        metrics: &[
            "packet_name",
            "wire_id",
            "chunk_position",
            "biome_id_or_environment_id",
            "parser_fixture_id",
            "live_receipt_path",
            "malformed_fixture_status_where_supported",
        ],
        non_claims: &["all_worldgen_packets", "all_biomes", "production_readiness"],
    },
    RowContract {
        id: "protocol-command-recipe-advancement-family-coverage",
        label: "command/recipe/advancement packet family",
        metrics: &[
            "packet_family",
            "wire_id",
            "semantic_fixture_id",
            "parser_fixture_result",
            "malformed_fixture_status",
            "live_scenario_feature",
            "receipt_path",
            "digest",
        ],
        non_claims: &["full_command_semantics", "all_recipes", "all_advancements"],
    },
    RowContract {
        id: "protocol-entity-metadata-family-coverage",
        label: "entity metadata packet family",
        metrics: &[
            "wire_id",
            "valence_packet_name",
            "stevenarella_semantic",
            "parser_fixture_id",
            "positive_payload_fixture",
            "malformed_rejection_fixture_where_semantic_decoding_exists",
            "live_receipt_evidence_path",
        ],
        non_claims: &["all_entity_metadata", "mob_ai_semantics", "production_readiness"],
    },
    RowContract {
        id: "protocol-equipment-permutation-family-coverage",
        label: "equipment permutation packet family",
        metrics: &[
            "equipment_packet_name",
            "wire_id",
            "entity_id",
            "slot",
            "item_id",
            "count",
            "parser_fixture_id",
            "live_observer_receipt",
            "digest",
        ],
        non_claims: &["all_equipment_permutations", "armor_mitigation", "combat_balancing"],
    },
    RowContract {
        id: "protocol-inventory-transaction-family-coverage",
        label: "inventory transaction packet family",
        metrics: &[
            "transaction_packet_name",
            "state_side",
            "wire_id",
            "slot_window_state_id_fields",
            "parser_fixture_id",
            "malformed_fixture_status",
            "live_scenario",
            "receipt_digest",
        ],
        non_claims: &["all_inventory_semantics", "all_window_types", "production_readiness"],
    },
    RowContract {
        id: "vanilla-combat-reference-parity",
        label: "vanilla combat parity",
        metrics: &[
            "attacker_identity",
            "victim_identity",
            "weapon",
            "armor_state",
            "pre_post_health",
            "damage_delta",
            "velocity_vector_or_knockback_displacement",
            "tolerance_bounds",
            "reference_version",
        ],
        non_claims: &["all_combat_mechanics", "pvp_balance", "production_readiness"],
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RowContract {
    id: &'static str,
    label: &'static str,
    metrics: &'static [&'static str],
    non_claims: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceDoc {
    row: String,
    revision_status: String,
    source_revision: String,
    metrics: BTreeMap<String, String>,
    standards: BTreeMap<String, String>,
    non_claims: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliConfig {
    row: String,
    evidence_path: String,
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("mc compat row contract self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match parse_cli(&args).and_then(|config| run_config(&config)) {
        Ok(contract) => {
            println!("{} contract check passed", contract.label);
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
        eprintln!("mc compat row contract check failed: {error}");
    }
}

fn parse_cli(args: &[String]) -> Result<CliConfig, Vec<String>> {
    if args.len() != REQUIRED_ARGUMENT_COUNT {
        return Err(vec![usage()]);
    }

    let mut row = None;
    let mut evidence_path = None;
    let mut index = PROGRAM_ARGUMENT_COUNT;
    while index < args.len() {
        let flag = args[index].as_str();
        let Some(value) = args.get(index + PROGRAM_ARGUMENT_COUNT) else {
            return Err(vec![usage()]);
        };
        match flag {
            ROW_FLAG => row = Some(value.clone()),
            EVIDENCE_FLAG => evidence_path = Some(value.clone()),
            _ => return Err(vec![format!("unknown argument: {flag}")]),
        }
        index += 2;
    }

    Ok(CliConfig {
        row: row.ok_or_else(|| vec![usage()])?,
        evidence_path: evidence_path.ok_or_else(|| vec![usage()])?,
    })
}

fn usage() -> String {
    format!("usage: check_mc_compat_row_contracts {ROW_FLAG} <row-id> {EVIDENCE_FLAG} <evidence>")
}

fn run_config(config: &CliConfig) -> Result<RowContract, Vec<String>> {
    let contract = row_contract(&config.row)?;
    let text = read_file(&config.evidence_path)?;
    validate_document(contract, &text)?;
    Ok(contract)
}

fn read_file(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(Path::new(path)).map_err(|error| vec![format!("{path}: {error}")])
}

fn row_contract(row: &str) -> Result<RowContract, Vec<String>> {
    ROWS.iter()
        .copied()
        .find(|contract| contract.id == row)
        .ok_or_else(|| vec![format!("unknown row contract: {row}")])
}

fn validate_document(contract: RowContract, text: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let evidence = parse_evidence(text, &mut errors);
    if let Some(evidence) = &evidence {
        validate_evidence_shape(contract, evidence, &mut errors);
        validate_required_metrics(contract, evidence, &mut errors);
        validate_required_standards(evidence, &mut errors);
        validate_required_non_claims(contract, evidence, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn parse_evidence(text: &str, errors: &mut Vec<String>) -> Option<EvidenceDoc> {
    let mut fields = BTreeMap::new();
    let mut metrics = BTreeMap::new();
    let mut standards = BTreeMap::new();
    let mut non_claims = BTreeMap::new();

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
        if let Some(metric_name) = key.strip_prefix("metric.") {
            metrics.insert(metric_name.to_string(), value.to_string());
        } else if let Some(standard_name) = key.strip_prefix("standard.") {
            standards.insert(standard_name.to_string(), value.to_string());
        } else if let Some(non_claim_name) = key.strip_prefix("nonclaim.") {
            non_claims.insert(non_claim_name.to_string(), value.to_string());
        } else {
            fields.insert(key.to_string(), value.to_string());
        }
    }

    Some(EvidenceDoc {
        row: required_field(&fields, "row", errors)?,
        revision_status: required_field(&fields, "revision_status", errors)?,
        source_revision: required_field(&fields, "source_revision", errors)?,
        metrics,
        standards,
        non_claims,
    })
}

fn required_field(
    fields: &BTreeMap<String, String>,
    key: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    match fields.get(key) {
        Some(value) if !value.is_empty() => Some(value.clone()),
        _ => {
            errors.push(format!("missing required field: {key}"));
            None
        }
    }
}

fn validate_evidence_shape(
    contract: RowContract,
    evidence: &EvidenceDoc,
    errors: &mut Vec<String>,
) {
    if evidence.row != contract.id {
        errors.push(format!(
            "{} evidence has wrong row: {}",
            contract.label, evidence.row
        ));
    }
    if evidence.revision_status != CLEAN_REVISION_STATUS {
        errors.push(format!(
            "{} evidence has stale revision status: {}",
            contract.label, evidence.revision_status
        ));
    }
    if evidence.source_revision == UNKNOWN_REVISION || evidence.source_revision == DIRTY_REVISION {
        errors.push(format!(
            "{} evidence lacks committed source revision metadata or oracle checkpoint",
            contract.label
        ));
    }
}

fn validate_required_metrics(
    contract: RowContract,
    evidence: &EvidenceDoc,
    errors: &mut Vec<String>,
) {
    for metric in contract.metrics {
        match evidence.metrics.get(*metric) {
            Some(value) if value == EXPECTED_METRIC_VALUE => {}
            Some(value) => errors.push(format!(
                "{} mismatched metric {metric}: expected {EXPECTED_METRIC_VALUE}, got {value}",
                contract.label
            )),
            None => errors.push(format!(
                "{} evidence missing metric: {metric}",
                contract.label
            )),
        }
    }
}

fn validate_required_standards(evidence: &EvidenceDoc, errors: &mut Vec<String>) {
    for standard in COMMON_STANDARDS {
        match evidence.standards.get(*standard) {
            Some(value) if value == EXPECTED_STANDARD_VALUE => {}
            Some(value) => errors.push(format!(
                "mismatched evidence standard {standard}: expected {EXPECTED_STANDARD_VALUE}, got {value}"
            )),
            None => errors.push(format!("missing evidence standard: {standard}")),
        }
    }
}

fn validate_required_non_claims(
    contract: RowContract,
    evidence: &EvidenceDoc,
    errors: &mut Vec<String>,
) {
    for non_claim in COMMON_NON_CLAIMS.iter().chain(contract.non_claims.iter()) {
        match evidence.non_claims.get(*non_claim) {
            Some(value) if value == EXPECTED_NON_CLAIM_VALUE => {}
            Some(value) => errors.push(format!(
                "mismatched non-claim {non_claim}: expected {EXPECTED_NON_CLAIM_VALUE}, got {value}"
            )),
            None => errors.push(format!("missing non-claim: {non_claim}")),
        }
    }
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let mut exercised_rows = BTreeSet::new();
    for contract in ROWS {
        let positive = fixture_evidence(*contract);
        validate_document(*contract, &positive)?;
        exercised_rows.insert(contract.id);

        assert_contains(
            &validate_document(*contract, &remove_first_metric(*contract, &positive))
                .expect_err("missing metric fixture should fail"),
            "missing metric",
        )?;

        assert_contains(
            &validate_document(*contract, &mismatch_first_metric(*contract, &positive))
                .expect_err("mismatched metric fixture should fail"),
            "mismatched metric",
        )?;

        assert_contains(
            &validate_document(*contract, &positive.replace(
                "revision_status=clean",
                "revision_status=dirty",
            ))
            .expect_err("dirty revision fixture should fail"),
            "stale revision",
        )?;

        assert_contains(
            &validate_document(*contract, &positive.replace(
                "source_revision=abc1234",
                "source_revision=unknown",
            ))
            .expect_err("unknown revision fixture should fail"),
            "source revision",
        )?;

        assert_contains(
            &validate_document(*contract, &positive.replace(
                "standard.bounded_contract=present\n",
                "",
            ))
            .expect_err("missing standard fixture should fail"),
            "missing evidence standard",
        )?;

        assert_contains(
            &validate_document(*contract, &remove_first_non_claim(*contract, &positive))
                .expect_err("missing non-claim fixture should fail"),
            "missing non-claim",
        )?;

        let wrong_contract = ROWS
            .iter()
            .copied()
            .find(|candidate| candidate.id != contract.id)
            .expect("self-test requires more than one row");
        assert_contains(
            &validate_document(wrong_contract, &positive)
                .expect_err("wrong row fixture should fail"),
            "wrong row",
        )?;
    }

    if exercised_rows.len() != ROWS.len() {
        return Err(vec!["not every row contract was exercised".to_string()]);
    }

    Ok(format!("{} row contracts exercised", exercised_rows.len()))
}

fn fixture_evidence(contract: RowContract) -> String {
    let mut text = format!(
        "row={}\nrevision_status=clean\nsource_revision=abc1234\n",
        contract.id
    );
    for metric in contract.metrics {
        text.push_str(&format!("metric.{metric}={EXPECTED_METRIC_VALUE}\n"));
    }
    for standard in COMMON_STANDARDS {
        text.push_str(&format!("standard.{standard}={EXPECTED_STANDARD_VALUE}\n"));
    }
    for non_claim in COMMON_NON_CLAIMS.iter().chain(contract.non_claims.iter()) {
        text.push_str(&format!("nonclaim.{non_claim}={EXPECTED_NON_CLAIM_VALUE}\n"));
    }
    text
}

fn remove_first_metric(contract: RowContract, evidence: &str) -> String {
    evidence.replace(
        &format!(
            "metric.{}={EXPECTED_METRIC_VALUE}\n",
            contract.metrics[0]
        ),
        "",
    )
}

fn mismatch_first_metric(contract: RowContract, evidence: &str) -> String {
    evidence.replace(
        &format!(
            "metric.{}={EXPECTED_METRIC_VALUE}\n",
            contract.metrics[0]
        ),
        &format!("metric.{}=different\n", contract.metrics[0]),
    )
}

fn remove_first_non_claim(contract: RowContract, evidence: &str) -> String {
    evidence.replace(
        &format!(
            "nonclaim.{}={EXPECTED_NON_CLAIM_VALUE}\n",
            contract.non_claims[0]
        ),
        "",
    )
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
