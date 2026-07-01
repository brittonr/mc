## ADDED Requirements

### Requirement: Crafting recipe selected-matrix receipt baseline

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.baseline] Selected-matrix receipt-handoff work MUST record the current fixture and core validation baseline before changing checker or handoff logic.

#### Scenario: Baseline is captured before handoff changes

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.baseline.captured]
- GIVEN the selected-matrix fixture and pure core already exist
- WHEN handoff implementation starts
- THEN baseline validation records the fixture validator result and core checker fixture-handoff result before new receipt-handoff logic is trusted.

### Requirement: Crafting recipe selected-matrix receipt handoff

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts] Selected-matrix crafting receipt-handoff work MUST verify that the validated Java Edition 1.20.1 / protocol 763 fixture rows match reviewable Paper/reference and Valence receipt evidence before promoting selected-matrix target-version behavior beyond local unit semantics.

#### Scenario: Handoff is bounded to the selected matrix

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.bounded]
- GIVEN the selected crafting fixture and candidate Paper/reference plus Valence receipt evidence are available
- WHEN reviewers inspect the handoff contract
- THEN it maps target edition/version/protocol, shaped row identity, shapeless row identity, invalid/no-result row identity, input items, output items, output counts, primary-click collection mode, target inventory slots, backend identity, receipt paths, and required non-claims to normalized comparison fields
- AND it states that all-recipe breadth, arbitrary collection modes, shift-click/drag/split handling, data-pack loading, recipe-book UI behavior, recipe discovery or advancement behavior, automated crafter behavior, Valence runtime integration, default plugin membership, broad vanilla parity, public-server safety, and production readiness remain non-claims.

### Requirement: Crafting recipe selected-matrix receipt checker

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.checker] The handoff implementation MUST provide a focused checker with a pure deterministic comparison core plus a thin file-reading shell.

#### Scenario: Matching selected-matrix evidence passes

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.checker.positive]
- GIVEN the validated fixture rows and Paper/reference plus Valence receipt inputs describe the same selected shaped chest row, shapeless oak-planks row, invalid stick-input rejection row, and primary-click collection mode
- WHEN the checker runs
- THEN it passes with deterministic diagnostics naming the matched recipe ids, input items, output items, output counts, invalid/no-result diagnostic, target inventory slots, collection mode, and receipt inputs.

#### Scenario: Mismatched or overbroad evidence fails

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.checker.negative]
- GIVEN receipt evidence is missing, Valence-only, Paper-only, stale, malformed, scoped to the wrong row, mismatches shaped inputs, mismatches shapeless inputs, mismatches output items, mismatches output counts, mismatches target inventory slots, uses unsupported collection modes, omits required non-claims, or claims all-recipe/all-collection breadth
- WHEN the checker runs
- THEN it fails with a diagnostic naming the rejected field or overclaim.

### Requirement: Crafting recipe selected-matrix receipt evidence

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.evidence] Selected-matrix handoff work MAY reuse archived Paper/reference and Valence crafting receipts only when the checker proves they match the validated selected-matrix fixture and preserve the required non-claims; otherwise it MUST stop or produce fresh selected-matrix receipt evidence before promotion.

#### Scenario: Archived receipts are reused safely

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.evidence.reuse]
- GIVEN archived survival-crafting recipe-breadth receipt artifacts are selected as handoff inputs
- WHEN the handoff evidence is promoted
- THEN the promoted log records checker success, exact receipt input paths, BLAKE3 coverage, target scope, selected-matrix metrics, and retained non-claims
- AND no new live Paper/Valence run is implied unless a fresh receipt log is cited.

### Requirement: Crafting recipe selected-matrix receipt docs

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.docs] Crafting selected-matrix documentation MUST distinguish local fixture/core semantics, selected-matrix receipt handoff evidence, and deferred Valence runtime shell work.

#### Scenario: Handoff docs do not overclaim

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.docs.non_overclaiming]
- GIVEN reviewers inspect crafting selected-matrix docs after handoff
- WHEN they compare docs with promoted evidence
- THEN they can identify what the handoff proves, which receipt artifacts were used, and which runtime, breadth, data-pack, recipe-book, automated-crafter, or collection-mode claims remain deferred.

### Requirement: Crafting recipe selected-matrix receipt closeout

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.closeout] Selected-matrix receipt-handoff work MUST record baseline validation, focused positive and negative checker tests, handoff validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Receipt handoff closeout is reviewable

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts.closeout.log]
- GIVEN the selected-matrix receipt handoff change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline fixture/core validation, focused checker positive and negative tests, selected-matrix handoff validation, Cairn gates, Cairn validation, task-evidence validation, accepted-spec IDs, evidence-manifest freshness, flake evidence checks, and archive receipts
- AND the evidence preserves non-claims for all-recipe breadth, arbitrary collection modes, Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, public-server safety, and production readiness.
