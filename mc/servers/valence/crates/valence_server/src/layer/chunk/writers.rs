use bevy_ecs::prelude::Entity;
use valence_protocol::encode::{PacketWriter, WritePacket};
use valence_protocol::{BlockPos, ChunkPos, Encode, Packet};

use super::{ChunkLayer, GlobalMsg, LocalMsg};

impl WritePacket for ChunkLayer {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.messages.send_global(GlobalMsg::Packet, |b| {
            PacketWriter::new(b, self.info.threshold).write_packet_fallible(packet)
        })
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.messages
            .send_global_infallible(GlobalMsg::Packet, |b| b.extend_from_slice(bytes));
    }
}

pub struct ExceptWriter<'a> {
    pub(super) layer: &'a mut ChunkLayer,
    pub(super) except: Entity,
}

impl WritePacket for ExceptWriter<'_> {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.layer.messages.send_global(
            GlobalMsg::PacketExcept {
                except: self.except,
            },
            |b| PacketWriter::new(b, self.layer.info.threshold).write_packet_fallible(packet),
        )
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.layer.messages.send_global_infallible(
            GlobalMsg::PacketExcept {
                except: self.except,
            },
            |b| b.extend_from_slice(bytes),
        )
    }
}

pub struct ViewWriter<'a> {
    pub(super) layer: &'a mut ChunkLayer,
    pub(super) pos: ChunkPos,
}

impl WritePacket for ViewWriter<'_> {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.layer
            .messages
            .send_local(LocalMsg::PacketAt { pos: self.pos }, |b| {
                PacketWriter::new(b, self.layer.info.threshold).write_packet_fallible(packet)
            })
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.layer
            .messages
            .send_local_infallible(LocalMsg::PacketAt { pos: self.pos }, |b| {
                b.extend_from_slice(bytes)
            });
    }
}

pub struct ViewExceptWriter<'a> {
    pub(super) layer: &'a mut ChunkLayer,
    pub(super) pos: ChunkPos,
    pub(super) except: Entity,
}

impl WritePacket for ViewExceptWriter<'_> {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.layer.messages.send_local(
            LocalMsg::PacketAtExcept {
                pos: self.pos,
                except: self.except,
            },
            |b| PacketWriter::new(b, self.layer.info.threshold).write_packet_fallible(packet),
        )
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.layer.messages.send_local_infallible(
            LocalMsg::PacketAtExcept {
                pos: self.pos,
                except: self.except,
            },
            |b| b.extend_from_slice(bytes),
        );
    }
}

pub struct RadiusWriter<'a> {
    pub(super) layer: &'a mut ChunkLayer,
    pub(super) center: BlockPos,
    pub(super) radius_squared: u32,
}

impl WritePacket for RadiusWriter<'_> {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.layer.messages.send_local(
            LocalMsg::RadiusAt {
                center: self.center,
                radius_squared: self.radius_squared,
            },
            |b| PacketWriter::new(b, self.layer.info.threshold).write_packet_fallible(packet),
        )
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.layer.messages.send_local_infallible(
            LocalMsg::RadiusAt {
                center: self.center,
                radius_squared: self.radius_squared,
            },
            |b| b.extend_from_slice(bytes),
        );
    }
}

pub struct RadiusExceptWriter<'a> {
    pub(super) layer: &'a mut ChunkLayer,
    pub(super) center: BlockPos,
    pub(super) radius_squared: u32,
    pub(super) except: Entity,
}

impl WritePacket for RadiusExceptWriter<'_> {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.layer.messages.send_local(
            LocalMsg::RadiusAtExcept {
                center: self.center,
                radius_squared: self.radius_squared,
                except: self.except,
            },
            |b| PacketWriter::new(b, self.layer.info.threshold).write_packet_fallible(packet),
        )
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.layer.messages.send_local_infallible(
            LocalMsg::RadiusAtExcept {
                center: self.center,
                radius_squared: self.radius_squared,
                except: self.except,
            },
            |b| b.extend_from_slice(bytes),
        );
    }
}
