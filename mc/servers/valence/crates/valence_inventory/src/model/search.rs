use super::*;

impl Inventory {
    /// Returns the first empty slot in the given range, or `None` if there are
    /// no empty slots in the range.
    ///
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::*;
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// inv.set_slot(2, ItemStack::new(ItemKind::GoldIngot, 1, None));
    /// inv.set_slot(3, ItemStack::new(ItemKind::IronIngot, 1, None));
    /// assert_eq!(inv.first_empty_slot_in(0..6), Some(1));
    /// assert_eq!(inv.first_empty_slot_in(2..6), Some(4));
    /// ```
    #[track_caller]
    #[must_use]
    pub fn first_empty_slot_in(&self, mut range: std::ops::Range<u16>) -> Option<u16> {
        assert!(
            (0..=self.slot_count()).contains(&range.start),
            "slot range start out of range"
        );
        assert!(
            (0..=self.slot_count()).contains(&range.end),
            "slot range end out of range"
        );

        range.find(|&idx| self.slots[usize::from(idx)].is_empty())
    }

    /// Returns the first empty slot in the inventory, or `None` if there are no
    /// empty slots.
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::*;
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// inv.set_slot(2, ItemStack::new(ItemKind::GoldIngot, 1, None));
    /// inv.set_slot(3, ItemStack::new(ItemKind::IronIngot, 1, None));
    /// assert_eq!(inv.first_empty_slot(), Some(1));
    /// ```
    #[inline]
    pub fn first_empty_slot(&self) -> Option<u16> {
        self.first_empty_slot_in(0..self.slot_count())
    }

    /// Returns the first slot with the given [`ItemKind`] in the inventory
    /// where `count < stack_max`, or `None` if there are no empty slots.
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::*;
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// inv.set_slot(2, ItemStack::new(ItemKind::GoldIngot, 64, None));
    /// inv.set_slot(3, ItemStack::new(ItemKind::IronIngot, 1, None));
    /// inv.set_slot(4, ItemStack::new(ItemKind::GoldIngot, 1, None));
    /// assert_eq!(
    ///     inv.first_slot_with_item_in(ItemKind::GoldIngot, 64, 0..5),
    ///     Some(4)
    /// );
    /// ```
    pub fn first_slot_with_item_in(
        &self,
        item: ItemKind,
        stack_max: i8,
        mut range: std::ops::Range<u16>,
    ) -> Option<u16> {
        assert!(
            (0..=self.slot_count()).contains(&range.start),
            "slot range start out of range"
        );
        assert!(
            (0..=self.slot_count()).contains(&range.end),
            "slot range end out of range"
        );
        assert!(stack_max > 0, "stack_max must be greater than 0");

        range.find(|&idx| {
            let stack = &self.slots[usize::from(idx)];
            stack.item == item && stack.count < stack_max
        })
    }

    /// Returns the first slot with the given [`ItemKind`] in the inventory
    /// where `count < stack_max`, or `None` if there are no empty slots.
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::*;
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// inv.set_slot(2, ItemStack::new(ItemKind::GoldIngot, 64, None));
    /// inv.set_slot(3, ItemStack::new(ItemKind::IronIngot, 1, None));
    /// inv.set_slot(4, ItemStack::new(ItemKind::GoldIngot, 1, None));
    /// assert_eq!(inv.first_slot_with_item(ItemKind::GoldIngot, 64), Some(4));
    /// ```
    #[inline]
    pub fn first_slot_with_item(&self, item: ItemKind, stack_max: i8) -> Option<u16> {
        self.first_slot_with_item_in(item, stack_max, 0..self.slot_count())
    }
}
