## Context

Furnace smelting now has three accepted building blocks:

- `docs/furnace-smelting-behavior-card.md` defines the bounded selected-row claim and requires extracted data plus Paper/vanilla receipts before implementation claims.
- `tools/check_furnace_smelting_core.rs` and `docs/furnace-smelting-selected-row-core.md` define a pure selected-row standard-furnace core that remains local unit evidence.
- `compat/config/furnace-smelting-selected-row-fixture.ncl`, `tools/check_furnace_smelting_data_fixture.rs`, and `docs/furnace-smelting-selected-row-data-fixture.md` define and validate one Java Edition 1.20.1 / protocol 763 fixture row: RawIron input, Coal fuel, IronIngot output, cook ticks, burn ticks, and stack constants.

The archived `survival-furnace-smelting-breadth` receipt bundle already records Paper/reference and Valence normalized metrics for the same configured row. This change should bridge those artifacts without broadening scope.

## Decisions

### 1. Verify fixture-to-receipt equivalence before broader claims

**Choice:** Add a selected-row handoff checker that compares the validated fixture row to normalized Paper/reference and Valence receipt evidence for row identity, target scope, item IDs, counts, cook ticks, burn ticks, backend identities, and required non-claims.

**Rationale:** The fixture and receipts were produced by separate Cairns. A deterministic bridge prevents manual inference from becoming an implicit vanilla-parity claim.

### 2. Keep comparison logic pure and the shell boring

**Choice:** Implement the checker as a pure core over in-memory `SelectedFixtureRow`, `NormalizedReceiptRow`, and `ReceiptHandoffDecision` values. The shell may export Nickel, read `.kv`/receipt files, call the pure core, print diagnostics, and exit non-zero on mismatch.

**Rationale:** Positive and negative fixtures can exercise all rule decisions without starting Valence, Paper, Docker, or Stevenarella.

### 3. Reuse archived receipts only if they pass the handoff contract

**Choice:** The implementation may reuse `docs/evidence/survival-furnace-smelting-breadth-paper-2026-06-21.kv` and `docs/evidence/survival-furnace-smelting-breadth-valence-2026-06-21.kv` as receipt inputs only when the checker validates row identity and non-claim boundaries. If they fail, the task must stop or rerun a selected Paper/Valence receipt rail and promote new evidence.

**Rationale:** Reuse is low-risk only with fail-closed validation. Stale or mismatched evidence must not be papered over.

### 4. Stop before Valence runtime shell integration

**Choice:** This target ends at selected-row receipt handoff evidence and documentation. It does not add a furnace Bevy/ECS system, schedule phase, DefaultPlugins membership, data-pack loader, or live Valence runtime claim.

**Rationale:** Runtime shell integration has separate schedule and mutation risks. The evidence bridge is the prerequisite seam.

## Risks / Trade-offs

- Existing receipt evidence may use item display names while the fixture uses namespaced item IDs. The checker should normalize only explicit accepted aliases and fail on ambiguous values.
- Archived receipts may be sufficient for review but not for future freshness policy. If freshness is required, the implementation must rerun a selected receipt and promote new logs under `docs/evidence/`.
- A handoff checker does not prove all recipes, all fuels, UI behavior, hoppers, XP, recipe-book sync, chunk unload, or a Valence runtime shell.
