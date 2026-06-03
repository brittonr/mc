use std::io::Write;

use bevy_ecs::prelude::*;

type AdvancementCachedBytes = crate::AdvancementCachedBytes;
type AdvancementClientUpdate = crate::AdvancementClientUpdate;
type AdvancementCriteria = crate::AdvancementCriteria;
type AdvancementDisplay = crate::AdvancementDisplay;
type AdvancementRequirements = crate::AdvancementRequirements;
type Advancement = crate::Advancement;
type Children = bevy_hierarchy::Children;
type Client = valence_server::client::Client;
type Parent = bevy_hierarchy::Parent;
type SelectTabPacket<'a> = valence_server::protocol::packets::play::SelectAdvancementTabS2c<'a>;
type WireCriteria<'a> = valence_server::protocol::packets::play::AdvancementCriteria<'a>;
type WireDisplay<'a, I> = valence_server::protocol::packets::play::AdvancementDisplay<'a, I>;
type WireNode<'a, I> = valence_server::protocol::packets::play::Advancement<'a, I>;
type WireRequirements<'a> = valence_server::protocol::packets::play::AdvancementRequirements<'a>;
type WireUpdate<'a, AM> =
    valence_server::protocol::packets::play::GenericAdvancementUpdateS2c<'a, AM>;

#[derive(bevy_ecs::system::SystemParam, Debug)]
pub(crate) struct UpdateAdvancementCachedBytesQuery<'w, 's> {
    advancement_id_query: Query<'w, 's, &'static Advancement>,
    criteria_query: Query<'w, 's, &'static AdvancementCriteria>,
}

struct WriteInput<'a> {
    identifier: &'a Advancement,
    requirements: &'a AdvancementRequirements,
    display: Option<&'a AdvancementDisplay>,
    children: Option<&'a Children>,
    parent: Option<&'a Parent>,
}

impl UpdateAdvancementCachedBytesQuery<'_, '_> {
    fn write(
        &self,
        input: WriteInput<'_>,
        w: impl Write,
    ) -> valence_server::protocol::anyhow::Result<()> {
        let Self {
            advancement_id_query,
            criteria_query,
        } = self;

        let mut pkt = WireNode {
            parent_id: None,
            display_data: None,
            criteria: vec![],
            requirements: vec![],
            sends_telemetry_data: false,
        };

        if let Some(a_parent) = input.parent {
            let a_identifier = advancement_id_query.get(a_parent.get())?;
            pkt.parent_id = Some(a_identifier.0.borrowed());
        }

        if let Some(a_display) = input.display {
            pkt.display_data = Some(WireDisplay {
                title: std::borrow::Cow::Borrowed(&a_display.title),
                description: std::borrow::Cow::Borrowed(&a_display.description),
                icon: &a_display.icon,
                frame_type: valence_server::protocol::VarInt(a_display.frame_type as i32),
                flags: a_display.flags(),
                background_texture: a_display.background_texture.as_ref().map(|v| v.borrowed()),
                x_coord: a_display.x_coord,
                y_coord: a_display.y_coord,
            });
        }

        if let Some(a_children) = input.children {
            for a_child in a_children {
                let Ok(c_identifier) = criteria_query.get(*a_child) else {
                    continue;
                };
                pkt.criteria.push((c_identifier.0.borrowed(), ()));
            }
        }

        for requirements in &input.requirements.0 {
            let mut requirements_p = Vec::with_capacity(requirements.len());
            for requirement in requirements {
                let c_identifier = criteria_query.get(*requirement)?;
                requirements_p.push(c_identifier.0.as_str());
            }
            pkt.requirements.push(WireRequirements {
                requirement: requirements_p,
            });
        }

        valence_server::protocol::Encode::encode(&(&input.identifier.0, pkt), w)
    }
}

pub(crate) fn update_advancement_cached_bytes(
    mut query: Query<
        (
            &Advancement,
            &AdvancementRequirements,
            &mut AdvancementCachedBytes,
            Option<&AdvancementDisplay>,
            Option<&Children>,
            Option<&Parent>,
        ),
        Or<(
            Changed<AdvancementDisplay>,
            Changed<Children>,
            Changed<Parent>,
            Changed<AdvancementRequirements>,
        )>,
    >,
    update_advancement_cached_bytes_query: UpdateAdvancementCachedBytesQuery,
) {
    for (a_identifier, a_requirements, mut a_bytes, a_display, a_children, a_parent) in &mut query {
        a_bytes.0.clear();
        if update_advancement_cached_bytes_query
            .write(
                WriteInput {
                    identifier: a_identifier,
                    requirements: a_requirements,
                    display: a_display,
                    children: a_children,
                    parent: a_parent,
                },
                &mut a_bytes.0,
            )
            .is_err()
        {
            a_bytes.0.clear();
        }
    }
}

#[derive(bevy_ecs::system::SystemParam, Debug)]
// Bevy query SystemParam types are verbose by design at this boundary.
#[allow(clippy::type_complexity)]
pub(crate) struct SingleAdvancementUpdateQuery<'w, 's> {
    advancement_bytes: Query<'w, 's, &'static AdvancementCachedBytes>,
    advancement_id: Query<'w, 's, &'static Advancement>,
    criteria: Query<'w, 's, &'static AdvancementCriteria>,
    parent: Query<'w, 's, &'static Parent>,
}

#[derive(Debug)]
pub(crate) struct AdvancementUpdateEncodeS2c<'w, 's, 'a> {
    client_update: AdvancementClientUpdate,
    queries: &'a SingleAdvancementUpdateQuery<'w, 's>,
}

impl valence_server::protocol::Encode for AdvancementUpdateEncodeS2c<'_, '_, '_> {
    fn encode(&self, w: impl Write) -> valence_server::protocol::anyhow::Result<()> {
        let SingleAdvancementUpdateQuery {
            advancement_bytes: advancement_bytes_query,
            advancement_id: advancement_id_query,
            criteria: criteria_query,
            parent: parent_query,
        } = self.queries;

        let AdvancementClientUpdate {
            new_advancements,
            remove_advancements,
            progress,
            reset,
            ..
        } = &self.client_update;

        let mut pkt = WireUpdate {
            reset: *reset,
            advancement_mapping: vec![],
            identifiers: vec![],
            progress_mapping: vec![],
        };

        for new_advancement in new_advancements {
            let a_cached_bytes = advancement_bytes_query.get(*new_advancement)?;
            pkt.advancement_mapping
                .push(valence_server::protocol::RawBytes(
                    a_cached_bytes.0.as_slice(),
                ));
        }

        for remove_advancement in remove_advancements {
            let a_identifier = advancement_id_query.get(*remove_advancement)?;
            pkt.identifiers.push(a_identifier.0.borrowed());
        }

        let mut progress_mapping: rustc_hash::FxHashMap<Entity, Vec<(Entity, Option<i64>)>> =
            rustc_hash::FxHashMap::with_capacity_and_hasher(
                progress.len(),
                rustc_hash::FxBuildHasher,
            );
        let progress_count = progress.len();
        for progress in progress {
            let advancement = parent_query.get(progress.0)?;
            progress_mapping
                .entry(advancement.get())
                .or_insert_with(|| Vec::with_capacity(progress_count))
                .push(*progress);
        }

        for (a, c_progresses) in progress_mapping {
            let a_identifier = advancement_id_query.get(a)?;
            let mut c_progresses_p = Vec::with_capacity(c_progresses.len());
            for (c, c_progress) in c_progresses {
                let c_identifier = criteria_query.get(c)?;
                c_progresses_p.push(WireCriteria {
                    criterion_identifier: c_identifier.0.borrowed(),
                    criterion_progress: c_progress,
                });
            }
            pkt.progress_mapping
                .push((a_identifier.0.borrowed(), c_progresses_p));
        }

        valence_server::protocol::Encode::encode(&pkt, w)
    }
}

fn has_pending_client_update(client_update: &AdvancementClientUpdate) -> bool {
    !client_update.new_advancements.is_empty()
        || !client_update.progress.is_empty()
        || !client_update.remove_advancements.is_empty()
        || client_update.reset
}

impl valence_server::protocol::Packet for AdvancementUpdateEncodeS2c<'_, '_, '_> {
    const ID: i32 = valence_server::protocol::packet_id::ADVANCEMENT_UPDATE_S2C;
    const NAME: &'static str = "AdvancementUpdateEncodeS2c";
    const SIDE: valence_server::protocol::PacketSide =
        valence_server::protocol::PacketSide::Clientbound;
    const STATE: valence_server::protocol::PacketState =
        valence_server::protocol::PacketState::Play;
}

// Bevy query tuple is intentionally kept at the system boundary.
#[allow(clippy::type_complexity)]
pub(crate) fn send_advancement_update_packet(
    mut client: Query<(&mut AdvancementClientUpdate, &mut Client)>,
    update_single_query: SingleAdvancementUpdateQuery,
) {
    for (mut advancement_client_update, mut client) in &mut client {
        match advancement_client_update.force_tab_update {
            crate::ForceTabUpdate::None => {}
            crate::ForceTabUpdate::First => valence_server::protocol::WritePacket::write_packet(
                &mut *client,
                &SelectTabPacket { identifier: None },
            ),
            crate::ForceTabUpdate::Spec(spec) => {
                if let Ok(a_identifier) = update_single_query.advancement_id.get(spec) {
                    valence_server::protocol::WritePacket::write_packet(
                        &mut *client,
                        &SelectTabPacket {
                            identifier: Some(a_identifier.0.borrowed()),
                        },
                    );
                }
            }
        }

        if crate::ForceTabUpdate::None != advancement_client_update.force_tab_update {
            advancement_client_update.force_tab_update = crate::ForceTabUpdate::None;
        }

        if !has_pending_client_update(&advancement_client_update) {
            continue;
        }

        let advancement_client_update = std::mem::replace(
            advancement_client_update.as_mut(),
            AdvancementClientUpdate {
                reset: false,
                new_advancements: Vec::new(),
                remove_advancements: Vec::new(),
                progress: Vec::new(),
                force_tab_update: crate::ForceTabUpdate::None,
            },
        );

        valence_server::protocol::WritePacket::write_packet(
            &mut *client,
            &AdvancementUpdateEncodeS2c {
                queries: &update_single_query,
                client_update: advancement_client_update,
            },
        );
    }
}
