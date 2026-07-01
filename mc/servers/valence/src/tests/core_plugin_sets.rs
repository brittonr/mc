use bevy_app::{App, Plugin, PluginGroup};
use bevy_ecs::event::Events;
use bevy_ecs::prelude::IntoSystemConfigs;
use bevy_ecs::schedule::Schedules;

use crate::DefaultPlugins;

const PRE_UPDATE_LABEL: &str = "PreUpdate";
const POST_UPDATE_LABEL: &str = "PostUpdate";
const EVENT_LOOP_PRE_UPDATE_LABEL: &str = "EventLoopPreUpdate";
const EVENT_LOOP_UPDATE_LABEL: &str = "EventLoopUpdate";
const EVENT_LOOP_POST_UPDATE_LABEL: &str = "EventLoopPostUpdate";
const MISSING_EVENT_LOOP_SET: &str = "MissingEventLoopSet";

const EVENT_LOOP_PRE_UPDATE_PHASE_SETS: &[&str] = &[
    "RawPacketObservers",
    "TypedAdapters",
    "DomainConsumers",
    "Diagnostics",
];
const EVENT_LOOP_UPDATE_PHASE_SETS: &[&str] = &["DomainConsumers", "Diagnostics"];
const EVENT_LOOP_POST_UPDATE_PHASE_SETS: &[&str] = &["Diagnostics"];
const EVENT_LOOP_RAW_OBSERVER_SYSTEMS: &[&str] = &["raw_packet_observer"];
const EVENT_LOOP_UPDATE_DOMAIN_SYSTEMS: &[&str] = &["event_loop_update_domain_consumer"];
const EVENT_LOOP_ACTION_SYSTEMS: &[&str] = &[
    "valence_server::action::emit_player_action_events",
    "valence_server::action::handle_player_action",
];
const EVENT_LOOP_DIAGNOSTIC_SYSTEMS: &[&str] = &["emit_event_loop_post_update_phase"];

const COMMAND_EVENT_LOOP_PRE_UPDATE_SETS: &[&str] = &["CommandTreeSet", "CommandSystemSet"];
const ADVANCEMENT_PRE_UPDATE_SETS: &[&str] =
    &["InitAdvancementClientsSet", "ReadAdvancementTabsSet"];
const ADVANCEMENT_POST_UPDATE_SETS: &[&str] = &[
    "WriteAdvancementToCacheSet",
    "WriteAdvancementPacketToClientsSet",
];
const EQUIPMENT_PRE_UPDATE_SETS: &[&str] =
    &["EquipmentInitSet", "EquipmentInputSet", "EquipmentSyncSet"];
const EQUIPMENT_POST_UPDATE_SETS: &[&str] = &["EquipmentBroadcastSet"];
const INVENTORY_PRE_UPDATE_SETS: &[&str] = &["InventoryInitSet", "InventoryMutationSet"];
const INVENTORY_EVENT_LOOP_PRE_UPDATE_SETS: &[&str] = &[
    "InventoryInputSet",
    "InventoryMutationSet",
    "InventoryCleanupSet",
];
const INVENTORY_POST_UPDATE_SETS: &[&str] = &[
    "InventoryCleanupSet",
    "InventoryWindowSyncSet",
    "InventoryPresentationSet",
];
const SCOREBOARD_POST_UPDATE_SETS: &[&str] = &["ScoreboardSet"];
const WEATHER_POST_UPDATE_SETS: &[&str] = &["WeatherClientUpdateSet", "WeatherLayerUpdateSet"];
const WORLD_BORDER_POST_UPDATE_SETS: &[&str] = &["UpdateWorldBorderSet"];
const BOSS_BAR_POST_UPDATE_SETS: &[&str] = &["BossBarUpdateSet"];

const MISSING_EVENT_LOOP_SETS: &[&str] = &[MISSING_EVENT_LOOP_SET];
const SCHEDULE_RECEIPT_COMMAND: &str = "cargo test core_plugin_sets";
const DEFAULT_PLUGIN_CONFIGURATION: &str = "DefaultPlugins without network/log";
const DISABLED_ACTION_PLUGIN_CONFIGURATION: &str = "DefaultPlugins without network/log/action";
const UNKNOWN_SCHEDULE_LABEL: &str = "MissingSchedule";
const AMBIGUITY_DISABLED: &str = "disabled";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScheduleAmbiguityMode {
    Disabled,
}

impl ScheduleAmbiguityMode {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => AMBIGUITY_DISABLED,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ScheduleReceiptExpectation {
    schedule_label: &'static str,
    plugin_configuration: &'static str,
    expected_sets: &'static [&'static str],
    expected_systems: &'static [&'static str],
    absent_sets: &'static [&'static str],
    absent_systems: &'static [&'static str],
    ambiguity_mode: ScheduleAmbiguityMode,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StructuredScheduleReceipt {
    command: &'static str,
    plugin_configuration: &'static str,
    schedule_label: &'static str,
    expected_sets: Vec<&'static str>,
    observed_sets: Vec<&'static str>,
    missing_sets: Vec<&'static str>,
    absent_sets: Vec<&'static str>,
    unexpected_sets: Vec<&'static str>,
    expected_systems: Vec<&'static str>,
    observed_systems: Vec<&'static str>,
    missing_systems: Vec<&'static str>,
    absent_systems: Vec<&'static str>,
    unexpected_systems: Vec<&'static str>,
    ambiguity_mode: &'static str,
    missing_schedule: bool,
    passed: bool,
    diagnostics: Vec<String>,
}

fn structured_schedule_receipt(
    command: &'static str,
    expectation: ScheduleReceiptExpectation,
    observed_graph: Option<&str>,
) -> StructuredScheduleReceipt {
    let missing_schedule = observed_graph.is_none();
    let observed_sets = observed_matching_facts(observed_graph, expectation.expected_sets);
    let observed_systems = observed_matching_facts(observed_graph, expectation.expected_systems);
    let missing_sets = missing_facts(observed_graph, expectation.expected_sets);
    let missing_systems = missing_facts(observed_graph, expectation.expected_systems);
    let unexpected_sets = unexpected_facts(observed_graph, expectation.absent_sets);
    let unexpected_systems = unexpected_facts(observed_graph, expectation.absent_systems);
    let passed = !missing_schedule
        && missing_sets.is_empty()
        && missing_systems.is_empty()
        && unexpected_sets.is_empty()
        && unexpected_systems.is_empty();
    let diagnostics = schedule_receipt_diagnostics(
        expectation.schedule_label,
        missing_schedule,
        &missing_sets,
        &missing_systems,
        &unexpected_sets,
        &unexpected_systems,
    );

    StructuredScheduleReceipt {
        command,
        plugin_configuration: expectation.plugin_configuration,
        schedule_label: expectation.schedule_label,
        expected_sets: expectation.expected_sets.to_vec(),
        observed_sets,
        missing_sets,
        absent_sets: expectation.absent_sets.to_vec(),
        unexpected_sets,
        expected_systems: expectation.expected_systems.to_vec(),
        observed_systems,
        missing_systems,
        absent_systems: expectation.absent_systems.to_vec(),
        unexpected_systems,
        ambiguity_mode: expectation.ambiguity_mode.as_str(),
        missing_schedule,
        passed,
        diagnostics,
    }
}

fn observed_matching_facts(
    observed_graph: Option<&str>,
    expected: &[&'static str],
) -> Vec<&'static str> {
    let Some(graph) = observed_graph else {
        return Vec::new();
    };
    expected
        .iter()
        .copied()
        .filter(|fact| graph.contains(fact))
        .collect()
}

fn missing_facts(observed_graph: Option<&str>, expected: &[&'static str]) -> Vec<&'static str> {
    let Some(graph) = observed_graph else {
        return expected.to_vec();
    };
    expected
        .iter()
        .copied()
        .filter(|fact| !graph.contains(fact))
        .collect()
}

fn unexpected_facts(observed_graph: Option<&str>, absent: &[&'static str]) -> Vec<&'static str> {
    let Some(graph) = observed_graph else {
        return Vec::new();
    };
    absent
        .iter()
        .copied()
        .filter(|fact| graph.contains(fact))
        .collect()
}

fn schedule_receipt_diagnostics(
    schedule_label: &str,
    missing_schedule: bool,
    missing_sets: &[&str],
    missing_systems: &[&str],
    unexpected_sets: &[&str],
    unexpected_systems: &[&str],
) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if missing_schedule {
        diagnostics.push(format!("missing schedule {schedule_label}"));
    }
    diagnostics.extend(
        missing_sets
            .iter()
            .map(|fact| format!("missing set {fact}")),
    );
    diagnostics.extend(
        missing_systems
            .iter()
            .map(|fact| format!("missing system {fact}")),
    );
    diagnostics.extend(
        unexpected_sets
            .iter()
            .map(|fact| format!("unexpected set {fact}")),
    );
    diagnostics.extend(
        unexpected_systems
            .iter()
            .map(|fact| format!("unexpected system {fact}")),
    );
    diagnostics
}

fn schedule_receipt_for_app(
    app: &App,
    command: &'static str,
    expectation: ScheduleReceiptExpectation,
) -> StructuredScheduleReceipt {
    let graph = schedule_graph_option(app, expectation.schedule_label);
    structured_schedule_receipt(command, expectation, graph.as_deref())
}

fn render_structured_schedule_receipt(receipt: &StructuredScheduleReceipt) -> String {
    format!(
        "command={}\nplugin_configuration={}\nschedule_label={}\nambiguity_mode={}\npassed={}\nexpected_sets={}\nobserved_sets={}\nmissing_sets={}\nexpected_systems={}\nobserved_systems={}\nmissing_systems={}\nabsent_sets={}\nunexpected_sets={}\nabsent_systems={}\nunexpected_systems={}\ndiagnostics={}\n",
        receipt.command,
        receipt.plugin_configuration,
        receipt.schedule_label,
        receipt.ambiguity_mode,
        receipt.passed,
        receipt.expected_sets.join(","),
        receipt.observed_sets.join(","),
        receipt.missing_sets.join(","),
        receipt.expected_systems.join(","),
        receipt.observed_systems.join(","),
        receipt.missing_systems.join(","),
        receipt.absent_sets.join(","),
        receipt.unexpected_sets.join(","),
        receipt.absent_systems.join(","),
        receipt.unexpected_systems.join(","),
        receipt.diagnostics.join(";"),
    )
}

#[test]
fn default_core_plugins_expose_selected_ordering_sets() {
    let app = app_with_default_core_plugins();

    let pre_update = schedule_graph(&app, PRE_UPDATE_LABEL);
    assert_schedule_contains_sets(&pre_update, ADVANCEMENT_PRE_UPDATE_SETS);
    assert_schedule_contains_sets(&pre_update, EQUIPMENT_PRE_UPDATE_SETS);
    assert_schedule_contains_sets(&pre_update, INVENTORY_PRE_UPDATE_SETS);

    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);
    assert_schedule_contains_sets(&event_loop_pre_update, COMMAND_EVENT_LOOP_PRE_UPDATE_SETS);
    assert_schedule_contains_sets(&event_loop_pre_update, INVENTORY_EVENT_LOOP_PRE_UPDATE_SETS);

    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);
    assert_schedule_contains_sets(&post_update, ADVANCEMENT_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, EQUIPMENT_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, INVENTORY_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, SCOREBOARD_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, WEATHER_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, WORLD_BORDER_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, BOSS_BAR_POST_UPDATE_SETS);

    assert!(app
        .world()
        .contains_resource::<Events<crate::command::manager::CommandExecutionPacketEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::command::manager::CommandExecutionEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::command::manager::CommandProcessedEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::advancement::event::AdvancementTabChangeEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::equipment::EquipmentChangeEvent>>());
    assert!(app
        .world()
        .contains_resource::<crate::inventory::InventorySettings>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::ClickSlotEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::DropItemStackEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::CreativeInventoryActionEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::UpdateSelectedSlotEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::ClickSlotPacketEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::CloseHandledScreenEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::CreativeInventoryActionPacketEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<crate::inventory::UpdateSelectedSlotPacketEvent>>());
}

#[test]
fn event_loop_phase_sets_are_orderable_for_selected_systems() {
    let app = app_with_event_loop_phase_systems();

    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);
    assert_schedule_contains_sets(&event_loop_pre_update, EVENT_LOOP_PRE_UPDATE_PHASE_SETS);
    assert_schedule_contains_systems(&event_loop_pre_update, EVENT_LOOP_RAW_OBSERVER_SYSTEMS);
    assert_schedule_contains_systems(&event_loop_pre_update, EVENT_LOOP_ACTION_SYSTEMS);

    let event_loop_update = schedule_graph(&app, EVENT_LOOP_UPDATE_LABEL);
    assert_schedule_contains_sets(&event_loop_update, EVENT_LOOP_UPDATE_PHASE_SETS);
    assert_schedule_contains_systems(&event_loop_update, EVENT_LOOP_UPDATE_DOMAIN_SYSTEMS);

    let event_loop_post_update = schedule_graph(&app, EVENT_LOOP_POST_UPDATE_LABEL);
    assert_schedule_contains_sets(&event_loop_post_update, EVENT_LOOP_POST_UPDATE_PHASE_SETS);
    assert_schedule_contains_systems(&event_loop_post_update, EVENT_LOOP_DIAGNOSTIC_SYSTEMS);
}

#[test]
#[should_panic(expected = "schedule graph is missing set MissingEventLoopSet")]
fn event_loop_phase_missing_set_fixture_fails_clearly() {
    let app = app_with_event_loop_phase_systems();
    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);

    assert_schedule_contains_sets(&event_loop_pre_update, &[MISSING_EVENT_LOOP_SET]);
}

#[test]
fn disabled_action_plugin_omits_selected_event_loop_adapters() {
    let app = app_without_plugin::<crate::action::ActionPlugin>();
    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);

    assert_schedule_omits_systems(&event_loop_pre_update, EVENT_LOOP_ACTION_SYSTEMS);
    assert!(!app
        .world()
        .contains_resource::<Events<crate::action::PlayerActionEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::action::DiggingEvent>>());
}

#[test]
fn disabled_command_plugin_omits_command_schedule_contract() {
    let app = app_without_plugin::<crate::command::manager::CommandPlugin>();
    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);

    assert_schedule_omits_sets(&event_loop_pre_update, COMMAND_EVENT_LOOP_PRE_UPDATE_SETS);
    assert!(!app
        .world()
        .contains_resource::<Events<crate::command::manager::CommandExecutionPacketEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::command::manager::CommandExecutionEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::command::manager::CommandProcessedEvent>>());
}

#[test]
fn disabled_advancement_plugin_omits_advancement_schedule_contract() {
    let app = app_without_plugin::<crate::advancement::AdvancementPlugin>();
    let pre_update = schedule_graph(&app, PRE_UPDATE_LABEL);
    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);

    assert_schedule_omits_sets(&pre_update, ADVANCEMENT_PRE_UPDATE_SETS);
    assert_schedule_omits_sets(&post_update, ADVANCEMENT_POST_UPDATE_SETS);
    assert!(!app
        .world()
        .contains_resource::<Events<crate::advancement::event::AdvancementTabChangeEvent>>());
}

#[test]
fn disabled_equipment_plugin_omits_equipment_schedule_contract() {
    let app = app_without_plugin::<crate::equipment::EquipmentPlugin>();
    let pre_update = schedule_graph(&app, PRE_UPDATE_LABEL);
    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);

    assert_schedule_omits_sets(&pre_update, EQUIPMENT_PRE_UPDATE_SETS);
    assert_schedule_omits_sets(&post_update, EQUIPMENT_POST_UPDATE_SETS);
    assert!(!app
        .world()
        .contains_resource::<Events<crate::equipment::EquipmentChangeEvent>>());
}

#[test]
fn disabled_inventory_plugin_omits_inventory_schedule_contract() {
    let app = app_without_plugin::<crate::inventory::InventoryPlugin>();
    let pre_update = schedule_graph(&app, PRE_UPDATE_LABEL);
    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);
    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);

    assert_schedule_omits_sets(&pre_update, INVENTORY_PRE_UPDATE_SETS);
    assert_schedule_omits_sets(&event_loop_pre_update, INVENTORY_EVENT_LOOP_PRE_UPDATE_SETS);
    assert_schedule_omits_sets(&post_update, INVENTORY_POST_UPDATE_SETS);
    assert!(!app
        .world()
        .contains_resource::<crate::inventory::InventorySettings>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::ClickSlotEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::DropItemStackEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::CreativeInventoryActionEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::UpdateSelectedSlotEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::ClickSlotPacketEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::CloseHandledScreenEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::CreativeInventoryActionPacketEvent>>());
    assert!(!app
        .world()
        .contains_resource::<Events<crate::inventory::UpdateSelectedSlotPacketEvent>>());
}

#[test]
fn disabled_scoreboard_plugin_omits_scoreboard_schedule_contract() {
    let app = app_without_plugin::<crate::scoreboard::ScoreboardPlugin>();
    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);

    assert_schedule_omits_sets(&post_update, SCOREBOARD_POST_UPDATE_SETS);
}

#[test]
fn disabled_weather_plugin_omits_weather_schedule_contract() {
    let app = app_without_plugin::<crate::weather::WeatherPlugin>();
    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);

    assert_schedule_omits_sets(&post_update, WEATHER_POST_UPDATE_SETS);
}

#[test]
fn disabled_world_border_plugin_omits_world_border_schedule_contract() {
    let app = app_without_plugin::<crate::world_border::WorldBorderPlugin>();
    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);

    assert_schedule_omits_sets(&post_update, WORLD_BORDER_POST_UPDATE_SETS);
}

#[test]
fn disabled_boss_bar_plugin_omits_boss_bar_schedule_contract() {
    let app = app_without_plugin::<crate::boss_bar::BossBarPlugin>();
    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);

    assert_schedule_omits_sets(&post_update, BOSS_BAR_POST_UPDATE_SETS);
}

#[test]
fn structured_schedule_receipt_reports_default_core_facts() {
    let app = app_with_default_core_plugins();
    let expectation = ScheduleReceiptExpectation {
        schedule_label: PRE_UPDATE_LABEL,
        plugin_configuration: DEFAULT_PLUGIN_CONFIGURATION,
        expected_sets: INVENTORY_PRE_UPDATE_SETS,
        expected_systems: &[],
        absent_sets: &[],
        absent_systems: &[],
        ambiguity_mode: ScheduleAmbiguityMode::Disabled,
    };

    let receipt = schedule_receipt_for_app(&app, SCHEDULE_RECEIPT_COMMAND, expectation);

    assert!(receipt.passed);
    assert_eq!(receipt.observed_sets, INVENTORY_PRE_UPDATE_SETS.to_vec());
    assert!(receipt.diagnostics.is_empty());
    let rendered = render_structured_schedule_receipt(&receipt);
    assert!(rendered.contains("schedule_label=PreUpdate"));
    assert!(rendered.contains("ambiguity_mode=disabled"));
    assert!(rendered.contains("passed=true"));
}

#[test]
fn structured_schedule_receipt_reports_disabled_plugin_absence() {
    let app = app_without_plugin::<crate::action::ActionPlugin>();
    let expectation = ScheduleReceiptExpectation {
        schedule_label: EVENT_LOOP_PRE_UPDATE_LABEL,
        plugin_configuration: DISABLED_ACTION_PLUGIN_CONFIGURATION,
        expected_sets: &[],
        expected_systems: &[],
        absent_sets: &[],
        absent_systems: EVENT_LOOP_ACTION_SYSTEMS,
        ambiguity_mode: ScheduleAmbiguityMode::Disabled,
    };

    let receipt = schedule_receipt_for_app(&app, SCHEDULE_RECEIPT_COMMAND, expectation);

    assert!(receipt.passed);
    assert!(receipt.unexpected_systems.is_empty());
}

#[test]
fn structured_schedule_receipt_rejects_unknown_schedule_and_missing_set() {
    let missing_schedule_expectation = ScheduleReceiptExpectation {
        schedule_label: UNKNOWN_SCHEDULE_LABEL,
        plugin_configuration: DEFAULT_PLUGIN_CONFIGURATION,
        expected_sets: EVENT_LOOP_PRE_UPDATE_PHASE_SETS,
        expected_systems: &[],
        absent_sets: &[],
        absent_systems: &[],
        ambiguity_mode: ScheduleAmbiguityMode::Disabled,
    };
    let missing_schedule_receipt =
        structured_schedule_receipt(SCHEDULE_RECEIPT_COMMAND, missing_schedule_expectation, None);
    assert!(!missing_schedule_receipt.passed);
    assert!(missing_schedule_receipt.missing_schedule);
    assert_eq!(
        missing_schedule_receipt.diagnostics,
        vec![
            "missing schedule MissingSchedule".to_string(),
            "missing set RawPacketObservers".to_string(),
            "missing set TypedAdapters".to_string(),
            "missing set DomainConsumers".to_string(),
            "missing set Diagnostics".to_string(),
        ]
    );

    let missing_set_expectation = ScheduleReceiptExpectation {
        schedule_label: EVENT_LOOP_PRE_UPDATE_LABEL,
        plugin_configuration: DEFAULT_PLUGIN_CONFIGURATION,
        expected_sets: MISSING_EVENT_LOOP_SETS,
        expected_systems: &[],
        absent_sets: &[],
        absent_systems: &[],
        ambiguity_mode: ScheduleAmbiguityMode::Disabled,
    };
    let missing_set_receipt =
        structured_schedule_receipt(SCHEDULE_RECEIPT_COMMAND, missing_set_expectation, Some(""));
    assert!(!missing_set_receipt.passed);
    assert_eq!(
        missing_set_receipt.missing_sets,
        MISSING_EVENT_LOOP_SETS.to_vec()
    );
    assert_eq!(
        missing_set_receipt.diagnostics,
        vec!["missing set MissingEventLoopSet".to_string()]
    );
}

#[test]
fn structured_schedule_receipt_rejects_unexpected_system_and_is_deterministic() {
    let app = app_with_default_core_plugins();
    let expectation = ScheduleReceiptExpectation {
        schedule_label: EVENT_LOOP_PRE_UPDATE_LABEL,
        plugin_configuration: DEFAULT_PLUGIN_CONFIGURATION,
        expected_sets: &[],
        expected_systems: &[],
        absent_sets: &[],
        absent_systems: EVENT_LOOP_ACTION_SYSTEMS,
        ambiguity_mode: ScheduleAmbiguityMode::Disabled,
    };

    let first_receipt = schedule_receipt_for_app(&app, SCHEDULE_RECEIPT_COMMAND, expectation);
    let second_receipt = schedule_receipt_for_app(&app, SCHEDULE_RECEIPT_COMMAND, expectation);

    assert_eq!(first_receipt, second_receipt);
    assert_eq!(
        render_structured_schedule_receipt(&first_receipt),
        render_structured_schedule_receipt(&second_receipt)
    );
    assert!(!first_receipt.passed);
    assert_eq!(
        first_receipt.unexpected_systems,
        EVENT_LOOP_ACTION_SYSTEMS.to_vec()
    );
    assert_eq!(
        first_receipt.diagnostics,
        vec![
            "unexpected system valence_server::action::emit_player_action_events".to_string(),
            "unexpected system valence_server::action::handle_player_action".to_string(),
        ]
    );
}

fn app_with_default_core_plugins() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<crate::network::NetworkPlugin>()
            .disable::<crate::log::LogPlugin>(),
    );
    app
}

fn app_with_event_loop_phase_systems() -> App {
    let mut app = app_with_default_core_plugins();
    app.add_plugins(crate::observability::ObservabilityPlugin)
        .add_systems(
            crate::event_loop::EventLoopPreUpdate,
            raw_packet_observer.in_set(crate::event_loop::EventLoopSet::RawPacketObservers),
        )
        .add_systems(
            crate::event_loop::EventLoopUpdate,
            event_loop_update_domain_consumer
                .in_set(crate::event_loop::EventLoopSet::DomainConsumers),
        );
    app
}

fn raw_packet_observer() {}

fn event_loop_update_domain_consumer() {}

fn app_without_plugin<P>() -> App
where
    P: Plugin,
{
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<crate::network::NetworkPlugin>()
            .disable::<crate::log::LogPlugin>()
            .disable::<P>(),
    );
    app
}

fn schedule_graph(app: &App, schedule_label: &str) -> String {
    schedule_graph_option(app, schedule_label)
        .unwrap_or_else(|| panic!("schedule {schedule_label} is not installed"))
}

fn schedule_graph_option(app: &App, schedule_label: &str) -> Option<String> {
    let schedules = app.world().resource::<Schedules>();
    let (_, schedule) = schedules
        .iter()
        .find(|(label, _)| format!("{label:?}") == schedule_label)?;

    Some(bevy_mod_debugdump::schedule_graph::schedule_graph_dot(
        schedule,
        app.world(),
        &bevy_mod_debugdump::schedule_graph::Settings {
            ambiguity_enable: false,
            ..Default::default()
        },
    ))
}

fn assert_schedule_contains_sets(graph: &str, expected_sets: &[&str]) {
    for expected_set in expected_sets {
        assert!(
            graph.contains(expected_set),
            "schedule graph is missing set {expected_set}"
        );
    }
}

fn assert_schedule_omits_sets(graph: &str, unexpected_sets: &[&str]) {
    for unexpected_set in unexpected_sets {
        assert!(
            !graph.contains(unexpected_set),
            "schedule graph unexpectedly contains set {unexpected_set}"
        );
    }
}

fn assert_schedule_contains_systems(graph: &str, expected_systems: &[&str]) {
    for expected_system in expected_systems {
        assert!(
            graph.contains(expected_system),
            "schedule graph is missing system {expected_system}"
        );
    }
}

fn assert_schedule_omits_systems(graph: &str, unexpected_systems: &[&str]) {
    for unexpected_system in unexpected_systems {
        assert!(
            !graph.contains(unexpected_system),
            "schedule graph unexpectedly contains system {unexpected_system}"
        );
    }
}
