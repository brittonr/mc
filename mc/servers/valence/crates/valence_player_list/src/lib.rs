#![doc = include_str!("../README.md")]

mod systems;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_server::text::IntoText;

type DisplayText = valence_server::Text;

type GameMode = valence_server::GameMode;
type Ping = valence_server::keepalive::Ping;
type Properties = valence_server::client::Properties;
type UniqueId = valence_server::UniqueId;
type Username = valence_server::client::Username;

pub struct PlayerListPlugin;

#[derive(SystemSet, Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct PlayerListSet;

impl Plugin for PlayerListPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerList::new())
            .configure_sets(
                PostUpdate,
                // Needs to happen before player entities are initialized. Otherwise, they will
                // appear invisible.
                PlayerListSet.before(valence_server::layer::UpdateLayersPreClientSet),
            )
            .add_systems(
                PostUpdate,
                (
                    systems::update_header_footer,
                    systems::add_new_clients_to_player_list,
                    apply_deferred, // So new clients get the packets for their own entry.
                    systems::update_entries,
                    systems::init_player_list_for_clients,
                    systems::remove_despawned_entries,
                    systems::write_player_list_changes,
                )
                    .in_set(PlayerListSet)
                    .chain(),
            );
    }
}

#[derive(Resource)]
pub struct PlayerList {
    cached_update_packets: Vec<u8>,
    header: DisplayText,
    footer: DisplayText,
    changed_header_or_footer: bool,
    /// If clients should be automatically added and removed from the player
    /// list with the proper components inserted. Enabled by default.
    pub manage_clients: bool,
}

impl PlayerList {
    fn new() -> Self {
        Self {
            cached_update_packets: vec![],
            header: DisplayText::text(""),
            footer: DisplayText::text(""),
            changed_header_or_footer: false,
            manage_clients: true,
        }
    }

    pub fn header(&self) -> &DisplayText {
        &self.header
    }

    pub fn footer(&self) -> &DisplayText {
        &self.footer
    }

    pub fn set_header<'a, T: IntoText<'a>>(&mut self, txt: T) {
        let txt = txt.into_cow_text().into_owned();

        if txt != self.header {
            self.changed_header_or_footer = true;
        }

        self.header = txt;
    }

    pub fn set_footer<'a, T: IntoText<'a>>(&mut self, txt: T) {
        let txt = txt.into_cow_text().into_owned();

        if txt != self.footer {
            self.changed_header_or_footer = true;
        }

        self.footer = txt;
    }
}

/// Bundle for spawning new player list entries. All components are required
/// unless otherwise stated.
///
/// # Despawning player list entries
///
/// The [`Despawned`] component must be used to despawn player list entries.
#[derive(Bundle, Default, Debug)]
pub struct PlayerListEntryBundle {
    pub player_list_entry: PlayerListEntry,
    /// Careful not to modify this!
    pub uuid: UniqueId,
    pub username: Username,
    pub properties: Properties,
    pub game_mode: GameMode,
    pub ping: Ping,
    pub display_name: DisplayName,
    pub listed: Listed,
}

/// Marker component for player list entries.
#[derive(Component, Default, Debug)]
pub struct PlayerListEntry;

/// Displayed name for a player list entry. Appears as [`Username`] if `None`.
#[derive(Component, Default, Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct DisplayName(pub Option<DisplayText>);

/// If a player list entry is visible. Defaults to `true`.
#[derive(Component, Copy, Clone, Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct Listed(pub bool);

impl Default for Listed {
    fn default() -> Self {
        Self(true)
    }
}
