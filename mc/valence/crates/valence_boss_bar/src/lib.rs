#![doc = include_str!("../README.md")]

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_server::protocol::WritePacket;
use valence_server::Layer;

mod components;
pub use components::*;
pub use valence_server::protocol::packets::play::{
    BossBarAction, BossBarColor, BossBarDivision, BossBarFlags,
};

type Action<'a> = valence_server::protocol::packets::play::BossBarAction<'a>;
type ChunkView = valence_server::ChunkView;
type Client = valence_server::client::Client;
type EntityLayer = valence_server::EntityLayer;
type EntityLayerId = valence_entity::EntityLayerId;
type OldPosition = valence_entity::OldPosition;
type OldViewDistance = valence_server::client::OldViewDistance;
type OldVisibleEntityLayers = valence_server::client::OldVisibleEntityLayers;
type Packet<'a> = valence_server::protocol::packets::play::BossBarS2c<'a>;
type Position = valence_entity::Position;
type UniqueId = valence_server::UniqueId;
type ViewDistance = valence_server::client::ViewDistance;
type VisibleEntityLayers = valence_server::client::VisibleEntityLayers;

pub struct BossBarPlugin;

impl Plugin for BossBarPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PostUpdate,
            (
                update_boss_bar::<BossBarTitle>,
                update_boss_bar::<BossBarHealth>,
                update_boss_bar::<BossBarStyle>,
                update_boss_bar::<BossBarFlags>,
                update_boss_bar_layer_view,
                update_boss_bar_chunk_view,
                boss_bar_despawn,
            )
                .before(valence_server::layer::UpdateLayersPreClientSet),
        );
    }
}

fn update_boss_bar<T: Component + ToPacketAction>(
    boss_bars_query: Query<(&UniqueId, &T, &EntityLayerId, Option<&Position>), Changed<T>>,
    mut entity_layers_query: Query<&mut EntityLayer>,
) {
    for (id, part, entity_layer_id, position) in boss_bars_query.iter() {
        if let Ok(mut entity_layer) = entity_layers_query.get_mut(entity_layer_id.0) {
            write_packet_to_layer(
                &mut *entity_layer,
                position,
                Packet {
                    id: id.0,
                    action: part.to_packet_action(),
                },
            );
        }
    }
}

fn update_boss_bar_layer_view(
    mut clients_query: Query<
        (
            &mut Client,
            &VisibleEntityLayers,
            &OldVisibleEntityLayers,
            &Position,
            &OldPosition,
            &ViewDistance,
            &OldViewDistance,
        ),
        Changed<VisibleEntityLayers>,
    >,
    boss_bars_query: Query<(
        &UniqueId,
        &BossBarTitle,
        &BossBarHealth,
        &BossBarStyle,
        &BossBarFlags,
        &EntityLayerId,
        Option<&Position>,
    )>,
) {
    for (
        mut client,
        visible_entity_layers,
        old_visible_entity_layers,
        position,
        _old_position,
        view_distance,
        _old_view_distance,
    ) in &mut clients_query
    {
        let view = ChunkView::new(position.0.into(), view_distance.get());
        let old_layers = old_visible_entity_layers.get();
        let current_layers = &visible_entity_layers.0;

        for &added_layer in current_layers.difference(old_layers) {
            for bar in boss_bars_query.iter().filter(|bar| bar.5 .0 == added_layer) {
                write_add_if_visible(&mut *client, &view, bar);
            }
        }

        for &removed_layer in old_layers.difference(current_layers) {
            for (id, _, _, _, _, _, boss_bar_position) in boss_bars_query
                .iter()
                .filter(|bar| bar.5 .0 == removed_layer)
            {
                write_remove_if_visible(&mut *client, &view, id, boss_bar_position);
            }
        }
    }
}

fn update_boss_bar_chunk_view(
    mut clients_query: Query<
        (
            &mut Client,
            &VisibleEntityLayers,
            &OldVisibleEntityLayers,
            &Position,
            &OldPosition,
            &ViewDistance,
            &OldViewDistance,
        ),
        Changed<Position>,
    >,
    boss_bars_query: Query<(
        &UniqueId,
        &BossBarTitle,
        &BossBarHealth,
        &BossBarStyle,
        &BossBarFlags,
        &EntityLayerId,
        &Position,
    )>,
) {
    for (
        mut client,
        visible_entity_layers,
        _old_visible_entity_layers,
        position,
        old_position,
        view_distance,
        old_view_distance,
    ) in &mut clients_query
    {
        let view = ChunkView::new(position.0.into(), view_distance.get());
        let old_view = ChunkView::new(old_position.get().into(), old_view_distance.get());

        for layer in &visible_entity_layers.0 {
            for bar in boss_bars_query.iter().filter(|bar| bar.5 .0 == *layer) {
                write_view_delta(&mut *client, &view, &old_view, bar);
            }
        }
    }
}

fn boss_bar_despawn(
    boss_bars_query: Query<
        (&UniqueId, &EntityLayerId, Option<&Position>),
        With<valence_server::Despawned>,
    >,
    mut entity_layer_query: Query<&mut EntityLayer>,
) {
    for (id, entity_layer_id, position) in boss_bars_query.iter() {
        if let Ok(mut entity_layer) = entity_layer_query.get_mut(entity_layer_id.0) {
            write_packet_to_layer(&mut *entity_layer, position, remove_packet(id));
        }
    }
}

fn write_packet_to_layer(
    entity_layer: &mut EntityLayer,
    position: Option<&Position>,
    packet: Packet<'_>,
) {
    if let Some(position) = position {
        entity_layer.view_writer(position.0).write_packet(&packet);
    } else {
        entity_layer.write_packet(&packet);
    }
}

fn write_add_if_visible(
    client: &mut Client,
    view: &ChunkView,
    bar: (
        &UniqueId,
        &BossBarTitle,
        &BossBarHealth,
        &BossBarStyle,
        &BossBarFlags,
        &EntityLayerId,
        Option<&Position>,
    ),
) {
    let (id, title, health, style, flags, _, position) = bar;
    if position.is_none_or(|position| view.contains(position.0.into())) {
        client.write_packet(&add_packet(id, title, health, style, flags));
    }
}

fn write_remove_if_visible(
    client: &mut Client,
    view: &ChunkView,
    id: &UniqueId,
    position: Option<&Position>,
) {
    if position.is_none_or(|position| view.contains(position.0.into())) {
        client.write_packet(&remove_packet(id));
    }
}

fn write_view_delta(
    client: &mut Client,
    view: &ChunkView,
    old_view: &ChunkView,
    bar: (
        &UniqueId,
        &BossBarTitle,
        &BossBarHealth,
        &BossBarStyle,
        &BossBarFlags,
        &EntityLayerId,
        &Position,
    ),
) {
    let (id, title, health, style, flags, _, position) = bar;
    let is_visible = view.contains(position.0.into());
    let was_visible = old_view.contains(position.0.into());
    if is_visible && !was_visible {
        client.write_packet(&add_packet(id, title, health, style, flags));
    } else if !is_visible && was_visible {
        client.write_packet(&remove_packet(id));
    }
}

fn add_packet<'a>(
    id: &UniqueId,
    title: &'a BossBarTitle,
    health: &BossBarHealth,
    style: &BossBarStyle,
    flags: &BossBarFlags,
) -> Packet<'a> {
    Packet {
        id: id.0,
        action: add_action(title, health, style, flags),
    }
}

fn remove_packet(id: &UniqueId) -> Packet<'_> {
    Packet {
        id: id.0,
        action: Action::Remove,
    }
}

fn add_action<'a>(
    title: &'a BossBarTitle,
    health: &BossBarHealth,
    style: &BossBarStyle,
    flags: &BossBarFlags,
) -> Action<'a> {
    Action::Add {
        title: std::borrow::Cow::Borrowed(&title.0),
        health: health.0,
        color: style.color,
        division: style.division,
        flags: *flags,
    }
}
