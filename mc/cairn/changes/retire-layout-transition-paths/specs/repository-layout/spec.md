# repository-layout Change Spec: Transition path retirement

## Requirements

### Requirement: Transition path inventory

r[repository_layout.transition_path_retirement.inventory] The repository MUST inventory legacy transition paths and active references before removing transition-path support from layout resolution.

#### Scenario: Legacy reference is classified

r[repository_layout.transition_path_retirement.inventory.classified]
- GIVEN a code, docs, flake, test, or evidence reference names a legacy transition path
- WHEN the inventory is reviewed
- THEN the reference is classified as active, historical, generated, or removable
- AND active references have a migration action.

### Requirement: Canonical role paths

r[repository_layout.transition_path_retirement.canonical_paths] Active layout docs and commands MUST use canonical role-based component paths after transition-path retirement.

#### Scenario: Active docs use canonical paths

r[repository_layout.transition_path_retirement.canonical_paths.docs]
- GIVEN a developer reads current README, architecture, or agent guidance
- WHEN component roots are named
- THEN the docs use canonical role paths for active client, server, compat, config, and fixture roots
- AND legacy paths appear only as historical or migration context.

### Requirement: Resolver transition support retirement

r[repository_layout.transition_path_retirement.resolver] The layout resolver SHOULD stop accepting legacy transition roots as active defaults once canonical role paths are established.

#### Scenario: Legacy root receives actionable diagnostic

r[repository_layout.transition_path_retirement.resolver.diagnostic]
- GIVEN only a legacy transition root exists for a component
- WHEN required layout resolution runs
- THEN the resolver reports the missing canonical role path and names the migration action
- AND it does not silently select the legacy root as the active component.

### Requirement: Layout resolver tests

r[repository_layout.transition_path_retirement.tests] Transition-path retirement MUST include positive tests for canonical roots and negative tests for ambiguous or invalid roots.

#### Scenario: Duplicate roots fail closed

r[repository_layout.transition_path_retirement.tests.duplicate]
- GIVEN both a canonical role root and legacy transition root exist for the same component
- WHEN layout resolution runs
- THEN deterministic diagnostics report ambiguity
- AND the runner does not guess which root to use.

### Requirement: Historical path documentation

r[repository_layout.transition_path_retirement.docs] Historical evidence MAY keep legacy path references only when docs make clear that they are historical and not active defaults.

#### Scenario: Historical evidence remains understandable

r[repository_layout.transition_path_retirement.docs.history]
- GIVEN archived evidence mentions a legacy transition path
- WHEN a reviewer reads active layout docs
- THEN the active docs explain the canonical path and, when necessary, the historical path context
- AND current tasks do not cite legacy paths as active roots.

### Requirement: Transition retirement validation

r[repository_layout.transition_path_retirement.validation] Transition-path retirement MUST record layout tests, missing-checkout diagnostics, runner dry-runs, and Cairn gates before archive.

#### Scenario: Retirement closeout is reviewable

r[repository_layout.transition_path_retirement.validation.log]
- GIVEN transition-path support has been retired or deprecated
- WHEN the change is archived
- THEN reviewable logs show canonical-root positive tests, invalid-root negative tests, selected runner dry-runs, Cairn proposal/design/tasks gates, and Cairn validation.
