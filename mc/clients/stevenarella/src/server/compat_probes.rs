#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProbeEnablement {
    pub(crate) active: bool,
    pub(crate) team: bool,
    pub(crate) combat: bool,
    pub(crate) respawn: bool,
    pub(crate) inventory: bool,
    pub(crate) inventory_stack_split_merge: bool,
    pub(crate) inventory_drag_transactions: bool,
    pub(crate) equipment: bool,
    pub(crate) projectile: bool,
    pub(crate) flag: bool,
    pub(crate) score_limit: bool,
    pub(crate) survival: bool,
    pub(crate) survival_chest: bool,
    pub(crate) survival_crafting: bool,
    pub(crate) survival_crafting_breadth: bool,
    pub(crate) survival_furnace: bool,
    pub(crate) survival_furnace_smelting_breadth: bool,
    pub(crate) survival_hunger_food: bool,
    pub(crate) survival_hunger_health: bool,
    pub(crate) survival_mob_drop: bool,
    pub(crate) survival_mob_ai_loot: bool,
    pub(crate) survival_redstone_toggle: bool,
    pub(crate) survival_redstone_circuit: bool,
    pub(crate) survival_world_persistence: bool,
    pub(crate) survival_block_entity: bool,
    pub(crate) survival_world_multichunk: bool,
    pub(crate) survival_container_block_entity: bool,
    pub(crate) survival_biome_dimension_travel: bool,
    pub(crate) survival_sign_editing: bool,
}

impl ProbeEnablement {
    pub(crate) fn any_enabled(self) -> bool {
        self.active
            || self.team
            || self.combat
            || self.respawn
            || self.inventory
            || self.inventory_stack_split_merge
            || self.inventory_drag_transactions
            || self.equipment
            || self.projectile
            || self.flag
            || self.score_limit
            || self.survival
            || self.survival_chest
            || self.survival_crafting
            || self.survival_crafting_breadth
            || self.survival_furnace
            || self.survival_furnace_smelting_breadth
            || self.survival_hunger_food
            || self.survival_hunger_health
            || self.survival_mob_drop
            || self.survival_mob_ai_loot
            || self.survival_redstone_toggle
            || self.survival_redstone_circuit
            || self.survival_world_persistence
            || self.survival_block_entity
            || self.survival_world_multichunk
            || self.survival_container_block_entity
            || self.survival_biome_dimension_travel
            || self.survival_sign_editing
    }

    pub(crate) fn movement_probe_enabled(self, stationary_combat_probe_enabled: bool) -> bool {
        !stationary_combat_probe_enabled
            && (self.active
                || self.team
                || self.combat
                || self.respawn
                || self.inventory
                || self.inventory_stack_split_merge
                || self.inventory_drag_transactions
                || self.equipment
                || self.projectile
                || self.flag
                || self.score_limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn disabled() -> ProbeEnablement {
        ProbeEnablement {
            active: false,
            team: false,
            combat: false,
            respawn: false,
            inventory: false,
            inventory_stack_split_merge: false,
            inventory_drag_transactions: false,
            equipment: false,
            projectile: false,
            flag: false,
            score_limit: false,
            survival: false,
            survival_chest: false,
            survival_crafting: false,
            survival_crafting_breadth: false,
            survival_furnace: false,
            survival_furnace_smelting_breadth: false,
            survival_hunger_food: false,
            survival_hunger_health: false,
            survival_mob_drop: false,
            survival_mob_ai_loot: false,
            survival_redstone_toggle: false,
            survival_redstone_circuit: false,
            survival_world_persistence: false,
            survival_block_entity: false,
            survival_world_multichunk: false,
            survival_container_block_entity: false,
            survival_biome_dimension_travel: false,
            survival_sign_editing: false,
        }
    }

    #[test]
    fn detects_enabled_probe_family() {
        let enabled = ProbeEnablement {
            survival_block_entity: true,
            ..disabled()
        };

        assert!(enabled.any_enabled());
        assert!(!disabled().any_enabled());
    }

    #[test]
    fn movement_probe_excludes_stationary_combat() {
        let enabled = ProbeEnablement {
            combat: true,
            ..disabled()
        };

        assert!(enabled.movement_probe_enabled(false));
        assert!(!enabled.movement_probe_enabled(true));
        assert!(!disabled().movement_probe_enabled(false));
    }
}
