use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_protocol::encode::WritePacket;
pub use valence_protocol::packets::play::synchronize_tags_s2c::RegistryMap;

type PacketWriter<'a> = valence_protocol::encode::PacketWriter<'a>;
type Server = valence_server_common::Server;
type SynchronizeTagsS2c<'a> = valence_protocol::packets::play::SynchronizeTagsS2c<'a>;

#[derive(Debug, Resource, Default)]
pub struct TagsRegistry {
    pub registries: RegistryMap,
    cached_packet: Vec<u8>,
}

pub(super) fn build(app: &mut App) {
    app.init_resource::<TagsRegistry>()
        .add_systems(PreStartup, init_tags_registry)
        .add_systems(PostUpdate, cache_tags_packet.in_set(crate::RegistrySet));
}

impl TagsRegistry {
    fn build_synchronize_tags(&self) -> SynchronizeTagsS2c<'_> {
        SynchronizeTagsS2c {
            groups: std::borrow::Cow::Borrowed(&self.registries),
        }
    }

    /// Returns bytes of the cached [`SynchronizeTagsS2c`] packet.
    pub fn sync_tags_packet(&self) -> &[u8] {
        &self.cached_packet
    }
}

fn init_tags_registry(mut tags: ResMut<TagsRegistry>) {
    let Ok(registries) =
        serde_json::from_str::<RegistryMap>(include_str!("../extracted/tags.json"))
    else {
        return;
    };

    tags.registries = registries;
}

pub(crate) fn cache_tags_packet(server: Res<Server>, tags: ResMut<TagsRegistry>) {
    if tags.is_changed() {
        let tags = tags.into_inner();
        let packet = tags.build_synchronize_tags();
        let mut bytes = vec![];
        let mut writer = PacketWriter::new(&mut bytes, server.compression_threshold());

        writer.write_packet(&packet);
        tags.cached_packet = bytes;
    }
}

#[cfg(test)]
mod tests {
    /* TODO: move this to src/tests/
    #[test]
    fn smoke_test() {
        let mut app = bevy_app::App::new();
        app.add_plugins(RegistryPlugin);
        // app.insert_resource(Server::default());
        app.update();

        let tags_registry = app.world.get_resource::<TagsRegistry>().unwrap();
        let packet = tags_registry.build_synchronize_tags();
        assert!(!packet.registries.is_empty());
        assert!(!tags_registry.cached_packet.is_empty());
    }
    */
}
