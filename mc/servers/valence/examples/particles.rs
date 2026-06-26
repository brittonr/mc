#![allow(clippy::type_complexity)]

use std::fmt;
use std::num::NonZeroU32;

use valence::prelude::*;

const SPAWN_Y: i32 = 64;
const PARTICLE_CADENCE_TICK_COUNT: u32 = 10;
const PARTICLE_CADENCE: TickCadence =
    TickCadence::new(nonzero_tick_count(PARTICLE_CADENCE_TICK_COUNT));

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                init_clients,
                despawn_disconnected_clients,
                manage_particles.run_if(every_ticks(PARTICLE_CADENCE)),
            ),
        )
        .run();
}

#[derive(Resource)]
struct ParticleVec(Vec<Particle>);

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

    layer.chunk.set_block([0, SPAWN_Y, 0], BlockState::BEDROCK);

    commands.spawn(layer);

    commands.insert_resource(ParticleVec(create_particle_vec()));
}

fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
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
    }
}

fn manage_particles(
    particles: Res<ParticleVec>,
    mut layers: Query<&mut ChunkLayer>,
    mut particle_idx: Local<usize>,
) {
    let particle = &particles.0[*particle_idx];

    *particle_idx = (*particle_idx + 1) % particles.0.len();

    let name = dbg_name(particle);

    let pos = [0.5, f64::from(SPAWN_Y) + 2.0, 5.0];
    let offset = [0.5, 0.5, 0.5];

    let mut layer = layers.single_mut();

    layer.play_particle(particle, true, pos, offset, 0.1, 100);
    layer.set_action_bar(name.bold());
}

const fn nonzero_tick_count(tick_count: u32) -> NonZeroU32 {
    match NonZeroU32::new(tick_count) {
        Some(tick_count) => tick_count,
        None => unreachable!(),
    }
}

fn dbg_name(dbg: &impl fmt::Debug) -> String {
    let string = format!("{dbg:?}");

    string
        .split_once(|ch: char| !ch.is_ascii_alphabetic())
        .map(|(fst, _)| fst.to_owned())
        .unwrap_or(string)
}

fn create_particle_vec() -> Vec<Particle> {
    vec![
        Particle::AmbientEntityEffect,
        Particle::AngryVillager,
        Particle::Block(BlockState::OAK_PLANKS),
        Particle::BlockMarker(BlockState::GOLD_BLOCK),
        Particle::Bubble,
        Particle::Cloud,
        Particle::Crit,
        Particle::DamageIndicator,
        Particle::DragonBreath,
        Particle::DrippingLava,
        Particle::FallingLava,
        Particle::LandingLava,
        Particle::DrippingWater,
        Particle::FallingWater,
        Particle::Dust {
            rgb: Vec3::new(1.0, 1.0, 0.0),
            scale: 2.0,
        },
        Particle::DustColorTransition {
            from_rgb: Vec3::new(1.0, 0.0, 0.0),
            scale: 2.0,
            to_rgb: Vec3::new(0.0, 1.0, 0.0),
        },
        Particle::Effect,
        Particle::ElderGuardian,
        Particle::EnchantedHit,
        Particle::Enchant,
        Particle::EndRod,
        Particle::EntityEffect,
        Particle::ExplosionEmitter,
        Particle::Explosion,
        Particle::SonicBoom,
        Particle::FallingDust(BlockState::RED_SAND),
        Particle::Firework,
        Particle::Fishing,
        Particle::Flame,
        Particle::CherryLeaves,
        Particle::SculkSoul,
        Particle::SculkCharge { roll: 1.0 },
        Particle::SculkChargePop,
        Particle::SoulFireFlame,
        Particle::Soul,
        Particle::Flash,
        Particle::HappyVillager,
        Particle::Composter,
        Particle::Heart,
        Particle::InstantEffect,
        Particle::Item(ItemStack::EMPTY),
        Particle::Item(ItemStack::new(ItemKind::IronPickaxe, 1, None)),
        Particle::VibrationBlock {
            block_pos: [0, SPAWN_Y, 0].into(),
            ticks: 50,
        },
        Particle::VibrationEntity {
            entity_id: 0,
            entity_eye_height: 1.0,
            ticks: 50,
        },
        Particle::ItemSlime,
        Particle::ItemSnowball,
        Particle::LargeSmoke,
        Particle::Lava,
        Particle::Mycelium,
        Particle::Note,
        Particle::Poof,
        Particle::Portal,
        Particle::Rain,
        Particle::Smoke,
        Particle::Sneeze,
        Particle::Spit,
        Particle::SquidInk,
        Particle::SweepAttack,
        Particle::TotemOfUndying,
        Particle::Underwater,
        Particle::Splash,
        Particle::Witch,
        Particle::BubblePop,
        Particle::CurrentDown,
        Particle::BubbleColumnUp,
        Particle::Nautilus,
        Particle::Dolphin,
        Particle::CampfireCosySmoke,
        Particle::CampfireSignalSmoke,
        Particle::DrippingHoney,
        Particle::FallingHoney,
        Particle::LandingHoney,
        Particle::FallingNectar,
        Particle::FallingSporeBlossom,
        Particle::Ash,
        Particle::CrimsonSpore,
        Particle::WarpedSpore,
        Particle::SporeBlossomAir,
        Particle::DrippingObsidianTear,
        Particle::FallingObsidianTear,
        Particle::LandingObsidianTear,
        Particle::ReversePortal,
        Particle::WhiteAsh,
        Particle::SmallFlame,
        Particle::Snowflake,
        Particle::DrippingDripstoneLava,
        Particle::FallingDripstoneLava,
        Particle::DrippingDripstoneWater,
        Particle::FallingDripstoneWater,
        Particle::GlowSquidInk,
        Particle::Glow,
        Particle::WaxOn,
        Particle::WaxOff,
        Particle::ElectricSpark,
        Particle::Scrape,
        Particle::Shriek { delay: 0 },
        Particle::EggCrack,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particle_cadence_preserves_previous_due_ticks() {
        let due_tick = i64::from(PARTICLE_CADENCE_TICK_COUNT);
        let not_due_tick = due_tick + 1;

        assert!(PARTICLE_CADENCE.is_due(due_tick));
        assert!(!PARTICLE_CADENCE.is_due(not_due_tick));
    }
}
