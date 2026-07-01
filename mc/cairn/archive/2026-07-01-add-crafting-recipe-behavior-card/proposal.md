## Candidate decision

Selected: `add-crafting-recipe-behavior-card`

Why now: the furnace smelting behavior-card, selected-row core, receipt handoff, shell contract, and opt-in runtime shell are already archived, so the roadmap can advance to the next bounded crafting/inventory sequence. Crafting already has paired shaped/shapeless/invalid recipe-breadth receipts and typed-event rails, but it does not yet have a composable-plugin behavior card that turns those row receipts into an implementation-ready pure-core/shell boundary.

Prerequisites satisfied: `README.md`, `AGENTS.md`, `docs/vanilla-composable-plugins-roadmap.md`, accepted `cairn/specs/vanilla-composable-plugins/spec.md`, accepted `cairn/specs/mc-compatibility/spec.md` crafting-breadth requirements, archived `2026-06-20-survival-crafting-recipe-breadth-parity`, archived `2026-06-22-survival-crafting-recipe-breadth-typed-event-migration`, and `docs/evidence/survival-crafting-recipe-breadth-receipts-2026-06-20.md`.

Deferred alternatives: hunger/food behavior-card work is deferred because the roadmap's first survival-stat seam has already advanced through the furnace runtime shell; armor mitigation is deferred to the equipment/combat sequence; all-recipe extraction, data-pack loading, recipe-book UI, automated crafter behavior, shift-click/drag/split breadth, broad redstone, mob AI, world generation, DefaultPlugins membership changes, and production/public-server claims remain broader or higher-risk scopes.

Non-claims: no crafting core implementation in this planning package, no Valence runtime shell, no new scenario rail, no new live Paper or Valence run, no all-recipe breadth, no data-pack engine, no recipe-book behavior, no automated crafter behavior, no default plugin membership change, no broad vanilla parity, no broad Minecraft compatibility, no public-server safety, and no production readiness.

## Why

The Minecraft Wiki-guided roadmap names crafting and inventory as the next bounded domain after the first survival-stat seam, with shaped/shapeless recipe matching as the first slice. The compatibility harness already has a bounded crafting recipe-breadth row for a shaped chest recipe, a shapeless oak-planks recipe, and an invalid insufficient-input rejection, but those receipts are row evidence, not a reusable Valence plugin design.

A behavior card is the smallest evidence-ready next target because the accepted `vanilla-composable-plugins` spec requires wiki-guided implementation work to start from a source-scoped card that records target data, pure rule-core inputs/outputs, Bevy/ECS shell boundaries, positive and negative tests, evidence needs, and non-claims before implementation starts.

## What Changes

- Add a crafting recipe behavior-card requirement to the `vanilla-composable-plugins` spec.
- Require a dedicated `docs/crafting-recipe-behavior-card.md` that scopes Java Edition 1.20.1 / protocol 763 crafting behavior to a finite selected recipe matrix.
- Require focused behavior-card validation with positive and negative fixtures, including overclaim rejection for all recipes, data-pack loading, recipe-book UI, and default plugin membership.
- Require future closeout evidence before any crafting core, Valence shell, or broader crafting claim can be promoted.

## Impact

- **Files**: future implementation will likely touch `docs/crafting-recipe-behavior-card.md`, a focused checker under `tools/`, `docs/evidence/`, accepted `cairn/specs/vanilla-composable-plugins/spec.md` during sync, and archived Cairn package/evidence. This target-hunt package adds only the Cairn proposal/design/tasks/spec delta.
- **Testing**: planned checks include focused card validation with positive and negative cases, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation when tasks are completed, accepted-spec sync verification, and evidence-manifest checks for promoted logs.
- **Non-claims**: no implementation, no runtime wiring, no new default plugin membership, no all recipes, no data packs, no recipe book, no public-server safety, and no production readiness.
