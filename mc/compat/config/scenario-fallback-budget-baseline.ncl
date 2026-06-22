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
  typed_event_ready_rows = ["smoke", "inventory-interaction", "inventory-stack-split-merge", "inventory-drag-transactions", "survival-break-place-pickup", "survival-crafting-table"],
  fallback_rows = [
    { name = "valence-compat-bot-probe", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "flag-score-repeat", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "blue-flag-score", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-chest-persistence", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-crafting-recipe-breadth", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-furnace-persistence", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-furnace-smelting-breadth", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-hunger-food", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-hunger-health-cycle", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-mob-drop", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-mob-ai-loot-breadth", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-redstone-toggle", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-redstone-circuit-breadth", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-world-persistence-restart", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-world-multichunk-durability", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-crash-recovery-parity", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-block-entity-persistence-parity", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-container-block-entity-breadth", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-biome-dimension-state", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-biome-dimension-travel", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "survival-sign-editing-live", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "mcp-controlled-smoke", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "combat-damage", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "combat-knockback", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "vanilla-combat-reference-parity", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "vanilla-combat-armor-reference-parity", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "armor-equipment-mitigation", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "equipment-update-observation", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "projectile-hit", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "projectile-damage-attribution", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "flag-carrier-death-return", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "reconnect-flag-state", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "ctf-invalid-pickup-ownership", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "ctf-invalid-return-drop", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "ctf-score-limit-win-condition", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "ctf-simultaneous-pickup-capture-race", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "ctf-spawn-team-balance-reset", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "reconnect-flag-score", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
    { name = "multi-client-load-score", owner = common_waiver.owner, reason = common_waiver.reason, non_claim = common_waiver.non_claim, next_action = common_waiver.next_action },
  ],
} | FallbackBudgetBaseline
