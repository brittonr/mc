# repository-layout Change Spec: Workspace component registry

## Requirements

### Requirement: Component registry contract

r[repository_layout.component_registry.contract] The workspace SHOULD define a typed component registry that records component path, role, owner, VCS boundary, command boundary, default gate participation, and evidence policy.

#### Scenario: Registry row is complete

r[repository_layout.component_registry.contract.complete]
- GIVEN a component is represented in the registry
- WHEN registry validation runs
- THEN the row includes a repository-relative path, role, owner, VCS boundary, build/test command notes, default gate participation, and evidence policy
- AND invalid enum values or missing required fields are rejected.

### Requirement: Current component inventory

r[repository_layout.component_registry.current_inventory] The initial registry MUST describe the current workspace components and documented nested-repo exceptions before it is used to drive path moves.

#### Scenario: Current layout is captured

r[repository_layout.component_registry.current_inventory.captured]
- GIVEN Stevenarella, Valence, Hyperion, compat runner/config/fixtures, Cairn, docs/evidence, and any classified reference clients exist
- WHEN the registry is reviewed
- THEN each current role or exception is represented with its current path
- AND no component is silently reclassified by registry introduction alone.

### Requirement: Registry fixtures

r[repository_layout.component_registry.fixtures] Registry validation MUST include positive and negative fixtures for component rows and layout edge cases.

#### Scenario: Invalid registry fails closed

r[repository_layout.component_registry.fixtures.negative]
- GIVEN a registry fixture has a missing owner, duplicate role key, unsafe path escape, undocumented nested Git boundary, or invalid gate-participation value
- WHEN validation evaluates the fixture
- THEN deterministic diagnostics identify the invalid row
- AND no generated layout artifact is accepted.

### Requirement: Registry-derived surfaces

r[repository_layout.component_registry.generated_surfaces] Registry-derived docs or checks MAY be generated only as checked-in static artifacts or check-time outputs; runtime code MUST NOT evaluate Nickel to discover component layout.

#### Scenario: Runtime remains static

r[repository_layout.component_registry.generated_surfaces.runtime]
- GIVEN registry-derived artifacts exist
- WHEN the compatibility runner starts
- THEN it consumes checked-in Rust/static data or existing CLI arguments
- AND it does not evaluate Nickel at runtime.

### Requirement: Registry layout guard

r[repository_layout.component_registry.guard] The repository SHOULD use the registry to guard against undocumented component roots, nested Git directories, and gate participation drift.

#### Scenario: Undocumented component is reported

r[repository_layout.component_registry.guard.undocumented]
- GIVEN a new component-like directory, nested Git checkout, or gate-participating path appears outside the registry
- WHEN the registry guard runs
- THEN the path is reported with a classification diagnostic
- AND default validation does not treat it as an owned component until the registry is updated.

### Requirement: Registry validation evidence

r[repository_layout.component_registry.validation] The registry change MUST record registry validation, fixture tests, generated freshness checks if added, and Cairn gates before archive.

#### Scenario: Registry closeout is reviewable

r[repository_layout.component_registry.validation.log]
- GIVEN the component registry is introduced
- WHEN the change is archived
- THEN reviewable logs show positive fixtures, negative fixtures, registry validation, any generated-surface freshness checks, Cairn proposal/design/tasks gates, and Cairn validation.
