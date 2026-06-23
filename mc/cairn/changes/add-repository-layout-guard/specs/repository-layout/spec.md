# repository-layout Change Spec: Repository layout guard

## Requirements

### Requirement: Layout guard contract

r[repository_layout.layout_guard.contract] The repository SHOULD define a layout guard that reports undocumented root directories, surprise nested Git checkouts, root transient artifacts, missing subtree documentation, generated marker drift, and component-registry mismatches.

#### Scenario: Guard scope is explicit

r[repository_layout.layout_guard.contract.scope]
- GIVEN the layout guard is introduced
- WHEN reviewers inspect its contract
- THEN the guard lists each diagnostic class, waiver mechanism, source-of-truth input, and non-claim
- AND it does not claim live compatibility, semantic parity, or evidence correctness outside layout policy.

### Requirement: Pure layout guard core

r[repository_layout.layout_guard.core] The layout guard core MUST be a pure deterministic function over an in-memory repository tree, registry/config inputs, and rule settings.

#### Scenario: Guard core has no side effects

r[repository_layout.layout_guard.core.pure]
- GIVEN the shell passes a modeled file tree and registry to the guard core
- WHEN guard validation runs
- THEN diagnostics are returned deterministically
- AND the core does not read files, inspect environment, spawn processes, use clocks, or mutate repository state.

### Requirement: Layout guard fixtures

r[repository_layout.layout_guard.fixtures] The guard MUST include positive and negative fixtures for valid layout and each enforced diagnostic class.

#### Scenario: Surprise nested Git fixture fails

r[repository_layout.layout_guard.fixtures.nested_git]
- GIVEN a fixture contains a nested Git checkout that is absent from documented exceptions or the component registry
- WHEN the guard evaluates the fixture
- THEN it reports the path as an undocumented nested Git boundary
- AND the fixture fails until the path is classified or removed.

### Requirement: Layout guard wiring

r[repository_layout.layout_guard.wiring] The guard MAY start as a focused or advisory check, but required diagnostics MUST fail once known transition-state findings are resolved or waived.

#### Scenario: Focused guard reports actionable diagnostics

r[repository_layout.layout_guard.wiring.focused]
- GIVEN the guard runs in focused mode
- WHEN layout findings exist
- THEN diagnostics include path, rule, owner or waiver hint, and suggested next action
- AND required findings fail the check.

### Requirement: Registry and artifact-rule integration

r[repository_layout.layout_guard.registry_integration] The guard SHOULD consume component-registry and artifact-boundary rules as inputs when those sources exist, instead of maintaining independent allowlists.

#### Scenario: Registry-owned root passes

r[repository_layout.layout_guard.registry_integration.registry]
- GIVEN a component root is documented in the component registry with expected VCS and evidence policy
- WHEN the layout guard evaluates the root
- THEN the root passes component-root classification checks
- AND any mismatch between registry data and observed layout is reported.

### Requirement: Layout guard validation

r[repository_layout.layout_guard.validation] Layout guard work MUST record guard fixture tests, focused check output, and Cairn gates before archive.

#### Scenario: Guard closeout is reviewable

r[repository_layout.layout_guard.validation.log]
- GIVEN the layout guard is implemented or wired
- WHEN the change is archived
- THEN reviewable logs show positive fixtures, negative fixtures for each diagnostic class, focused flake check output, Cairn proposal/design/tasks gates, and Cairn validation.
