use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InventoryKind {
    Generic9x1,
    Generic9x2,
    Generic9x3,
    Generic9x4,
    Generic9x5,
    Generic9x6,
    Generic3x3,
    Anvil,
    Beacon,
    BlastFurnace,
    BrewingStand,
    Crafting,
    Enchantment,
    Furnace,
    Grindstone,
    Hopper,
    Lectern,
    Loom,
    Merchant,
    ShulkerBox,
    Smithing,
    Smoker,
    Cartography,
    Stonecutter,
    Player,
}

impl InventoryKind {
    const ANVIL_SLOT_COUNT: u16 = 4;
    const BEACON_SLOT_COUNT: u16 = 1;
    const BLAST_FURNACE_SLOT_COUNT: u16 = 3;
    const BREWING_STAND_SLOT_COUNT: u16 = 5;
    const CARTOGRAPHY_SLOT_COUNT: u16 = 3;
    const CRAFTING_SLOT_COUNT: u16 = 10;
    const ENCHANTMENT_SLOT_COUNT: u16 = 2;
    const FURNACE_SLOT_COUNT: u16 = 3;
    const GENERIC_3X3_SLOT_COUNT: u16 = 9;
    const GENERIC_9X1_SLOT_COUNT: u16 = 9;
    const GENERIC_9X2_SLOT_COUNT: u16 = 18;
    const GENERIC_9X3_SLOT_COUNT: u16 = 27;
    const GENERIC_9X4_SLOT_COUNT: u16 = 36;
    const GENERIC_9X5_SLOT_COUNT: u16 = 45;
    const GENERIC_9X6_SLOT_COUNT: u16 = 54;
    const GRINDSTONE_SLOT_COUNT: u16 = 3;
    const HOPPER_SLOT_COUNT: u16 = 5;
    const LECTERN_SLOT_COUNT: u16 = 1;
    const LOOM_SLOT_COUNT: u16 = 4;
    const MERCHANT_SLOT_COUNT: u16 = 3;
    const PLAYER_SLOT_COUNT: u16 = 46;
    const SHULKER_BOX_SLOT_COUNT: u16 = 27;
    const SMITHING_SLOT_COUNT: u16 = 3;
    const SMOKER_SLOT_COUNT: u16 = 3;
    const STONECUTTER_SLOT_COUNT: u16 = 2;

    /// The number of slots in this inventory. When the inventory is shown to
    /// clients, this number does not include the player's main inventory slots.
    pub const fn slot_count(self) -> u16 {
        match self {
            InventoryKind::Generic9x1 => Self::GENERIC_9X1_SLOT_COUNT,
            InventoryKind::Generic9x2 => Self::GENERIC_9X2_SLOT_COUNT,
            InventoryKind::Generic9x3 => Self::GENERIC_9X3_SLOT_COUNT,
            InventoryKind::Generic9x4 => Self::GENERIC_9X4_SLOT_COUNT,
            InventoryKind::Generic9x5 => Self::GENERIC_9X5_SLOT_COUNT,
            InventoryKind::Generic9x6 => Self::GENERIC_9X6_SLOT_COUNT,
            InventoryKind::Generic3x3 => Self::GENERIC_3X3_SLOT_COUNT,
            InventoryKind::Anvil => Self::ANVIL_SLOT_COUNT,
            InventoryKind::Beacon => Self::BEACON_SLOT_COUNT,
            InventoryKind::BlastFurnace => Self::BLAST_FURNACE_SLOT_COUNT,
            InventoryKind::BrewingStand => Self::BREWING_STAND_SLOT_COUNT,
            InventoryKind::Crafting => Self::CRAFTING_SLOT_COUNT,
            InventoryKind::Enchantment => Self::ENCHANTMENT_SLOT_COUNT,
            InventoryKind::Furnace => Self::FURNACE_SLOT_COUNT,
            InventoryKind::Grindstone => Self::GRINDSTONE_SLOT_COUNT,
            InventoryKind::Hopper => Self::HOPPER_SLOT_COUNT,
            InventoryKind::Lectern => Self::LECTERN_SLOT_COUNT,
            InventoryKind::Loom => Self::LOOM_SLOT_COUNT,
            InventoryKind::Merchant => Self::MERCHANT_SLOT_COUNT,
            InventoryKind::ShulkerBox => Self::SHULKER_BOX_SLOT_COUNT,
            InventoryKind::Smithing => Self::SMITHING_SLOT_COUNT,
            InventoryKind::Smoker => Self::SMOKER_SLOT_COUNT,
            InventoryKind::Cartography => Self::CARTOGRAPHY_SLOT_COUNT,
            InventoryKind::Stonecutter => Self::STONECUTTER_SLOT_COUNT,
            InventoryKind::Player => Self::PLAYER_SLOT_COUNT,
        }
    }
}

impl From<InventoryKind> for WindowType {
    fn from(value: InventoryKind) -> Self {
        match value {
            InventoryKind::Generic9x1 => WindowType::Generic9x1,
            InventoryKind::Generic9x2 => WindowType::Generic9x2,
            InventoryKind::Generic9x3 => WindowType::Generic9x3,
            InventoryKind::Generic9x4 => WindowType::Generic9x4,
            InventoryKind::Generic9x5 => WindowType::Generic9x5,
            InventoryKind::Generic9x6 => WindowType::Generic9x6,
            InventoryKind::Generic3x3 => WindowType::Generic3x3,
            InventoryKind::Anvil => WindowType::Anvil,
            InventoryKind::Beacon => WindowType::Beacon,
            InventoryKind::BlastFurnace => WindowType::BlastFurnace,
            InventoryKind::BrewingStand => WindowType::BrewingStand,
            InventoryKind::Crafting => WindowType::Crafting,
            InventoryKind::Enchantment => WindowType::Enchantment,
            InventoryKind::Furnace => WindowType::Furnace,
            InventoryKind::Grindstone => WindowType::Grindstone,
            InventoryKind::Hopper => WindowType::Hopper,
            InventoryKind::Lectern => WindowType::Lectern,
            InventoryKind::Loom => WindowType::Loom,
            InventoryKind::Merchant => WindowType::Merchant,
            InventoryKind::ShulkerBox => WindowType::ShulkerBox,
            InventoryKind::Smithing => WindowType::Smithing,
            InventoryKind::Smoker => WindowType::Smoker,
            InventoryKind::Cartography => WindowType::Cartography,
            InventoryKind::Stonecutter => WindowType::Stonecutter,
            // arbitrarily chosen, because a player inventory technically does not have a window
            // type
            InventoryKind::Player => WindowType::Generic9x4,
        }
    }
}

impl From<WindowType> for InventoryKind {
    fn from(value: WindowType) -> Self {
        match value {
            WindowType::Generic9x1 => InventoryKind::Generic9x1,
            WindowType::Generic9x2 => InventoryKind::Generic9x2,
            WindowType::Generic9x3 => InventoryKind::Generic9x3,
            WindowType::Generic9x4 => InventoryKind::Generic9x4,
            WindowType::Generic9x5 => InventoryKind::Generic9x5,
            WindowType::Generic9x6 => InventoryKind::Generic9x6,
            WindowType::Generic3x3 => InventoryKind::Generic3x3,
            WindowType::Anvil => InventoryKind::Anvil,
            WindowType::Beacon => InventoryKind::Beacon,
            WindowType::BlastFurnace => InventoryKind::BlastFurnace,
            WindowType::BrewingStand => InventoryKind::BrewingStand,
            WindowType::Crafting => InventoryKind::Crafting,
            WindowType::Enchantment => InventoryKind::Enchantment,
            WindowType::Furnace => InventoryKind::Furnace,
            WindowType::Grindstone => InventoryKind::Grindstone,
            WindowType::Hopper => InventoryKind::Hopper,
            WindowType::Lectern => InventoryKind::Lectern,
            WindowType::Loom => InventoryKind::Loom,
            WindowType::Merchant => InventoryKind::Merchant,
            WindowType::ShulkerBox => InventoryKind::ShulkerBox,
            WindowType::Smithing => InventoryKind::Smithing,
            WindowType::Smoker => InventoryKind::Smoker,
            WindowType::Cartography => InventoryKind::Cartography,
            WindowType::Stonecutter => InventoryKind::Stonecutter,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub struct InventorySettings {
    pub validate_actions: bool,
}

impl Default for InventorySettings {
    fn default() -> Self {
        Self {
            validate_actions: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_player_slot() {
        assert_eq!(convert_to_player_slot_id(InventoryKind::Generic9x3, 27), 9);
        assert_eq!(convert_to_player_slot_id(InventoryKind::Generic9x3, 36), 18);
        assert_eq!(convert_to_player_slot_id(InventoryKind::Generic9x3, 54), 36);
        assert_eq!(convert_to_player_slot_id(InventoryKind::Generic9x1, 9), 9);
    }

    #[test]
    fn test_convert_hotbar_slot_id() {
        assert_eq!(PlayerInventory::hotbar_to_slot(0), 36);
        assert_eq!(PlayerInventory::hotbar_to_slot(4), 40);
        assert_eq!(PlayerInventory::hotbar_to_slot(8), 44);
    }
}
