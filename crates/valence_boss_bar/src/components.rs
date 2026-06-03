use bevy_ecs::prelude::*;

/// The bundle of components that make up a boss bar.
#[derive(Bundle, Default)]
pub struct BossBarBundle {
    pub id: valence_server::UniqueId,
    pub title: BossBarTitle,
    pub health: BossBarHealth,
    pub style: BossBarStyle,
    pub flags: valence_server::protocol::packets::play::BossBarFlags,
    pub layer: valence_entity::EntityLayerId,
}

/// The title of a boss bar.
#[derive(Component, Clone, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct BossBarTitle(pub valence_server::Text);

impl ToPacketAction for BossBarTitle {
    fn to_packet_action(&self) -> valence_server::protocol::packets::play::BossBarAction<'_> {
        valence_server::protocol::packets::play::BossBarAction::UpdateTitle(
            std::borrow::Cow::Borrowed(&self.0),
        )
    }
}

/// The health of a boss bar.
#[derive(Component, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct BossBarHealth(pub f32);

impl ToPacketAction for BossBarHealth {
    fn to_packet_action(&self) -> valence_server::protocol::packets::play::BossBarAction<'_> {
        valence_server::protocol::packets::play::BossBarAction::UpdateHealth(self.0)
    }
}

/// The style of a boss bar. This includes the color and division of the boss
/// bar.
#[derive(Component, Default)]
pub struct BossBarStyle {
    pub color: valence_server::protocol::packets::play::BossBarColor,
    pub division: valence_server::protocol::packets::play::BossBarDivision,
}

impl ToPacketAction for BossBarStyle {
    fn to_packet_action(&self) -> valence_server::protocol::packets::play::BossBarAction<'_> {
        valence_server::protocol::packets::play::BossBarAction::UpdateStyle(
            self.color,
            self.division,
        )
    }
}

impl ToPacketAction for valence_server::protocol::packets::play::BossBarFlags {
    fn to_packet_action(&self) -> valence_server::protocol::packets::play::BossBarAction<'_> {
        valence_server::protocol::packets::play::BossBarAction::UpdateFlags(*self)
    }
}

/// Trait for converting a component to a boss bar action.
pub(crate) trait ToPacketAction {
    fn to_packet_action(&self) -> valence_server::protocol::packets::play::BossBarAction<'_>;
}
