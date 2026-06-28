# mc-compat scenario fallback budget

## Report shape

`tools/check_scenario_manifest.rs` reads `compat/config/scenario-manifest.ncl` plus `compat/config/scenario-fallback-budget-baseline.ncl` and prints:

```text
fallback budget: approved=[...]; removed=[...]; new=[...]; typed_event_regressions=[...]; missing_waiver_metadata=[...]
```

- `approved` names current `substring-fallback` rows that are still present in the checked baseline with owner, reason, non-claim, and next-action waiver metadata.
- `removed` names baseline fallback rows that left fallback, which is migration progress.
- `new` names unapproved current fallback rows and fails the gate.
- `typed_event_regressions` names rows recorded as typed-event-ready in the baseline that moved back to fallback and fails the gate.
- `missing_waiver_metadata` names baseline rows with incomplete waiver metadata and fails the gate.

## Ratchet inventory

The pre-ratchet inventory in `docs/evidence/ratchet-scenario-fallback-budget-baseline.run.log` reported 46 maintained rows: 43 `typed-event-ready` rows, 3 `substring-fallback` rows, and 32 stale fallback approvals that had already migrated.

Current approved substring fallback rows after the ratchet:

- `valence-compat-bot-probe`
- `vanilla-combat-reference-parity`
- `vanilla-combat-armor-reference-parity`

Rows removed from the approved fallback list by this ratchet:

- `armor-equipment-mitigation`
- `blue-flag-score`
- `combat-damage`
- `combat-knockback`
- `ctf-invalid-pickup-ownership`
- `ctf-invalid-return-drop`
- `ctf-score-limit-win-condition`
- `ctf-simultaneous-pickup-capture-race`
- `ctf-spawn-team-balance-reset`
- `equipment-update-observation`
- `flag-carrier-death-return`
- `flag-score-repeat`
- `mcp-controlled-smoke`
- `multi-client-load-score`
- `projectile-damage-attribution`
- `projectile-hit`
- `reconnect-flag-score`
- `reconnect-flag-state`
- `survival-biome-dimension-travel`
- `survival-chest-persistence`
- `survival-container-block-entity-breadth`
- `survival-crafting-recipe-breadth`
- `survival-furnace-persistence`
- `survival-furnace-smelting-breadth`
- `survival-hunger-food`
- `survival-hunger-health-cycle`
- `survival-mob-ai-loot-breadth`
- `survival-mob-drop`
- `survival-redstone-circuit-breadth`
- `survival-redstone-toggle`
- `survival-sign-editing-live`
- `survival-world-multichunk-durability`

The ratcheted `typed_event_ready_rows` list records every current typed-event-ready manifest row, including previously migrated rows and rows that were already typed-event-ready before this accounting update. Future movement back to `substring-fallback` fails unless the fallback budget is deliberately re-waived with complete metadata.

## Non-claim boundary

The fallback-budget report is migration accounting only. It does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, production readiness, broad Minecraft compatibility, gameplay correctness, or row-level semantic parity for fallback scenarios.
