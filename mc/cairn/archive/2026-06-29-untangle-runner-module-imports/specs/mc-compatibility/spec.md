# mc-compatibility Change Spec: Runner module import boundaries

## Requirements

### Requirement: Explicit production imports

r[mc_compatibility.runner_modularity.explicit_imports] Production mc-compat runner modules MUST use explicit imports from owning modules instead of broad root wildcard imports.

#### Scenario: Production dependency is visible

r[mc_compatibility.runner_modularity.explicit_imports.visible]
- GIVEN a production runner module depends on a type, function, or constant
- WHEN the module imports that dependency
- THEN the import names the owning module explicitly
- AND production code does not rely on `use super::*` to reach root-owned symbols.

### Requirement: Shared type ownership

r[mc_compatibility.runner_modularity.shared_type_ownership] Shared runner data types SHOULD live in modules that own their responsibility rather than remaining in the root entrypoint solely to satisfy broad imports.

#### Scenario: Type home matches responsibility

r[mc_compatibility.runner_modularity.shared_type_ownership.home]
- GIVEN a shared runner type is used by multiple modules
- WHEN its owner is selected
- THEN the type is defined with the module that owns its responsibility
- AND consumers import it through that owner module.

### Requirement: Import-boundary positive tests

r[mc_compatibility.runner_modularity.import_boundary_positive_tests] The change MUST include positive coverage proving explicit production imports and scoped test imports are accepted.

#### Scenario: Allowed imports pass

r[mc_compatibility.runner_modularity.import_boundary_positive_tests.accepts]
- GIVEN production modules use explicit imports and test modules use scoped local imports
- WHEN the import-boundary check or focused tests run
- THEN the allowed import forms pass.

### Requirement: Import-boundary negative tests

r[mc_compatibility.runner_modularity.import_boundary_negative_tests] The change MUST include negative coverage proving production `use super::*` regressions are rejected.

#### Scenario: Wildcard production import fails

r[mc_compatibility.runner_modularity.import_boundary_negative_tests.rejects]
- GIVEN a production runner module reintroduces a root wildcard import
- WHEN the import-boundary check or focused test fixture runs
- THEN the regression is rejected with an actionable diagnostic.

### Requirement: Import-boundary validation

r[mc_compatibility.runner_modularity.import_boundary_validation] The change MUST record runner tests, import-boundary checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.import_boundary_validation.logs]
- GIVEN import cleanup is complete
- WHEN the change is closed
- THEN reviewable logs show runner behavior preserved and production wildcard imports mechanically guarded.
