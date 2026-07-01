## Context

Crafting recipe work now has three accepted building blocks:

- `docs/crafting-recipe-behavior-card.md` defines the bounded selected matrix: shaped `minecraft:chest`, shapeless `minecraft:oak_planks`, invalid stick-input/no-result rejection, and primary-click collection only.
- `tools/check_crafting_recipe_core.rs` and `docs/crafting-recipe-selected-matrix-core.md` define a pure deterministic selected-matrix core that remains local unit evidence.
- `compat/config/crafting-recipe-selected-matrix-fixture.ncl`, `tools/check_crafting_recipe_data_fixture.rs`, and `docs/crafting-recipe-selected-matrix-data-fixture.md` define and validate Java Edition 1.20.1 / protocol 763 selected fixture rows for the same matrix.

The archived `survival-crafting-recipe-breadth` receipt bundle already records Paper/reference and Valence normalized metrics for the same configured rows. This change should bridge those artifacts without broadening scope.

## Decisions

### 1. Verify fixture-to-receipt equivalence before broader claims

**Choice:** Add a selected-matrix handoff checker that compares the validated fixture rows to normalized Paper/reference and Valence receipt evidence for target scope, row identity, input items, output items, output counts, collection mode, target inventory slots, backend evidence, and required non-claims.

**Rationale:** The fixture and receipts were produced by separate Cairns. A deterministic bridge prevents manual inference from becoming an implicit selected-matrix parity claim.

### 2. Keep comparison logic pure and the shell boring

**Choice:** Implement the checker as a pure core over in-memory `SelectedCraftingFixture`, `NormalizedCraftingReceipt`, and `ReceiptHandoffDecision` values. The shell may export Nickel, read `.kv` or receipt files, call the pure core, print diagnostics, and exit non-zero on mismatch.

**Rationale:** Positive and negative fixtures can exercise all rule decisions without starting Valence, Paper, Docker, Stevenarella, or any Minecraft process.

### 3. Reuse archived receipts only if they pass the handoff contract

**Choice:** The implementation may reuse `docs/evidence/survival-crafting-recipe-breadth-paper-2026-06-20.kv` and `docs/evidence/survival-crafting-recipe-breadth-valence-2026-06-20.kv` as receipt inputs only when the checker validates row identity and non-claim boundaries. If they fail, the task must stop or rerun a selected Paper/Valence receipt rail and promote fresh evidence.

**Rationale:** Reuse is low-risk only with fail-closed validation. Stale, malformed, mismatched, or overbroad evidence must not be normalized away.

### 4. Stop before Valence runtime shell integration

**Choice:** This target ends at selected-matrix receipt handoff evidence and documentation. It does not add a crafting Bevy/ECS system, schedule phase, data loader, DefaultPlugins membership, or live runtime claim.

**Rationale:** Runtime shell integration has separate inventory, packet, schedule, disabled-plugin, and mutation risks. The evidence bridge is the prerequisite seam.

## Risks / Trade-offs

- Existing receipt evidence may use item display names while the fixture uses namespaced item IDs. The checker should normalize only explicit accepted aliases and fail on ambiguous values.
- Archived receipts may be sufficient for review but not for a future freshness policy. If freshness is required, the implementation must rerun a selected receipt and promote new logs under `docs/evidence/`.
- A handoff checker does not prove all recipes, arbitrary collection modes, shift-click/drag/split behavior, recipe-book behavior, data-pack loading, automated crafter behavior, or a Valence runtime shell.
- The invalid/no-result row has no output item or collection mode. The contract must model it separately from matched shaped/shapeless rows instead of forcing it through output-slot fields.
