#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryCategory {
    RuntimeConfig,
    ArenaSetup,
    TeamRules,
    FlagRules,
    ScoringRules,
    InventoryProbes,
    CombatProbes,
    ProjectileProbes,
    ScheduleContracts,
    MilestoneFormatting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModuleBoundary {
    pub category: BoundaryCategory,
    pub owner: &'static str,
    pub pure_core: &'static str,
    pub shell: &'static str,
    pub non_claims: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryIssue {
    MissingCategory(BoundaryCategory),
    EmptyOwner(BoundaryCategory),
    EmptyPureCore(BoundaryCategory),
    EmptyShell(BoundaryCategory),
    MissingNonClaims(BoundaryCategory),
}

pub const CTF_BOUNDARY_NON_CLAIMS: &[&str] = &[
    "no full CTF correctness",
    "no broad Minecraft compatibility",
    "no production readiness",
    "no public-server safety",
];

pub const REQUIRED_BOUNDARY_CATEGORIES: &[BoundaryCategory] = &[
    BoundaryCategory::RuntimeConfig,
    BoundaryCategory::ArenaSetup,
    BoundaryCategory::TeamRules,
    BoundaryCategory::FlagRules,
    BoundaryCategory::ScoringRules,
    BoundaryCategory::InventoryProbes,
    BoundaryCategory::CombatProbes,
    BoundaryCategory::ProjectileProbes,
    BoundaryCategory::ScheduleContracts,
    BoundaryCategory::MilestoneFormatting,
];

pub const CTF_MODULE_BOUNDARIES: &[ModuleBoundary] = &[
    ModuleBoundary {
        category: BoundaryCategory::RuntimeConfig,
        owner: "fixture_core::ctf::runtime_config",
        pure_core: "parse_runtime_config/runtime_config_issues",
        shell: "CtfRuntimeConfigSourcePlugin reads env and applies resource reload events",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::ArenaSetup,
        owner: "examples::ctf::arena shell",
        pure_core: "boundary only; LayerBundle mutation stays in shell",
        shell: "setup/build_flag/build_spawn_box own chunks, signs, portals, and layers",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::TeamRules,
        owner: "fixture_core::ctf::team and spawn",
        pure_core: "Team labels/opponents and spawn reset assignment decisions",
        shell: "do_team_selector_portals applies Bevy components, inventories, packets, and chat",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::FlagRules,
        owner: "fixture_core::ctf::flags",
        pure_core: "flag pickup, race, invalid pickup, and invalid return/drop decisions",
        shell: "digging and do_flag_capturing mutate FlagManager, blocks, score, and components",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::ScoringRules,
        owner: "fixture_core::ctf::scoring",
        pure_core: "score snapshots and score-limit milestone decisions",
        shell: "log_score_limit_capture_and_win updates WinConditionState and emits logs",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::InventoryProbes,
        owner: "fixture_core::ctf::inventory",
        pure_core: "stack split/merge and drag transaction classifiers",
        shell: "log_inventory_click_state maps ClickSlotEvent data and prints milestones",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::CombatProbes,
        owner: "fixture_core::ctf::combat",
        pure_core: "reference-hit, armor mitigation, and knockback metric decisions",
        shell: "handle_combat_events applies damage, velocity, status, flag return, and logs",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::ProjectileProbes,
        owner: "examples::ctf projectile shell plus fixture_core::ctf::combat",
        pure_core:
            "arrow damage policy and projectile evidence decisions are deterministic helpers",
        shell:
            "handle_projectile_events and combat projectile branch own events, health, and packets",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::ScheduleContracts,
        owner: "examples::ctf::schedule_contracts",
        pure_core: "constant contract data declares schedules, resources, events, and non-claims",
        shell: "CtfGameplayPlugin registers systems into Bevy schedules and phase sets",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
    ModuleBoundary {
        category: BoundaryCategory::MilestoneFormatting,
        owner: "fixture_core::ctf::flags/scoring/spawn plus example adapters",
        pure_core: "formatters return milestone strings from in-memory snapshots",
        shell: "systems own info!/println! emission and resource mutation",
        non_claims: CTF_BOUNDARY_NON_CLAIMS,
    },
];

pub fn validate_module_boundaries(boundaries: &[ModuleBoundary]) -> Result<(), BoundaryIssue> {
    for required in REQUIRED_BOUNDARY_CATEGORIES {
        if !boundaries
            .iter()
            .any(|boundary| boundary.category == *required)
        {
            return Err(BoundaryIssue::MissingCategory(*required));
        }
    }
    for boundary in boundaries {
        if boundary.owner.is_empty() {
            return Err(BoundaryIssue::EmptyOwner(boundary.category));
        }
        if boundary.pure_core.is_empty() {
            return Err(BoundaryIssue::EmptyPureCore(boundary.category));
        }
        if boundary.shell.is_empty() {
            return Err(BoundaryIssue::EmptyShell(boundary.category));
        }
        if boundary.non_claims.is_empty() {
            return Err(BoundaryIssue::MissingNonClaims(boundary.category));
        }
    }
    Ok(())
}
