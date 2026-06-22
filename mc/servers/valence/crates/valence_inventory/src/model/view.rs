use super::*;

/// A helper to represent the inventory window that the player is currently
/// viewing. Handles dispatching reads to the correct inventory.
///
/// This is a read-only version of [`InventoryWindowMut`].
///
/// ```
/// # use valence_inventory::*;
/// # use valence_server::item::*;
/// let mut player_inventory = Inventory::new(InventoryKind::Player);
/// player_inventory.set_slot(36, ItemStack::new(ItemKind::Diamond, 1, None));
///
/// let target_inventory = Inventory::new(InventoryKind::Generic9x3);
/// let window = InventoryWindow::new(&player_inventory, Some(&target_inventory));
///
/// assert_eq!(window.slot(54), &ItemStack::new(ItemKind::Diamond, 1, None));
/// ```
pub struct InventoryWindow<'a> {
    pub(crate) player_inventory: &'a Inventory,
    pub(crate) open_inventory: Option<&'a Inventory>,
}

impl<'a> InventoryWindow<'a> {
    pub fn new(player_inventory: &'a Inventory, open_inventory: Option<&'a Inventory>) -> Self {
        Self {
            player_inventory,
            open_inventory,
        }
    }

    #[track_caller]
    pub fn slot(&self, idx: u16) -> &ItemStack {
        if let Some(open_inv) = self.open_inventory.as_ref() {
            if idx < open_inv.slot_count() {
                open_inv.slot(idx)
            } else {
                self.player_inventory
                    .slot(convert_to_player_slot_id(open_inv.kind(), idx))
            }
        } else {
            self.player_inventory.slot(idx)
        }
    }

    #[track_caller]
    pub fn slot_count(&self) -> u16 {
        if let Some(open_inv) = &self.open_inventory {
            // when the window is split, we can only access the main slots of player's
            // inventory
            let Some(count) = PlayerInventory::MAIN_SIZE.checked_add(open_inv.slot_count()) else {
                unreachable!();
            };
            count
        } else {
            self.player_inventory.slot_count()
        }
    }
}

/// A helper to represent the inventory window that the player is currently
/// viewing. Handles dispatching reads/writes to the correct inventory.
///
/// This is a writable version of [`InventoryWindow`].
///
/// ```
/// # use valence_inventory::*;
/// # use valence_server::item::*;
/// let mut player_inventory = Inventory::new(InventoryKind::Player);
/// let mut target_inventory = Inventory::new(InventoryKind::Generic9x3);
/// let mut window = InventoryWindowMut::new(&mut player_inventory, Some(&mut target_inventory));
///
/// window.set_slot(54, ItemStack::new(ItemKind::Diamond, 1, None));
///
/// assert_eq!(
///     player_inventory.slot(36),
///     &ItemStack::new(ItemKind::Diamond, 1, None)
/// );
/// ```
pub struct InventoryWindowMut<'a> {
    pub(crate) player_inventory: &'a mut Inventory,
    pub(crate) open_inventory: Option<&'a mut Inventory>,
}

impl<'a> InventoryWindowMut<'a> {
    pub fn new(
        player_inventory: &'a mut Inventory,
        open_inventory: Option<&'a mut Inventory>,
    ) -> Self {
        Self {
            player_inventory,
            open_inventory,
        }
    }

    #[track_caller]
    pub fn slot(&self, idx: u16) -> &ItemStack {
        if let Some(open_inv) = self.open_inventory.as_ref() {
            if idx < open_inv.slot_count() {
                open_inv.slot(idx)
            } else {
                self.player_inventory
                    .slot(convert_to_player_slot_id(open_inv.kind(), idx))
            }
        } else {
            self.player_inventory.slot(idx)
        }
    }

    #[track_caller]
    #[must_use]
    pub fn replace_slot<I: Into<ItemStack>>(&mut self, idx: u16, item: I) -> ItemStack {
        assert!(idx < self.slot_count(), "slot index of {idx} out of bounds");

        if let Some(open_inv) = self.open_inventory.as_mut() {
            if idx < open_inv.slot_count() {
                open_inv.replace_slot(idx, item)
            } else {
                self.player_inventory
                    .replace_slot(convert_to_player_slot_id(open_inv.kind(), idx), item)
            }
        } else {
            self.player_inventory.replace_slot(idx, item)
        }
    }

    #[track_caller]
    #[inline]
    pub fn set_slot<I: Into<ItemStack>>(&mut self, idx: u16, item: I) {
        let _ = self.replace_slot(idx, item);
    }

    pub fn slot_count(&self) -> u16 {
        if let Some(open_inv) = &self.open_inventory {
            // when the window is split, we can only access the main slots of player's
            // inventory
            let Some(count) = PlayerInventory::MAIN_SIZE.checked_add(open_inv.slot_count()) else {
                unreachable!();
            };
            count
        } else {
            self.player_inventory.slot_count()
        }
    }
}
