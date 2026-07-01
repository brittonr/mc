## Candidate decision

Selected: `add-crafting-recipe-selected-matrix-core`

Why now: the crafting recipe behavior card is archived and names a finite selected matrix, but the repository still lacks the reusable pure recipe core that future data-fixture, receipt-handoff, and Valence shell work can test without starting the world.

Prerequisites satisfied: `README.md`, `AGENTS.md`, `docs/vanilla-composable-plugins-roadmap.md`, `docs/crafting-recipe-behavior-card.md`, accepted `cairn/specs/vanilla-composable-plugins/spec.md`, accepted `cairn/specs/mc-compatibility/spec.md` crafting-breadth requirements, archived `2026-07-01-add-crafting-recipe-behavior-card`, archived `2026-06-20-survival-crafting-recipe-breadth-parity`, archived `2026-06-22-survival-crafting-recipe-breadth-typed-event-migration`, and `docs/evidence/survival-crafting-recipe-breadth-receipts-2026-06-20.md`.

Deferred alternatives: crafting target-version data extraction or fixture validation is deferred until a selected-matrix core exists to consume rows; a Valence crafting shell is deferred until the core, data fixture, receipt handoff, and shell contract exist; hunger/food and armor mitigation remain separate roadmap sequences; all-recipe breadth, data-pack loading, recipe-book UI behavior, automated crafter behavior, shift-click/drag/split collection breadth, broad redstone, mob AI, world generation, DefaultPlugins membership changes, public-server safety, and production readiness remain broader or higher-risk scopes.

Non-claims: no target-version recipe JSON extraction, no Paper or live Valence rerun, no Valence runtime shell, no DefaultPlugins membership change, no all-recipe breadth, no data-pack engine, no recipe-book behavior, no automated crafter behavior, no arbitrary collection modes beyond the selected primary-click request, no broad vanilla parity, no broad Minecraft compatibility, no public-server safety, and no production readiness.

## Why

The roadmap sequence moves from the completed furnace runtime shell to the crafting/inventory seam. The accepted crafting behavior card already records the selected shaped chest row, shapeless oak-planks row, invalid stick-input rejection, primary-click collection boundary, pure-core shape, shell boundary, evidence needs, and non-claims.

The next smallest implementation target is a pure deterministic selected-matrix recipe core. It can prove matching, no-result rejection, output blocking, malformed-row diagnostics, and unsupported collection-mode rejection with positive and negative tests before any recipe extraction, Valence shell wiring, live receipt, or default plugin membership is attempted.

## What Changes

- Add `vanilla-composable-plugins` requirements for crafting selected-matrix core inventory, a pure recipe core, focused positive and negative tests, documentation, and closeout evidence.
- Plan a `tools/check_crafting_recipe_core.rs` checker with a pure deterministic core and thin CLI self-test shell over in-memory selected recipe rows.
- Require documentation that distinguishes local selected-matrix unit semantics from target-version data extraction, receipt handoff, and future Valence runtime behavior.

## Impact

- **Files**: future implementation will likely touch `tools/check_crafting_recipe_core.rs`, `docs/crafting-recipe-selected-matrix-core.md`, `docs/evidence/`, accepted `cairn/specs/vanilla-composable-plugins/spec.md` during sync, and archived Cairn package/evidence. This target-hunt package adds only the Cairn proposal/design/tasks/spec delta.
- **Testing**: planned checks include baseline card validation, focused positive and negative core self-tests, formatter or compile checks for the Rust checker, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation after implementation tasks cite logs, accepted-spec sync verification, and evidence-manifest checks for promoted artifacts.
- **Non-claims**: this does not add target-version extraction, a Valence shell, new scenario rails, live Paper/Valence receipts, default Valence gameplay, all recipes, data packs, recipe book, broad vanilla parity, public-server safety, or production readiness.
