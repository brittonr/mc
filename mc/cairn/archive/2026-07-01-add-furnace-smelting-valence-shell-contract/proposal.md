## Candidate decision

Selected: `add-furnace-smelting-valence-shell-contract`

Why now: furnace smelting now has a behavior card, a pure selected-row core, a validated Java Edition 1.20.1 / protocol 763 fixture, and a deterministic receipt handoff to archived Paper/reference plus Valence evidence. The named remaining prerequisite before runtime behavior claims is a Valence Bevy/ECS shell contract with schedule and mutation boundaries.

Prerequisites satisfied: `docs/vanilla-composable-plugins-roadmap.md`, `docs/furnace-smelting-behavior-card.md`, `docs/furnace-smelting-selected-row-core.md`, `docs/furnace-smelting-selected-row-data-fixture.md`, `docs/furnace-smelting-selected-row-receipt-handoff.md`, accepted `cairn/specs/vanilla-composable-plugins/spec.md`, and accepted `cairn/specs/valence-bevy-ecs/spec.md` schedule/gameplay-plugin requirements.

Deferred alternatives: hunger/food behavior-card work is deferred because the furnace chain already has stronger predecessor artifacts; crafting recipe-core work is deferred until the first survival-stat shell seam is bounded; armor mitigation is deferred despite existing combat checks because it is a separate combat plugin group; redstone, mobs, world generation, all-recipe breadth, smoker/blast-furnace breadth, hopper behavior, XP, recipe-book sync, and DefaultPlugins membership changes remain higher-risk or broader scopes.

Non-claims: no Valence runtime integration in this planning package, no new Bevy/ECS system, no schedule registration, no DefaultPlugins membership change, no all-recipe or all-fuel breadth, no broad vanilla parity, no broad Minecraft compatibility, no public-server safety, and no production readiness.

## Why

The current selected-row furnace evidence stops at local core semantics plus a receipt handoff. A future Valence plugin shell must decide which ECS resources, components, events, schedules, and mutation boundaries it owns before any runtime behavior can be claimed. Without that contract, shell code could mix rule decisions into Bevy systems, mutate inventory state before schedule evidence exists, or overclaim from the selected-row receipt bridge.

## What Changes

- Add a furnace smelting Valence shell contract requirement to the `vanilla-composable-plugins` spec.
- Require a reviewable contract document that maps the existing selected-row core inputs and outputs to a thin opt-in Bevy/ECS shell boundary.
- Require focused positive and negative validation for contract completeness and overclaim rejection.
- Require future closeout evidence before any runtime-shell implementation can claim selected-row Valence behavior.

## Impact

- **Files**: future implementation will likely touch `docs/furnace-smelting-valence-shell-contract.md`, furnace selected-row docs, a focused checker under `tools/`, `docs/evidence/`, and the accepted `vanilla-composable-plugins` spec during sync. This target-hunt package adds only the Cairn proposal/design/tasks/spec delta.
- **Testing**: planned checks include existing fixture/core/receipt-handoff baseline validation, a focused shell-contract checker with positive and negative fixtures, Cairn gates/validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and the smallest relevant Valence schedule checks if implementation work touches Valence scheduling.
- **Non-claims**: the change will not add a runtime furnace plugin, data-pack loader, broad recipe extraction, Paper rerun, Valence default behavior, production readiness claim, or broad vanilla parity claim.
