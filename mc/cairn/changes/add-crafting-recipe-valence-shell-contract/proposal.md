## Candidate decision

Selected: `add-crafting-recipe-valence-shell-contract`

Why now: the crafting sequence now has the behavior card, selected-matrix pure core, Java Edition 1.20.1 / protocol 763 data fixture, and receipt handoff evidence needed before runtime shell planning. The next bounded seam is a Valence Bevy/ECS shell contract that defines ownership, schedule, mutation, and disabled-plugin boundaries before any crafting-table shell code exists.

Prerequisites satisfied: `docs/vanilla-composable-plugins-roadmap.md`, `docs/crafting-recipe-behavior-card.md`, `docs/crafting-recipe-selected-matrix-core.md`, `docs/crafting-recipe-selected-matrix-data-fixture.md`, `docs/crafting-recipe-selected-matrix-receipt-handoff.md`, `docs/evidence/crafting-recipe-selected-matrix-receipts-2026-07-01.md`, accepted `cairn/specs/vanilla-composable-plugins/spec.md`, and accepted `cairn/specs/valence-bevy-ecs/spec.md` schedule/gameplay-plugin requirements.

Deferred alternatives: crafting runtime-shell implementation is deferred until the shell contract exists; all-recipe extraction, data-pack loading, recipe-book behavior, automated crafter behavior, shift-click/drag/split handling, arbitrary collection modes, and DefaultPlugins membership changes are broader follow-ons; hunger/food, armor, redstone, mobs, and world-generation targets are separate plugin groups with weaker immediate predecessor chains.

Non-claims: no Valence runtime integration, no new Bevy/ECS system, no schedule registration, no inventory mutation adapter, no target-version all-recipe extraction, no DefaultPlugins membership change, no broad vanilla parity, no broad Minecraft compatibility, no public-server safety, and no production readiness.

## Why

Crafting selected-matrix evidence now stops at local core semantics plus a deterministic handoff to archived Paper/reference and Valence receipts. Future runtime work still needs a reviewable boundary for how a Valence shell snapshots crafting-grid state, output-slot state, selected recipe rows, collection requests, and client/inventory ownership into the pure core, then applies only returned deltas or diagnostics.

Without that contract, implementation could mix recipe decisions into Bevy systems, mutate inventory state before disabled-plugin behavior is defined, or overclaim from selected-matrix receipts as if they proved broad crafting behavior.

## What Changes

- Add crafting recipe Valence shell-contract requirements to the `vanilla-composable-plugins` spec.
- Require a shell-planning inventory of accepted crafting artifacts, evidence boundaries, and relevant Valence schedule/plugin contract sources.
- Require a contract document that maps selected-matrix core inputs/outputs to an opt-in Bevy/ECS shell boundary.
- Require focused positive and negative validation for contract completeness, core/shell separation, disabled-plugin behavior, schedule facts, test coverage, and overclaim rejection.
- Require future closeout evidence before any runtime-shell implementation can claim selected-matrix Valence behavior.

## Impact

- **Files**: future implementation will likely add `docs/crafting-recipe-valence-shell-contract.md`, a focused checker under `tools/`, evidence under `docs/evidence/`, updates to selected-matrix crafting docs, and accepted `vanilla-composable-plugins` spec sync. This target-hunt package adds only the Cairn proposal/design/tasks/spec delta.
- **Testing**: planned checks include current fixture/core/receipt-handoff baseline validation, focused shell-contract checker self-tests with positive and negative fixtures, Cairn gates/validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and the smallest relevant Valence schedule check if implementation later changes plugin wiring or schedule registration.
- **Non-claims**: the change will not add a crafting Bevy/ECS shell, change Valence defaults, claim all recipes, claim all collection modes, claim data-pack or recipe-book behavior, or claim broad vanilla parity.
