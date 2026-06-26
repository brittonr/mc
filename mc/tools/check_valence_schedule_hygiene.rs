#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-valence-schedule-hygiene-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const SUCCESS_MESSAGE: &str = "valence schedule hygiene check passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "valence schedule hygiene self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;

const INVENTORY_PATH: &str = "docs/evidence/add-schedule-hygiene-gates.inventory.md";
const EVENT_LOOP_PHASE_INVENTORY_PATH: &str =
    "docs/evidence/add-event-loop-phase-system-sets.inventory.md";
const CHECK_TIERS_PATH: &str = "docs/check-tiers.md";
const NIX_CHECKS_PATH: &str = "nix/checks.nix";
const DUMP_SCHEDULE_README_PATH: &str = "servers/valence/tools/dump_schedule/README.md";
const DUMP_SCHEDULE_MAIN_PATH: &str = "servers/valence/tools/dump_schedule/src/main.rs";
const VALENCE_LIB_PATH: &str = "servers/valence/src/lib.rs";
const EVENT_LOOP_PATH: &str = "servers/valence/crates/valence_server/src/event/loop.rs";
const ACTION_PATH: &str = "servers/valence/crates/valence_server/src/action.rs";
const OBSERVABILITY_PATH: &str = "servers/valence/crates/valence_server/src/observability.rs";
const ANTICHEAT_PATH: &str = "servers/valence/crates/valence_server/src/anticheat.rs";
const ENTITY_PATH: &str = "servers/valence/crates/valence_entity/src/lib.rs";
const HITBOX_PATH: &str = "servers/valence/crates/valence_entity/src/hitbox.rs";
const CLIENT_PATH: &str = "servers/valence/crates/valence_server/src/client.rs";
const LAYER_PATH: &str = "servers/valence/crates/valence_server/src/layer.rs";
const REGISTRY_PATH: &str = "servers/valence/crates/valence_registry/src/lib.rs";
const COMMAND_PATH: &str = "servers/valence/crates/valence_command/src/lib.rs";
const SCOREBOARD_PATH: &str = "servers/valence/crates/valence_scoreboard/src/lib.rs";
const ADVANCEMENT_PATH: &str = "servers/valence/crates/valence_advancement/src/lib.rs";
const WORLD_BORDER_PATH: &str = "servers/valence/crates/valence_world_border/src/lib.rs";
const PLAYER_LIST_PATH: &str = "servers/valence/crates/valence_player_list/src/lib.rs";

const REQUIRED_PATHS: &[&str] = &[
    INVENTORY_PATH,
    EVENT_LOOP_PHASE_INVENTORY_PATH,
    CHECK_TIERS_PATH,
    NIX_CHECKS_PATH,
    DUMP_SCHEDULE_README_PATH,
    DUMP_SCHEDULE_MAIN_PATH,
    VALENCE_LIB_PATH,
    EVENT_LOOP_PATH,
    ACTION_PATH,
    OBSERVABILITY_PATH,
    ANTICHEAT_PATH,
    ENTITY_PATH,
    HITBOX_PATH,
    CLIENT_PATH,
    LAYER_PATH,
    REGISTRY_PATH,
    COMMAND_PATH,
    SCOREBOARD_PATH,
    ADVANCEMENT_PATH,
    WORLD_BORDER_PATH,
    PLAYER_LIST_PATH,
];

const REQUIREMENT_IDS: &[&str] = &[
    "r[valence_bevy_ecs.schedule_hygiene.inventory]",
    "r[valence_bevy_ecs.schedule_hygiene.policy]",
    "r[valence_bevy_ecs.schedule_hygiene.receipts]",
    "r[valence_bevy_ecs.schedule_hygiene.tests]",
    "r[valence_bevy_ecs.schedule_hygiene.evidence]",
    "r[valence_bevy_ecs.schedule_hygiene.validation]",
];

const INVENTORY_REQUIRED_PHRASES: &[&str] = &[
    "## Current schedule tooling",
    "## Named schedules",
    "## Named system sets",
    "## Default plugin behavior",
    "## Trigger policy",
    "## Receipt contract",
    "## Active checks and evidence gaps",
    "disabled-plugin comparison",
    "ambiguity policy",
    "No broad Minecraft compatibility",
];

const EVENT_LOOP_PHASE_INVENTORY_REQUIRED_PHRASES: &[&str] = &[
    "r[valence_bevy_ecs.event_loop_phase_sets.inventory]",
    "r[valence_bevy_ecs.event_loop_phase_sets.contract]",
    "r[valence_bevy_ecs.event_loop_phase_sets.wiring]",
    "r[valence_bevy_ecs.event_loop_phase_sets.compatibility]",
    "r[valence_bevy_ecs.event_loop_phase_sets.tests]",
    "r[valence_bevy_ecs.event_loop_phase_sets.validation]",
    "## Current event-loop schedule inventory",
    "## Phase-set contract",
    "## Selected wiring classification",
    "## Compatibility and private boundaries",
    "## Focused tests and negative fixtures",
    "No broad Minecraft compatibility",
];

const DUMP_SCHEDULE_README_REQUIRED_PHRASES: &[&str] = &[
    "## Schedule hygiene gate",
    "### Schedule evidence triggers",
    "### Focused receipt fields",
    "tools/check_valence_schedule_hygiene.rs --root .",
    "cargo r -p dump_schedule -- PostUpdate",
    "cargo r -p dump_schedule -- EventLoopPreUpdate",
    "plugin configuration",
    "disabled-plugin comparison",
    "ambiguity",
    "BLAKE3",
];

const CHECK_TIERS_REQUIRED_PHRASES: &[&str] = &[
    "tools/check_valence_schedule_hygiene.rs --self-test",
    "tools/check_valence_schedule_hygiene.rs --root .",
    "nix build .#checks.x86_64-linux.mc-valence-schedule-hygiene --no-link -L",
];

#[derive(Debug, Clone, Copy)]
struct TokenRequirement {
    path: &'static str,
    code: &'static str,
    token: &'static str,
}

const SOURCE_REQUIREMENTS: &[TokenRequirement] = &[
    TokenRequirement {
        path: DUMP_SCHEDULE_MAIN_PATH,
        code: "dump_schedule_uses_default_plugins",
        token: "app.add_plugins(DefaultPlugins)",
    },
    TokenRequirement {
        path: DUMP_SCHEDULE_MAIN_PATH,
        code: "dump_schedule_lists_schedules",
        token: "print_available_schedules(schedules)",
    },
    TokenRequirement {
        path: DUMP_SCHEDULE_MAIN_PATH,
        code: "dump_schedule_rejects_unknown_schedule",
        token: "Unknown schedule",
    },
    TokenRequirement {
        path: DUMP_SCHEDULE_MAIN_PATH,
        code: "dump_schedule_records_ambiguity_setting",
        token: "ambiguity_enable: false",
    },
    TokenRequirement {
        path: DUMP_SCHEDULE_MAIN_PATH,
        code: "dump_schedule_uses_debugdump_graph",
        token: "schedule_graph_dot",
    },
    TokenRequirement {
        path: VALENCE_LIB_PATH,
        code: "default_plugins_include_server",
        token: ".add(ServerPlugin)",
    },
    TokenRequirement {
        path: VALENCE_LIB_PATH,
        code: "default_plugins_include_event_loop",
        token: ".add(EventLoopPlugin)",
    },
    TokenRequirement {
        path: VALENCE_LIB_PATH,
        code: "default_plugins_gate_network",
        token: "#[cfg(feature = \"network\")]",
    },
    TokenRequirement {
        path: VALENCE_LIB_PATH,
        code: "default_plugins_add_network",
        token: "group = group.add(valence_network::NetworkPlugin)",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_adds_run_schedule",
        token: "add_schedule(Schedule::new(RunEventLoop))",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_adds_pre_update_schedule",
        token: "add_schedule(Schedule::new(EventLoopPreUpdate))",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_adds_update_schedule",
        token: "add_schedule(Schedule::new(EventLoopUpdate))",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_adds_post_update_schedule",
        token: "add_schedule(Schedule::new(EventLoopPostUpdate))",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_defines_phase_set_enum",
        token: "pub enum EventLoopSet",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_defines_raw_observer_set",
        token: "RawPacketObservers",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_defines_typed_adapter_set",
        token: "TypedAdapters",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_defines_domain_consumer_set",
        token: "DomainConsumers",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_defines_diagnostics_set",
        token: "Diagnostics",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_orders_raw_before_typed",
        token: "EventLoopSet::TypedAdapters.after(EventLoopSet::RawPacketObservers)",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_orders_typed_before_domain",
        token: "EventLoopSet::DomainConsumers.after(EventLoopSet::TypedAdapters)",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_orders_domain_before_diagnostics",
        token: "EventLoopSet::Diagnostics.after(EventLoopSet::DomainConsumers)",
    },
    TokenRequirement {
        path: EVENT_LOOP_PATH,
        code: "event_loop_ordered_after_pre_update",
        token: "insert_after(PreUpdate, RunEventLoop)",
    },
    TokenRequirement {
        path: ACTION_PATH,
        code: "action_adapter_in_typed_phase",
        token: "emit_player_action_events.in_set(EventLoopSet::TypedAdapters)",
    },
    TokenRequirement {
        path: ACTION_PATH,
        code: "action_consumer_in_domain_phase",
        token: "handle_player_action.in_set(EventLoopSet::DomainConsumers)",
    },
    TokenRequirement {
        path: ACTION_PATH,
        code: "action_tests_raw_access_phase",
        token: "count_raw_packets.in_set(EventLoopSet::RawPacketObservers)",
    },
    TokenRequirement {
        path: OBSERVABILITY_PATH,
        code: "observability_diagnostics_phase",
        token: "emit_network_packet_records.in_set(EventLoopSet::Diagnostics)",
    },
    TokenRequirement {
        path: ANTICHEAT_PATH,
        code: "anticheat_diagnostics_phase",
        token: "sample_anticheat_statistics.in_set(EventLoopSet::Diagnostics)",
    },
    TokenRequirement {
        path: ENTITY_PATH,
        code: "entity_set_init_entities",
        token: "pub struct InitEntitiesSet;",
    },
    TokenRequirement {
        path: ENTITY_PATH,
        code: "entity_set_tracked_data",
        token: "pub struct UpdateTrackedDataSet;",
    },
    TokenRequirement {
        path: ENTITY_PATH,
        code: "entity_set_clear_changes",
        token: "pub struct ClearEntityChangesSet;",
    },
    TokenRequirement {
        path: HITBOX_PATH,
        code: "hitbox_set_shape_update",
        token: "pub struct HitboxShapeUpdateSet;",
    },
    TokenRequirement {
        path: HITBOX_PATH,
        code: "hitbox_set_components_add",
        token: "pub struct HitboxComponentsAddSet;",
    },
    TokenRequirement {
        path: HITBOX_PATH,
        code: "hitbox_set_update",
        token: "pub struct HitboxUpdateSet;",
    },
    TokenRequirement {
        path: CLIENT_PATH,
        code: "client_set_flush_packets",
        token: "pub struct FlushPacketsSet;",
    },
    TokenRequirement {
        path: CLIENT_PATH,
        code: "client_set_spawn_clients",
        token: "pub struct SpawnClientsSet;",
    },
    TokenRequirement {
        path: CLIENT_PATH,
        code: "client_set_update_clients",
        token: "pub struct UpdateClientsSet;",
    },
    TokenRequirement {
        path: LAYER_PATH,
        code: "layer_set_pre_client",
        token: "pub struct UpdateLayersPreClientSet;",
    },
    TokenRequirement {
        path: LAYER_PATH,
        code: "layer_set_post_client",
        token: "pub struct UpdateLayersPostClientSet;",
    },
    TokenRequirement {
        path: REGISTRY_PATH,
        code: "registry_set",
        token: "pub struct RegistrySet;",
    },
    TokenRequirement {
        path: COMMAND_PATH,
        code: "command_system_set",
        token: "pub struct CommandSystemSet;",
    },
    TokenRequirement {
        path: SCOREBOARD_PATH,
        code: "scoreboard_set",
        token: "pub struct ScoreboardSet;",
    },
    TokenRequirement {
        path: ADVANCEMENT_PATH,
        code: "advancement_packet_set",
        token: "pub struct WriteAdvancementPacketToClientsSet;",
    },
    TokenRequirement {
        path: ADVANCEMENT_PATH,
        code: "advancement_cache_set",
        token: "pub struct WriteAdvancementToCacheSet;",
    },
    TokenRequirement {
        path: WORLD_BORDER_PATH,
        code: "world_border_set",
        token: "pub struct UpdateWorldBorderSet;",
    },
    TokenRequirement {
        path: PLAYER_LIST_PATH,
        code: "player_list_set",
        token: "struct PlayerListSet;",
    },
    TokenRequirement {
        path: NIX_CHECKS_PATH,
        code: "nix_check_wired",
        token: "mc-valence-schedule-hygiene",
    },
    TokenRequirement {
        path: NIX_CHECKS_PATH,
        code: "nix_check_compiles_checker",
        token: "tools/check_valence_schedule_hygiene.rs",
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScheduleReceipt<'a> {
    command: &'a str,
    schedule_label: &'a str,
    plugin_config: &'a str,
    sets: Vec<&'a str>,
    systems: Vec<&'a str>,
    plugins: Vec<&'a str>,
    ambiguities: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScheduleExpectation<'a> {
    schedule_label: &'a str,
    plugin_config: &'a str,
    expected_sets: Vec<&'a str>,
    expected_systems: Vec<&'a str>,
    forbidden_plugins: Vec<&'a str>,
    allowed_ambiguities: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileText {
    path: &'static str,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    root: PathBuf,
    self_test: bool,
}

fn validate_schedule_receipt(
    receipt: &ScheduleReceipt<'_>,
    expectation: &ScheduleExpectation<'_>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if receipt.command.trim().is_empty() {
        diagnostics.push(Diagnostic {
            code: "missing_command",
            message: String::from("schedule receipt must record command provenance"),
        });
    }

    if receipt.schedule_label != expectation.schedule_label {
        diagnostics.push(Diagnostic {
            code: "unknown_schedule",
            message: format!(
                "expected schedule {}, got {}",
                expectation.schedule_label, receipt.schedule_label
            ),
        });
    }

    if receipt.plugin_config != expectation.plugin_config {
        diagnostics.push(Diagnostic {
            code: "plugin_config_mismatch",
            message: format!(
                "expected plugin configuration {}, got {}",
                expectation.plugin_config, receipt.plugin_config
            ),
        });
    }

    push_missing_values(
        &mut diagnostics,
        "missing_set",
        "missing expected set",
        &expectation.expected_sets,
        &receipt.sets,
    );
    push_missing_values(
        &mut diagnostics,
        "missing_system",
        "missing expected system",
        &expectation.expected_systems,
        &receipt.systems,
    );
    push_forbidden_values(
        &mut diagnostics,
        "unintended_default_plugin",
        "forbidden plugin present",
        &expectation.forbidden_plugins,
        &receipt.plugins,
    );
    push_forbidden_ambiguities(
        &mut diagnostics,
        &expectation.allowed_ambiguities,
        &receipt.ambiguities,
    );

    diagnostics
}

fn push_missing_values(
    diagnostics: &mut Vec<Diagnostic>,
    code: &'static str,
    message_prefix: &str,
    expected: &[&str],
    actual: &[&str],
) {
    let actual_set = actual.iter().copied().collect::<BTreeSet<_>>();
    for value in expected {
        if !actual_set.contains(value) {
            diagnostics.push(Diagnostic {
                code,
                message: format!("{message_prefix}: {value}"),
            });
        }
    }
}

fn push_forbidden_values(
    diagnostics: &mut Vec<Diagnostic>,
    code: &'static str,
    message_prefix: &str,
    forbidden: &[&str],
    actual: &[&str],
) {
    let actual_set = actual.iter().copied().collect::<BTreeSet<_>>();
    for value in forbidden {
        if actual_set.contains(value) {
            diagnostics.push(Diagnostic {
                code,
                message: format!("{message_prefix}: {value}"),
            });
        }
    }
}

fn push_forbidden_ambiguities(
    diagnostics: &mut Vec<Diagnostic>,
    allowed: &[&str],
    actual: &[&str],
) {
    let allowed_set = allowed.iter().copied().collect::<BTreeSet<_>>();
    for ambiguity in actual {
        if !allowed_set.contains(ambiguity) {
            diagnostics.push(Diagnostic {
                code: "forbidden_ambiguity",
                message: format!("forbidden ambiguity present: {ambiguity}"),
            });
        }
    }
}

fn validate_repository_texts(texts: &[FileText]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    push_missing_phrases(
        &mut diagnostics,
        texts,
        INVENTORY_PATH,
        "inventory_missing_requirement_id",
        REQUIREMENT_IDS,
    );
    push_missing_phrases(
        &mut diagnostics,
        texts,
        INVENTORY_PATH,
        "inventory_missing_phrase",
        INVENTORY_REQUIRED_PHRASES,
    );
    push_missing_phrases(
        &mut diagnostics,
        texts,
        EVENT_LOOP_PHASE_INVENTORY_PATH,
        "event_loop_phase_inventory_missing_phrase",
        EVENT_LOOP_PHASE_INVENTORY_REQUIRED_PHRASES,
    );
    push_missing_phrases(
        &mut diagnostics,
        texts,
        DUMP_SCHEDULE_README_PATH,
        "dump_schedule_readme_missing_phrase",
        DUMP_SCHEDULE_README_REQUIRED_PHRASES,
    );
    push_missing_phrases(
        &mut diagnostics,
        texts,
        CHECK_TIERS_PATH,
        "check_tiers_missing_schedule_command",
        CHECK_TIERS_REQUIRED_PHRASES,
    );

    for requirement in SOURCE_REQUIREMENTS {
        match text_for_path(texts, requirement.path) {
            Some(text) => {
                if !text.contains(requirement.token) {
                    diagnostics.push(Diagnostic {
                        code: requirement.code,
                        message: format!(
                            "{} is missing token {:?}",
                            requirement.path, requirement.token
                        ),
                    });
                }
            }
            None => diagnostics.push(Diagnostic {
                code: "missing_loaded_text",
                message: format!("{} was not loaded", requirement.path),
            }),
        }
    }

    diagnostics
}

fn push_missing_phrases(
    diagnostics: &mut Vec<Diagnostic>,
    texts: &[FileText],
    path: &'static str,
    code: &'static str,
    phrases: &[&str],
) {
    match text_for_path(texts, path) {
        Some(text) => {
            for phrase in phrases {
                if !text.contains(phrase) {
                    diagnostics.push(Diagnostic {
                        code,
                        message: format!("{path} is missing phrase {phrase:?}"),
                    });
                }
            }
        }
        None => diagnostics.push(Diagnostic {
            code: "missing_loaded_text",
            message: format!("{path} was not loaded"),
        }),
    }
}

fn text_for_path<'a>(texts: &'a [FileText], path: &str) -> Option<&'a str> {
    texts
        .iter()
        .find(|file_text| file_text.path == path)
        .map(|file_text| file_text.text.as_str())
}

fn load_repository_texts(root: &Path) -> Result<Vec<FileText>, String> {
    let mut texts = Vec::new();
    for path in REQUIRED_PATHS {
        let full_path = root.join(path);
        let text = fs::read_to_string(&full_path)
            .map_err(|error| format!("failed to read {}: {error}", full_path.display()))?;
        texts.push(FileText { path, text });
    }
    Ok(texts)
}

fn valid_expectation<'a>() -> ScheduleExpectation<'a> {
    ScheduleExpectation {
        schedule_label: "PostUpdate",
        plugin_config: "default-without-network",
        expected_sets: vec!["FlushPacketsSet", "UpdateClientsSet"],
        expected_systems: vec!["flush_packets", "update_clients"],
        forbidden_plugins: vec!["NetworkPlugin"],
        allowed_ambiguities: Vec::new(),
    }
}

fn valid_receipt<'a>() -> ScheduleReceipt<'a> {
    ScheduleReceipt {
        command: "cargo r -p dump_schedule -- PostUpdate --output docs/evidence/post-update.svg",
        schedule_label: "PostUpdate",
        plugin_config: "default-without-network",
        sets: vec!["FlushPacketsSet", "UpdateClientsSet"],
        systems: vec!["flush_packets", "update_clients"],
        plugins: vec!["ServerPlugin", "ClientPlugin", "EventLoopPlugin"],
        ambiguities: Vec::new(),
    }
}

fn run_self_test() -> Result<(), String> {
    let expectation = valid_expectation();
    let receipt = valid_receipt();
    expect_no_diagnostics("positive valid schedule", &receipt, &expectation)?;

    let mut unknown_schedule = valid_receipt();
    unknown_schedule.schedule_label = "MissingSchedule";
    expect_diagnostic(
        "negative unknown schedule",
        &unknown_schedule,
        &expectation,
        "unknown_schedule",
    )?;

    let mut missing_set = valid_receipt();
    missing_set.sets = vec!["UpdateClientsSet"];
    expect_diagnostic(
        "negative missing set",
        &missing_set,
        &expectation,
        "missing_set",
    )?;

    let mut unintended_plugin = valid_receipt();
    unintended_plugin.plugins = vec![
        "ServerPlugin",
        "ClientPlugin",
        "EventLoopPlugin",
        "NetworkPlugin",
    ];
    expect_diagnostic(
        "negative unintended default plugin",
        &unintended_plugin,
        &expectation,
        "unintended_default_plugin",
    )?;

    let mut ambiguity = valid_receipt();
    ambiguity.ambiguities = vec!["update_clients <-> flush_packets"];
    expect_diagnostic(
        "negative ambiguity regression",
        &ambiguity,
        &expectation,
        "forbidden_ambiguity",
    )?;

    Ok(())
}

fn expect_no_diagnostics(
    name: &str,
    receipt: &ScheduleReceipt<'_>,
    expectation: &ScheduleExpectation<'_>,
) -> Result<(), String> {
    let diagnostics = validate_schedule_receipt(receipt, expectation);
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{name}: expected no diagnostics, got {diagnostics:?}"
        ))
    }
}

fn expect_diagnostic(
    name: &str,
    receipt: &ScheduleReceipt<'_>,
    expectation: &ScheduleExpectation<'_>,
    expected_code: &str,
) -> Result<(), String> {
    let diagnostics = validate_schedule_receipt(receipt, expectation);
    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == expected_code)
    {
        Ok(())
    } else {
        Err(format!(
            "{name}: expected diagnostic {expected_code:?}, got {diagnostics:?}"
        ))
    }
}

fn parse_args() -> Result<Command, String> {
    let mut args = env::args().skip(1);
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut self_test = false;

    while let Some(arg) = args.next() {
        if arg == ROOT_FLAG {
            let value = args
                .next()
                .ok_or_else(|| format!("{ROOT_FLAG} requires a path"))?;
            root = PathBuf::from(value);
        } else if arg == SELF_TEST_FLAG {
            self_test = true;
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }

    Ok(Command { root, self_test })
}

fn run(command: Command) -> Result<String, String> {
    if command.self_test {
        run_self_test()?;
        return Ok(String::from(SELF_TEST_SUCCESS_MESSAGE));
    }

    let texts = load_repository_texts(&command.root)?;
    let diagnostics = validate_repository_texts(&texts);
    if diagnostics.is_empty() {
        Ok(String::from(SUCCESS_MESSAGE))
    } else {
        Err(format_diagnostics(&diagnostics))
    }
}

fn format_diagnostics(diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() -> ExitCode {
    let command = match parse_args() {
        Ok(command) => command,
        Err(error) => {
            eprintln!("{error}");
            return FAILURE;
        }
    };

    match run(command) {
        Ok(message) => {
            println!("{message}");
            SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}
