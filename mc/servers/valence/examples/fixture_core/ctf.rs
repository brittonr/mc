mod boundaries;
mod combat;
mod flags;
mod inventory;
mod projectile;
mod runtime_config;
mod scoring;
mod spawn;
mod team;

pub use boundaries::{
    validate_module_boundaries, BoundaryCategory, BoundaryIssue, ModuleBoundary,
    CTF_BOUNDARY_NON_CLAIMS, CTF_MODULE_BOUNDARIES, REQUIRED_BOUNDARY_CATEGORIES,
};
pub use combat::{
    combat_armor_mitigation_for, knockback_metric, reference_hit_for, vanilla_armor_mitigation_for,
    ArmorState,
};
pub use flags::{
    evaluate_flag_pickup, invalid_flag_pickup_rejection_milestone,
    invalid_return_drop_rejection_milestone, race_accepted_transition_milestone,
    race_duplicate_pickup_blocked, race_final_state_milestone, race_rejected_transition_milestone,
    FlagPickupDecision, FlagPresence, FlagSnapshot, RaceFinalContract,
};
pub use inventory::{
    classify_inventory_drag_transactions_event, classify_inventory_stack_split_merge_event,
    InventoryClickMode, InventoryClickSnapshot, InventoryDragAction, InventoryDragContract,
    InventoryDragState, InventoryItemStack, InventoryProbeItem, InventorySlotChange,
    InventoryStackAction, InventoryStackContract, InventoryStackState,
};
pub use projectile::{
    projectile_sequence_matches, projectile_travel_collision_markers, ProjectileProbeContract,
    ProjectileTravelCollisionMarkers,
};
pub use runtime_config::{
    parse_nonzero_env_flag, parse_present_env_flag, parse_runtime_config, runtime_config_issues,
    ArrowPolicyConfig, ProbeConfig, RuntimeConfig, RuntimeConfigInputs, RuntimeConfigIssue,
    ENV_FLAG_DISABLED_VALUE, ENV_FLAG_ENABLED_VALUE,
};
pub use scoring::{
    score_limit_duplicate_win_milestone, score_limit_final_capture_milestone,
    score_limit_post_win_score_mutation_milestone, score_limit_pre_state_milestone,
    score_limit_win_condition_milestone, ScoreSnapshot,
};
pub use spawn::{
    defer_spawn_assignment, spawn_resource_reset_state_milestone, spawn_team_assignment_milestone,
    spawn_team_balance_milestone, SpawnResetContract, SpawnResetState,
};
pub use team::Team;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_USER_A: &str = "compatbota";
    const TEST_USER_B: &str = "compatbotb";
    const TEST_OTHER_USER: &str = "compatbotc";
    const TEST_STACK_WINDOW: u8 = 0;
    const TEST_STACK_SOURCE_SLOT: i16 = 37;
    const TEST_STACK_DESTINATION_SLOT: i16 = 38;
    const TEST_DRAG_TARGET_A: i16 = 38;
    const TEST_DRAG_TARGET_B: i16 = 39;
    const TEST_DRAG_OUTSIDE_SLOT: i16 = -999;
    const TEST_STACK_FULL_COUNT: i8 = 64;
    const TEST_STACK_HALF_COUNT: i8 = 32;
    const TEST_STACK_EMPTY_COUNT: i8 = 0;
    const TEST_LEFT_BUTTON: i8 = 0;
    const TEST_RIGHT_BUTTON: i8 = 1;
    const TEST_DRAG_START_BUTTON: i8 = 0;
    const TEST_DRAG_ADD_SLOT_BUTTON: i8 = 1;
    const TEST_DRAG_END_BUTTON: i8 = 2;
    const TEST_SCORE_LIMIT: u32 = 2;
    const TEST_WIN_EMISSIONS: u32 = 1;
    const TEST_RACE_WINDOW_TICKS: u32 = 40;
    const TEST_RED_CAPTURE_SCORE: ScoreSnapshot = ScoreSnapshot { red: 1, blue: 0 };
    const TEST_RED_SCORE_LIMIT_SCORE: ScoreSnapshot = ScoreSnapshot {
        red: TEST_SCORE_LIMIT,
        blue: 0,
    };
    const TEST_COMPAT_MITIGATION: f32 = 2.0;
    const TEST_DIAMOND_ARMOR_POINTS: f32 = 8.0;
    const TEST_DIAMOND_TOUGHNESS: f32 = 2.0;
    const TEST_BASE_DAMAGE: f32 = 6.0;
    const TEST_REFERENCE_MITIGATION: f32 = 1.344;
    const TEST_FLOAT_TOLERANCE: f32 = 0.0001;
    const TEST_KNOCKBACK_X: f32 = 8.0;
    const TEST_KNOCKBACK_Y: f32 = 6.432;
    const TEST_KNOCKBACK_Z: f32 = 0.0;
    const TEST_NORMALIZED_KNOCKBACK: f64 = 0.40;
    const TEST_PROJECTILE_SEQUENCE: i32 = 303;
    const TEST_PROJECTILE_ID: &str = "arrow_probe_sequence_303";
    const TEST_PROJECTILE_WEAPON: &str = "Bow";
    const TEST_PROJECTILE_DAMAGE: f32 = 3.0;
    const TEST_PROJECTILE_POLICY: &str = "damage-linear";
    const TEST_PROJECTILE_GENERATION: u64 = 0;
    const TEST_PROJECTILE_PROOF_BASIS: &str = "bounded_fixture_not_entity_physics";
    const TEST_PROJECTILE_TRAVEL_SAMPLE_KIND: &str = "synthetic_midpoint";
    const TEST_PROJECTILE_TRAVEL_SAMPLE_INDEX: u32 = 1;
    const TEST_PROJECTILE_COLLISION_KIND: &str = "synthetic_entity_hit";
    const TEST_PROJECTILE_PLAYER_MAX_HEALTH: f32 = 20.0;
    const TEST_PROJECTILE_EXPECTED_HEALTH_AFTER: f32 =
        TEST_PROJECTILE_PLAYER_MAX_HEALTH - TEST_PROJECTILE_DAMAGE;
    const TEST_WRONG_PROJECTILE_SEQUENCE: i32 = 404;

    fn stack_contract() -> InventoryStackContract {
        InventoryStackContract {
            window_id: TEST_STACK_WINDOW,
            source_slot: TEST_STACK_SOURCE_SLOT,
            destination_slot: TEST_STACK_DESTINATION_SLOT,
            full_count: TEST_STACK_FULL_COUNT,
            half_count: TEST_STACK_HALF_COUNT,
            empty_count: TEST_STACK_EMPTY_COUNT,
            left_button: TEST_LEFT_BUTTON,
            right_button: TEST_RIGHT_BUTTON,
        }
    }

    fn drag_contract() -> InventoryDragContract {
        InventoryDragContract {
            window_id: TEST_STACK_WINDOW,
            source_slot: TEST_STACK_SOURCE_SLOT,
            target_slot_a: TEST_DRAG_TARGET_A,
            target_slot_b: TEST_DRAG_TARGET_B,
            outside_slot: TEST_DRAG_OUTSIDE_SLOT,
            full_count: TEST_STACK_FULL_COUNT,
            half_count: TEST_STACK_HALF_COUNT,
            empty_count: TEST_STACK_EMPTY_COUNT,
            left_button: TEST_LEFT_BUTTON,
            drag_start_button: TEST_DRAG_START_BUTTON,
            drag_add_slot_button: TEST_DRAG_ADD_SLOT_BUTTON,
            drag_end_button: TEST_DRAG_END_BUTTON,
        }
    }

    fn item(count: i8) -> InventoryItemStack {
        InventoryItemStack {
            item: InventoryProbeItem::ExpectedStackItem,
            count,
        }
    }

    fn click(
        slot_id: i16,
        button: i8,
        carried_item: InventoryItemStack,
        slot_changes: Vec<InventorySlotChange>,
    ) -> InventoryClickSnapshot {
        InventoryClickSnapshot {
            actor_matches: true,
            window_id: TEST_STACK_WINDOW,
            slot_id,
            button,
            mode: InventoryClickMode::Click,
            carried_item,
            slot_changes,
        }
    }

    fn drag(
        slot_id: i16,
        button: i8,
        carried_item: InventoryItemStack,
        slot_changes: Vec<InventorySlotChange>,
    ) -> InventoryClickSnapshot {
        InventoryClickSnapshot {
            actor_matches: true,
            window_id: TEST_STACK_WINDOW,
            slot_id,
            button,
            mode: InventoryClickMode::Drag,
            carried_item,
            slot_changes,
        }
    }

    fn slot(slot: i16, stack: InventoryItemStack) -> InventorySlotChange {
        InventorySlotChange { slot, stack }
    }

    #[test]
    fn flag_pickup_accepts_enemy_base_flag_and_rejects_wrong_team_or_duplicate() {
        assert_eq!(
            evaluate_flag_pickup(Team::Red, Team::Blue, FlagPresence::AtBase),
            FlagPickupDecision::Accept
        );
        assert_eq!(
            evaluate_flag_pickup(Team::Red, Team::Red, FlagPresence::AtBase),
            FlagPickupDecision::RejectOwnFlag
        );
        assert_eq!(
            evaluate_flag_pickup(Team::Red, Team::Blue, FlagPresence::Held),
            FlagPickupDecision::RejectAlreadyHeld
        );
    }

    #[test]
    fn race_and_score_milestones_preserve_receipt_strings_and_fail_closed() {
        let accepted = race_accepted_transition_milestone(
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            "pickup",
            TEST_RACE_WINDOW_TICKS,
        );
        let rejected = race_rejected_transition_milestone(
            TEST_USER_B,
            Team::Red,
            Team::Blue,
            "duplicate_pickup",
            TEST_RACE_WINDOW_TICKS,
        );
        let final_contract = RaceFinalContract {
            expected_capture_team: Team::Red,
            expected_carried_flag: Team::Blue,
            expected_score: TEST_RED_CAPTURE_SCORE,
            expected_flag_state: FlagPresence::AtBase,
            flag_state_label: "at_base",
            race_window_ticks: TEST_RACE_WINDOW_TICKS,
            accepted_transition: "pickup",
            rejected_transition: "duplicate_pickup",
        };
        let final_state = race_final_state_milestone(
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            TEST_RED_CAPTURE_SCORE,
            FlagSnapshot {
                red: FlagPresence::AtBase,
                blue: FlagPresence::AtBase,
            },
            final_contract,
        )
        .expect("valid final state should emit milestone");
        let rejected_final_contract = RaceFinalContract {
            expected_capture_team: Team::Red,
            expected_carried_flag: Team::Blue,
            expected_score: TEST_RED_CAPTURE_SCORE,
            expected_flag_state: FlagPresence::AtBase,
            flag_state_label: "at_base",
            race_window_ticks: TEST_RACE_WINDOW_TICKS,
            accepted_transition: "pickup",
            rejected_transition: "duplicate_pickup",
        };
        let rejected_final_state = race_final_state_milestone(
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            TEST_RED_SCORE_LIMIT_SCORE,
            FlagSnapshot {
                red: FlagPresence::AtBase,
                blue: FlagPresence::AtBase,
            },
            rejected_final_contract,
        );
        let win = score_limit_win_condition_milestone(
            TEST_USER_A,
            Team::Red,
            TEST_RED_SCORE_LIMIT_SCORE,
            TEST_WIN_EMISSIONS,
            TEST_SCORE_LIMIT,
        );

        assert!(
            accepted.contains("ctf_race_accepted_transition"),
            "{accepted}"
        );
        assert!(accepted.contains("username=compatbota"), "{accepted}");
        assert!(rejected.contains("reason=flag_already_held"), "{rejected}");
        assert!(
            final_state.contains("ctf_race_final_state"),
            "{final_state}"
        );
        assert!(rejected_final_state.is_none());
        let score_limit_fragment = format!("score_limit={TEST_SCORE_LIMIT}");
        assert!(win.contains(&score_limit_fragment), "{win}");
        assert!(score_limit_duplicate_win_milestone(TEST_USER_A, Team::Red)
            .contains("outcome=forbidden_duplicate_win"));
    }

    #[test]
    fn spawn_reset_core_records_balanced_assignments_and_rejects_malformed_state() {
        let contract = SpawnResetContract {
            expected_red_count: 1,
            expected_blue_count: 1,
            expected_blue_username: TEST_USER_B,
            reset_score: TEST_RED_CAPTURE_SCORE,
            slot36_resource: "WoodenSword:1",
            red_slot37_resource: "RedWool:64",
            blue_slot37_resource: "BlueWool:64",
            reset_slot37_resource: "TeamWool:64",
            reset_state: "scoreboard_flags_and_resources_coherent",
        };
        let mut state = SpawnResetState::default();
        state.record_assignment(TEST_USER_A, Team::Red);
        state.record_assignment(TEST_USER_B, Team::Blue);

        let balance = spawn_team_balance_milestone(&state, &contract)
            .expect("balanced state should emit milestone");
        let reset = spawn_resource_reset_state_milestone(
            &state,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            TEST_RED_CAPTURE_SCORE,
            &contract,
        )
        .expect("valid reset should emit milestone");
        let malformed = spawn_resource_reset_state_milestone(
            &state,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            TEST_RED_SCORE_LIMIT_SCORE,
            &contract,
        );

        assert!(balance.contains("red_count=1 blue_count=1"), "{balance}");
        assert!(
            reset.contains("reset_state=scoreboard_flags_and_resources_coherent"),
            "{reset}"
        );
        assert!(malformed.is_none());
        assert!(defer_spawn_assignment(TEST_USER_B, Team::Red, TEST_USER_B));
        assert!(!defer_spawn_assignment(
            TEST_OTHER_USER,
            Team::Red,
            TEST_USER_B
        ));
    }

    #[test]
    fn inventory_stack_core_accepts_ordered_sequence_and_rejects_invalid_inventory() {
        let contract = stack_contract();
        let split_pickup = click(
            TEST_STACK_SOURCE_SLOT,
            TEST_RIGHT_BUTTON,
            item(TEST_STACK_HALF_COUNT),
            vec![slot(TEST_STACK_SOURCE_SLOT, item(TEST_STACK_HALF_COUNT))],
        );
        let split_place = click(
            TEST_STACK_DESTINATION_SLOT,
            TEST_LEFT_BUTTON,
            InventoryItemStack::empty(TEST_STACK_EMPTY_COUNT),
            vec![slot(
                TEST_STACK_DESTINATION_SLOT,
                item(TEST_STACK_HALF_COUNT),
            )],
        );
        let wrong_actor = InventoryClickSnapshot {
            actor_matches: false,
            ..split_pickup.clone()
        };
        let wrong_count = click(
            TEST_STACK_SOURCE_SLOT,
            TEST_RIGHT_BUTTON,
            item(TEST_STACK_FULL_COUNT),
            vec![slot(TEST_STACK_SOURCE_SLOT, item(TEST_STACK_FULL_COUNT))],
        );

        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &split_pickup,
                InventoryStackState::default(),
                contract,
            ),
            Some(InventoryStackAction::SplitPickup)
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &split_place,
                InventoryStackState {
                    split_pickup_seen: true,
                    split_place_seen: false,
                    merge_pickup_seen: false,
                },
                contract,
            ),
            Some(InventoryStackAction::SplitPlace)
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &wrong_actor,
                InventoryStackState::default(),
                contract
            ),
            None
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &wrong_count,
                InventoryStackState::default(),
                contract
            ),
            None
        );
    }

    #[test]
    fn inventory_drag_core_accepts_ordered_sequence_and_rejects_out_of_order_or_bad_distribution() {
        let contract = drag_contract();
        let pickup = click(
            TEST_STACK_SOURCE_SLOT,
            TEST_LEFT_BUTTON,
            item(TEST_STACK_FULL_COUNT),
            vec![slot(
                TEST_STACK_SOURCE_SLOT,
                InventoryItemStack::empty(TEST_STACK_EMPTY_COUNT),
            )],
        );
        let start = drag(
            TEST_DRAG_OUTSIDE_SLOT,
            TEST_DRAG_START_BUTTON,
            item(TEST_STACK_FULL_COUNT),
            Vec::new(),
        );
        let end_bad_distribution = drag(
            TEST_DRAG_OUTSIDE_SLOT,
            TEST_DRAG_END_BUTTON,
            InventoryItemStack::empty(TEST_STACK_EMPTY_COUNT),
            vec![slot(TEST_DRAG_TARGET_A, item(TEST_STACK_FULL_COUNT))],
        );

        assert_eq!(
            classify_inventory_drag_transactions_event(
                &pickup,
                InventoryDragState::default(),
                contract
            ),
            Some(InventoryDragAction::PickupSource)
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                &start,
                InventoryDragState {
                    pickup_seen: true,
                    drag_start_seen: false,
                    target_a_seen: false,
                    target_b_seen: false,
                },
                contract,
            ),
            Some(InventoryDragAction::DragStart)
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                &start,
                InventoryDragState::default(),
                contract
            ),
            None
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                &end_bad_distribution,
                InventoryDragState {
                    pickup_seen: true,
                    drag_start_seen: true,
                    target_a_seen: true,
                    target_b_seen: true,
                },
                contract,
            ),
            None
        );
    }

    #[test]
    fn combat_core_handles_reference_hits_and_armor_mitigation() {
        assert!(reference_hit_for(
            true,
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            TEST_USER_B,
        ));
        assert!(!reference_hit_for(
            false,
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            TEST_USER_B,
        ));
        assert!(!reference_hit_for(
            true,
            TEST_USER_B,
            TEST_USER_A,
            TEST_USER_A,
            TEST_USER_B,
        ));
        let mitigation = combat_armor_mitigation_for(
            true,
            false,
            ArmorState::DiamondChestplate,
            TEST_BASE_DAMAGE,
            TEST_COMPAT_MITIGATION,
            TEST_DIAMOND_ARMOR_POINTS,
            TEST_DIAMOND_TOUGHNESS,
        );
        assert!((mitigation - TEST_REFERENCE_MITIGATION).abs() < TEST_FLOAT_TOLERANCE);
        assert_eq!(
            combat_armor_mitigation_for(
                false,
                true,
                ArmorState::DiamondChestplate,
                TEST_BASE_DAMAGE,
                TEST_COMPAT_MITIGATION,
                TEST_DIAMOND_ARMOR_POINTS,
                TEST_DIAMOND_TOUGHNESS,
            ),
            TEST_COMPAT_MITIGATION
        );
        assert_eq!(
            combat_armor_mitigation_for(
                true,
                false,
                ArmorState::Other,
                TEST_BASE_DAMAGE,
                TEST_COMPAT_MITIGATION,
                TEST_DIAMOND_ARMOR_POINTS,
                TEST_DIAMOND_TOUGHNESS,
            ),
            0.0
        );
        assert_eq!(
            knockback_metric([TEST_KNOCKBACK_X, TEST_KNOCKBACK_Y, TEST_KNOCKBACK_Z]),
            TEST_NORMALIZED_KNOCKBACK
        );
    }

    #[test]
    fn projectile_core_formats_bounded_markers_and_rejects_sequence_mismatch() {
        let markers = projectile_travel_collision_markers(
            TEST_USER_A,
            TEST_USER_B,
            ProjectileProbeContract {
                sequence: TEST_PROJECTILE_SEQUENCE,
                projectile_id: TEST_PROJECTILE_ID,
                weapon: TEST_PROJECTILE_WEAPON,
                damage: TEST_PROJECTILE_DAMAGE,
                policy_id: TEST_PROJECTILE_POLICY,
                generation: TEST_PROJECTILE_GENERATION,
                proof_basis: TEST_PROJECTILE_PROOF_BASIS,
                travel_sample_kind: TEST_PROJECTILE_TRAVEL_SAMPLE_KIND,
                travel_sample_index: TEST_PROJECTILE_TRAVEL_SAMPLE_INDEX,
                collision_kind: TEST_PROJECTILE_COLLISION_KIND,
                player_max_health: TEST_PROJECTILE_PLAYER_MAX_HEALTH,
            },
        );

        assert!(markers.use_marker.contains("projectile_use"), "{markers:?}");
        assert!(
            markers.use_marker.contains("sequence_matches=true"),
            "{markers:?}"
        );
        assert!(
            markers.travel.contains("projectile_travel_sample"),
            "{markers:?}"
        );
        assert!(
            markers.collision.contains("synthetic_entity_hit"),
            "{markers:?}"
        );
        let expected_health_after_fragment = format!(
            "victim_health_after={:.1}",
            TEST_PROJECTILE_EXPECTED_HEALTH_AFTER
        );
        assert!(
            markers.hit.contains(&expected_health_after_fragment),
            "{markers:?}"
        );
        assert!(projectile_sequence_matches(
            TEST_PROJECTILE_SEQUENCE,
            TEST_PROJECTILE_SEQUENCE
        ));
        assert!(!projectile_sequence_matches(
            TEST_WRONG_PROJECTILE_SEQUENCE,
            TEST_PROJECTILE_SEQUENCE
        ));
    }

    #[test]
    fn runtime_config_core_preserves_env_flag_contracts_and_rejects_reload_gaps() {
        let config = parse_runtime_config(&RuntimeConfigInputs {
            inventory_stack_split_merge_probe: Some(ENV_FLAG_ENABLED_VALUE.to_owned()),
            inventory_drag_transactions_probe: Some(ENV_FLAG_DISABLED_VALUE.to_owned()),
            vanilla_combat_reference_probe: None,
            vanilla_combat_armor_reference_probe: Some(ENV_FLAG_ENABLED_VALUE.to_owned()),
            arrow_policy_config: Some("policy.scm".to_owned()),
            arrow_policy_reload_request: Some("reload-1".to_owned()),
            ctf_score_limit_win_probe: Some("nonzero".to_owned()),
            ctf_race_probe: Some(ENV_FLAG_DISABLED_VALUE.to_owned()),
            ctf_spawn_team_reset_probe: Some(ENV_FLAG_DISABLED_VALUE.to_owned()),
            ctf_invalid_return_drop_probe: Some(ENV_FLAG_ENABLED_VALUE.to_owned()),
            ctf_invalid_opponent_base_return_drop_probe: None,
            projectile_probe: Some(ENV_FLAG_ENABLED_VALUE.to_owned()),
            armor_mitigation_probe: Some(ENV_FLAG_DISABLED_VALUE.to_owned()),
            equipment_update_probe: Some("true".to_owned()),
        });
        let previous = parse_runtime_config(&RuntimeConfigInputs {
            arrow_policy_reload_request: Some("reload-1".to_owned()),
            ..Default::default()
        });
        let stale_missing_path = parse_runtime_config(&RuntimeConfigInputs {
            arrow_policy_reload_request: Some("reload-1".to_owned()),
            ..Default::default()
        });
        let disabled_projectile = parse_runtime_config(&RuntimeConfigInputs {
            arrow_policy_config: Some("policy.scm".to_owned()),
            projectile_probe: Some(ENV_FLAG_DISABLED_VALUE.to_owned()),
            ..Default::default()
        });

        assert!(config.probes.inventory_stack_split_merge);
        assert!(!config.probes.inventory_drag_transactions);
        assert!(config.probes.vanilla_combat_reference);
        assert!(config.probes.vanilla_combat_armor_reference);
        assert!(config.probes.score_limit_win);
        assert!(!config.probes.race);
        assert!(config.probes.spawn_team_reset);
        assert!(config.probes.invalid_return_drop);
        assert!(!config.probes.invalid_opponent_base_return_drop);
        assert!(config.probes.projectile);
        assert!(!config.probes.armor_mitigation);
        assert!(config.probes.equipment_update);
        assert_eq!(
            runtime_config_issues(Some(&previous), &stale_missing_path),
            vec![
                RuntimeConfigIssue::MissingReloadPolicyPath,
                RuntimeConfigIssue::StaleReloadRequest,
            ]
        );
        assert_eq!(
            runtime_config_issues(None, &disabled_projectile),
            vec![RuntimeConfigIssue::DisabledProjectilePolicy]
        );
    }

    #[test]
    fn module_boundary_map_covers_required_ctf_families_and_rejects_missing_boundary() {
        let mut missing = CTF_MODULE_BOUNDARIES.to_vec();
        missing.retain(|boundary| boundary.category != BoundaryCategory::ProjectileProbes);
        let mut empty_owner = CTF_MODULE_BOUNDARIES.to_vec();
        empty_owner.push(ModuleBoundary {
            category: BoundaryCategory::ProjectileProbes,
            owner: "",
            pure_core: "projectile core",
            shell: "projectile shell",
            non_claims: CTF_BOUNDARY_NON_CLAIMS,
        });

        validate_module_boundaries(CTF_MODULE_BOUNDARIES).unwrap();
        assert_eq!(
            validate_module_boundaries(&missing),
            Err(BoundaryIssue::MissingCategory(
                BoundaryCategory::ProjectileProbes
            ))
        );
        assert_eq!(
            validate_module_boundaries(&empty_owner),
            Err(BoundaryIssue::EmptyOwner(
                BoundaryCategory::ProjectileProbes
            ))
        );
    }
}
