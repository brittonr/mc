## Candidate decision

Selected: `add-furnace-smelting-valence-runtime-shell`

Why now: the furnace-smelting sequence now has the accepted behavior card, selected-row pure core, target-scoped selected fixture, selected-row receipt handoff, and Valence shell contract. The next bounded seam is to turn that contract into an explicit opt-in Valence Bevy/ECS shell that exercises only the selected standard-furnace row in focused tests.

Prerequisites satisfied: `docs/vanilla-composable-plugins-roadmap.md`, `docs/furnace-smelting-behavior-card.md`, `docs/furnace-smelting-selected-row-core.md`, `docs/furnace-smelting-selected-row-data-fixture.md`, `docs/furnace-smelting-selected-row-receipt-handoff.md`, `docs/furnace-smelting-valence-shell-contract.md`, accepted `cairn/specs/vanilla-composable-plugins/spec.md`, and accepted `cairn/specs/valence-bevy-ecs/spec.md` schedule/gameplay-plugin contract requirements.

Deferred alternatives: hunger/food behavior-card work is deferred because the furnace chain already has stronger predecessor evidence; crafting recipe-core work is deferred until the first survival-stat runtime shell is proven; armor mitigation is deferred as a separate combat plugin group; all-recipe extraction, smoker/blast-furnace breadth, hopper behavior, XP, recipe-book synchronization, chunk-unload semantics, redstone, mobs, world generation, and DefaultPlugins membership changes remain broader or higher-risk scopes.

Non-claims: no DefaultPlugins membership change, no all-recipe or all-fuel breadth, no smoker or blast-furnace behavior, no hopper automation, no XP behavior, no recipe-book synchronization, no chunk-unload semantics, no live Paper rerun by default, no broad vanilla parity, no broad Minecraft compatibility, no public-server safety, and no production readiness.

## Why

The selected-row furnace evidence currently stops at a pure local core plus a receipt handoff and a shell contract. Reviewers still cannot point at a Valence runtime shell that snapshots furnace state, calls the pure core, commits returned state, records schedule facts, and proves disabled-plugin behavior without broadening the claim.

A bounded opt-in shell is the smallest next implementation target because it exercises the already accepted contract while preserving the selected-row non-claims. It also creates the first reusable runtime seam for later furnace breadth, recipe extraction, or survival-stat work without changing Valence defaults.

## What Changes

- Add `vanilla-composable-plugins` requirements for selected-row Valence runtime-shell inventory, opt-in shell wiring, focused positive and negative shell tests, runtime-shell documentation/evidence, and closeout.
- Plan an explicit opt-in furnace shell that snapshots selected-row state into the existing pure core, applies only returned state/diagnostics, and records schedule/disabled-plugin evidence.
- Require tests for enabled selected-row behavior and negative disabled/invalid/stale/blocked cases before promoting any runtime-shell claim.

## Impact

- **Files**: future implementation will likely touch `servers/valence/examples/survival_compat.rs` or a focused example module under `servers/valence/examples/`, existing furnace docs, a focused checker or Valence test surface, `docs/evidence/`, and the accepted `vanilla-composable-plugins` spec during sync. This target-hunt package adds only the Cairn proposal/design/tasks/spec delta.
- **Testing**: planned checks include baseline fixture/core/receipt/contract validation, focused Valence/Bevy shell tests with positive and negative cases, schedule hygiene when plugin wiring or system ordering changes, Cairn gates/validation, task-evidence validation, accepted-spec sync verification, and evidence-manifest checks.
- **Non-claims**: the change does not add default Valence gameplay, all-furnace parity, all recipes/fuels, a data-pack loader, Paper parity beyond the existing selected-row receipt handoff, production readiness, public-server safety, or broad Minecraft compatibility.
