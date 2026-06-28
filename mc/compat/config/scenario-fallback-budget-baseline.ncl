# Checked-in fallback budget baseline for `compat/config/scenario-manifest.ncl`.
# The Rust checker consumes this file as migration accounting only; it does not
# prove typed-event coverage, live compatibility, or semantic equivalence.

let FallbackBudgetEntry = {
  name | String,
  owner | String,
  reason | String,
  non_claim | String,
  next_action | String,
} in
let FallbackBudgetBaseline = {
  schema_version | Number,
  schema | String,
  fallback_rows | Array FallbackBudgetEntry,
  typed_event_ready_rows | Array String,
} in
let common_waiver = {
  owner = "mc-compat",
  reason = "legacy rows still rely on substring log evidence",
  non_claim = "typed-event migration changes observability only",
  next_action = "migrate rows when typed-event fixtures cover client server and forbidden surfaces",
} in
{
  schema_version = 1,
  schema = "mc.compat.scenario-fallback-budget-baseline.v1",
  typed_event_ready_rows = ["smoke", "flag-score-repeat", "blue-flag-score", "inventory-interaction", "inventory-stack-split-merge", "inventory-drag-transactions", "survival-break-place-pickup", "survival-chest-persistence", "survival-crafting-table", "survival-crafting-recipe-breadth", "survival-furnace-persistence", "survival-furnace-smelting-breadth", "survival-hunger-food", "survival-hunger-health-cycle", "survival-mob-drop", "survival-mob-ai-loot-breadth", "survival-redstone-toggle", "survival-redstone-circuit-breadth", "survival-world-persistence-restart", "survival-world-multichunk-durability", "survival-crash-recovery-parity", "survival-block-entity-persistence-parity", "survival-container-block-entity-breadth", "survival-biome-dimension-state", "survival-biome-dimension-travel", "survival-sign-editing-live", "mcp-controlled-smoke", "combat-damage", "combat-knockback", "armor-equipment-mitigation", "equipment-update-observation", "projectile-hit", "projectile-damage-attribution", "flag-carrier-death-return", "reconnect-flag-state", "ctf-invalid-pickup-ownership", "ctf-invalid-return-drop", "ctf-invalid-opponent-base-return-drop", "ctf-score-limit-win-condition", "ctf-simultaneous-pickup-capture-race", "ctf-spawn-team-balance-reset", "reconnect-flag-score", "multi-client-load-score"],
  fallback_rows = [
    { name = "valence-compat-bot-probe", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "vanilla-combat-reference-parity", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "vanilla-combat-armor-reference-parity", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
  ],
} | FallbackBudgetBaseline
