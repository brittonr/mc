use bevy_ecs::prelude::*;
use valence_server::protocol::WritePacket;
use valence_server::text::IntoText;

type Client = valence_server::client::Client;
type DisplayPacket<'a> = valence_server::protocol::packets::play::ScoreboardDisplayS2c<'a>;
type DisplayPosition = valence_server::protocol::packets::play::ScoreboardPosition;
type Layer = valence_server::EntityLayer;
type LayerId = valence_server::entity::EntityLayerId;
type Mode<'a> = valence_server::protocol::packets::play::ObjectiveMode<'a>;
type ObjectiveEntry<'a> = (
    &'a super::Objective,
    &'a super::ObjectiveDisplay,
    &'a RenderType,
    &'a DisplayPosition,
    &'a super::ObjectiveScores,
    &'a LayerId,
);
type ObjectivePacket<'a> =
    valence_server::protocol::packets::play::ScoreboardObjectiveUpdateS2c<'a>;
type PlayerAction<'a> = valence_server::protocol::packets::play::ScoreboardPlayerUpdateAction<'a>;
type PlayerPacket<'a> = valence_server::protocol::packets::play::ScoreboardPlayerUpdateS2c<'a>;
type RenderType = valence_server::protocol::packets::play::ObjectiveRenderType;

pub(super) fn create_or_update_objectives(
    objectives: Query<
        (
            Ref<super::Objective>,
            &super::ObjectiveDisplay,
            &RenderType,
            &LayerId,
        ),
        Or<(Changed<super::ObjectiveDisplay>, Changed<RenderType>)>,
    >,
    mut layers: Query<&mut Layer>,
) {
    for (objective, display, render_type, layer_id) in objectives.iter() {
        if objective.name().is_empty() {
            tracing::warn!("Objective name is empty");
        }
        let Ok(mut layer) = layer_for_id(&mut layers, layer_id, "update scoreboard objective")
        else {
            continue;
        };
        layer.write_packet(&ObjectivePacket {
            objective_name: &objective.0,
            mode: objective_mode(&objective, display, render_type),
        });
    }
}

pub(super) fn display_objectives(
    objectives: Query<
        (&super::Objective, Ref<DisplayPosition>, &LayerId),
        Changed<DisplayPosition>,
    >,
    mut layers: Query<&mut Layer>,
) {
    for (objective, position, layer_id) in objectives.iter() {
        let Ok(mut layer) = layer_for_id(&mut layers, layer_id, "update scoreboard display") else {
            continue;
        };
        layer.write_packet(&DisplayPacket {
            score_name: &objective.0,
            position: *position,
        });
    }
}

pub(super) fn remove_despawned_objectives(
    mut commands: Commands,
    objectives: Query<(Entity, &super::Objective, &LayerId), With<valence_server::Despawned>>,
    mut layers: Query<&mut Layer>,
) {
    for (entity, objective, layer_id) in objectives.iter() {
        commands.entity(entity).despawn();
        let Ok(mut layer) = layer_for_id(&mut layers, layer_id, "remove scoreboard objective")
        else {
            continue;
        };
        layer.write_packet(&ObjectivePacket {
            objective_name: &objective.0,
            mode: Mode::Remove,
        });
    }
}

pub(super) fn handle_new_clients(
    mut clients: Query<
        (
            &mut Client,
            &valence_server::client::VisibleEntityLayers,
            &valence_server::client::OldVisibleEntityLayers,
        ),
        Or<(
            Added<Client>,
            Changed<valence_server::client::VisibleEntityLayers>,
        )>,
    >,
    objectives: Query<ObjectiveEntry<'_>, Without<valence_server::Despawned>>,
) {
    for (mut client, visible_layers, old_visible_layers) in &mut clients {
        let is_new_client = client.is_added();
        remove_old_layer_objectives(&mut client, visible_layers, old_visible_layers, &objectives);
        add_new_layer_objectives(
            &mut client,
            is_new_client,
            visible_layers,
            old_visible_layers,
            &objectives,
        );
    }
}

pub(super) fn update_scores(
    mut objectives: Query<
        (
            &super::Objective,
            &super::ObjectiveScores,
            &mut super::OldObjectiveScores,
            &LayerId,
        ),
        (
            Changed<super::ObjectiveScores>,
            Without<valence_server::Despawned>,
        ),
    >,
    mut layers: Query<&mut Layer>,
) {
    for (objective, scores, mut old_scores, layer_id) in &mut objectives {
        let Ok(mut layer) = layer_for_id(&mut layers, layer_id, "update scores") else {
            continue;
        };
        for changed_key in old_scores.diff(scores) {
            layer.write_packet(&score_update_packet(objective, scores, changed_key));
        }
        old_scores.0.clone_from(&scores.0);
    }
}

fn objective_mode<'a>(
    objective: &Ref<'_, super::Objective>,
    display: &'a super::ObjectiveDisplay,
    render_type: &'a RenderType,
) -> Mode<'a> {
    if objective.is_added() {
        Mode::Create {
            objective_display_name: (&display.0).into_cow_text(),
            render_type: *render_type,
        }
    } else {
        Mode::Update {
            objective_display_name: (&display.0).into_cow_text(),
            render_type: *render_type,
        }
    }
}

fn remove_old_layer_objectives(
    client: &mut Client,
    visible_layers: &valence_server::client::VisibleEntityLayers,
    old_visible_layers: &valence_server::client::OldVisibleEntityLayers,
    objectives: &Query<ObjectiveEntry<'_>, Without<valence_server::Despawned>>,
) {
    let removed_layers = old_visible_layers
        .get()
        .difference(&visible_layers.0)
        .collect::<std::collections::BTreeSet<_>>();
    for (objective, _, _, _, _, layer_id) in objectives.iter() {
        if removed_layers.contains(&layer_id.0) {
            client.write_packet(&ObjectivePacket {
                objective_name: &objective.0,
                mode: Mode::Remove,
            });
        }
    }
}

fn add_new_layer_objectives(
    client: &mut Client,
    is_new_client: bool,
    visible_layers: &valence_server::client::VisibleEntityLayers,
    old_visible_layers: &valence_server::client::OldVisibleEntityLayers,
    objectives: &Query<ObjectiveEntry<'_>, Without<valence_server::Despawned>>,
) {
    let added_layers = added_layers_for_client(is_new_client, visible_layers, old_visible_layers);
    for entry in objectives.iter() {
        if added_layers.contains(&entry.5 .0) {
            add_objective_to_client(client, entry);
        }
    }
}

fn added_layers_for_client(
    is_new_client: bool,
    visible_layers: &valence_server::client::VisibleEntityLayers,
    old_visible_layers: &valence_server::client::OldVisibleEntityLayers,
) -> std::collections::BTreeSet<bevy_ecs::prelude::Entity> {
    if is_new_client {
        tracing::debug!("client is new, sending all objectives");
        visible_layers.0.clone()
    } else {
        visible_layers
            .0
            .difference(old_visible_layers.get())
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
    }
}

fn add_objective_to_client(client: &mut Client, entry: ObjectiveEntry<'_>) {
    let (objective, display, render_type, position, scores, _) = entry;
    client.write_packet(&ObjectivePacket {
        objective_name: &objective.0,
        mode: Mode::Create {
            objective_display_name: (&display.0).into_cow_text(),
            render_type: *render_type,
        },
    });
    client.write_packet(&DisplayPacket {
        score_name: &objective.0,
        position: *position,
    });
    for (key, score) in &scores.0 {
        client.write_packet(&score_set_packet(objective, key, *score));
    }
}

fn score_update_packet<'a>(
    objective: &'a super::Objective,
    scores: &'a super::ObjectiveScores,
    changed_key: &'a str,
) -> PlayerPacket<'a> {
    match scores.0.get(changed_key) {
        Some(score) => score_set_packet(objective, changed_key, *score),
        None => PlayerPacket {
            entity_name: changed_key,
            action: PlayerAction::Remove {
                objective_name: &objective.0,
            },
        },
    }
}

fn score_set_packet<'a>(
    objective: &'a super::Objective,
    key: &'a str,
    score: i32,
) -> PlayerPacket<'a> {
    PlayerPacket {
        entity_name: key,
        action: PlayerAction::Update {
            objective_name: &objective.0,
            objective_score: valence_server::protocol::VarInt(score),
        },
    }
}

fn layer_for_id<'w, 's>(
    layers: &'w mut Query<&'s mut Layer>,
    layer_id: &LayerId,
    action: &str,
) -> Result<Mut<'w, Layer>, ()> {
    layers.get_mut(layer_id.0).map_err(|_| {
        tracing::warn!(
            "No layer found for entity layer ID {:?}, can't {}",
            layer_id,
            action
        );
    })
}
