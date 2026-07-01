## ADDED Requirements

### Requirement: Furnace smelting selected-row receipt baseline

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.baseline] Selected-row receipt handoff work MUST record the current fixture and core validation baseline before changing checker or handoff logic.

#### Scenario: Baseline is captured before handoff changes

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.baseline.captured]
- GIVEN the selected-row fixture and pure core already exist
- WHEN handoff implementation starts
- THEN baseline validation records the fixture validator result and core checker fixture-handoff result before new receipt-handoff logic is trusted.

### Requirement: Furnace smelting selected-row receipt handoff

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts] Selected-row furnace receipt-handoff work MUST verify that the validated Java Edition 1.20.1 / protocol 763 fixture row matches reviewable Paper/reference and Valence receipt evidence before promoting selected-row target-version behavior beyond local unit semantics.

#### Scenario: Handoff is bounded to one selected row

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.bounded]
- GIVEN the selected standard-furnace fixture and candidate Paper/reference plus Valence receipt evidence are available
- WHEN reviewers inspect the handoff contract
- THEN it maps target edition/version/protocol, furnace kind, input item, fuel item, output item, output count, cook ticks, burn ticks, backend identity, receipt paths, and required non-claims to normalized comparison fields
- AND it states that all-recipe breadth, all-fuel breadth, smoker behavior, blast-furnace behavior, hoppers, XP behavior, recipe-book synchronization, chunk-unload semantics, Valence runtime integration, default plugin membership, broad vanilla parity, public-server safety, and production readiness remain non-claims.

### Requirement: Furnace smelting selected-row receipt checker

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.checker] The handoff implementation MUST provide a focused checker with a pure deterministic comparison core plus a thin file-reading shell.

#### Scenario: Matching selected-row evidence passes

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.checker.positive]
- GIVEN the validated fixture row and Paper/reference plus Valence receipt inputs describe the same selected standard-furnace row
- WHEN the checker runs
- THEN it passes with deterministic diagnostics naming the matched input item, fuel item, output item, output count, cook ticks, burn ticks, and receipt inputs.

#### Scenario: Mismatched or overbroad evidence fails

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.checker.negative]
- GIVEN receipt evidence is missing, Valence-only, Paper-only, stale, malformed, scoped to the wrong row, mismatches item IDs, mismatches counts, mismatches cook ticks, mismatches burn ticks, omits required non-claims, or claims all-furnace/all-recipe breadth
- WHEN the checker runs
- THEN it fails with a diagnostic naming the rejected field or overclaim.

### Requirement: Furnace smelting selected-row receipt evidence

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.evidence] Selected-row handoff work MAY reuse archived Paper/reference and Valence furnace receipts only when the checker proves they match the validated selected-row fixture and preserve the required non-claims; otherwise it MUST stop or produce fresh selected-row receipt evidence before promotion.

#### Scenario: Archived receipts are reused safely

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.evidence.reuse]
- GIVEN archived survival-furnace smelting receipt artifacts are selected as handoff inputs
- WHEN the handoff evidence is promoted
- THEN the promoted log records checker success, exact receipt input paths, BLAKE3 coverage, target scope, selected-row metrics, and retained non-claims
- AND no new live Paper/Valence run is implied unless a fresh receipt log is cited.

### Requirement: Furnace smelting selected-row receipt docs

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.docs] Furnace selected-row documentation MUST distinguish local fixture/core semantics, selected-row receipt handoff evidence, and deferred Valence runtime shell work.

#### Scenario: Handoff docs do not overclaim

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.docs.non_overclaiming]
- GIVEN reviewers inspect furnace selected-row docs after handoff
- WHEN they compare docs with promoted evidence
- THEN they can identify what the handoff proves, which receipt artifacts were used, and which runtime or breadth claims remain deferred.

### Requirement: Furnace smelting selected-row receipt closeout

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.closeout] Selected-row receipt handoff work MUST record baseline validation, focused positive and negative checker tests, handoff validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Receipt handoff closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.closeout.log]
- GIVEN the selected-row receipt handoff change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline fixture/core validation, focused checker positive and negative tests, selected-row handoff validation, Cairn gates, Cairn validation, task-evidence validation, accepted-spec IDs, evidence-manifest freshness, flake evidence checks, and archive receipts
- AND the evidence preserves non-claims for all-recipe breadth, all-fuel breadth, Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, public-server safety, and production readiness.
