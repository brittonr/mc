# repository-layout Change Spec: Checker crate consolidation

## Requirements

### Requirement: Checker crate contract

r[repository_layout.checker_crate_consolidation.contract] Evidence checkers SHOULD live in a repo-owned Rust checker crate with stable binary or wrapper names for existing flake checks.

#### Scenario: Existing command surface remains available

r[repository_layout.checker_crate_consolidation.contract.compat]
- GIVEN a checker moves from a standalone `tools/*.rs` file into a checker crate
- WHEN the corresponding flake check or documented command executes
- THEN the command name and high-level usage remain available
- AND any intentional rename is tied to a separate reviewed change.

### Requirement: Shared checker core

r[repository_layout.checker_crate_consolidation.shared_core] Shared checker parsing and diagnostic helpers MUST be pure functions over in-memory evidence text and typed records.

#### Scenario: Shared parser rejects malformed evidence

r[repository_layout.checker_crate_consolidation.shared_core.negative]
- GIVEN evidence text has malformed key-value rows, duplicate keys, missing required values, or broad truthy overclaims
- WHEN the shared checker core parses and validates it
- THEN deterministic diagnostics identify the invalid condition
- AND no filesystem, process, network, clock, or environment access occurs in the pure core.

### Requirement: Rust checker migration

r[repository_layout.checker_crate_consolidation.rust_migration] Migrated Rust checkers MUST preserve their evidence contract while moving domain-specific validation into crate binaries.

#### Scenario: Migrated checker remains equivalent

r[repository_layout.checker_crate_consolidation.rust_migration.parity]
- GIVEN a Rust checker has been migrated into the checker crate
- WHEN its valid and invalid fixtures run through the new binary
- THEN valid evidence still passes
- AND invalid evidence still fails with diagnostics that cover the same claim boundary as before.

### Requirement: Python checker migration policy

r[repository_layout.checker_crate_consolidation.python_policy] Legacy Python evidence gates MAY remain only as inventoried migration debt; touched or extended gates SHOULD migrate to Rust unless an explicit waiver records owner, reason, and next action.

#### Scenario: Touched Python gate is not silently extended

r[repository_layout.checker_crate_consolidation.python_policy.touched]
- GIVEN a legacy Python checker needs new validation behavior
- WHEN implementation work starts
- THEN the gate is migrated to Rust or a waiver records why migration is blocked
- AND the waiver includes owner, reason, non-claim impact, and next action.

### Requirement: Checker fixture coverage

r[repository_layout.checker_crate_consolidation.fixtures] Every migrated checker MUST include positive and negative fixtures for evidence it accepts and evidence it rejects.

#### Scenario: Negative fixture proves fail-closed behavior

r[repository_layout.checker_crate_consolidation.fixtures.negative]
- GIVEN a migrated checker has invalid evidence with missing fields, wrong values, malformed rows, or overclaiming claim keys
- WHEN fixture tests run
- THEN the checker fails closed with a specific diagnostic
- AND no invalid evidence is reported as acceptable.

### Requirement: Checker consolidation validation

r[repository_layout.checker_crate_consolidation.validation] Checker consolidation MUST record checker tests, selected flake checks, touched evidence gates, and Cairn gates before archive.

#### Scenario: Checker closeout is reviewable

r[repository_layout.checker_crate_consolidation.validation.log]
- GIVEN checker binaries have been consolidated
- WHEN the change is archived
- THEN reviewable logs show positive fixtures, negative fixtures, selected flake checks, any touched evidence/task gates, Cairn proposal/design/tasks gates, and Cairn validation.
