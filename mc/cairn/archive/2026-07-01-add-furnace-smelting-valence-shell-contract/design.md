## Context

The furnace-smelting sequence has already produced the bounded predecessor artifacts needed before shell planning:

- `docs/furnace-smelting-behavior-card.md` defines the selected standard-furnace row, pure-core boundary, future Bevy/ECS shell boundary, evidence needs, and non-claims.
- `tools/check_furnace_smelting_core.rs` and `docs/furnace-smelting-selected-row-core.md` implement and document a pure local selected-row rule core.
- `compat/config/furnace-smelting-selected-row-fixture.ncl` and `tools/check_furnace_smelting_data_fixture.rs` validate one Java Edition 1.20.1 / protocol 763 fixture row.
- `tools/check_furnace_smelting_receipt_handoff.rs` and `docs/furnace-smelting-selected-row-receipt-handoff.md` prove that the fixture row matches archived Paper/reference plus Valence receipt evidence for RawIron + Coal -> IronIngot timing.

The remaining documented gap is Valence runtime shell planning: which opt-in plugin, schedules, resources, components, events, and mutation boundaries may call the pure core without turning selected-row local evidence into an unreviewed runtime or breadth claim.

## Decisions

### 1. Start with a shell contract, not shell code

**Choice:** The next target is a contract document plus focused validation. It should not add a Bevy plugin, register systems, change `DefaultPlugins`, or mutate Valence runtime behavior.

**Rationale:** The accepted Valence Bevy/ECS requirements require schedule evidence for plugin wiring, named sets, ordering constraints, and default membership changes. A contract lets reviewers agree on the shell boundary before code introduces schedule risk.

### 2. Preserve the pure core as the only rule-decision owner

**Choice:** The contract must state that future ECS systems snapshot furnace state, recipe/fuel data, and item stack facts into explicit core inputs, call the selected-row core, then apply only the returned state, transition, or typed error.

**Rationale:** This keeps functional-core semantics testable without a running server and prevents file reads, packet emission, logging, schedule decisions, or Bevy world mutation from entering the rule core.

### 3. Make ownership and schedule facts reviewable

**Choice:** The contract must name planned opt-in plugin membership, owned resources/components/events, candidate schedule phase, ordering dependencies, disabled-plugin behavior, and required schedule evidence before any implementation promotes runtime claims.

**Rationale:** Furnace shell behavior crosses block-entity ticking, inventory mutation, data resources, and client-visible updates. These are schedule-sensitive seams even for one selected row.

### 4. Fail closed on missing boundaries and overclaims

**Choice:** The planned validator should accept a complete contract and reject missing target scope, missing core/shell separation, missing schedule facts, missing disabled-plugin behavior, missing positive/negative tests, missing non-claims, DefaultPlugins membership overclaims, and broad recipe/furnace parity claims.

**Rationale:** Negative validation prevents a documentation-only contract from silently becoming permission for broad runtime work.

## Risks / Trade-offs

- Valence inventory and block-entity APIs may require more source inspection during implementation. This package avoids claiming exact runtime types until implementation reads the affected crates.
- A contract checker validates review shape, not runtime behavior. Future shell work still needs focused Valence tests and schedule evidence.
- Existing selected-row receipts prove only one bounded row. The contract must not imply all recipes, all fuels, smoker/blast-furnace behavior, hoppers, XP, recipe-book sync, chunk-unload semantics, or default plugin membership.
- Schedule evidence may show a different phase is safer than the initially documented candidate. The contract should permit revision before shell code lands, as long as evidence and non-claims stay explicit.
