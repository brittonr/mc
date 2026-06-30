# mc-compatibility Change Spec: Checker crate migration

## Requirements

### Requirement: Checker crate migration inventory

r[mc_compatibility.checker_crate_migration.inventory] Checker crate migration work MUST inventory loose Rust checkers, legacy Python gates, already migrated checker crate rows, flake wiring, evidence inputs, owner, next action, and non-claim impact before migrating behavior.

#### Scenario: Checker migration scope is reviewable

r[mc_compatibility.checker_crate_migration.inventory.reviewable]
- GIVEN a checker migration wave is selected
- WHEN reviewers inspect the inventory
- THEN selected checkers, untouched debt rows, current command surfaces, flake checks, evidence inputs, owners, and next actions are named
- AND baseline checker validation is recorded before migration.

### Requirement: Checker core and shell boundary

r[mc_compatibility.checker_crate_migration.core_shell] Migrated checker logic MUST place deterministic validation cores in `tools/checkers/src/checkers/*` or shared library modules, while CLI shells own arguments, filesystem reads, stdout/stderr, and exit-code handling.

#### Scenario: Checker core is testable without filesystem access

r[mc_compatibility.checker_crate_migration.core_shell.testable]
- GIVEN explicit evidence text, modeled file metadata, or parsed fixture input
- WHEN a migrated checker core validates the input
- THEN tests can verify diagnostics without reading files, inspecting environment, spawning processes, or writing output
- AND the CLI shell remains responsible for side effects.

### Requirement: Checker wrapper parity

r[mc_compatibility.checker_crate_migration.wrapper_parity] Checker migration MUST preserve legacy `tools/check_*.rs` command surfaces, flake check names, evidence formats, diagnostics, self-test text, exit-code behavior, and non-claim boundaries unless a separate Cairn changes them.

#### Scenario: Existing checker consumers remain compatible

r[mc_compatibility.checker_crate_migration.wrapper_parity.stable]
- GIVEN an existing checker command, evidence file, or flake check invocation
- WHEN the checker is migrated behind a wrapper
- THEN the invocation, pass/fail result, diagnostics, self-test behavior, and copied evidence outputs remain compatible
- AND no new compatibility or parity claim is introduced by the migration.

### Requirement: Checker migration docs

r[mc_compatibility.checker_crate_migration.docs] The checker crate documentation SHOULD list migrated checker rows and untouched debt rows with owner, reason, non-claim impact, and next action.

#### Scenario: Checker debt is visible

r[mc_compatibility.checker_crate_migration.docs.visible]
- GIVEN a checker remains outside the crate after a migration wave
- WHEN reviewers inspect checker documentation
- THEN the row records why it remains standalone, who owns it, what non-claim impact applies, and what should happen before future behavior changes.

### Requirement: Checker crate migration tests

r[mc_compatibility.checker_crate_migration.tests] Each migrated checker MUST include positive tests for valid evidence and negative tests for malformed input, missing fields, stale baselines, unsafe paths, overclaims, and wrapper drift where applicable.

#### Scenario: Valid checker evidence passes

r[mc_compatibility.checker_crate_migration.tests.positive]
- GIVEN valid representative evidence for a migrated checker
- WHEN the checker core, CLI, and compatibility wrapper process it
- THEN tests prove the expected pass result and report output are produced.

#### Scenario: Invalid checker evidence fails clearly

r[mc_compatibility.checker_crate_migration.tests.negative]
- GIVEN malformed evidence, missing required fields, stale baselines, unsafe paths, overclaim markers, or wrapper drift
- WHEN migrated checker validation runs
- THEN tests prove the checker reports a specific diagnostic and exits unsuccessfully without promoting invalid evidence.

### Requirement: Checker crate migration validation

r[mc_compatibility.checker_crate_migration.validation] Checker migration work MUST record baseline and post-change checker self-tests/current-tree checks, crate tests, wrapper parity checks, affected flake checks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Checker migration closeout is reviewable

r[mc_compatibility.checker_crate_migration.validation.logs]
- GIVEN a checker migration wave is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change checker validation, positive and negative regression coverage, wrapper parity, affected flake checks, Cairn gates, and Cairn validation passing.
