## Context

Crafting recipe work now has four accepted predecessor artifacts:

- `docs/crafting-recipe-behavior-card.md` defines the bounded selected matrix: shaped `minecraft:chest`, shapeless `minecraft:oak_planks`, invalid stick-input/no-result rejection, and primary-click collection only.
- `tools/check_crafting_recipe_core.rs` and `docs/crafting-recipe-selected-matrix-core.md` define a pure deterministic selected-matrix core that remains local unit evidence.
- `compat/config/crafting-recipe-selected-matrix-fixture.ncl`, `tools/check_crafting_recipe_data_fixture.rs`, and `docs/crafting-recipe-selected-matrix-data-fixture.md` define and validate Java Edition 1.20.1 / protocol 763 selected fixture rows for the same matrix.
- `tools/check_crafting_recipe_receipt_handoff.rs`, `docs/crafting-recipe-selected-matrix-receipt-handoff.md`, and `docs/evidence/crafting-recipe-selected-matrix-receipts-2026-07-01.md` prove the selected fixture rows match archived Paper/reference plus Valence receipt evidence while retaining non-claims.

The remaining documented gap is runtime shell planning: which opt-in Valence plugin, schedules, resources, components, events, inventory ownership, diagnostics, and mutation boundaries may call the pure core without turning selected-matrix evidence into an unreviewed runtime or breadth claim.

## Decisions

### 1. Start with a shell contract, not shell code

**Choice:** This target adds a contract document and focused validation requirements. It must not add a crafting Bevy plugin, register systems, change `DefaultPlugins`, add a target-version data loader, or mutate Valence runtime behavior.

**Rationale:** Crafting touches client inventory state, crafting-grid state, output slots, collection requests, packet-driven inventory events, and schedule-sensitive mutations. A contract lets reviewers agree on the shell boundary before runtime code introduces ordering risk.

### 2. Preserve the pure core as the only recipe-decision owner

**Choice:** The contract must state that future ECS systems snapshot selected crafting state into plain core inputs, call the selected-matrix core, and apply only returned recipe matches, inventory deltas, output-blocked decisions, no-result decisions, or typed malformed-data diagnostics.

**Rationale:** This keeps recipe matching and collection semantics testable without Valence, Paper, Stevenarella, network packets, filesystem reads, logging, wall-clock time, or Bevy world mutation.

### 3. Make ownership, disabled behavior, and schedule facts reviewable

**Choice:** The contract must name planned opt-in plugin ownership, shell-owned resources/components/events, inventory and client state read/write boundaries, candidate schedule phase, ordering dependencies, disabled-plugin behavior, data-loading boundaries, packet/logging boundaries, and required schedule evidence before any implementation promotes runtime claims.

**Rationale:** Selected-matrix crafting can still fail by mutating stale inventory state, replaying stale packet events, double-handling raw and typed inventory actions, or installing behavior when the plugin is disabled.

### 4. Fail closed on missing boundaries and overclaims

**Choice:** The planned validator should accept a complete contract and reject missing target scope, missing selected-matrix prerequisites, missing core/shell separation, missing shell ownership, missing schedule facts, missing disabled-plugin behavior, missing positive/negative tests, missing evidence requirements, missing non-claims, DefaultPlugins overclaims, all-recipe breadth, arbitrary collection-mode breadth, data-pack claims, recipe-book claims, automated-crafter claims, broad vanilla parity, public-server safety, and production readiness.

**Rationale:** Negative validation prevents a documentation-only contract from becoming implicit permission for broad runtime crafting work.

## Risks / Trade-offs

- Valence crafting-table, inventory, and packet-event APIs may require more source inspection during implementation. This package avoids claiming exact runtime types until the implementation reads affected crates.
- A contract checker validates review shape, not runtime behavior. Future shell work still needs focused Valence tests, disabled-plugin evidence, and schedule evidence.
- Existing selected-matrix receipts prove only finite shaped, shapeless, invalid/no-result, and primary-click collection rows. The contract must not imply all recipes, arbitrary collection modes, shift-click/drag/split handling, data-pack loading, recipe-book behavior, recipe discovery, advancement behavior, automated crafter behavior, or default plugin membership.
- Schedule evidence may show a different phase is safer than the initially documented candidate. The contract should permit revision before shell code lands, as long as evidence and non-claims stay explicit.
