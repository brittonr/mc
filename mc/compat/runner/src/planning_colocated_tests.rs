#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

const TEST_ROUTER_TIMEOUT_SECS: u64 = 33;
const TEST_ROUTER_TIMEOUT_ARG: &str = "33";
const TEST_ROUTER_ZERO_TIMEOUT_ARG: &str = "0";
const TEST_ROUTER_RECEIPT: &str = "docs/evidence/router-receipt.json";
const TEST_ROUTER_ALIAS_RECEIPT: &str = "target/router-alias-receipt.json";

fn assert_plan_is_deterministic(cfg: &Config) -> HarnessPlan {
    let first = harness_plan_from_config(cfg).expect("first plan succeeds");
    let second = harness_plan_from_config(cfg).expect("second plan succeeds");
    assert_eq!(first, second);
    first
}

fn plan_diagnostic_text(result: Result<HarnessPlan, Vec<PlanningDiagnostic>>) -> String {
    format_plan_diagnostics(result.expect_err("plan should fail"))
}

fn scenario_route_error(args: &[&str]) -> String {
    let args: Vec<String> = args.iter().map(|arg| (*arg).to_string()).collect();
    parse_scenario_route_request(&args).expect_err("scenario route should fail")
}

#[test]
fn scenario_router_parses_typed_request_before_planning() {
    let cfg = test_config(
        &[
            SCENARIO_ROUTER_COMMAND,
            SCENARIO_ROUTER_RUN_SUBCOMMAND,
            "inventory-interaction",
            SCENARIO_ROUTER_RUN_FLAG,
            SCENARIO_ROUTER_BACKEND_FLAG,
            "paper",
            SCENARIO_ROUTER_RECEIPT_FLAG,
            TEST_ROUTER_RECEIPT,
            SCENARIO_ROUTER_TIMEOUT_FLAG,
            TEST_ROUTER_TIMEOUT_ARG,
            SCENARIO_ROUTER_PACKET_CAPTURE_FLAG,
            SCENARIO_ROUTER_PROXY_ROUTE_FLAG,
            "velocity-local",
            SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG,
            "modern",
        ],
        &[],
    )
    .expect("typed scenario route config parses");
    let route = cfg
        .scenario_route
        .as_ref()
        .expect("typed route request is preserved");
    assert_eq!(route.scenario, Scenario::InventoryInteraction);
    assert_eq!(route.backend, ServerBackend::Paper);
    assert_eq!(route.mode, Mode::Run);
    assert_eq!(
        route.receipt_path.as_deref(),
        Some(Path::new(TEST_ROUTER_RECEIPT))
    );
    assert_eq!(route.timeout_secs, Some(TEST_ROUTER_TIMEOUT_SECS));
    assert!(route.packet_capture_summary);
    assert_eq!(route.proxy_route.as_deref(), Some("velocity-local"));
    assert_eq!(route.proxy_forwarding_mode.as_deref(), Some("modern"));

    let plan = assert_plan_is_deterministic(&cfg);
    let route_plan = plan.scenario_route.expect("typed route plan present");
    assert_eq!(route_plan.scenario, "inventory-interaction");
    assert_eq!(route_plan.backend, "paper");
    assert_eq!(route_plan.mode, "run");
    assert_eq!(
        route_plan.receipt_path.as_deref(),
        Some(TEST_ROUTER_RECEIPT)
    );
    assert_eq!(route_plan.timeout_secs, TEST_ROUTER_TIMEOUT_SECS);
    assert_eq!(route_plan.non_claims, scenario_route_non_claims());
}

#[test]
fn scenario_router_negative_fixtures_fail_closed() {
    assert!(
        scenario_route_error(&[SCENARIO_ROUTER_COMMAND, SCENARIO_ROUTER_RUN_SUBCOMMAND])
            .contains("missing scenario")
    );
    assert!(scenario_route_error(&[
        SCENARIO_ROUTER_COMMAND,
        SCENARIO_ROUTER_RUN_SUBCOMMAND,
        "missing-scenario",
    ])
    .contains("unknown scenario"));
    assert!(scenario_route_error(&[
        SCENARIO_ROUTER_COMMAND,
        SCENARIO_ROUTER_RUN_SUBCOMMAND,
        "smoke",
        SCENARIO_ROUTER_BACKEND_FLAG,
        "bukkit",
    ])
    .contains("unknown server backend"));
    assert!(scenario_route_error(&[
        SCENARIO_ROUTER_COMMAND,
        SCENARIO_ROUTER_RUN_SUBCOMMAND,
        "smoke",
        SCENARIO_ROUTER_RECEIPT_FLAG,
        "../escape.json",
    ])
    .contains("parent traversal"));
    assert!(scenario_route_error(&[
        SCENARIO_ROUTER_COMMAND,
        SCENARIO_ROUTER_RUN_SUBCOMMAND,
        "smoke",
        SCENARIO_ROUTER_TIMEOUT_FLAG,
        TEST_ROUTER_ZERO_TIMEOUT_ARG,
    ])
    .contains("greater than zero"));
    assert!(scenario_route_error(&[
        SCENARIO_ROUTER_COMMAND,
        SCENARIO_ROUTER_RUN_SUBCOMMAND,
        "smoke",
        "--claim-production-readiness",
    ])
    .contains("overclaiming option"));
    assert!(scenario_route_error(&[
        SCENARIO_ROUTER_COMMAND,
        SCENARIO_ROUTER_RUN_SUBCOMMAND,
        "smoke",
        "--run-matrix",
    ])
    .contains("blocks non-scenario command option"));
}

#[test]
fn scenario_router_alias_plan_matches_legacy_shape() {
    let legacy = test_config(
        &[
            SCENARIO_ROUTER_DRY_RUN_FLAG,
            "--server-backend=valence",
            "--scenario=inventory-interaction",
            "--receipt=target/router-alias-receipt.json",
        ],
        &[],
    )
    .expect("legacy alias-shaped config parses");
    let routed = test_config(
        &[
            SCENARIO_ROUTER_COMMAND,
            SCENARIO_ROUTER_RUN_SUBCOMMAND,
            "inventory-interaction",
            SCENARIO_ROUTER_DRY_RUN_FLAG,
            SCENARIO_ROUTER_SERVER_BACKEND_FLAG,
            "valence",
            SCENARIO_ROUTER_RECEIPT_FLAG,
            TEST_ROUTER_ALIAS_RECEIPT,
        ],
        &[],
    )
    .expect("router alias-shaped config parses");
    let legacy_plan = assert_plan_is_deterministic(&legacy);
    let routed_plan = assert_plan_is_deterministic(&routed);

    assert_eq!(routed_plan.server, legacy_plan.server);
    assert_eq!(routed_plan.client_sessions, legacy_plan.client_sessions);
    assert_eq!(routed_plan.receipt, legacy_plan.receipt);
    assert_eq!(routed_plan.non_claims, legacy_plan.non_claims);
    assert!(legacy_plan.scenario_route.is_none());
    assert!(routed_plan.scenario_route.is_some());
}

#[test]
fn planning_core_positive_fixtures_cover_representative_modes() {
    let dry_paper = test_config(
        &[
            "--dry-run",
            "--server-backend=paper",
            "--receipt=docs/evidence/smoke.json",
        ],
        &[],
    )
    .expect("dry paper config parses");
    let dry_plan = assert_plan_is_deterministic(&dry_paper);
    assert_eq!(dry_plan.server.backend, "paper");
    assert!(dry_plan.server.eula_acceptance_required);
    assert_eq!(dry_plan.client_sessions.len(), 1);
    assert_eq!(
        dry_plan.receipt.receipt_path.as_deref(),
        Some("docs/evidence/smoke.json")
    );

    let live_valence = test_config(
        &[
            "--run",
            "--server-backend=valence",
            "--scenario=inventory-interaction",
            "--receipt=docs/evidence/live.json",
        ],
        &[],
    )
    .expect("live valence config parses");
    let live_plan = assert_plan_is_deterministic(&live_valence);
    assert_eq!(live_plan.server.backend, "valence");
    assert_eq!(
        live_plan.client_sessions[0].scenario,
        "inventory-interaction"
    );
    assert!(!live_plan.server.keep_server);

    let matrix = test_config(
        &[
            "--run-matrix",
            "--dry-run",
            "--receipt-dir=target/matrix-plan",
        ],
        &[],
    )
    .expect("matrix config parses");
    let matrix_plan = assert_plan_is_deterministic(&matrix)
        .matrix
        .expect("matrix plan present");
    assert!(matrix_plan.dry_run);
    assert!(matrix_plan.paper_receipt.ends_with("paper.json"));
    assert!(matrix_plan.valence_receipt.ends_with("valence.json"));

    let reconnect = test_config(&["--dry-run", "--scenario=reconnect-flag-state"], &[])
        .expect("reconnect config parses");
    let reconnect_plan = assert_plan_is_deterministic(&reconnect);
    assert_eq!(
        reconnect_plan.client_sessions[0].session_count,
        RECONNECT_SEQUENCE_SESSION_COUNT
    );
    assert_eq!(
        reconnect_plan.client_sessions[0].log_path_strategy,
        PLAN_CLIENT_LOG_RECONNECT_TEMP
    );

    let multi_client = test_config(&["--dry-run", "--scenario=multi-client-load-score"], &[])
        .expect("multi-client config parses");
    let multi_plan = assert_plan_is_deterministic(&multi_client);
    assert_eq!(multi_plan.client_sessions.len(), MULTI_CLIENT_READY_COUNT);
    assert_eq!(
        multi_plan.client_sessions[SECOND_CLIENT_INDEX].username,
        "compatbotb"
    );

    let cleanup = test_config(&["--cleanup", "--dry-run"], &[]).expect("cleanup config parses");
    let cleanup_plan = assert_plan_is_deterministic(&cleanup).cleanup;
    assert!(!cleanup_plan.apply);
    assert!(cleanup_plan
        .path_actions
        .iter()
        .any(|action| action.label == "valence target dir"));
    assert!(cleanup_plan
        .path_actions
        .iter()
        .any(|action| action.label == "valence log"));
    assert_eq!(
        cleanup_plan.client_log_discovery,
        PLAN_CLEANUP_CLIENT_LOG_DISCOVERY
    );

    let failure_bundle = test_config(
        &[
            "--run",
            "--receipt=docs/evidence/failed-receipt.json",
            "--failure-bundle=docs/evidence/failed-bundle.json",
        ],
        &[("VALENCE_LOG", "docs/evidence/failed-valence.log")],
    )
    .expect("failure bundle config parses");
    let failure_plan = assert_plan_is_deterministic(&failure_bundle);
    assert_eq!(
        failure_plan.receipt.failure_bundle_path.as_deref(),
        Some("docs/evidence/failed-bundle.json")
    );
    assert!(failure_plan
        .artifacts
        .failure_artifact_candidates
        .iter()
        .any(|artifact| artifact.kind == FAILURE_BUNDLE_ARTIFACT_RECEIPT));
}

#[test]
fn planning_core_negative_fixtures_fail_before_side_effects() {
    let missing_receipt = test_config(
        &["--run", "--failure-bundle=docs/evidence/failed-bundle.json"],
        &[],
    )
    .expect("missing receipt config parses");
    let missing_receipt_err = plan_diagnostic_text(harness_plan_from_config(&missing_receipt));
    assert!(missing_receipt_err.contains("requires a receipt path"));

    let path_escape = test_config(
        &[
            "--run",
            "--receipt=docs/evidence/failed-receipt.json",
            "--failure-bundle=../failed-bundle.json",
        ],
        &[],
    )
    .expect("path escape config parses");
    let path_escape_err = plan_diagnostic_text(harness_plan_from_config(&path_escape));
    assert!(path_escape_err.contains("escapes repo"));

    let target_artifact = test_config(
        &[
            "--run",
            "--receipt=target/failed-receipt.json",
            "--failure-bundle=docs/evidence/failed-bundle.json",
        ],
        &[],
    )
    .expect("target artifact config parses");
    let target_artifact_err = plan_diagnostic_text(harness_plan_from_config(&target_artifact));
    assert!(target_artifact_err.contains("target-only"));

    let mut matrix_conflict =
        test_config(&["--run-matrix"], &[]).expect("matrix conflict base config parses");
    matrix_conflict.receipt_path = Some(PathBuf::from("docs/evidence/one.json"));
    let matrix_conflict_err = plan_diagnostic_text(harness_plan_from_config(&matrix_conflict));
    assert!(matrix_conflict_err.contains("run-matrix planning"));

    let mut cleanup_hazard =
        test_config(&["--cleanup", "--apply"], &[]).expect("cleanup hazard config parses");
    cleanup_hazard.valence_target_dir = PathBuf::from(CLEANUP_ROOT_PATH);
    let cleanup_hazard_err = plan_diagnostic_text(harness_plan_from_config(&cleanup_hazard));
    assert!(cleanup_hazard_err.contains("too broad for cleanup"));
}
