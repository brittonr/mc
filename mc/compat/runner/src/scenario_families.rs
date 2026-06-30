use super::{Scenario, ScenarioLiveCapability, ScenarioSpec};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScenarioFamily {
    Baseline,
    Ctf,
    Inventory,
    Survival,
    CombatProjectileEquipment,
    Negative,
    Mcp,
    TargetedPacketLiveCapability,
}

const REQUIRED_SCENARIO_FAMILIES: &[ScenarioFamily] = &[
    ScenarioFamily::Ctf,
    ScenarioFamily::Inventory,
    ScenarioFamily::Survival,
    ScenarioFamily::CombatProjectileEquipment,
    ScenarioFamily::Negative,
    ScenarioFamily::Mcp,
];
const TARGETED_PACKET_CAPABILITY_FAMILY: ScenarioFamily =
    ScenarioFamily::TargetedPacketLiveCapability;

pub(crate) fn scenario_family(scenario: Scenario) -> ScenarioFamily {
    match scenario {
        Scenario::Smoke => ScenarioFamily::Baseline,
        Scenario::CompatBotProbe
        | Scenario::FlagScoreRepeat
        | Scenario::BlueFlagScore
        | Scenario::FlagCarrierDeathReturn
        | Scenario::ReconnectFlagState
        | Scenario::ReconnectFlagScore
        | Scenario::MultiClientLoadScore
        | Scenario::CtfInvalidPickupOwnership
        | Scenario::CtfInvalidReturnDrop
        | Scenario::CtfInvalidOpponentBaseReturnDrop
        | Scenario::CtfScoreLimitWinCondition
        | Scenario::CtfSimultaneousPickupCaptureRace
        | Scenario::CtfSpawnTeamBalanceReset => ScenarioFamily::Ctf,
        Scenario::InventoryInteraction
        | Scenario::InventoryStackSplitMerge
        | Scenario::InventoryDragTransactions => ScenarioFamily::Inventory,
        Scenario::SurvivalBreakPlacePickup
        | Scenario::SurvivalChestPersistence
        | Scenario::SurvivalCraftingTable
        | Scenario::SurvivalCraftingRecipeBreadth
        | Scenario::SurvivalFurnacePersistence
        | Scenario::SurvivalFurnaceSmeltingBreadth
        | Scenario::SurvivalHungerFood
        | Scenario::SurvivalHungerHealthCycle
        | Scenario::SurvivalMobDrop
        | Scenario::SurvivalMobAiLootBreadth
        | Scenario::SurvivalRedstoneToggle
        | Scenario::SurvivalRedstoneCircuitBreadth
        | Scenario::SurvivalWorldPersistenceRestart
        | Scenario::SurvivalWorldMultichunkDurability
        | Scenario::SurvivalCrashRecoveryParity
        | Scenario::SurvivalBlockEntityPersistenceParity
        | Scenario::SurvivalContainerBlockEntityBreadth
        | Scenario::SurvivalBiomeDimensionState
        | Scenario::SurvivalBiomeDimensionTravel
        | Scenario::SurvivalSignEditingLive => ScenarioFamily::Survival,
        Scenario::CombatDamage
        | Scenario::CombatKnockback
        | Scenario::VanillaCombatReferenceParity
        | Scenario::VanillaCombatArmorReferenceParity
        | Scenario::ArmorEquipmentMitigation
        | Scenario::ArmorLoadoutEnchantmentStatusMatrix
        | Scenario::EquipmentUpdateObservation
        | Scenario::EquipmentSlotItemMatrixExpansion
        | Scenario::ProjectileHit
        | Scenario::ProjectileDamageAttribution => ScenarioFamily::CombatProjectileEquipment,
        Scenario::NegativeInventoryStaleState
        | Scenario::NegativeInventoryInvalidClick
        | Scenario::NegativeCustomPayload
        | Scenario::NegativeReconnectRace
        | Scenario::NegativeCtfWrongScore => ScenarioFamily::Negative,
        Scenario::McpControlledSmoke => ScenarioFamily::Mcp,
    }
}

pub(crate) fn scenario_family_name(family: ScenarioFamily) -> &'static str {
    match family {
        ScenarioFamily::Baseline => "baseline",
        ScenarioFamily::Ctf => "ctf",
        ScenarioFamily::Inventory => "inventory",
        ScenarioFamily::Survival => "survival",
        ScenarioFamily::CombatProjectileEquipment => "combat-projectile-equipment",
        ScenarioFamily::Negative => "negative",
        ScenarioFamily::Mcp => "mcp",
        ScenarioFamily::TargetedPacketLiveCapability => "targeted-packet-live-capability",
    }
}

pub(super) fn validate_scenario_family_coverage(specs: &[ScenarioSpec]) -> Result<(), String> {
    for required in REQUIRED_SCENARIO_FAMILIES {
        if specs
            .iter()
            .any(|spec| scenario_family(spec.scenario) == *required)
        {
            continue;
        }
        return Err(format!(
            "scenario family {} has no scenario specs",
            scenario_family_name(*required)
        ));
    }
    Ok(())
}

pub(super) fn validate_targeted_packet_capability_family_coverage(
    capabilities: &[ScenarioLiveCapability],
) -> Result<(), String> {
    if capabilities.is_empty() {
        return Err(format!(
            "scenario family {} has no live capability rows",
            scenario_family_name(TARGETED_PACKET_CAPABILITY_FAMILY)
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PACKET_ROWS: &[&str] = &[];
    const TEST_REQUIRED_SIGNALS: &[&str] = &[];
    const TEST_REQUIRED_NONCLAIMS: &[&str] = &[];
    const TEST_CAPABILITY: ScenarioLiveCapability = ScenarioLiveCapability {
        scenario: "test-scenario",
        targeted_row: "test-row",
        packet_rows: TEST_PACKET_ROWS,
        capability_kind: "test-kind",
        backend_path: "test-backend",
        client_path: "test-client",
        evidence_mode: "test-mode",
        required_signals: TEST_REQUIRED_SIGNALS,
        required_nonclaims: TEST_REQUIRED_NONCLAIMS,
        blocker_reason: None,
    };

    #[test]
    fn targeted_packet_family_accepts_nonempty_capability_rows() {
        validate_targeted_packet_capability_family_coverage(&[TEST_CAPABILITY])
            .expect("nonempty targeted-packet family accepted");
    }

    #[test]
    fn targeted_packet_family_rejects_empty_capability_rows() {
        let err = validate_targeted_packet_capability_family_coverage(&[]).unwrap_err();
        assert!(err.contains("targeted-packet-live-capability"), "{err}");
    }
}
