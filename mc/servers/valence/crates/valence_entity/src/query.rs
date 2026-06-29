use std::mem;

use bevy_ecs::prelude::DetectChanges;
use bevy_ecs::query::QueryData;
use bevy_ecs::world::Ref;
use valence_math::DVec3;
use valence_protocol::encode::WritePacket;
use valence_protocol::packets::play::{
    EntityAnimationS2c, EntityAttributesS2c, EntityPositionS2c, EntitySetHeadYawS2c,
    EntitySpawnS2c, EntityStatusS2c, EntityTrackerUpdateS2c, EntityVelocityUpdateS2c,
    ExperienceOrbSpawnS2c, MoveRelativeS2c, PlayerSpawnS2c, RotateAndMoveRelativeS2c, RotateS2c,
};
use valence_protocol::var_int::VarInt;
use valence_protocol::ByteAngle;
use valence_server_common::UniqueId;

use crate::attributes::TrackedEntityAttributes;
use crate::tracked_data::TrackedData;
use crate::{
    EntityAnimations, EntityId, EntityKind, EntityLayerId, EntityStatuses, HeadYaw, Look,
    ObjectData, OldEntityLayerId, OldPosition, OnGround, Position, Velocity,
};

const TELEPORT_DELTA_THRESHOLD_BLOCKS: f64 = 8.0;
const RELATIVE_MOVE_PACKET_SCALE: f64 = 4_096.0;
const ENTITY_STATUS_PACKET_BIT_COUNT: usize = mem::size_of::<EntityStatuses>();
const ENTITY_ANIMATION_PACKET_BIT_COUNT: usize = mem::size_of::<EntityAnimations>();

#[derive(Copy, Clone, Debug, PartialEq)]
struct EntityMovementDecision {
    position_delta: DVec3,
    changed_position: bool,
    needs_teleport: bool,
}

fn entity_movement_decision(position: DVec3, old_position: DVec3) -> EntityMovementDecision {
    let position_delta = position - old_position;
    let needs_teleport = position_delta.abs().max_element() >= TELEPORT_DELTA_THRESHOLD_BLOCKS;
    let changed_position = position != old_position;

    EntityMovementDecision {
        position_delta,
        changed_position,
        needs_teleport,
    }
}

fn relative_move_delta(position_delta: DVec3) -> [i16; 3] {
    (position_delta * RELATIVE_MOVE_PACKET_SCALE)
        .to_array()
        .map(|value| value as i16)
}

fn active_packet_indices(bits: u64, bit_count: usize) -> Vec<u8> {
    (0..bit_count)
        .filter(|index| (bits >> index) & 1 == 1)
        .map(|index| index as u8)
        .collect()
}

#[derive(QueryData)]
pub struct EntityInitQuery {
    pub entity_id: &'static EntityId,
    pub uuid: &'static UniqueId,
    pub kind: &'static EntityKind,
    pub look: &'static Look,
    pub head_yaw: &'static HeadYaw,
    pub on_ground: &'static OnGround,
    pub object_data: &'static ObjectData,
    pub velocity: &'static Velocity,
    pub tracked_data: &'static TrackedData,
}

impl EntityInitQueryItem<'_> {
    /// Writes the appropriate packets to initialize an entity. This will spawn
    /// the entity and initialize tracked data. `pos` is the initial position of
    /// the entity.
    pub fn write_init_packets<W: WritePacket>(&self, pos: DVec3, mut writer: W) {
        match *self.kind {
            EntityKind::MARKER => {}
            EntityKind::EXPERIENCE_ORB => {
                writer.write_packet(&ExperienceOrbSpawnS2c {
                    entity_id: self.entity_id.get().into(),
                    position: pos,
                    count: self.object_data.0 as i16,
                });
            }
            EntityKind::PLAYER => {
                writer.write_packet(&PlayerSpawnS2c {
                    entity_id: self.entity_id.get().into(),
                    player_uuid: self.uuid.0,
                    position: pos,
                    yaw: ByteAngle::from_degrees(self.look.yaw),
                    pitch: ByteAngle::from_degrees(self.look.pitch),
                });

                // Player spawn packet doesn't include head yaw for some reason.
                writer.write_packet(&EntitySetHeadYawS2c {
                    entity_id: self.entity_id.get().into(),
                    head_yaw: ByteAngle::from_degrees(self.head_yaw.0),
                });
            }
            _ => writer.write_packet(&EntitySpawnS2c {
                entity_id: self.entity_id.get().into(),
                object_uuid: self.uuid.0,
                kind: self.kind.get().into(),
                position: pos,
                pitch: ByteAngle::from_degrees(self.look.pitch),
                yaw: ByteAngle::from_degrees(self.look.yaw),
                head_yaw: ByteAngle::from_degrees(self.head_yaw.0),
                data: self.object_data.0.into(),
                velocity: self.velocity.to_packet_units(),
            }),
        }

        if let Some(init_data) = self.tracked_data.init_data() {
            writer.write_packet(&EntityTrackerUpdateS2c {
                entity_id: self.entity_id.get().into(),
                tracked_values: init_data.into(),
            });
        }
    }
}

#[derive(QueryData)]
pub struct UpdateEntityQuery {
    pub id: &'static EntityId,
    pub pos: &'static Position,
    pub old_pos: &'static OldPosition,
    pub loc: &'static EntityLayerId,
    pub old_loc: &'static OldEntityLayerId,
    pub look: Ref<'static, Look>,
    pub head_yaw: Ref<'static, HeadYaw>,
    pub on_ground: &'static OnGround,
    pub velocity: Ref<'static, Velocity>,
    pub tracked_data: &'static TrackedData,
    pub statuses: &'static EntityStatuses,
    pub animations: &'static EntityAnimations,
    // Option because not all entities have attributes, only LivingEntity.
    pub tracked_attributes: Option<&'static TrackedEntityAttributes>,
}

impl UpdateEntityQueryItem<'_> {
    pub fn write_update_packets<W: WritePacket>(&self, mut writer: W) {
        // TODO: @RJ I saw you're using UpdateEntityPosition and UpdateEntityRotation sometimes. These two packets are actually broken on the client and will erase previous position/rotation https://bugs.mojang.com/browse/MC-255263 -Moulberry

        let entity_id = VarInt(self.id.get());
        let movement = entity_movement_decision(self.pos.0, self.old_pos.get());

        if movement.changed_position && !movement.needs_teleport && self.look.is_changed() {
            writer.write_packet(&RotateAndMoveRelativeS2c {
                entity_id,
                delta: relative_move_delta(movement.position_delta),
                yaw: ByteAngle::from_degrees(self.look.yaw),
                pitch: ByteAngle::from_degrees(self.look.pitch),
                on_ground: self.on_ground.0,
            });
        } else {
            if movement.changed_position && !movement.needs_teleport {
                writer.write_packet(&MoveRelativeS2c {
                    entity_id,
                    delta: relative_move_delta(movement.position_delta),
                    on_ground: self.on_ground.0,
                });
            }

            if self.look.is_changed() {
                writer.write_packet(&RotateS2c {
                    entity_id,
                    yaw: ByteAngle::from_degrees(self.look.yaw),
                    pitch: ByteAngle::from_degrees(self.look.pitch),
                    on_ground: self.on_ground.0,
                });
            }
        }

        if movement.needs_teleport {
            writer.write_packet(&EntityPositionS2c {
                entity_id,
                position: self.pos.0,
                yaw: ByteAngle::from_degrees(self.look.yaw),
                pitch: ByteAngle::from_degrees(self.look.pitch),
                on_ground: self.on_ground.0,
            });
        }

        if self.velocity.is_changed() {
            writer.write_packet(&EntityVelocityUpdateS2c {
                entity_id,
                velocity: self.velocity.to_packet_units(),
            });
        }

        if self.head_yaw.is_changed() {
            writer.write_packet(&EntitySetHeadYawS2c {
                entity_id,
                head_yaw: ByteAngle::from_degrees(self.head_yaw.0),
            });
        }

        if let Some(update_data) = self.tracked_data.update_data() {
            writer.write_packet(&EntityTrackerUpdateS2c {
                entity_id,
                tracked_values: update_data.into(),
            });
        }

        for entity_status in active_packet_indices(self.statuses.0, ENTITY_STATUS_PACKET_BIT_COUNT)
        {
            writer.write_packet(&EntityStatusS2c {
                entity_id: entity_id.0,
                entity_status,
            });
        }

        for animation in active_packet_indices(
            u64::from(self.animations.0),
            ENTITY_ANIMATION_PACKET_BIT_COUNT,
        ) {
            writer.write_packet(&EntityAnimationS2c {
                entity_id,
                animation,
            });
        }

        if let Some(attributes) = self.tracked_attributes {
            let properties = attributes.get_properties();

            if !properties.is_empty() {
                writer.write_packet(&EntityAttributesS2c {
                    entity_id,
                    properties,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use valence_protocol::{Encode, Packet};

    const INIT_PACKET_ENTITY_ID: i32 = 123;
    const INIT_METADATA_INDEX: u8 = 0;
    const INIT_METADATA_TYPE_ID: u8 = 0;
    const INIT_METADATA_VALUE: u8 = 1;
    const RELATIVE_POSITION_DELTA: DVec3 = DVec3::new(1.0, 0.0, 0.0);
    const TELEPORT_POSITION_DELTA: DVec3 = DVec3::new(TELEPORT_DELTA_THRESHOLD_BLOCKS, 0.0, 0.0);
    const FIRST_PACKET_BIT: u64 = 0b0000_0001;
    const EMPTY_PACKET_BIT_COUNT: usize = 0;

    #[derive(Default)]
    struct PacketNameWriter {
        names: Vec<&'static str>,
    }

    impl WritePacket for PacketNameWriter {
        fn write_packet_fallible<P>(&mut self, _packet: &P) -> anyhow::Result<()>
        where
            P: Packet + Encode,
        {
            self.names.push(P::NAME);
            Ok(())
        }

        fn write_packet_bytes(&mut self, _bytes: &[u8]) {}
    }

    #[test]
    fn query_core_selects_relative_movement() {
        let movement = entity_movement_decision(RELATIVE_POSITION_DELTA, DVec3::ZERO);

        assert!(movement.changed_position);
        assert!(!movement.needs_teleport);
        assert_eq!(movement.position_delta, RELATIVE_POSITION_DELTA);
    }

    #[test]
    fn query_core_selects_teleport_movement() {
        let movement = entity_movement_decision(TELEPORT_POSITION_DELTA, DVec3::ZERO);

        assert!(movement.changed_position);
        assert!(movement.needs_teleport);
    }

    #[test]
    fn empty_query_bit_inputs_emit_no_packet_indices() {
        assert!(active_packet_indices(0, ENTITY_STATUS_PACKET_BIT_COUNT).is_empty());
        assert!(active_packet_indices(FIRST_PACKET_BIT, EMPTY_PACKET_BIT_COUNT).is_empty());
    }

    #[test]
    fn init_packets_send_spawn_before_metadata() {
        let entity_id = EntityId(INIT_PACKET_ENTITY_ID);
        let uuid = UniqueId(Uuid::nil());
        let kind = EntityKind::ITEM;
        let look = Look::default();
        let head_yaw = HeadYaw::default();
        let on_ground = OnGround::default();
        let object_data = ObjectData::default();
        let velocity = Velocity::default();
        let mut tracked_data = TrackedData::default();
        tracked_data.insert_init_value(
            INIT_METADATA_INDEX,
            INIT_METADATA_TYPE_ID,
            INIT_METADATA_VALUE,
        );
        let init_item = EntityInitQueryItem {
            entity_id: &entity_id,
            uuid: &uuid,
            kind: &kind,
            look: &look,
            head_yaw: &head_yaw,
            on_ground: &on_ground,
            object_data: &object_data,
            velocity: &velocity,
            tracked_data: &tracked_data,
        };
        let mut writer = PacketNameWriter::default();

        init_item.write_init_packets(DVec3::ZERO, &mut writer);

        assert_eq!(
            writer.names,
            [EntitySpawnS2c::NAME, EntityTrackerUpdateS2c::NAME]
        );
    }
}
