#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[test]
fn mcp_controlled_live_preflight_allows_bounded_local_rail() {
    let cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp-controlled live config parses");
    validate_mcp_controlled_live_preflight(&cfg)
        .expect("bounded local MCP-controlled live rail preflight passes");
}

#[test]
fn mcp_controlled_live_preflight_rejects_unbounded_timeout() {
    let mut cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp-controlled live config parses");
    cfg.client_timeout = Duration::from_secs(SAFETY_MAX_DURATION_SECS + 1);

    let err = validate_mcp_controlled_live_preflight(&cfg)
        .expect_err("unbounded MCP-controlled live rail fails preflight");

    assert!(err.contains("client timeout exceeds"), "{err}");
}

#[test]
fn mcp_controlled_live_receipt_uses_observed_control_and_frame_evidence() {
    let cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp-controlled live config parses");
    let child_revision = GitRevisionEvidence {
        requested_rev: None,
        resolved_rev: Some("4d1b1554650bd91924f7ce99c9dab69a91142edc".to_string()),
        status: GIT_STATUS_CLEAN,
        dirty: false,
        diagnostics: Vec::new(),
    };
    let client = ClientRunEvidence {
        log_path: Some(PathBuf::from("docs/evidence/mcp.transcript.log")),
        log_paths: vec![PathBuf::from("docs/evidence/mcp.transcript.log")],
        usernames: vec![TEST_USERNAME.to_string()],
        exit_code: None,
        classification: "mcp-controlled-live-evidence",
        matched_success_pattern: Some("mcp_command_outcomes".to_string()),
        scenario: Some(evaluate_scenario_for_config(
            &cfg,
            &mcp_controlled_success_output(),
        )),
        server_scenario: Some(evaluate_server_scenario(
            Scenario::McpControlledSmoke,
            "",
            TEST_USERNAME,
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: Some(McpControlRunEvidence {
            handshake_success: true,
            tool_list_digest: mcp_control_tool_list_digest(),
            tool_names: MCP_CONTROL_TOOL_NAMES.to_vec(),
            calls_attempted: MCP_CONTROL_LIVE_CALLS.to_vec(),
            calls_succeeded: MCP_CONTROL_LIVE_CALLS.to_vec(),
            first_failure: None,
            stdout_clean: true,
            command_outcome_ids: MCP_CONTROL_LIVE_OUTCOME_IDS.to_vec(),
        }),
        frame_artifacts: Some(FrameArtifactsReceiptEvidence {
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
                blake3: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
                redaction: "not_reviewed".to_string(),
                includes_ui: true,
            }],
            missing_digests: Vec::new(),
            path_containment_checked: true,
            promotion_ready: true,
            non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
        }),
    };

    let mcp = evaluate_mcp_control_receipt(&cfg, &child_revision, Some(&client));
    let frame = evaluate_frame_artifacts_receipt(&cfg, Some(&client));

    assert!(mcp.passed, "{mcp:?}");
    assert!(mcp.live_receipt);
    assert!(!mcp.dry_run_fixture);
    assert_eq!(mcp.first_failure, None);
    assert!(frame.selected);
    assert_eq!(frame.artifact_count, 1);
    assert!(frame.promotion_ready);
}

#[test]
fn mcp_controlled_live_receipt_fails_dirty_child_revision() {
    let cfg = test_config(&["--run", "--scenario", MCP_CONTROLLED_SMOKE_SCENARIO], &[])
        .expect("mcp-controlled live config parses");
    let child_revision = GitRevisionEvidence {
        requested_rev: None,
        resolved_rev: Some("4d1b1554650bd91924f7ce99c9dab69a91142edc".to_string()),
        status: GIT_STATUS_DIRTY,
        dirty: true,
        diagnostics: Vec::new(),
    };
    let client = mcp_controlled_dry_run_evidence(&cfg);

    let mcp = evaluate_mcp_control_receipt(&cfg, &child_revision, Some(&client));

    assert!(!mcp.passed, "{mcp:?}");
    assert_eq!(mcp.first_failure, Some(MCP_CONTROL_FAILURE_REVISION_DIRTY));
}

#[test]
fn multi_client_scenario_tracks_client_and_server_evidence() {
    let cfg = test_config(
        &["--scenario", "multi-client-load-score"],
        &[("CLIENT_TIMEOUT", "150")],
    )
    .expect("multi-client config parses");
    assert_eq!(
        planned_client_usernames(&cfg),
        vec!["compatbota", "compatbotb"]
    );
    assert_eq!(client_timeout_secs(&cfg, 0), 150);
    assert_eq!(
        client_timeout_secs(&cfg, 1),
        MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS
    );

    let client = evaluate_scenario(
        Scenario::MultiClientLoadScore,
        "mc_compat_multi_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\n",
    );
    assert!(client.passed, "{client:?}");

    let server = evaluate_server_scenario(
        Scenario::MultiClientLoadScore,
        "compatbota joined\ncompatbotb joined\nred flag captured\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_peer = evaluate_server_scenario(
        Scenario::MultiClientLoadScore,
        "compatbota joined\nred flag captured\n",
        "compatbot",
    );
    assert!(!missing_peer.passed, "{missing_peer:?}");
    assert!(missing_peer
        .missing_milestones
        .contains(&"server_client_b_seen"));
}

fn projectile_travel_attacker_log() -> String {
    format!(
        "{} hand=main {} projectile_id={} weapon={}\n{} projectile_id={} weapon={} proof_basis={}\n{} hand=main projectile_id={}\n{} projectile_id={} proof_basis={}\n",
        PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_TRAVEL_COLLISION_CLIENT_SPAWN_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS,
        PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_CLIENT_TRAVEL_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS
    )
}

fn projectile_travel_client_logs<'a>(attacker_log: &'a str) -> [ClientLogSlice<'a>; 2] {
    [
        ClientLogSlice {
            username: TEST_ATTACKER_USERNAME,
            output: attacker_log,
        },
        ClientLogSlice {
            username: TEST_VICTIM_USERNAME,
            output: "",
        },
    ]
}

fn projectile_travel_use_line(target: &str, weapon: &str) -> String {
    format!(
        "{} attacker={} victim={} hand=Main {} projectile_id={} weapon={} {}",
        PROJECTILE_DAMAGE_SERVER_USE_NEEDLE,
        TEST_ATTACKER_USERNAME,
        target,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        weapon,
        PROJECTILE_DAMAGE_AMOUNT_NEEDLE
    )
}

fn projectile_travel_sample_line(target: &str, weapon: &str) -> String {
    format!(
        "{} attacker={} target={} {} projectile_id={} weapon={} sample={} sample_index={} proof_basis={}",
        PROJECTILE_TRAVEL_COLLISION_SERVER_TRAVEL_NEEDLE,
        TEST_ATTACKER_USERNAME,
        target,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        weapon,
        PROJECTILE_TRAVEL_COLLISION_SAMPLE_KIND,
        PROJECTILE_TRAVEL_COLLISION_SAMPLE_INDEX,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS
    )
}

fn projectile_travel_collision_line(target: &str, weapon: &str) -> String {
    format!(
        "{} attacker={} target={} {} projectile_id={} weapon={} collision={} proof_basis={}",
        PROJECTILE_TRAVEL_COLLISION_SERVER_COLLISION_NEEDLE,
        TEST_ATTACKER_USERNAME,
        target,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        weapon,
        PROJECTILE_TRAVEL_COLLISION_COLLISION_KIND,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS
    )
}

fn projectile_travel_hit_line(target: &str, weapon: &str) -> String {
    format!(
        "{} attacker={} victim={} {} projectile_id={} weapon={} {} victim_health_before={:.1} victim_health_after={:.1}",
        PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE,
        TEST_ATTACKER_USERNAME,
        target,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        weapon,
        PROJECTILE_DAMAGE_AMOUNT_NEEDLE,
        PROJECTILE_DAMAGE_VICTIM_START_HEALTH,
        PROJECTILE_TRAVEL_COLLISION_VICTIM_END_HEALTH
    )
}

fn projectile_travel_server_log(target: &str, weapon: &str) -> String {
    format!(
        "{}\n{}\n{}\n{}\n",
        projectile_travel_use_line(target, weapon),
        projectile_travel_sample_line(target, weapon),
        projectile_travel_collision_line(target, weapon),
        projectile_travel_hit_line(target, weapon)
    )
}

fn projectile_travel_evidence_for(server_log: &str) -> ProjectileTravelCollisionEvidence {
    let attacker_log = projectile_travel_attacker_log();
    let client_logs = projectile_travel_client_logs(&attacker_log);
    evaluate_projectile_travel_collision(&client_logs, server_log, TEST_USERNAME)
}

#[test]
fn projectile_travel_collision_synthetic_row_tracks_client_and_server_evidence() {
    let cfg =
        test_config(&["--scenario", "projectile-hit"], &[]).expect("projectile-hit config parses");
    let dry_run = projectile_travel_collision_dry_run_evidence(&cfg);
    assert!(dry_run
        .scenario
        .as_ref()
        .is_some_and(|evidence| evidence.passed));
    assert!(dry_run
        .server_scenario
        .as_ref()
        .is_some_and(|evidence| evidence.passed));
    let travel = dry_run
        .projectile_travel_collision
        .as_ref()
        .expect("travel collision evidence");
    assert!(travel.passed, "{travel:?}");
    assert_eq!(travel.row_id, PROJECTILE_TRAVEL_COLLISION_ROW_ID);
    assert!(travel
        .observed_steps
        .contains(&"server_projectile_travel_sample"));
    assert!(travel
        .observed_steps
        .contains(&"server_projectile_collision"));

    let server_log =
        projectile_travel_server_log(TEST_VICTIM_USERNAME, PROJECTILE_TRAVEL_COLLISION_WEAPON);
    let direct = projectile_travel_evidence_for(&server_log);
    assert!(direct.passed, "{direct:?}");
    assert!(direct.identity_violations.is_empty(), "{direct:?}");
    assert!(direct
        .non_claims
        .contains(&"not_exact_vanilla_projectile_parity"));
}

#[test]
fn projectile_travel_collision_fails_closed_for_bad_evidence() {
    let attacker_log = projectile_travel_attacker_log();
    let client_logs = projectile_travel_client_logs(&attacker_log);
    let good_server =
        projectile_travel_server_log(TEST_VICTIM_USERNAME, PROJECTILE_TRAVEL_COLLISION_WEAPON);

    let missing_travel = good_server.replace(
        &format!(
            "{}\n",
            projectile_travel_sample_line(TEST_VICTIM_USERNAME, PROJECTILE_TRAVEL_COLLISION_WEAPON)
        ),
        "",
    );
    let evidence =
        evaluate_projectile_travel_collision(&client_logs, &missing_travel, TEST_USERNAME);
    assert!(!evidence.passed, "{evidence:?}");
    assert!(evidence
        .missing_steps
        .contains(&"server_projectile_travel_sample"));
    assert!(evidence
        .order_violations
        .contains(&"server_collision_or_hit_without_travel"));

    let missing_collision = good_server.replace(
        &format!(
            "{}\n",
            projectile_travel_collision_line(
                TEST_VICTIM_USERNAME,
                PROJECTILE_TRAVEL_COLLISION_WEAPON
            )
        ),
        "",
    );
    let evidence =
        evaluate_projectile_travel_collision(&client_logs, &missing_collision, TEST_USERNAME);
    assert!(!evidence.passed, "{evidence:?}");
    assert!(evidence
        .missing_steps
        .contains(&"server_projectile_collision"));

    let wrong_target =
        projectile_travel_server_log("compatbotz", PROJECTILE_TRAVEL_COLLISION_WEAPON);
    let evidence = evaluate_projectile_travel_collision(&client_logs, &wrong_target, TEST_USERNAME);
    assert!(!evidence.passed, "{evidence:?}");
    assert!(evidence.identity_violations.contains(&"wrong_target"));

    let wrong_weapon = projectile_travel_server_log(TEST_VICTIM_USERNAME, "Crossbow");
    let evidence = evaluate_projectile_travel_collision(&client_logs, &wrong_weapon, TEST_USERNAME);
    assert!(!evidence.passed, "{evidence:?}");
    assert!(evidence.identity_violations.contains(&"wrong_weapon"));

    let out_of_order = format!(
        "{}\n{}\n{}\n{}\n",
        projectile_travel_sample_line(TEST_VICTIM_USERNAME, PROJECTILE_TRAVEL_COLLISION_WEAPON),
        projectile_travel_use_line(TEST_VICTIM_USERNAME, PROJECTILE_TRAVEL_COLLISION_WEAPON),
        projectile_travel_collision_line(TEST_VICTIM_USERNAME, PROJECTILE_TRAVEL_COLLISION_WEAPON),
        projectile_travel_hit_line(TEST_VICTIM_USERNAME, PROJECTILE_TRAVEL_COLLISION_WEAPON)
    );
    let evidence = evaluate_projectile_travel_collision(&client_logs, &out_of_order, TEST_USERNAME);
    assert!(!evidence.passed, "{evidence:?}");
    assert!(evidence
        .order_violations
        .contains(&"server_projectile_use_before_travel"));

    let ambiguous_identity = format!(
        "{}\n{}\n",
        good_server,
        projectile_travel_use_line("compatbotz", PROJECTILE_TRAVEL_COLLISION_WEAPON)
    );
    let evidence =
        evaluate_projectile_travel_collision(&client_logs, &ambiguous_identity, TEST_USERNAME);
    assert!(!evidence.passed, "{evidence:?}");
    assert!(evidence
        .identity_violations
        .contains(&"ambiguous_projectile_identity"));

    let overbroad_claim = format!("{good_server}claim=exact_vanilla_projectile_parity\n");
    let evidence =
        evaluate_projectile_travel_collision(&client_logs, &overbroad_claim, TEST_USERNAME);
    assert!(!evidence.passed, "{evidence:?}");
    assert!(evidence
        .identity_violations
        .contains(&"overbroad_parity_claim"));
}

#[test]
fn projectile_travel_collision_receipt_preserves_non_claims() {
    let cfg =
        test_config(&["--scenario", "projectile-hit"], &[]).expect("projectile-hit config parses");
    let evidence = projectile_travel_collision_dry_run_evidence(&cfg);
    let receipt = smoke_receipt_json(&cfg, Ok(&Some(evidence)));
    assert!(
        receipt.contains("\"projectile_travel_collision\""),
        "{receipt}"
    );
    assert!(receipt.contains("\"selected\": true"), "{receipt}");
    assert!(receipt.contains("not_full_projectile_physics"), "{receipt}");
    assert!(
        receipt.contains("not_exact_vanilla_projectile_parity"),
        "{receipt}"
    );
    assert!(
        receipt.contains("\"claims_semantic_equivalence\": false"),
        "{receipt}"
    );
    assert!(
        receipt.contains("\"claims_correctness\": false"),
        "{receipt}"
    );
}

#[test]
fn projectile_damage_attribution_scenario_tracks_client_and_server_evidence() {
    let cfg = test_config(
        &["--scenario", "projectile-damage-attribution"],
        &[("CLIENT_TIMEOUT", "150")],
    )
    .expect("projectile damage config parses");
    assert_eq!(
        planned_client_usernames(&cfg),
        vec!["compatbota", "compatbotb"]
    );

    let client = evaluate_scenario(
        Scenario::ProjectileDamageAttribution,
        "mc_compat_projectile_damage_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\nprojectile_probe_use_item_sent\nprojectile_probe_swing_sent\nupdate_health health=17.0\n",
    );
    assert!(client.passed, "{client:?}");

    let server = evaluate_server_scenario(
        Scenario::ProjectileDamageAttribution,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE projectile_loadout username=compatbota slot=0 item=Bow arrows=16\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\nMC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\n",
        "compatbot",
    );
    assert!(server.passed, "{server:?}");

    let missing_hit = evaluate_server_scenario(
        Scenario::ProjectileDamageAttribution,
        "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE projectile_loadout username=compatbota slot=0 item=Bow arrows=16\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\n",
        "compatbot",
    );
    assert!(!missing_hit.passed, "{missing_hit:?}");
    assert!(missing_hit
        .missing_milestones
        .contains(&"server_projectile_hit"));

    let attacker_log = "projectile_probe_use_item_sent hand=main sequence=303\nprojectile_probe_swing_sent hand=main\n";
    let victim_log = "update_health health=17.0 food=20 saturation=0.0\n";
    let server_log = "MC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\nMC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\n";
    let client_logs = [
        ClientLogSlice {
            username: "compatbota",
            output: attacker_log,
        },
        ClientLogSlice {
            username: "compatbotb",
            output: victim_log,
        },
    ];
    let causal = evaluate_projectile_damage_causality(&client_logs, server_log, "compatbot");
    assert!(causal.passed, "{causal:?}");
    assert!(causal.missing_steps.is_empty(), "{causal:?}");
    assert!(causal.order_violations.is_empty(), "{causal:?}");

    let out_of_order_server = "MC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\n";
    let causal_order_fail =
        evaluate_projectile_damage_causality(&client_logs, out_of_order_server, "compatbot");
    assert!(!causal_order_fail.passed, "{causal_order_fail:?}");
    assert!(causal_order_fail
        .order_violations
        .contains(&"server_projectile_use_before_hit"));

    let missing_victim_health = evaluate_projectile_damage_causality(
        &[ClientLogSlice {
            username: "compatbota",
            output: attacker_log,
        }],
        server_log,
        "compatbot",
    );
    assert!(!missing_victim_health.passed, "{missing_victim_health:?}");
    assert!(missing_victim_health
        .missing_steps
        .contains(&"victim_client_damage_update"));
}

#[test]
fn projectile_damage_dry_run_uses_steel_arrow_policy() {
    let mut cfg = test_config(
        &[
            "--scenario",
            "projectile-damage-attribution",
            "--valence-rev",
            PINNED_PROJECTILE_DAMAGE_VALENCE_REV,
        ],
        &[],
    )
    .expect("projectile damage config parses");
    cfg.arrow_damage_policy = runtime_config::ArrowDamagePolicy {
        base_damage: 4.0,
        velocity_multiplier: DEFAULT_ARROW_VELOCITY_MULTIPLIER,
        max_damage: DEFAULT_ARROW_MAX_DAMAGE,
    };

    let evidence = projectile_damage_dry_run_evidence(&cfg);
    assert!(
        evidence
            .scenario
            .as_ref()
            .expect("scenario evidence")
            .passed,
        "{evidence:?}"
    );
    let causality = evidence
        .projectile_damage_causality
        .as_ref()
        .expect("causality evidence");
    assert!(causality.passed, "{causality:?}");
    assert!(causality
        .observed_steps
        .contains(&"server_projectile_hit_attacker_victim_health_delta"));
}

#[test]
fn projectile_damage_attribution_requires_pinned_valence_revision() {
    let cfg = test_config(
        &[
            "--dry-run",
            "--scenario",
            "projectile-damage-attribution",
            "--valence-rev",
            "HEAD",
        ],
        &[],
    )
    .expect("config parses before execution validation");
    let err = validate_projectile_damage_dependency(&cfg).unwrap_err();
    assert!(err.contains(PINNED_PROJECTILE_DAMAGE_VALENCE_REV), "{err}");

    let pinned = test_config(
        &[
            "--dry-run",
            "--scenario",
            "projectile-damage-attribution",
            "--valence-rev",
            PINNED_PROJECTILE_DAMAGE_VALENCE_REV,
        ],
        &[],
    )
    .expect("pinned config parses");
    validate_projectile_damage_dependency(&pinned).expect("pinned revision accepted");
}
