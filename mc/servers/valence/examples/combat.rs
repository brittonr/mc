#![allow(clippy::type_complexity)]

use bevy_ecs::prelude::SystemSet;
use bevy_ecs::query::QueryData;
use rand::Rng;
use valence::entity::EntityStatuses;
use valence::math::Vec3Swizzles;
use valence::prelude::*;
use valence::tick_scheduler::ServerTickScheduler;

const SPAWN_Y: i32 = 64;
const ARENA_RADIUS: i32 = 32;
const ATTACK_COOLDOWN_TICKS: i64 = 10;
const FIRST_COOLDOWN_GENERATION: u64 = 0;
const COOLDOWN_GENERATION_STEP: u64 = 1;

/// Attached to every client.
#[derive(Component, Debug)]
struct CombatState {
    cooldown: Option<AttackCooldown>,
    next_cooldown_generation: u64,
    has_bonus_knockback: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttackCooldown {
    generation: u64,
    expires_at_tick: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttackCooldownExpired {
    victim: Entity,
    generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttackCooldownPlan {
    cooldown: AttackCooldown,
    event: AttackCooldownExpired,
    next_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CombatCooldownError {
    InvalidDueTick,
    GenerationExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CombatCooldownExpiration {
    Applied,
    MissingTarget,
    NoActiveCooldown,
    StaleGeneration,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct CombatGameplayPluginContract {
    update_phase_order: &'static [CombatGameplayPhase],
    event_loop_phase_order: &'static [CombatGameplayPhase],
}

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum CombatGameplayPhase {
    Input,
    RuleEvaluation,
    WorldMutation,
    Presentation,
    Cleanup,
}

const COMBAT_GAMEPLAY_PHASE_ORDER: &[CombatGameplayPhase] = &[
    CombatGameplayPhase::Input,
    CombatGameplayPhase::RuleEvaluation,
    CombatGameplayPhase::WorldMutation,
    CombatGameplayPhase::Presentation,
    CombatGameplayPhase::Cleanup,
];

#[derive(Default)]
struct CombatCooldownPlugin;

struct CombatGameplayPlugin;

impl Default for CombatState {
    fn default() -> Self {
        Self {
            cooldown: None,
            next_cooldown_generation: FIRST_COOLDOWN_GENERATION,
            has_bonus_knockback: false,
        }
    }
}

impl Plugin for CombatCooldownPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ServerTickScheduler<AttackCooldownExpired>>()
            .add_systems(
                Update,
                expire_combat_cooldowns.in_set(CombatGameplayPhase::RuleEvaluation),
            );
    }
}

impl Plugin for CombatGameplayPlugin {
    fn build(&self, app: &mut App) {
        let contract = CombatGameplayPluginContract {
            update_phase_order: COMBAT_GAMEPLAY_PHASE_ORDER,
            event_loop_phase_order: COMBAT_GAMEPLAY_PHASE_ORDER,
        };

        app.insert_resource(contract)
            .configure_sets(
                Update,
                (
                    CombatGameplayPhase::Input,
                    CombatGameplayPhase::RuleEvaluation,
                    CombatGameplayPhase::WorldMutation,
                    CombatGameplayPhase::Presentation,
                    CombatGameplayPhase::Cleanup,
                )
                    .chain(),
            )
            .configure_sets(
                EventLoopUpdate,
                (
                    CombatGameplayPhase::Input,
                    CombatGameplayPhase::RuleEvaluation,
                    CombatGameplayPhase::WorldMutation,
                    CombatGameplayPhase::Presentation,
                    CombatGameplayPhase::Cleanup,
                )
                    .chain(),
            )
            .add_plugins(CombatCooldownPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                EventLoopUpdate,
                handle_combat_events.in_set(CombatGameplayPhase::WorldMutation),
            )
            .add_systems(Update, init_clients.in_set(CombatGameplayPhase::Input))
            .add_systems(
                Update,
                teleport_oob_clients.in_set(CombatGameplayPhase::RuleEvaluation),
            )
            .add_systems(
                Update,
                despawn_disconnected_clients.in_set(CombatGameplayPhase::Cleanup),
            );
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CombatGameplayPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    let mut rng = rand::thread_rng();

    // Create circular arena.
    for z in -ARENA_RADIUS..ARENA_RADIUS {
        for x in -ARENA_RADIUS..ARENA_RADIUS {
            let dist = f64::hypot(f64::from(x), f64::from(z)) / f64::from(ARENA_RADIUS);

            if dist > 1.0 {
                continue;
            }

            let block = if rng.gen::<f64>() < dist {
                BlockState::STONE
            } else {
                BlockState::DEEPSLATE
            };

            for y in 0..SPAWN_Y {
                layer.chunk.set_block([x, y, z], block);
            }
        }
    }

    commands.spawn(layer);
}

fn expire_combat_cooldowns(
    server: Res<Server>,
    mut cooldowns: ResMut<ServerTickScheduler<AttackCooldownExpired>>,
    mut clients: Query<&mut CombatState, With<Client>>,
) {
    let current_tick = server.current_tick();

    for event in cooldowns.drain_due(&current_tick) {
        match clients.get_mut(event.victim) {
            Ok(mut state) => {
                apply_cooldown_expiration(Some(&mut *state), event);
            }
            Err(_) => {
                apply_cooldown_expiration(None, event);
            }
        }
    }
}

fn attack_cooldown_active(state: &CombatState, current_tick: i64) -> bool {
    state
        .cooldown
        .is_some_and(|cooldown| cooldown.expires_at_tick > current_tick)
}

fn plan_attack_cooldown(
    current_tick: i64,
    cooldown_ticks: i64,
    victim: Entity,
    state: &CombatState,
) -> Result<AttackCooldownPlan, CombatCooldownError> {
    let Some(expires_at_tick) = current_tick.checked_add(cooldown_ticks) else {
        return Err(CombatCooldownError::InvalidDueTick);
    };
    if expires_at_tick <= current_tick {
        return Err(CombatCooldownError::InvalidDueTick);
    }
    let Some(next_generation) = state
        .next_cooldown_generation
        .checked_add(COOLDOWN_GENERATION_STEP)
    else {
        return Err(CombatCooldownError::GenerationExhausted);
    };

    let cooldown = AttackCooldown {
        generation: state.next_cooldown_generation,
        expires_at_tick,
    };
    let event = AttackCooldownExpired {
        victim,
        generation: cooldown.generation,
    };

    Ok(AttackCooldownPlan {
        cooldown,
        event,
        next_generation,
    })
}

fn activate_attack_cooldown(state: &mut CombatState, plan: AttackCooldownPlan) {
    state.cooldown = Some(plan.cooldown);
    state.next_cooldown_generation = plan.next_generation;
}

fn apply_cooldown_expiration(
    state: Option<&mut CombatState>,
    event: AttackCooldownExpired,
) -> CombatCooldownExpiration {
    let Some(state) = state else {
        return CombatCooldownExpiration::MissingTarget;
    };

    match state.cooldown {
        Some(cooldown) if cooldown.generation == event.generation => {
            state.cooldown = None;
            CombatCooldownExpiration::Applied
        }
        Some(_) => CombatCooldownExpiration::StaleGeneration,
        None => CombatCooldownExpiration::NoActiveCooldown,
    }
}

fn init_clients(
    mut clients: Query<
        (
            Entity,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
    mut commands: Commands,
) {
    for (
        entity,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, f64::from(SPAWN_Y) + 1.0, 0.0]);
        *game_mode = GameMode::Creative;

        commands.entity(entity).insert(CombatState::default());
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
struct CombatQuery {
    client: &'static mut Client,
    pos: &'static Position,
    state: &'static mut CombatState,
    statuses: &'static mut EntityStatuses,
}

fn handle_combat_events(
    server: Res<Server>,
    mut cooldowns: ResMut<ServerTickScheduler<AttackCooldownExpired>>,
    mut clients: Query<CombatQuery>,
    mut sprinting: EventReader<SprintEvent>,
    mut interact_entity: EventReader<InteractEntityEvent>,
) {
    let current_tick = server.current_tick();

    for &SprintEvent { client, state } in sprinting.read() {
        if let Ok(mut client) = clients.get_mut(client) {
            client.state.has_bonus_knockback = state == SprintState::Start;
        }
    }

    for &InteractEntityEvent {
        client: attacker_client,
        entity: victim_client,
        ..
    } in interact_entity.read()
    {
        let Ok([mut attacker, mut victim]) = clients.get_many_mut([attacker_client, victim_client])
        else {
            // Victim or attacker does not exist, or the attacker is attacking itself.
            continue;
        };

        if attack_cooldown_active(&victim.state, current_tick) {
            // Victim is still on attack cooldown.
            continue;
        }

        let Ok(cooldown_plan) = plan_attack_cooldown(
            current_tick,
            ATTACK_COOLDOWN_TICKS,
            victim_client,
            &victim.state,
        ) else {
            continue;
        };
        if cooldowns
            .schedule(cooldown_plan.cooldown.expires_at_tick, cooldown_plan.event)
            .is_err()
        {
            continue;
        }
        activate_attack_cooldown(&mut victim.state, cooldown_plan);

        let victim_pos = victim.pos.0.xz();
        let attacker_pos = attacker.pos.0.xz();

        let dir = (victim_pos - attacker_pos).normalize().as_vec2();

        let knockback_xz = if attacker.state.has_bonus_knockback {
            18.0
        } else {
            8.0
        };
        let knockback_y = if attacker.state.has_bonus_knockback {
            8.432
        } else {
            6.432
        };

        victim
            .client
            .set_velocity([dir.x * knockback_xz, knockback_y, dir.y * knockback_xz]);

        attacker.state.has_bonus_knockback = false;

        victim.client.trigger_status(EntityStatus::PlayAttackSound);

        victim.statuses.trigger(EntityStatus::PlayAttackSound);
    }
}

fn teleport_oob_clients(mut clients: Query<&mut Position, With<Client>>) {
    for mut pos in &mut clients {
        if pos.0.y < 0.0 {
            pos.set([0.0, f64::from(SPAWN_Y), 0.0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bevy_ecs::schedule::Schedule;

    const TEST_CURRENT_TICK: i64 = 100;
    const BEFORE_DUE_OFFSET_TICKS: i64 = 1;
    const EXPECTED_DUE_TICK: i64 = TEST_CURRENT_TICK + ATTACK_COOLDOWN_TICKS;
    const BEFORE_DUE_TICK: i64 = EXPECTED_DUE_TICK - BEFORE_DUE_OFFSET_TICKS;
    const INVALID_COOLDOWN_TICKS: i64 = 0;

    #[test]
    fn combat_gameplay_plugin_installs_contract_and_cooldown_resource() {
        let mut app = app_with_combat_event_loop_schedule();

        app.add_plugins(CombatGameplayPlugin);

        let contract = app.world().resource::<CombatGameplayPluginContract>();
        assert_eq!(contract.update_phase_order, COMBAT_GAMEPLAY_PHASE_ORDER);
        assert_eq!(contract.event_loop_phase_order, COMBAT_GAMEPLAY_PHASE_ORDER);
        assert!(app
            .world()
            .contains_resource::<ServerTickScheduler<AttackCooldownExpired>>());
    }

    #[test]
    fn disabled_combat_gameplay_plugin_installs_no_contract_or_scheduler() {
        let app = app_with_combat_event_loop_schedule();

        assert!(!app
            .world()
            .contains_resource::<CombatGameplayPluginContract>());
        assert!(!app
            .world()
            .contains_resource::<ServerTickScheduler<AttackCooldownExpired>>());
    }

    #[test]
    fn combat_cooldown_due_work_applies_once() {
        let victim = test_entity();
        let mut state = CombatState::default();
        let plan = plan_attack_cooldown(TEST_CURRENT_TICK, ATTACK_COOLDOWN_TICKS, victim, &state)
            .expect("valid cooldown plan");
        let mut scheduler = ServerTickScheduler::new();
        scheduler
            .schedule(plan.cooldown.expires_at_tick, plan.event)
            .expect("valid scheduled cooldown");
        activate_attack_cooldown(&mut state, plan);

        assert!(attack_cooldown_active(&state, BEFORE_DUE_TICK));
        assert!(scheduler.drain_due(&BEFORE_DUE_TICK).is_empty());

        let due = scheduler.drain_due(&EXPECTED_DUE_TICK);

        assert_eq!(due, vec![plan.event]);
        assert_eq!(
            apply_cooldown_expiration(Some(&mut state), plan.event),
            CombatCooldownExpiration::Applied
        );
        assert!(!attack_cooldown_active(&state, EXPECTED_DUE_TICK));
        assert!(scheduler.drain_due(&EXPECTED_DUE_TICK).is_empty());
    }

    #[test]
    fn cancelled_combat_cooldown_does_not_emit_due_work() {
        let victim = test_entity();
        let mut state = CombatState::default();
        let plan = plan_attack_cooldown(TEST_CURRENT_TICK, ATTACK_COOLDOWN_TICKS, victim, &state)
            .expect("valid cooldown plan");
        let mut scheduler = ServerTickScheduler::new();
        let handle = scheduler
            .schedule(plan.cooldown.expires_at_tick, plan.event)
            .expect("valid scheduled cooldown");
        activate_attack_cooldown(&mut state, plan);

        assert!(scheduler.cancel(handle));
        state.cooldown = None;

        assert!(scheduler.drain_due(&EXPECTED_DUE_TICK).is_empty());
        assert!(!attack_cooldown_active(&state, BEFORE_DUE_TICK));
    }

    #[test]
    fn stale_combat_cooldown_target_fails_closed() {
        let victim = test_entity();
        let state = CombatState::default();
        let plan = plan_attack_cooldown(TEST_CURRENT_TICK, ATTACK_COOLDOWN_TICKS, victim, &state)
            .expect("valid cooldown plan");

        assert_eq!(
            apply_cooldown_expiration(None, plan.event),
            CombatCooldownExpiration::MissingTarget
        );
    }

    #[test]
    fn duplicate_combat_cooldown_event_applies_only_once() {
        let victim = test_entity();
        let mut state = CombatState::default();
        let plan = plan_attack_cooldown(TEST_CURRENT_TICK, ATTACK_COOLDOWN_TICKS, victim, &state)
            .expect("valid cooldown plan");
        activate_attack_cooldown(&mut state, plan);

        assert_eq!(
            apply_cooldown_expiration(Some(&mut state), plan.event),
            CombatCooldownExpiration::Applied
        );
        assert_eq!(
            apply_cooldown_expiration(Some(&mut state), plan.event),
            CombatCooldownExpiration::NoActiveCooldown
        );
    }

    #[test]
    fn invalid_combat_cooldown_tick_is_rejected() {
        let victim = test_entity();
        let state = CombatState::default();

        let error = plan_attack_cooldown(TEST_CURRENT_TICK, INVALID_COOLDOWN_TICKS, victim, &state)
            .expect_err("zero-duration cooldown must be rejected");

        assert_eq!(error, CombatCooldownError::InvalidDueTick);
    }

    #[test]
    fn combat_cooldown_plugin_disabled_leaves_scheduler_absent() {
        let app = App::new();

        assert!(app
            .world()
            .get_resource::<ServerTickScheduler<AttackCooldownExpired>>()
            .is_none());
    }

    fn app_with_combat_event_loop_schedule() -> App {
        let mut app = App::new();
        app.add_schedule(Schedule::new(EventLoopUpdate));
        app
    }

    fn test_entity() -> Entity {
        let mut app = App::new();
        app.world_mut().spawn_empty().id()
    }
}
