type SlotRange = std::ops::RangeInclusive<u16>;

// Public API compatibility keeps the historical `PlayerInventory` name.
#[allow(path_segment_repetition)]
pub struct PlayerInventory;

impl PlayerInventory {
    pub const HOTBAR_INDEX_MAX: u16 = 8;
    pub const MAIN_SIZE: u16 = 36;
    pub const SLOT_CHEST: u16 = 6;
    pub const SLOT_CRAFT_RESULT: u16 = 0;
    pub const SLOT_FEET: u16 = 8;
    pub const SLOT_HEAD: u16 = 5;
    pub const SLOT_LEGS: u16 = 7;
    pub const SLOT_OFFHAND: u16 = 45;
    pub const SLOTS_CRAFT_INPUT: SlotRange = 1..=4;
    pub const SLOTS_HOTBAR: SlotRange = Self::SLOTS_HOTBAR_START..=Self::SLOTS_HOTBAR_END;
    pub const SLOTS_HOTBAR_END: u16 = 44;
    pub const SLOTS_HOTBAR_START: u16 = 36;
    pub const SLOTS_MAIN: SlotRange = Self::SLOTS_MAIN_START..=Self::SLOTS_MAIN_END;
    pub const SLOTS_MAIN_END: u16 = 44;
    pub const SLOTS_MAIN_START: u16 = 9;

    pub fn hotbar_to_slot(hotbar: u8) -> u16 {
        Self::SLOTS_HOTBAR_START
            .checked_add(u16::from(hotbar))
            .unwrap_or(Self::SLOTS_HOTBAR_END)
    }

    pub fn slot_to_hotbar(slot: u16) -> u8 {
        debug_assert!(Self::SLOTS_HOTBAR.contains(&slot));
        let hotbar_index = slot.saturating_sub(Self::SLOTS_HOTBAR_START);
        let Ok(hotbar_index) = u8::try_from(hotbar_index) else {
            unreachable!();
        };
        hotbar_index
    }
}
