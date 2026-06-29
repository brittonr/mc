#![allow(clippy::type_complexity)]

//! Demonstrates command API usage while keeping app setup, command definitions,
//! handler decisions, fixture helpers, and Bevy side effects in focused sections.

use std::ops::DerefMut;

use bevy_ecs::prelude::SystemSet;
use command::graph::CommandGraphBuilder;
use command::handler::CommandResultEvent;
use command::parsers::entity_selector::{EntitySelector, EntitySelectors};
use command::parsers::{CommandArg, GreedyString, QuotableString};
use command::scopes::CommandScopes;
use command::{parsers, AddCommand, Command, CommandScopeRegistry, ModifierValue};
use command_macros::Command;
use parsers::{Vec2 as Vec2Parser, Vec3 as Vec3Parser};
use rand::prelude::IteratorRandom;
use valence::entity::living::LivingEntity;
use valence::prelude::*;
use valence::*;
use valence_server::op_level::OpLevel;

const SPAWN_Y: i32 = 64;
const SPAWN_X: f64 = 0.0;
const SPAWN_Z: f64 = 0.0;
const CLIENT_SPAWN_Y_OFFSET: f64 = 1.0;
const CHUNK_RADIUS: i32 = 5;
const GROUND_RADIUS: i32 = 25;
const REQUESTED_ADMIN_OP_LEVEL: u8 = 4;
const EXAMPLE_DIMENSION_NAME: &str = "pooland";

pub fn main() {
    app_setup::run();
}

/// App setup shell: plugin installation, schedule ordering, and system wiring.
mod app_setup {
    use super::*;

    #[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub(crate) enum CommandExamplePhase {
        Input,
        WorldMutation,
        Cleanup,
    }

    #[derive(Resource, Clone, Copy, Debug, PartialEq)]
    pub(crate) struct CommandExamplePluginContract {
        pub(crate) update_phase_order: &'static [CommandExamplePhase],
    }

    pub(crate) const COMMAND_EXAMPLE_PHASE_ORDER: &[CommandExamplePhase] = &[
        CommandExamplePhase::Input,
        CommandExamplePhase::WorldMutation,
        CommandExamplePhase::Cleanup,
    ];

    pub(crate) struct CommandExamplePlugin;

    impl Plugin for CommandExamplePlugin {
        fn build(&self, app: &mut App) {
            let contract = CommandExamplePluginContract {
                update_phase_order: COMMAND_EXAMPLE_PHASE_ORDER,
            };

            app.insert_resource(contract).configure_sets(
                Update,
                (
                    CommandExamplePhase::Input,
                    CommandExamplePhase::WorldMutation,
                    CommandExamplePhase::Cleanup,
                )
                    .chain(),
            );

            super::command_definitions::register_commands(app);
            super::world_setup::register_systems(app);
            super::handler_shell::register_systems(app);
        }
    }

    pub(crate) fn run() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(CommandExamplePlugin)
            .run();
    }
}

/// Command definitions: derive-macro examples plus one manual graph example.
mod command_definitions {
    use super::*;

    #[cfg(test)]
    pub(crate) const DOCUMENTED_COMMAND_ROOTS: &[&str] = &[
        "teleport", "tp", "gamemode", "gm", "struct", "test", "t", "complex",
    ];

    pub(crate) fn register_commands(app: &mut App) {
        app.add_command::<TestCommand>()
            .add_command::<TeleportCommand>()
            .add_command::<GamemodeCommand>()
            .add_command::<ComplexRedirectionCommand>()
            .add_command::<StructCommand>();
    }

    #[cfg(test)]
    pub(crate) fn is_documented_command_root(root: &str) -> bool {
        DOCUMENTED_COMMAND_ROOTS.contains(&root)
    }

    #[derive(Command, Debug, Clone)]
    #[paths("teleport", "tp")]
    #[scopes("valence.command.teleport")]
    pub(crate) enum TeleportCommand {
        #[paths = "{location}"]
        ExecutorToLocation { location: Vec3Parser },
        #[paths = "{target}"]
        ExecutorToTarget { target: EntitySelector },
        #[paths = "{from} {to}"]
        TargetToTarget {
            from: EntitySelector,
            to: EntitySelector,
        },
        #[paths = "{target} {location}"]
        TargetToLocation {
            target: EntitySelector,
            location: Vec3Parser,
        },
    }

    #[derive(Command, Debug, Clone)]
    #[paths("gamemode", "gm")]
    #[scopes("valence.command.gamemode")]
    pub(crate) enum GamemodeCommand {
        #[paths("survival {target?}", "{/} gms {target?}")]
        Survival { target: Option<EntitySelector> },
        #[paths("creative {target?}", "{/} gmc {target?}")]
        Creative { target: Option<EntitySelector> },
        #[paths("adventure {target?}", "{/} gma {target?}")]
        Adventure { target: Option<EntitySelector> },
        #[paths("spectator {target?}", "{/} gmspec {target?}")]
        Spectator { target: Option<EntitySelector> },
    }

    #[derive(Command, Debug, Clone)]
    #[paths("struct {gamemode} {target?}")]
    #[scopes("valence.command.gamemode")]
    #[allow(dead_code)]
    pub(crate) struct StructCommand {
        gamemode: GameMode,
        target: Option<EntitySelector>,
    }

    #[derive(Command, Debug, Clone)]
    #[paths("test", "t")]
    #[scopes("valence.command.test")]
    #[allow(dead_code)]
    pub(crate) enum TestCommand {
        // Three literals with an arg each.
        #[paths("a {a} b {b} c {c}", "{a} {b} {c}")]
        A { a: String, b: i32, c: f32 },
        // Two literals with an arg last being optional. Because of the greedy string before the
        // end this is technically unreachable.
        #[paths = "a {a} {b} b {c?}"]
        B {
            a: Vec3Parser,
            b: GreedyString,
            c: Option<String>,
        },
        // Greedy string optional arg.
        #[paths = "a {a} b {b?}"]
        C { a: String, b: Option<GreedyString> },
        // Greedy string required arg.
        #[paths = "a {a} b {b}"]
        D { a: String, b: GreedyString },
        // Five optional args and an ending greedyString.
        #[paths("options {a?} {b?} {c?} {d?} {e?}", "options {b?} {a?} {d?} {c?} {e?}")]
        E {
            a: Option<i32>,
            b: Option<QuotableString>,
            c: Option<Vec2Parser>,
            d: Option<Vec3Parser>,
            e: Option<GreedyString>,
        },
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub(crate) enum ComplexRedirectionCommand {
        A(Vec3Parser),
        B,
        C(Vec2Parser),
        D,
        E(Vec3Parser),
    }

    impl Command for ComplexRedirectionCommand {
        fn assemble_graph(graph: &mut CommandGraphBuilder<Self>)
        where
            Self: Sized,
        {
            let root = graph.root().id();

            let command_root = graph
                .literal("complex")
                .with_scopes(vec!["valence.command.complex"])
                .id();
            let a = graph.literal("a").id();

            graph
                .at(a)
                .argument("a")
                .with_parser::<Vec3Parser>()
                .with_executable(|input| {
                    ComplexRedirectionCommand::A(Vec3Parser::parse_arg(input).unwrap())
                });

            let b = graph.literal("b").id();

            graph
                .at(b)
                .with_executable(|_| ComplexRedirectionCommand::B);
            graph.at(b).redirect_to(root);

            let c = graph.literal("c").id();

            graph
                .at(c)
                .argument("c")
                .with_parser::<Vec2Parser>()
                .with_executable(|input| {
                    ComplexRedirectionCommand::C(Vec2Parser::parse_arg(input).unwrap())
                });

            let d = graph
                .at(command_root)
                .literal("d")
                .with_modifier(|_, modifiers| {
                    let entry = modifiers.entry("d_pass_count".into()).or_insert(0.into());
                    if let ModifierValue::I32(i) = entry {
                        *i += 1;
                    }
                })
                .id();

            graph
                .at(d)
                .with_executable(|_| ComplexRedirectionCommand::D);
            graph.at(d).redirect_to(command_root);

            let e = graph.literal("e").id();

            graph
                .at(e)
                .argument("e")
                .with_parser::<Vec3Parser>()
                .with_executable(|input| {
                    ComplexRedirectionCommand::E(Vec3Parser::parse_arg(input).unwrap())
                });
        }
    }
}

/// Pure handler decisions that can be tested without a Bevy app or ECS queries.
mod handler_core {
    use super::*;
    use command_definitions::{GamemodeCommand, TeleportCommand, TestCommand};

    #[derive(Debug, Clone)]
    pub(crate) struct TeleportPlan {
        pub(crate) targets: TeleportTargetPlan,
        pub(crate) destination: TeleportDestinationPlan,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum TeleportTargetPlan {
        Executor,
        Selector(EntitySelector),
    }

    #[derive(Debug, Clone)]
    pub(crate) enum TeleportDestinationPlan {
        Location(Vec3Parser),
        FirstSelectorTarget(EntitySelector),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct GamemodePlan {
        pub(crate) game_mode: GameMode,
        pub(crate) target: GamemodeTargetPlan,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum GamemodeTargetPlan {
        Executor,
        AllClients,
        SinglePlayer,
        NearestPlayer,
        RandomPlayer,
        ComplexUnsupported,
    }

    pub(crate) fn teleport_plan(command: &TeleportCommand) -> TeleportPlan {
        match command {
            TeleportCommand::ExecutorToLocation { location } => TeleportPlan {
                targets: TeleportTargetPlan::Executor,
                destination: TeleportDestinationPlan::Location(*location),
            },
            TeleportCommand::ExecutorToTarget { target } => TeleportPlan {
                targets: TeleportTargetPlan::Executor,
                destination: TeleportDestinationPlan::FirstSelectorTarget(target.clone()),
            },
            TeleportCommand::TargetToTarget { from, to } => TeleportPlan {
                targets: TeleportTargetPlan::Selector(from.clone()),
                destination: TeleportDestinationPlan::FirstSelectorTarget(to.clone()),
            },
            TeleportCommand::TargetToLocation { target, location } => TeleportPlan {
                targets: TeleportTargetPlan::Selector(target.clone()),
                destination: TeleportDestinationPlan::Location(*location),
            },
        }
    }

    pub(crate) fn gamemode_plan(command: &GamemodeCommand) -> GamemodePlan {
        let game_mode = gamemode_for_command(command);
        let target = gamemode_target_for_command(command);

        GamemodePlan { game_mode, target }
    }

    pub(crate) fn test_command_message(result: &TestCommand) -> String {
        format!("Test command executed with data:\n {result:#?}")
    }

    pub(crate) fn complex_command_message(
        result: &command_definitions::ComplexRedirectionCommand,
        modifiers: &std::collections::HashMap<ModifierValue, ModifierValue>,
    ) -> String {
        format!(
            "complex command executed with data:\n {result:#?}\n and with the modifiers:\n \
             {modifiers:#?}"
        )
    }

    pub(crate) fn struct_command_message(result: &command_definitions::StructCommand) -> String {
        format!("Struct command executed with data:\n {result:#?}")
    }

    pub(crate) fn gamemode_message(
        message_kind: GamemodeMessageKind,
        result: &GamemodeCommand,
    ) -> String {
        match message_kind {
            GamemodeMessageKind::SelfPlayer => {
                format!("Gamemode command executor -> self executed with data:\n {result:#?}")
            }
            GamemodeMessageKind::AllEntities => format!(
                "Gamemode command executor -> all entities executed with data:\n {result:#?}"
            ),
            GamemodeMessageKind::SinglePlayer => format!(
                "Gamemode command executor -> single player executed with data:\n {result:#?}"
            ),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum GamemodeMessageKind {
        SelfPlayer,
        AllEntities,
        SinglePlayer,
    }

    fn gamemode_for_command(command: &GamemodeCommand) -> GameMode {
        match command {
            GamemodeCommand::Survival { .. } => GameMode::Survival,
            GamemodeCommand::Creative { .. } => GameMode::Creative,
            GamemodeCommand::Adventure { .. } => GameMode::Adventure,
            GamemodeCommand::Spectator { .. } => GameMode::Spectator,
        }
    }

    fn gamemode_target_for_command(command: &GamemodeCommand) -> GamemodeTargetPlan {
        let selector = match command {
            GamemodeCommand::Survival { target }
            | GamemodeCommand::Creative { target }
            | GamemodeCommand::Adventure { target }
            | GamemodeCommand::Spectator { target } => target,
        };

        match selector {
            None => GamemodeTargetPlan::Executor,
            Some(EntitySelector::SimpleSelector(selector)) => match selector {
                EntitySelectors::AllEntities | EntitySelectors::AllPlayers => {
                    GamemodeTargetPlan::AllClients
                }
                EntitySelectors::SinglePlayer(_) => GamemodeTargetPlan::SinglePlayer,
                EntitySelectors::SelfPlayer => GamemodeTargetPlan::Executor,
                EntitySelectors::NearestPlayer => GamemodeTargetPlan::NearestPlayer,
                EntitySelectors::RandomPlayer => GamemodeTargetPlan::RandomPlayer,
            },
            Some(EntitySelector::ComplexSelector(_, _)) => GamemodeTargetPlan::ComplexUnsupported,
        }
    }
}

/// Imperative handler shells: ECS queries, mutation, packet side effects, and logging.
mod handler_shell {
    use super::*;
    use command_definitions::{
        ComplexRedirectionCommand, GamemodeCommand, StructCommand, TeleportCommand, TestCommand,
    };
    use handler_core::{
        GamemodeMessageKind, GamemodeTargetPlan, TeleportDestinationPlan, TeleportTargetPlan,
    };

    #[derive(Debug)]
    enum TeleportDestination {
        Location(Vec3Parser),
        Target(Option<Entity>),
    }

    pub(crate) fn register_systems(app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_test_command,
                handle_teleport_command,
                handle_complex_command,
                handle_gamemode_command,
                handle_struct_command,
            )
                .in_set(app_setup::CommandExamplePhase::WorldMutation),
        );
    }

    fn handle_teleport_command(
        mut events: EventReader<CommandResultEvent<TeleportCommand>>,
        living_entities: Query<Entity, With<LivingEntity>>,
        mut clients: Query<(Entity, &mut Client)>,
        entity_layers: Query<&EntityLayerId>,
        mut positions: Query<&mut Position>,
        usernames: Query<(Entity, &Username)>,
    ) {
        for event in events.read() {
            let plan = handler_core::teleport_plan(&event.result);
            let targets = match &plan.targets {
                TeleportTargetPlan::Executor => vec![event.executor],
                TeleportTargetPlan::Selector(target) => find_targets(
                    &living_entities,
                    &mut clients,
                    &positions,
                    &entity_layers,
                    &usernames,
                    event,
                    target,
                ),
            };
            let destination = match &plan.destination {
                TeleportDestinationPlan::Location(location) => {
                    TeleportDestination::Location(*location)
                }
                TeleportDestinationPlan::FirstSelectorTarget(target) => {
                    TeleportDestination::Target(
                        find_targets(
                            &living_entities,
                            &mut clients,
                            &positions,
                            &entity_layers,
                            &usernames,
                            event,
                            target,
                        )
                        .first()
                        .copied(),
                    )
                }
            };

            println!("executing teleport command {targets:#?} -> {destination:#?}");
            match destination {
                TeleportDestination::Location(location) => {
                    for target in targets {
                        let mut pos = positions.get_mut(target).unwrap();
                        pos.0.x = f64::from(location.x.get(pos.0.x as f32));
                        pos.0.y = f64::from(location.y.get(pos.0.y as f32));
                        pos.0.z = f64::from(location.z.get(pos.0.z as f32));
                    }
                }
                TeleportDestination::Target(target) => {
                    let target = target.unwrap();
                    let target_pos = **positions.get(target).unwrap();
                    for target in targets {
                        let mut position = positions.get_mut(target).unwrap();
                        position.0 = target_pos;
                    }
                }
            }
        }
    }

    fn find_targets(
        living_entities: &Query<Entity, With<LivingEntity>>,
        clients: &mut Query<(Entity, &mut Client)>,
        positions: &Query<&mut Position>,
        entity_layers: &Query<&EntityLayerId>,
        usernames: &Query<(Entity, &Username)>,
        event: &CommandResultEvent<TeleportCommand>,
        target: &EntitySelector,
    ) -> Vec<Entity> {
        match target {
            EntitySelector::SimpleSelector(selector) => match selector {
                EntitySelectors::AllEntities => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    living_entities
                        .iter()
                        .filter(|entity| {
                            let entity_layer = entity_layers.get(*entity).unwrap();
                            entity_layer.0 == executor_entity_layer.0
                        })
                        .collect()
                }
                EntitySelectors::SinglePlayer(name) => {
                    let target = usernames.iter().find(|(_, username)| username.0 == *name);
                    match target {
                        None => {
                            let client = &mut clients.get_mut(event.executor).unwrap().1;
                            client.send_chat_message(format!("Could not find target: {name}"));
                            vec![]
                        }
                        Some(target_entity) => {
                            vec![target_entity.0]
                        }
                    }
                }
                EntitySelectors::AllPlayers => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    clients
                        .iter_mut()
                        .filter_map(|(entity, ..)| {
                            let entity_layer = entity_layers.get(entity).unwrap();
                            if entity_layer.0 == executor_entity_layer.0 {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .collect()
                }
                EntitySelectors::SelfPlayer => {
                    vec![event.executor]
                }
                EntitySelectors::NearestPlayer => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    let executor_pos = positions.get(event.executor).unwrap();
                    let target = clients
                        .iter_mut()
                        .filter(|(entity, ..)| {
                            *entity_layers.get(*entity).unwrap() == executor_entity_layer
                        })
                        .filter(|(target, ..)| *target != event.executor)
                        .map(|(target, ..)| target)
                        .min_by(|target, target2| {
                            let target_pos = positions.get(*target).unwrap();
                            let target2_pos = positions.get(*target2).unwrap();
                            let target_dist = target_pos.distance(**executor_pos);
                            let target2_dist = target2_pos.distance(**executor_pos);
                            target_dist.partial_cmp(&target2_dist).unwrap()
                        });
                    match target {
                        None => {
                            let mut client = clients.get_mut(event.executor).unwrap().1;
                            client.send_chat_message("Could not find target".to_owned());
                            vec![]
                        }
                        Some(target_entity) => {
                            vec![target_entity]
                        }
                    }
                }
                EntitySelectors::RandomPlayer => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    let target = clients
                        .iter_mut()
                        .filter(|(entity, ..)| {
                            *entity_layers.get(*entity).unwrap() == executor_entity_layer
                        })
                        .choose(&mut rand::thread_rng())
                        .map(|(target, ..)| target);
                    match target {
                        None => {
                            let mut client = clients.get_mut(event.executor).unwrap().1;
                            client.send_chat_message("Could not find target".to_owned());
                            vec![]
                        }
                        Some(target_entity) => {
                            vec![target_entity]
                        }
                    }
                }
            },
            EntitySelector::ComplexSelector(_, _) => {
                let mut client = clients.get_mut(event.executor).unwrap().1;
                client.send_chat_message("complex selector not implemented".to_owned());
                vec![]
            }
        }
    }

    fn handle_test_command(
        mut events: EventReader<CommandResultEvent<TestCommand>>,
        mut clients: Query<&mut Client>,
    ) {
        for event in events.read() {
            let client = &mut clients.get_mut(event.executor).unwrap();
            client.send_chat_message(handler_core::test_command_message(&event.result));
        }
    }

    fn handle_complex_command(
        mut events: EventReader<CommandResultEvent<ComplexRedirectionCommand>>,
        mut clients: Query<&mut Client>,
    ) {
        for event in events.read() {
            let client = &mut clients.get_mut(event.executor).unwrap();
            client.send_chat_message(handler_core::complex_command_message(
                &event.result,
                &event.modifiers,
            ));
        }
    }

    fn handle_struct_command(
        mut events: EventReader<CommandResultEvent<StructCommand>>,
        mut clients: Query<&mut Client>,
    ) {
        for event in events.read() {
            let client = &mut clients.get_mut(event.executor).unwrap();
            client.send_chat_message(handler_core::struct_command_message(&event.result));
        }
    }

    fn handle_gamemode_command(
        mut events: EventReader<CommandResultEvent<GamemodeCommand>>,
        mut clients: Query<(&mut Client, &mut GameMode, &Username, Entity)>,
        positions: Query<&Position>,
    ) {
        for event in events.read() {
            let plan = handler_core::gamemode_plan(&event.result);

            match plan.target {
                GamemodeTargetPlan::Executor => {
                    let (mut client, mut game_mode, ..) = clients.get_mut(event.executor).unwrap();
                    *game_mode = plan.game_mode;
                    client.send_chat_message(handler_core::gamemode_message(
                        GamemodeMessageKind::SelfPlayer,
                        &event.result,
                    ));
                }
                GamemodeTargetPlan::AllClients => {
                    for (mut client, mut game_mode, ..) in &mut clients.iter_mut() {
                        *game_mode = plan.game_mode;
                        client.send_chat_message(handler_core::gamemode_message(
                            GamemodeMessageKind::AllEntities,
                            &event.result,
                        ));
                    }
                }
                GamemodeTargetPlan::SinglePlayer => {
                    let target_name = gamemode_target_name(&event.result).unwrap();
                    let target = clients
                        .iter_mut()
                        .find(|(.., username, _)| username.0 == target_name)
                        .map(|(.., target)| target);

                    match target {
                        None => {
                            let client = &mut clients.get_mut(event.executor).unwrap().0;
                            client
                                .send_chat_message(format!("Could not find target: {target_name}"));
                        }
                        Some(target) => {
                            let mut game_mode = clients.get_mut(target).unwrap().1;
                            *game_mode = plan.game_mode;

                            let client = &mut clients.get_mut(event.executor).unwrap().0;
                            client.send_chat_message(handler_core::gamemode_message(
                                GamemodeMessageKind::SinglePlayer,
                                &event.result,
                            ));
                        }
                    }
                }
                GamemodeTargetPlan::NearestPlayer => {
                    let executor_pos = positions.get(event.executor).unwrap();
                    let target = clients
                        .iter_mut()
                        .filter(|(.., target)| *target != event.executor)
                        .min_by(|(.., target), (.., target2)| {
                            let target_pos = positions.get(*target).unwrap();
                            let target2_pos = positions.get(*target2).unwrap();
                            let target_dist = target_pos.distance(**executor_pos);
                            let target2_dist = target2_pos.distance(**executor_pos);
                            target_dist.partial_cmp(&target2_dist).unwrap()
                        })
                        .map(|(.., target)| target);

                    match target {
                        None => {
                            let client = &mut clients.get_mut(event.executor).unwrap().0;
                            client.send_chat_message("Could not find target".to_owned());
                        }
                        Some(target) => {
                            let mut game_mode = clients.get_mut(target).unwrap().1;
                            *game_mode = plan.game_mode;

                            let client = &mut clients.get_mut(event.executor).unwrap().0;
                            client.send_chat_message(handler_core::gamemode_message(
                                GamemodeMessageKind::SinglePlayer,
                                &event.result,
                            ));
                        }
                    }
                }
                GamemodeTargetPlan::RandomPlayer => {
                    let target = clients
                        .iter_mut()
                        .choose(&mut rand::thread_rng())
                        .map(|(.., target)| target);

                    match target {
                        None => {
                            let client = &mut clients.get_mut(event.executor).unwrap().0;
                            client.send_chat_message("Could not find target".to_owned());
                        }
                        Some(target) => {
                            let mut game_mode = clients.get_mut(target).unwrap().1;
                            *game_mode = plan.game_mode;

                            let client = &mut clients.get_mut(event.executor).unwrap().0;
                            client.send_chat_message(handler_core::gamemode_message(
                                GamemodeMessageKind::SinglePlayer,
                                &event.result,
                            ));
                        }
                    }
                }
                GamemodeTargetPlan::ComplexUnsupported => {
                    let client = &mut clients.get_mut(event.executor).unwrap().0;
                    client
                        .send_chat_message("Complex selectors are not implemented yet".to_owned());
                }
            }
        }
    }

    fn gamemode_target_name(command: &GamemodeCommand) -> Option<&str> {
        let selector = match command {
            GamemodeCommand::Survival { target }
            | GamemodeCommand::Creative { target }
            | GamemodeCommand::Adventure { target }
            | GamemodeCommand::Spectator { target } => target.as_ref(),
        };

        match selector {
            Some(EntitySelector::SimpleSelector(EntitySelectors::SinglePlayer(name))) => Some(name),
            _ => None,
        }
    }
}

/// World setup shell: dimensions, chunk fixtures, command scopes, and client initialization.
mod world_setup {
    use super::*;

    pub(crate) fn register_systems(app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                init_clients.in_set(app_setup::CommandExamplePhase::Input),
            )
            .add_systems(
                Update,
                despawn_disconnected_clients.in_set(app_setup::CommandExamplePhase::Cleanup),
            );
    }

    fn setup(
        mut commands: Commands,
        server: Res<Server>,
        mut dimensions: ResMut<DimensionTypeRegistry>,
        biomes: Res<BiomeRegistry>,
        mut command_scopes: ResMut<CommandScopeRegistry>,
    ) {
        dimensions.deref_mut().insert(
            Ident::new(EXAMPLE_DIMENSION_NAME).unwrap(),
            DimensionType::default(),
        );

        let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

        for z in -CHUNK_RADIUS..CHUNK_RADIUS {
            for x in -CHUNK_RADIUS..CHUNK_RADIUS {
                layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
            }
        }

        for z in -GROUND_RADIUS..GROUND_RADIUS {
            for x in -GROUND_RADIUS..GROUND_RADIUS {
                layer
                    .chunk
                    .set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
            }
        }

        command_scopes.link("valence.admin", "valence.command");

        commands.spawn(layer);
    }

    fn init_clients(
        mut clients: Query<
            (
                &mut EntityLayerId,
                &mut VisibleChunkLayer,
                &mut VisibleEntityLayers,
                &mut CommandScopes,
                &mut Position,
                &mut GameMode,
                &mut OpLevel,
            ),
            Added<Client>,
        >,
        layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
    ) {
        for (
            mut layer_id,
            mut visible_chunk_layer,
            mut visible_entity_layers,
            mut permissions,
            mut pos,
            mut game_mode,
            mut op_level,
        ) in &mut clients
        {
            let layer = layers.single();

            layer_id.0 = layer;
            visible_chunk_layer.0 = layer;
            visible_entity_layers.0.insert(layer);

            pos.0 = [SPAWN_X, f64::from(SPAWN_Y) + CLIENT_SPAWN_Y_OFFSET, SPAWN_Z].into();
            *game_mode = GameMode::Creative;
            op_level.set(REQUESTED_ADMIN_OP_LEVEL);

            permissions.add("valence.admin");
        }
    }
}

#[cfg(test)]
mod fixture_helpers {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct CommandExampleFixture {
        pub(crate) handler_enabled: bool,
        pub(crate) executor: Option<Entity>,
        pub(crate) target: Option<Entity>,
        pub(crate) target_has_position: bool,
        pub(crate) username_valid: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum FixtureError {
        HandlerDisabled,
        MissingExecutor,
        MissingTarget,
        MissingTargetPosition,
        MalformedUsername,
    }

    pub(crate) fn validate_command_fixture(
        fixture: CommandExampleFixture,
    ) -> Result<(), FixtureError> {
        if !fixture.handler_enabled {
            return Err(FixtureError::HandlerDisabled);
        }
        if fixture.executor.is_none() {
            return Err(FixtureError::MissingExecutor);
        }
        if fixture.target.is_none() {
            return Err(FixtureError::MissingTarget);
        }
        if !fixture.target_has_position {
            return Err(FixtureError::MissingTargetPosition);
        }
        if !fixture.username_valid {
            return Err(FixtureError::MalformedUsername);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use command::parsers::ParseInput;
    use valence::command::CommandRegistry;
    use valence::protocol::packets::play::command_tree_s2c::{CommandTreeS2c, NodeData};

    use super::*;
    use command_definitions::{
        ComplexRedirectionCommand, GamemodeCommand, TeleportCommand, TestCommand,
    };
    use fixture_helpers::{CommandExampleFixture, FixtureError};
    use handler_core::{GamemodeTargetPlan, TeleportDestinationPlan, TeleportTargetPlan};

    const EXECUTOR_ENTITY_ID: u32 = 1;
    const TARGET_ENTITY_ID: u32 = 2;
    const VALID_LOCATION_INPUT: &str = "1 2 3";
    const INVALID_LOCATION_INPUT: &str = "not_a_location";
    const UNKNOWN_COMMAND_ROOT: &str = "unknown-command";
    const TARGET_USERNAME: &str = "Steve";

    #[test]
    fn command_example_plugin_installs_contract() {
        let mut app = App::new();

        app.add_plugins(app_setup::CommandExamplePlugin);

        let contract = app
            .world()
            .resource::<app_setup::CommandExamplePluginContract>();
        assert_eq!(
            contract.update_phase_order,
            app_setup::COMMAND_EXAMPLE_PHASE_ORDER
        );
    }

    #[test]
    fn disabled_command_example_plugin_installs_no_contract() {
        let app = App::new();

        assert!(!app
            .world()
            .contains_resource::<app_setup::CommandExamplePluginContract>());
    }

    #[test]
    fn documented_command_roots_include_aliases_and_manual_graph() {
        assert!(command_definitions::is_documented_command_root("teleport"));
        assert!(command_definitions::is_documented_command_root("tp"));
        assert!(command_definitions::is_documented_command_root("complex"));
    }

    #[test]
    fn unknown_command_root_is_not_documented() {
        assert!(!command_definitions::is_documented_command_root(
            UNKNOWN_COMMAND_ROOT
        ));
    }

    #[test]
    fn complex_redirection_graph_keeps_documented_literals() {
        let (tree, _parser_count) = graph_tree::<ComplexRedirectionCommand>();
        let literal_names = literal_names(&tree);

        for expected_literal in ["complex", "a", "b", "c", "d", "e"] {
            assert!(literal_names.contains(&expected_literal));
        }
    }

    #[test]
    fn teleport_location_parser_accepts_documented_coordinates() {
        let (_tree, parser_count) = graph_tree::<TeleportCommand>();
        assert!(parser_count > 0);

        let location_parser = graph_parser::<TeleportCommand>("location");
        let mut input = ParseInput::new(VALID_LOCATION_INPUT);

        assert!(location_parser(&mut input));
    }

    #[test]
    fn teleport_location_parser_rejects_bad_argument() {
        let location_parser = graph_parser::<TeleportCommand>("location");
        let mut input = ParseInput::new(INVALID_LOCATION_INPUT);

        assert!(!location_parser(&mut input));
    }

    #[test]
    fn gamemode_core_plans_executor_creative() {
        let command = GamemodeCommand::Creative { target: None };
        let plan = handler_core::gamemode_plan(&command);

        assert_eq!(plan.game_mode, GameMode::Creative);
        assert_eq!(plan.target, GamemodeTargetPlan::Executor);
    }

    #[test]
    fn gamemode_core_plans_named_survival_target() {
        let command = GamemodeCommand::Survival {
            target: Some(single_player_selector(TARGET_USERNAME)),
        };
        let plan = handler_core::gamemode_plan(&command);

        assert_eq!(plan.game_mode, GameMode::Survival);
        assert_eq!(plan.target, GamemodeTargetPlan::SinglePlayer);
    }

    #[test]
    fn gamemode_core_rejects_complex_selector_as_unsupported() {
        let command = GamemodeCommand::Adventure {
            target: Some(EntitySelector::ComplexSelector(
                EntitySelectors::AllPlayers,
                "distance=..5".to_owned(),
            )),
        };
        let plan = handler_core::gamemode_plan(&command);

        assert_eq!(plan.target, GamemodeTargetPlan::ComplexUnsupported);
    }

    #[test]
    fn teleport_core_plans_executor_to_location() {
        let location = parse_location(VALID_LOCATION_INPUT);
        let command = TeleportCommand::ExecutorToLocation { location };
        let plan = handler_core::teleport_plan(&command);

        assert_eq!(plan.targets, TeleportTargetPlan::Executor);
        assert!(matches!(
            plan.destination,
            TeleportDestinationPlan::Location(_)
        ));
    }

    #[test]
    fn teleport_core_plans_target_to_target_selectors() {
        let from = single_player_selector("Alex");
        let to = single_player_selector(TARGET_USERNAME);
        let command = TeleportCommand::TargetToTarget {
            from: from.clone(),
            to: to.clone(),
        };
        let plan = handler_core::teleport_plan(&command);

        assert_eq!(plan.targets, TeleportTargetPlan::Selector(from));
        assert!(matches!(
            plan.destination,
            TeleportDestinationPlan::FirstSelectorTarget(selector) if selector == to
        ));
    }

    #[test]
    fn test_command_message_keeps_documented_debug_text() {
        let command = TestCommand::A {
            a: "left".to_owned(),
            b: 7,
            c: 1.5,
        };
        let message = handler_core::test_command_message(&command);

        assert!(message.contains("Test command executed with data"));
        assert!(message.contains("left"));
    }

    #[test]
    fn valid_fixture_setup_is_accepted() {
        let fixture = valid_fixture();

        fixture_helpers::validate_command_fixture(fixture).unwrap();
    }

    #[test]
    fn fixture_validation_rejects_missing_executor_entity() {
        let fixture = CommandExampleFixture {
            executor: None,
            ..valid_fixture()
        };

        assert_eq!(
            fixture_helpers::validate_command_fixture(fixture),
            Err(FixtureError::MissingExecutor)
        );
    }

    #[test]
    fn fixture_validation_rejects_missing_target_entity() {
        let fixture = CommandExampleFixture {
            target: None,
            ..valid_fixture()
        };

        assert_eq!(
            fixture_helpers::validate_command_fixture(fixture),
            Err(FixtureError::MissingTarget)
        );
    }

    #[test]
    fn fixture_validation_rejects_disabled_handler() {
        let fixture = CommandExampleFixture {
            handler_enabled: false,
            ..valid_fixture()
        };

        assert_eq!(
            fixture_helpers::validate_command_fixture(fixture),
            Err(FixtureError::HandlerDisabled)
        );
    }

    #[test]
    fn fixture_validation_rejects_malformed_target_position() {
        let fixture = CommandExampleFixture {
            target_has_position: false,
            ..valid_fixture()
        };

        assert_eq!(
            fixture_helpers::validate_command_fixture(fixture),
            Err(FixtureError::MissingTargetPosition)
        );
    }

    #[test]
    fn fixture_validation_rejects_malformed_username_state() {
        let fixture = CommandExampleFixture {
            username_valid: false,
            ..valid_fixture()
        };

        assert_eq!(
            fixture_helpers::validate_command_fixture(fixture),
            Err(FixtureError::MalformedUsername)
        );
    }

    fn graph_tree<T: Command>() -> (CommandTreeS2c, usize) {
        let mut registry = CommandRegistry::default();
        let mut executable_map = HashMap::new();
        let mut parser_map = HashMap::new();
        let mut modifier_map = HashMap::new();
        let mut builder = CommandGraphBuilder::<T>::new(
            &mut registry,
            &mut executable_map,
            &mut parser_map,
            &mut modifier_map,
        );

        T::assemble_graph(&mut builder);

        (
            CommandTreeS2c::from(registry.graph.clone()),
            parser_map.len(),
        )
    }

    fn graph_parser<T: Command>(argument_name: &str) -> Box<dyn Fn(&mut ParseInput) -> bool> {
        let mut registry = CommandRegistry::default();
        let mut executable_map = HashMap::new();
        let mut parser_map = HashMap::new();
        let mut modifier_map = HashMap::new();
        let mut builder = CommandGraphBuilder::<T>::new(
            &mut registry,
            &mut executable_map,
            &mut parser_map,
            &mut modifier_map,
        );

        T::assemble_graph(&mut builder);

        let tree = CommandTreeS2c::from(registry.graph.clone());
        let argument_index = tree
            .commands
            .iter()
            .position(|node| {
                matches!(
                    &node.data,
                    NodeData::Argument { name, .. } if name == argument_name
                )
            })
            .unwrap();
        let parser = parser_map
            .into_iter()
            .find_map(|(node, parser)| (node.index() == argument_index).then_some(parser))
            .unwrap();

        Box::new(parser)
    }

    fn literal_names(tree: &CommandTreeS2c) -> Vec<&str> {
        tree.commands
            .iter()
            .filter_map(|node| match &node.data {
                NodeData::Literal { name } => Some(name.as_str()),
                NodeData::Root | NodeData::Argument { .. } => None,
            })
            .collect()
    }

    fn parse_location(input: &str) -> Vec3Parser {
        let mut input = ParseInput::new(input);
        Vec3Parser::parse_arg(&mut input).unwrap()
    }

    fn single_player_selector(name: &str) -> EntitySelector {
        EntitySelector::SimpleSelector(EntitySelectors::SinglePlayer(name.to_owned()))
    }

    fn valid_fixture() -> CommandExampleFixture {
        CommandExampleFixture {
            handler_enabled: true,
            executor: Some(Entity::from_raw(EXECUTOR_ENTITY_ID)),
            target: Some(Entity::from_raw(TARGET_ENTITY_ID)),
            target_has_position: true,
            username_valid: true,
        }
    }
}
