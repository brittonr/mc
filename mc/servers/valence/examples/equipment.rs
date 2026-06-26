#![allow(clippy::type_complexity)]

const SPAWN_Y: i32 = 64;

use rand::Rng;
use valence::entity::armor_stand::ArmorStandEntityBundle;
use valence::entity::zombie::ZombieEntityBundle;
use valence::prelude::*;
use valence_equipment::{EquipmentInteractionBroadcast, EquipmentInventorySync};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (despawn_disconnected_clients,))
        .add_systems(
            Update,
            (
                init_clients,
                despawn_disconnected_clients,
                randomize_equipment.run_if(once_per_second()),
            ),
        )
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

    for z in -25..25 {
        for x in -25..25 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
        }
    }

    let layer_id = commands.spawn(layer).id();

    commands.spawn(ZombieEntityBundle {
        position: Position::new(DVec3::new(0.0, f64::from(SPAWN_Y) + 1.0, 0.0)),
        layer: EntityLayerId(layer_id),
        ..Default::default()
    });

    commands.spawn(ArmorStandEntityBundle {
        position: Position::new(DVec3::new(1.0, f64::from(SPAWN_Y) + 1.0, 0.0)),
        layer: EntityLayerId(layer_id),
        ..Default::default()
    });
}

fn init_clients(
    mut commands: Commands,
    mut clients: Query<
        (
            Entity,
            &mut Position,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut GameMode,
            &mut Inventory,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        player,
        mut pos,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut game_mode,
        mut inv,
    ) in &mut clients
    {
        let layer = layers.single();

        pos.0 = [0.0, f64::from(SPAWN_Y) + 1.0, 0.0].into();
        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        *game_mode = GameMode::Survival;
        inv.set_slot(36, ItemStack::new(ItemKind::Bow, 1, None));
        inv.set_slot(37, ItemStack::new(ItemKind::Crossbow, 1, None));
        inv.set_slot(38, ItemStack::new(ItemKind::GoldenApple, 1, None));
        inv.set_slot(44, ItemStack::new(ItemKind::Arrow, 1, None));
        inv.set_slot(45, ItemStack::new(ItemKind::FireworkRocket, 1, None));

        commands
            .entity(player)
            .insert((EquipmentInventorySync, EquipmentInteractionBroadcast));
    }
}

fn randomize_equipment(mut query: Query<&mut Equipment, Without<Client>>) {
    for mut equipment in &mut query {
        equipment.clear();

        let (slot, item_stack) = match rand::thread_rng().gen_range(0..=5) {
            0 => (
                Equipment::MAIN_HAND_IDX,
                ItemStack::new(ItemKind::DiamondSword, 1, None),
            ),
            1 => (
                Equipment::OFF_HAND_IDX,
                ItemStack::new(ItemKind::Shield, 1, None),
            ),
            2 => (
                Equipment::FEET_IDX,
                ItemStack::new(ItemKind::DiamondBoots, 1, None),
            ),
            3 => (
                Equipment::LEGS_IDX,
                ItemStack::new(ItemKind::DiamondLeggings, 1, None),
            ),
            4 => (
                Equipment::CHEST_IDX,
                ItemStack::new(ItemKind::DiamondChestplate, 1, None),
            ),
            5 => (
                Equipment::HEAD_IDX,
                ItemStack::new(ItemKind::DiamondHelmet, 1, None),
            ),
            _ => unreachable!(),
        };

        equipment.set_slot(slot, item_stack);
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;

    const EQUIPMENT_RANDOMIZE_TICK_RATE_VALUE: u32 = 20;
    const EQUIPMENT_RANDOMIZE_TICK_RATE: NonZeroU32 =
        match NonZeroU32::new(EQUIPMENT_RANDOMIZE_TICK_RATE_VALUE) {
            Some(tick_rate) => tick_rate,
            None => unreachable!(),
        };

    #[test]
    fn equipment_cadence_preserves_previous_one_second_due_ticks() {
        let due_tick = i64::from(EQUIPMENT_RANDOMIZE_TICK_RATE_VALUE);
        let not_due_tick = due_tick + 1;
        let cadence = TickCadence::from_tick_rate(EQUIPMENT_RANDOMIZE_TICK_RATE);

        assert!(cadence.is_due(due_tick));
        assert!(!cadence.is_due(not_due_tick));
    }
}
