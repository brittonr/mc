use super::*;

#[derive(Debug, Clone, Component)]
pub struct Inventory {
    pub(crate) title: Text,
    pub(crate) kind: InventoryKind,
    pub(crate) slots: Box<[ItemStack]>,
    /// Contains a set bit for each modified slot in `slots`.
    #[doc(hidden)]
    pub changed: u64,
    /// Makes an inventory read-only for clients. This will prevent adding
    /// or removing items. If this is a player inventory
    /// This will also make it impossible to drop items while not
    /// in the inventory (e.g. by pressing Q)
    pub readonly: bool,
}

impl Inventory {
    pub fn new(kind: InventoryKind) -> Self {
        // TODO: default title to the correct translation key instead
        Self::with_title(kind, "Inventory")
    }

    pub fn with_title<'a, T: IntoText<'a>>(kind: InventoryKind, title: T) -> Self {
        Inventory {
            title: title.into_cow_text().into_owned(),
            kind,
            slots: vec![ItemStack::EMPTY; usize::from(kind.slot_count())].into(),
            changed: 0,
            readonly: false,
        }
    }

    #[track_caller]
    pub fn slot(&self, idx: u16) -> &ItemStack {
        assert!(idx < self.slot_count(), "slot index out of range");
        &self.slots[usize::from(idx)]
    }

    /// Sets the slot at the given index to the given item stack.
    ///
    /// See also [`Inventory::replace_slot`].
    ///
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::{ItemStack, ItemKind};
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// assert_eq!(inv.slot(0).item, ItemKind::Diamond);
    /// ```
    #[track_caller]
    #[inline]
    pub fn set_slot<I: Into<ItemStack>>(&mut self, idx: u16, item: I) {
        let _ = self.replace_slot(idx, item);
    }

    /// Replaces the slot at the given index with the given item stack, and
    /// returns the old stack in that slot.
    ///
    /// See also [`Inventory::set_slot`].
    ///
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::{ItemStack, ItemKind};
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// let old = inv.replace_slot(0, ItemStack::new(ItemKind::IronIngot, 1, None));
    /// assert_eq!(old.item, ItemKind::Diamond);
    /// ```
    #[track_caller]
    #[must_use]
    pub fn replace_slot<I: Into<ItemStack>>(&mut self, idx: u16, item: I) -> ItemStack {
        assert!(idx < self.slot_count(), "slot index of {idx} out of bounds");

        let new = item.into();
        let old = &mut self.slots[usize::from(idx)];

        if new != *old {
            self.changed |= 1 << idx;
        }

        std::mem::replace(old, new)
    }

    /// Swap the contents of two slots. If the slots are the same, nothing
    /// happens.
    ///
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::{ItemStack, ItemKind};
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// assert!(inv.slot(1).is_empty());
    /// inv.swap_slot(0, 1);
    /// assert_eq!(inv.slot(1).item, ItemKind::Diamond);
    /// ```
    #[track_caller]
    pub fn swap_slot(&mut self, idx_a: u16, idx_b: u16) {
        assert!(
            idx_a < self.slot_count(),
            "slot index of {idx_a} out of bounds"
        );
        assert!(
            idx_b < self.slot_count(),
            "slot index of {idx_b} out of bounds"
        );

        let slot_a = usize::from(idx_a);
        let slot_b = usize::from(idx_b);

        if idx_a == idx_b || self.slots[slot_a] == self.slots[slot_b] {
            // Nothing to do here, ignore.
            return;
        }

        self.changed |= 1 << idx_a;
        self.changed |= 1 << idx_b;

        self.slots.swap(slot_a, slot_b);
    }

    /// Set the amount of items in the given slot without replacing the slot
    /// entirely. Valid values are 1-127, inclusive, and `amount` will be
    /// clamped to this range. If the slot is empty, nothing happens.
    ///
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::{ItemStack, ItemKind};
    /// let mut inv = Inventory::new(InventoryKind::Generic9x1);
    /// inv.set_slot(0, ItemStack::new(ItemKind::Diamond, 1, None));
    /// inv.set_slot_amount(0, 64);
    /// assert_eq!(inv.slot(0).count, 64);
    /// ```
    #[track_caller]
    pub fn set_slot_amount(&mut self, idx: u16, amount: i8) {
        assert!(idx < self.slot_count(), "slot index out of range");

        let item = &mut self.slots[usize::from(idx)];

        if !item.is_empty() {
            if item.count == amount {
                return;
            }
            item.count = amount;
            self.changed |= 1 << idx;
        }
    }

    pub fn slot_count(&self) -> u16 {
        let Ok(slot_count) = u16::try_from(self.slots.len()) else {
            unreachable!();
        };
        slot_count
    }

    pub fn slots(
        &self,
    ) -> impl ExactSizeIterator<Item = &ItemStack> + DoubleEndedIterator + FusedIterator + Clone + '_
    {
        self.slots.iter()
    }

    pub fn kind(&self) -> InventoryKind {
        self.kind
    }

    /// The text displayed on the inventory's title bar.
    ///
    /// ```
    /// # use valence_inventory::*;
    /// # use valence_server::item::{ItemStack, ItemKind};
    /// # use valence_server::text::Text;
    /// let inv = Inventory::with_title(InventoryKind::Generic9x3, "Box of Holding");
    /// assert_eq!(inv.title(), &Text::from("Box of Holding"));
    /// ```
    pub fn title(&self) -> &Text {
        &self.title
    }

    /// Set the text displayed on the inventory's title bar.
    ///
    /// To get the old title, use [`Inventory::replace_title`].
    ///
    /// ```
    /// # use valence_inventory::*;
    /// let mut inv = Inventory::new(InventoryKind::Generic9x3);
    /// inv.set_title("Box of Holding");
    /// ```
    #[inline]
    pub fn set_title<'a, T: IntoText<'a>>(&mut self, title: T) {
        let _ = self.replace_title(title);
    }

    /// Replace the text displayed on the inventory's title bar, and returns the
    /// old text.
    #[must_use]
    pub fn replace_title<'a, T: IntoText<'a>>(&mut self, title: T) -> Text {
        // TODO: set title modified flag
        std::mem::replace(&mut self.title, title.into_cow_text().into_owned())
    }

    pub(crate) fn slot_slice(&self) -> &[ItemStack] {
        &self.slots
    }
}
