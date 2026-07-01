## Context

`docs/crafting-recipe-behavior-card.md` defines a finite crafting recipe matrix for Java Edition 1.20.1 / protocol 763 vocabulary: one shaped chest recipe, one shapeless oak-planks recipe, one invalid insufficient stick-input rejection, and one primary-click collection mode. Existing `survival-crafting-recipe-breadth` receipts are predecessor row evidence only. They do not provide a reusable pure recipe core, target-version recipe extraction, data-pack loading, or Valence shell ownership.

The accepted `vanilla-composable-plugins` spec already requires behavior cards, functional core / Bevy shell separation, positive and negative tests, and non-overclaiming evidence. This change adds the next implementation slice in that sequence: local selected-matrix core semantics that can run without Valence, clients, filesystem recipe loading, packet emission, or wall-clock state.

## Decisions

### 1. Implement the pure core before data extraction or shell wiring

**Choice:** The first crafting implementation target is a pure deterministic core plus a thin checker shell. The core consumes in-memory grid state, selected recipe rows, output-slot state, collection requests, and named limits. It returns deterministic match, no-result, output-blocked, collection, or typed malformed-data diagnostics.

**Rationale:** The behavior card explicitly separates recipe decisions from Bevy/ECS mutation. A local pure core is testable with plain positive and negative fixtures and creates a stable target for later recipe data extraction, receipt handoff, and Valence shell work.

### 2. Bound the implementation to the selected matrix

**Choice:** The core covers only the shaped chest row, shapeless oak-planks row, invalid stick-input rejection, and primary-click collection mode named by the behavior card. The checker must name grid dimensions, selected slots, output counts, stack limits, and collection behavior through constants or data fields rather than unexplained numeric literals.

**Rationale:** Existing evidence covers those rows as predecessor vocabulary. Keeping the core selected and explicit prevents silent claims about all recipes, arbitrary collection modes, recipe-book behavior, automated crafters, data packs, or default gameplay.

### 3. Treat malformed data and unsupported modes as first-class negative paths

**Choice:** The core must reject missing selected rows, duplicate recipe ids, malformed shaped pattern/key data, malformed shapeless ingredients, invalid item ids, zero output counts, output-slot conflicts, missing target data placeholders, unsupported recipe kinds, and unsupported collection modes such as shift-click, drag, split, recipe-book UI, or automated crafter requests.

**Rationale:** Negative cases are the safety boundary for later data extraction and shell wiring. Failing closed with typed diagnostics is more reviewable than implicitly returning no result for malformed or out-of-scope inputs.

### 4. Preserve state on rejection

**Choice:** No-result, output-blocked, malformed-row, and unsupported-mode paths must preserve grid and inventory state that the core is not allowed to mutate. Any proposed inventory delta appears only on a successful selected primary-click collection decision.

**Rationale:** This matches the functional-core contract and makes failure semantics inspectable without standing up Valence or a client.

### 5. Record local semantics without vanilla parity claims

**Choice:** Implementation evidence may claim only local selected-matrix unit semantics over in-memory rows. Target-version recipe JSON extraction, Paper/reference comparison, Valence runtime shell behavior, all-recipe breadth, and live scenario behavior remain deferred.

**Rationale:** The behavior card says target-version data and parity evidence are required before broader behavior claims. This target intentionally produces the core those later rails can consume.

## Risks / Trade-offs

- A local in-memory core does not prove target-version vanilla behavior until selected recipe JSON is extracted and validated in a separate change.
- Primary-click collection handling may expose future inventory-capacity details; this change should keep the model minimal and document every assumption.
- Overbroad success diagnostics could be mistaken for all-recipe support; docs and evidence must preserve selected-matrix non-claims.
- Future Valence APIs may require shell-contract adjustments, but those belong to a later shell-contract or runtime-shell Cairn.
