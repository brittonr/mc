use bevy_app::{App, Plugin, PluginGroup};
use bevy_ecs::event::Events;
use bevy_ecs::schedule::Schedules;

use crate::DefaultPlugins;

const PRE_UPDATE_LABEL: &str = "PreUpdate";
const POST_UPDATE_LABEL: &str = "PostUpdate";
const EVENT_LOOP_PRE_UPDATE_LABEL: &str = "EventLoopPreUpdate";

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
const SCOREBOARD_POST_UPDATE_SETS: &[&str] = &["ScoreboardSet"];
const WEATHER_POST_UPDATE_SETS: &[&str] = &["WeatherClientUpdateSet", "WeatherLayerUpdateSet"];
const WORLD_BORDER_POST_UPDATE_SETS: &[&str] = &["UpdateWorldBorderSet"];
const BOSS_BAR_POST_UPDATE_SETS: &[&str] = &["BossBarUpdateSet"];

#[test]
fn default_core_plugins_expose_selected_ordering_sets() {
    let app = app_with_default_core_plugins();

    let pre_update = schedule_graph(&app, PRE_UPDATE_LABEL);
    assert_schedule_contains_sets(&pre_update, ADVANCEMENT_PRE_UPDATE_SETS);
    assert_schedule_contains_sets(&pre_update, EQUIPMENT_PRE_UPDATE_SETS);

    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);
    assert_schedule_contains_sets(&event_loop_pre_update, COMMAND_EVENT_LOOP_PRE_UPDATE_SETS);

    let post_update = schedule_graph(&app, POST_UPDATE_LABEL);
    assert_schedule_contains_sets(&post_update, ADVANCEMENT_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, EQUIPMENT_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, SCOREBOARD_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, WEATHER_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, WORLD_BORDER_POST_UPDATE_SETS);
    assert_schedule_contains_sets(&post_update, BOSS_BAR_POST_UPDATE_SETS);

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
}

#[test]
fn disabled_command_plugin_omits_command_schedule_contract() {
    let app = app_without_plugin::<crate::command::manager::CommandPlugin>();
    let event_loop_pre_update = schedule_graph(&app, EVENT_LOOP_PRE_UPDATE_LABEL);

    assert_schedule_omits_sets(&event_loop_pre_update, COMMAND_EVENT_LOOP_PRE_UPDATE_SETS);
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
    let schedules = app.world().resource::<Schedules>();
    let Some((_, schedule)) = schedules
        .iter()
        .find(|(label, _)| format!("{label:?}") == schedule_label)
    else {
        panic!("schedule {schedule_label} is not installed");
    };

    bevy_mod_debugdump::schedule_graph::schedule_graph_dot(
        schedule,
        app.world(),
        &bevy_mod_debugdump::schedule_graph::Settings {
            ambiguity_enable: false,
            ..Default::default()
        },
    )
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
