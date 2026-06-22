use super::*;

/// Miscellaneous inventory data.
#[derive(Component, Debug)]
pub struct ClientInventoryState {
    /// The current window ID. Incremented when inventories are opened.
    pub(crate) window_id: u8,
    pub(crate) state_id: std::num::Wrapping<i32>,
    /// Tracks what slots have been changed by this client in this tick, so we
    /// don't need to send updates for them.
    pub(crate) slots_changed: u64,
    /// If `Some`: The item the user thinks they updated their cursor item to on
    /// the last tick.
    /// If `None`: the user did not update their cursor item in the last tick.
    /// This is so we can inform the user of the update through change detection
    /// when they differ in a given tick
    pub(crate) client_updated_cursor_item: Option<ItemStack>,
}

impl ClientInventoryState {
    #[doc(hidden)]
    pub fn window_id(&self) -> u8 {
        self.window_id
    }

    #[doc(hidden)]
    pub fn state_id(&self) -> std::num::Wrapping<i32> {
        self.state_id
    }
}

/// Indicates which hotbar slot the player is currently holding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, derive_more::Deref)]
pub struct HeldItem {
    pub(crate) held_item_slot: u16,
}

impl HeldItem {
    /// The slot ID of the currently held item, in the range 36-44 inclusive.
    /// This value is safe to use on the player's inventory directly.
    pub fn slot(&self) -> u16 {
        self.held_item_slot
    }

    pub fn hotbar_idx(&self) -> u8 {
        PlayerInventory::slot_to_hotbar(self.held_item_slot)
    }

    pub fn set_slot(&mut self, slot: u16) {
        // temp
        assert!(
            PlayerInventory::SLOTS_HOTBAR.contains(&slot),
            "slot index of {slot} out of bounds"
        );

        self.held_item_slot = slot;
    }

    pub fn set_hotbar_idx(&mut self, hotbar_idx: u8) {
        self.set_slot(PlayerInventory::hotbar_to_slot(hotbar_idx))
    }
}

/// The item stack that the client thinks it's holding under the mouse
/// cursor.
#[derive(
    Component, Clone, PartialEq, Default, Debug, derive_more::Deref, derive_more::DerefMut,
)]
pub struct CursorItem(pub ItemStack);

/// Used to indicate that the client with this component is currently viewing
/// an inventory.
#[derive(Component, Clone, Debug)]
pub struct OpenInventory {
    /// The entity with the `Inventory` component that the client is currently
    /// viewing.
    pub entity: Entity,
    pub(crate) client_changed: u64,
}

impl OpenInventory {
    pub fn new(entity: Entity) -> Self {
        OpenInventory {
            entity,
            client_changed: 0,
        }
    }
}
