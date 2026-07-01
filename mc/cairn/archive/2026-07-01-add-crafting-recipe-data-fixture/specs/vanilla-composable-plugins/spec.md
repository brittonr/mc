## ADDED Requirements

### Requirement: Crafting recipe selected-matrix data fixture

r[vanilla_composable_plugins.crafting_recipe_data_fixture] Crafting selected-matrix follow-on work MUST define a Java Edition 1.20.1 / protocol 763 selected recipe-data fixture before using target-version rows for stronger behavior evidence.

#### Scenario: Fixture contract is target scoped

r[vanilla_composable_plugins.crafting_recipe_data_fixture.target_scope]
- GIVEN the selected-matrix crafting core needs target-version recipe data
- WHEN reviewers inspect the fixture contract
- THEN it records source/provenance fields, target edition, target game version, target protocol, one selected shaped `minecraft:chest` recipe row, one selected shapeless `minecraft:oak_planks` recipe row, invalid/no-result fixture data, named grid and stack constants, and explicit non-claims
- AND it does not claim all recipes, arbitrary collection modes, shift-click/drag/split handling, data-pack loading, recipe-book UI behavior, automated crafter behavior, Paper/vanilla parity, Valence runtime integration, default plugin membership, broad vanilla parity, public-server safety, or production readiness.

### Requirement: Crafting recipe data fixture validation

r[vanilla_composable_plugins.crafting_recipe_data_fixture.validation] The selected-matrix fixture implementation MUST include focused validation with positive and negative tests before fixture rows are used by the core.

#### Scenario: Valid selected fixture passes

r[vanilla_composable_plugins.crafting_recipe_data_fixture.validation.positive]
- GIVEN a fixture declares Java Edition 1.20.1 / protocol 763, required provenance, the selected shaped chest row, the selected shapeless oak-planks row, valid item IDs, valid output counts, supported recipe kinds, supported primary-click collection boundaries, and required non-claims
- WHEN the focused fixture validator runs
- THEN it passes and records deterministic evidence under `docs/evidence/`.

#### Scenario: Invalid selected fixture fails clearly

r[vanilla_composable_plugins.crafting_recipe_data_fixture.validation.negative]
- GIVEN a fixture is missing target scope, missing provenance, missing selected rows, duplicates row IDs, has malformed shaped pattern or key data, has malformed shapeless ingredient data, has invalid item IDs, has zero output counts, uses unsupported recipe kinds, names unsupported collection modes, omits required non-claims, or claims all-recipe/default-runtime breadth
- WHEN the focused fixture validator runs
- THEN it fails with diagnostics naming the rejected field, malformed row, or overclaim.

### Requirement: Crafting recipe fixture-to-core handoff

r[vanilla_composable_plugins.crafting_recipe_data_fixture.core_handoff] Selected-matrix fixture work MAY wire the validated fixture into the existing selected-matrix core checker only as local unit evidence and MUST preserve broader non-claims.

#### Scenario: Fixture handoff remains local

r[vanilla_composable_plugins.crafting_recipe_data_fixture.core_handoff.local]
- GIVEN validated selected fixture rows feed the selected-matrix crafting core checker
- WHEN evidence is promoted
- THEN it claims only local selected-matrix core behavior over the validated fixture rows
- AND it does not claim Paper/vanilla parity, all-recipe breadth, arbitrary collection modes, Valence Bevy/ECS shell behavior, default plugin membership, public-server safety, or production readiness.

### Requirement: Crafting recipe data fixture documentation

r[vanilla_composable_plugins.crafting_recipe_data_fixture.docs] Selected-matrix fixture work MUST document the fixture scope, validation coverage, local core handoff boundary, evidence paths, and retained non-claims.

#### Scenario: Fixture docs are reviewable

r[vanilla_composable_plugins.crafting_recipe_data_fixture.docs.reviewable]
- GIVEN reviewers inspect selected-matrix crafting docs after fixture work
- WHEN they compare the behavior card, selected-matrix core, validated fixture, and promoted evidence
- THEN they can identify which artifacts prove local fixture/core semantics, which receipt handoff or runtime shell claims remain deferred, and which broad crafting claims remain out of scope.

### Requirement: Crafting recipe data fixture closeout

r[vanilla_composable_plugins.crafting_recipe_data_fixture.closeout] Selected-matrix fixture work MUST record baseline core validation, focused positive and negative fixture validation, local core handoff validation when touched, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and archive receipts before closeout.

#### Scenario: Fixture closeout is reviewable

r[vanilla_composable_plugins.crafting_recipe_data_fixture.closeout.log]
- GIVEN selected-matrix crafting data-fixture work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline crafting-core validation, focused positive/negative fixture validation, local core handoff validation when touched, Cairn gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, and archive receipts
- AND the evidence preserves non-claims for all-recipe breadth, Paper/vanilla parity, Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, public-server safety, and production readiness.
