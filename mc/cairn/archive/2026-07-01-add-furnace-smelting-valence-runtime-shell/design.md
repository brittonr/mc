## Context

The accepted furnace-smelting chain has the prerequisites for a bounded shell implementation:

- `docs/furnace-smelting-behavior-card.md` defines the selected Java Edition 1.20.1 / protocol 763 standard-furnace row and non-claims.
- `tools/check_furnace_smelting_core.rs` keeps selected-row rule decisions in a pure deterministic core.
- `compat/config/furnace-smelting-selected-row-fixture.ncl` validates one RawIron + Coal -> IronIngot fixture row with named timing and stack constants.
- `tools/check_furnace_smelting_receipt_handoff.rs` bridges that fixture row to archived Paper/reference and Valence receipt metrics without rerunning live rails.
- `docs/furnace-smelting-valence-shell-contract.md` defines the future opt-in Bevy/ECS shell boundary, schedule expectations, disabled-plugin behavior, data-loading boundary, mutation boundary, and non-claims.

The remaining gap is implementation of a small opt-in Valence shell. Current Valence examples already use shared gameplay phase metadata and schedule-contract tests; the implementation must inspect exact furnace, inventory, block-entity, and layer APIs before choosing the final module and system wiring.

## Decisions

### 1. Implement an explicit opt-in shell only

**Choice:** The runtime shell must be installed only when an explicit furnace-smelting plugin or focused example fixture adds it. It must not join `DefaultPlugins`, alter default Valence behavior, or imply a public survival mode.

**Rationale:** The shell contract and roadmap both reject default membership changes until separately scoped. Explicit opt-in wiring gives reviewers a disabled-plugin negative test and avoids broad gameplay claims.

### 2. Keep the pure core as the semantic owner

**Choice:** The shell snapshots furnace kind, slots, counters, selected recipe/fuel rows, and limits into plain values, calls the existing pure core, and applies only the returned state, transition, or typed error.

**Rationale:** This preserves the functional-core/imperative-shell boundary. Bevy queries, resources, events, packet/logging adapters, file reads, and schedule registration remain outside rule decisions.

### 3. Scope runtime behavior to one selected row

**Choice:** The first shell claim is limited to standard furnace, `minecraft:raw_iron` input, `minecraft:coal` fuel, and `minecraft:iron_ingot` output using the validated fixture values. Additional recipes, fuels, furnace kinds, hoppers, XP, recipe book, and chunk-unload semantics stay out of scope.

**Rationale:** The accepted evidence proves only the selected row. Broad recipe or block-entity behavior would require separate extraction and parity evidence.

### 4. Require focused schedule and disabled-plugin evidence

**Choice:** If implementation adds plugin wiring, system sets, ordering constraints, schedule labels, events, or resources, it must record schedule hygiene evidence and a disabled-plugin comparison that proves shell-owned behavior is absent when the plugin is not installed.

**Rationale:** Furnace shell work crosses inventory mutation, block-entity state, and presentation boundaries. Schedule evidence is required by the accepted Valence Bevy/ECS policy for this kind of change.

### 5. Prefer in-process Valence tests before live rails

**Choice:** The initial runtime-shell implementation should prove behavior with focused unit or minimal-app tests. Live Paper/Valence rails are deferred unless the implementation explicitly changes scenario behavior or needs fresh receipt evidence.

**Rationale:** The next target is to prove shell wiring and disabled behavior, not to broaden parity. In-process tests are faster, deterministic, and sufficient for the selected-row shell claim.

## Risks / Trade-offs

- Exact Valence inventory or block-entity APIs may force the implementation to revise the shell location or schedule phase; the task plan requires source inventory before code.
- Focused shell tests do not prove live client behavior, all-recipe breadth, Paper parity, or production readiness.
- Reusing the existing selected-row fixture limits breadth, but prevents silent overclaims from archived receipts.
- Adding system sets or events can affect schedule shape; schedule hygiene evidence is mandatory when wiring changes.
