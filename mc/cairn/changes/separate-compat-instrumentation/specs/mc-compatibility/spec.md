# mc-compatibility Change Spec: Compatibility instrumentation boundary

## Requirements

### Requirement: Instrumentation inventory

r[mc_compatibility.compat_instrumentation_boundary.inventory] The harness MUST inventory compatibility-specific client probes, capture hooks, MCP surfaces, server fixture milestones, and scenario toggles before moving or gating instrumentation code.

#### Scenario: Evidence hook is identified

r[mc_compatibility.compat_instrumentation_boundary.inventory.hook]
- GIVEN a probe, event, milestone, or scenario toggle contributes to compatibility evidence
- WHEN the instrumentation inventory is reviewed
- THEN the hook is listed with owner, component path, scenario usage, event vocabulary, and migration status.

### Requirement: Instrumentation boundary contract

r[mc_compatibility.compat_instrumentation_boundary.contract] Compatibility instrumentation SHOULD be isolated behind explicit modules, Cargo features, environment toggles, or harness-only entrypoints rather than being implicit in core client/server logic.

#### Scenario: Instrumentation is opt-in

r[mc_compatibility.compat_instrumentation_boundary.contract.opt_in]
- GIVEN a compat probe or harness-only action exists
- WHEN core component code is reviewed
- THEN the probe is reachable through an explicit instrumentation boundary
- AND default product behavior is not silently coupled to scenario-specific harness actions.

### Requirement: Instrumentation migration

r[mc_compatibility.compat_instrumentation_boundary.migration] Moving instrumentation MUST preserve required typed-event and milestone vocabulary unless the evidence checkers and fixtures migrate in the same change.

#### Scenario: Event vocabulary remains stable

r[mc_compatibility.compat_instrumentation_boundary.migration.events]
- GIVEN an instrumentation family is moved behind a boundary
- WHEN typed-event fixtures and scenario dry-runs execute
- THEN required event names, milestone IDs, correlation IDs, and non-claim fields remain equivalent
- OR any vocabulary change is accompanied by checker and fixture updates.

### Requirement: Instrumentation tests

r[mc_compatibility.compat_instrumentation_boundary.tests] Instrumentation boundary changes MUST include positive tests for enabled instrumentation and negative tests for disabled or core-only paths.

#### Scenario: Disabled instrumentation does not fire

r[mc_compatibility.compat_instrumentation_boundary.tests.disabled]
- GIVEN compat instrumentation is disabled or the component runs outside the harness profile
- WHEN the tested core path executes
- THEN harness-only events or scenario actions are not emitted
- AND core behavior remains valid for the tested path.

### Requirement: Instrumentation documentation

r[mc_compatibility.compat_instrumentation_boundary.docs] Component and harness docs SHOULD describe instrumentation boundaries, feature flags, event vocabulary ownership, and evidence implications.

#### Scenario: Instrumentation owner is discoverable

r[mc_compatibility.compat_instrumentation_boundary.docs.owner]
- GIVEN a developer needs to change a compat probe
- WHEN they read subtree-local docs or harness docs
- THEN they can find the owning module/feature, scenarios affected, tests to run, and evidence checker implications.

### Requirement: Instrumentation validation

r[mc_compatibility.compat_instrumentation_boundary.validation] Instrumentation boundary work MUST record affected component tests, typed-event fixtures, selected dry-runs/live checks if required, and Cairn gates before archive.

#### Scenario: Boundary closeout is reviewable

r[mc_compatibility.compat_instrumentation_boundary.validation.log]
- GIVEN compat instrumentation has been isolated or gated
- WHEN the change is archived
- THEN reviewable logs show enabled-path tests, disabled-path tests, typed-event fixture checks, selected scenario checks if required, Cairn proposal/design/tasks gates, and Cairn validation.
