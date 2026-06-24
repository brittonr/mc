# repository-layout Change Spec: Compat runner modularization

## Requirements

### Requirement: Runner functional-core boundary

r[repository_layout.compat_runner_modularization.boundary] The compatibility runner MUST document and enforce a boundary between pure deterministic core logic and imperative shell orchestration.

#### Scenario: Boundary is reviewable

r[repository_layout.compat_runner_modularization.boundary.review]
- GIVEN the runner modularization begins
- WHEN reviewers inspect the design and code layout
- THEN scenario parsing, scenario metadata, receipt models, receipt validation, and config normalization are assigned to pure core modules
- AND CLI parsing, filesystem access, process execution, Docker/Paper handling, sockets, clocks, environment access, stdout/stderr, and exit-code handling are assigned to the shell.

### Requirement: Scenario core extraction

r[repository_layout.compat_runner_modularization.scenario_core] Scenario definitions, milestone specs, forbidden-pattern specs, aliases, behavior metadata, and dry-run metadata MUST live outside the runner shell while preserving existing scenario semantics.

#### Scenario: Scenario behavior remains stable

r[repository_layout.compat_runner_modularization.scenario_core.parity]
- GIVEN scenario metadata has moved out of the shell
- WHEN the runner enumerates, parses, and dry-runs every maintained scenario
- THEN scenario names, aliases, required client milestones, required server milestones, forbidden patterns, behavior kinds, and migration states match the pre-move behavior.

### Requirement: Pure validation modules

r[repository_layout.compat_runner_modularization.pure_validation] Receipt, config, and evidence validation SHOULD be expressed as pure functions over in-memory inputs before any shell writes receipts or exits.

#### Scenario: Invalid validation input fails closed

r[repository_layout.compat_runner_modularization.pure_validation.negative]
- GIVEN an in-memory receipt/config fixture is missing required fields, has malformed values, has wrong typed fields, or contains broad compatibility overclaims
- WHEN the pure validation module evaluates it
- THEN deterministic diagnostics are returned
- AND no filesystem mutation, process execution, network access, or runtime state mutation occurs.

### Requirement: Dependency direction is shell-to-core

r[repository_layout.compat_runner_modularization.dependency_direction] Core runner modules MUST NOT import constants, helpers, or side-effecting functions from `main.rs` or another shell-only module.

#### Scenario: Core dependency audit passes

r[repository_layout.compat_runner_modularization.dependency_direction.audit]
- GIVEN the runner core modules are extracted
- WHEN dependency direction is inspected by tests, static checks, or review
- THEN shell modules depend on core modules
- AND core modules do not depend on shell-owned constants, process orchestration helpers, filesystem helpers, or CLI exit behavior.

### Requirement: Modularization validation

r[repository_layout.compat_runner_modularization.validation] The modularization MUST be validated with focused positive and negative tests plus existing dry-run/evidence gates before archive.

#### Scenario: Refactor closeout is reviewable

r[repository_layout.compat_runner_modularization.validation.log]
- GIVEN the runner internals are modularized
- WHEN the change is archived
- THEN reviewable logs show focused positive tests, focused negative tests, maintained dry-run receipt checks, any touched generated-surface freshness checks, Cairn proposal/design/tasks gates, and Cairn validation.
