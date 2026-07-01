## Context

The crafting recipe chain currently has two accepted prerequisites:

- `docs/crafting-recipe-behavior-card.md` defines the selected matrix: shaped `minecraft:chest`, shapeless `minecraft:oak_planks`, an invalid stick-input/no-result row, and primary-click collection only.
- `docs/crafting-recipe-selected-matrix-core.md` implements a pure deterministic core over local in-memory selected rows and explicitly does not claim target-version recipe extraction, all-recipe breadth, Valence shell behavior, or vanilla parity.

The roadmap sequence names crafting and inventory after bounded survival slices, but it stops before all-recipe breadth, data-pack loading, automated crafter behavior, and broader collection modes. The immediate gap is target-version selected recipe fixture data plus malformed-data validation.

## Decisions

### 1. Select a target-version fixture before receipt handoff

**Choice:** The next implementation should add a Java Edition 1.20.1 / protocol 763 selected recipe-data fixture for exactly the accepted matrix: one shaped chest row, one shapeless oak-planks row, and explicit invalid or malformed-data cases.

**Rationale:** Receipt handoff and Valence shell work need reviewable target-version data. A fixture step is smaller and safer than jumping directly to runtime integration.

### 2. Keep fixture semantics pure and validated before core use

**Choice:** The fixture validator should have a pure deterministic validation core and a thin shell for reading/exporting the fixture. It must reject malformed shaped patterns, malformed shapeless ingredient lists, missing or duplicate selected rows, invalid item IDs, zero counts, unsupported recipe kinds, unsupported collection modes, missing provenance, and missing non-claims before rows reach the selected-matrix core.

**Rationale:** The accepted crafting core already treats malformed data as typed errors. Validating fixture data first keeps bad target data from masquerading as recipe semantics and keeps positive/negative evidence focused.

### 3. Treat core handoff as local unit evidence only

**Choice:** The implementation may feed the validated fixture rows into `tools/check_crafting_recipe_core.rs`, but only as local selected-matrix unit evidence.

**Rationale:** A fixture-to-core handoff can prove that the core accepts the target-scoped selected rows, but it does not prove Paper/vanilla parity, all recipes, or Valence runtime behavior.

### 4. Defer runtime and breadth work

**Choice:** This package does not plan data-pack loading, recipe-book behavior, automated crafter behavior, arbitrary collection modes, Valence Bevy/ECS shell systems, or DefaultPlugins membership changes.

**Rationale:** Those areas require different schedule, packet, inventory, UI, and parity evidence. Keeping them out of this slice preserves the bounded evidence path.

## Risks / Trade-offs

- The selected fixture is narrow and will not satisfy all-recipe breadth; that is intentional and must remain a non-claim.
- Target-version provenance can be fragile if future implementation relies on ad-hoc constants. The fixture must record source/provenance fields and validation evidence under `docs/evidence/`.
- A core handoff can be misread as runtime behavior. Documentation and evidence must distinguish fixture/core semantics from receipt parity and Valence shell claims.
- If the selected target-version recipe JSON differs from the predecessor row vocabulary, the implementation must stop and record the mismatch rather than normalize it away without evidence.
