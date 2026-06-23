# valence-hyperion-integration Change Spec: Observability and profiling hooks

## Requirements

### Requirement: Observability scope

r[valence_hyperion_integration.observability.scope] The integration MUST review Hyperion observability/profiling patterns and Valence tracing/logging surfaces before adding instrumentation hooks.

#### Scenario: Hook scope is bounded

r[valence_hyperion_integration.observability.scope.bounded]
- GIVEN observability work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify selected subsystems, disabled-mode behavior, optional adapters, and non-goals such as mandatory profiler dependency or production-capacity proof.

### Requirement: Observability contract

r[valence_hyperion_integration.observability.contract] Observability hooks MUST define span and metric names, bounded labels, redaction policy, overhead expectations, adapter boundaries, and disabled-mode behavior.

#### Scenario: High-cardinality label is rejected

r[valence_hyperion_integration.observability.contract.cardinality]
- GIVEN a proposed metric label contains unbounded player text, addresses, packet payloads, or arbitrary identifiers
- WHEN the observability contract validator or review checklist evaluates it
- THEN the label is rejected or transformed according to redaction policy.

### Requirement: Pure observability classification

r[valence_hyperion_integration.observability.core] Mapping subsystem events to observability records SHOULD be pure deterministic classification over explicit inputs, with clocks, exporters, and profilers in shells.

#### Scenario: Redaction is deterministic

r[valence_hyperion_integration.observability.core.redaction]
- GIVEN an event includes sensitive fields and public fields
- WHEN the classification core creates an observability record
- THEN sensitive fields are omitted, hashed, or redacted according to policy
- AND public labels remain bounded and deterministic.

### Requirement: Optional observability wiring

r[valence_hyperion_integration.observability.wiring] Valence MAY wire optional observability hooks into selected subsystems, but disabled hooks MUST preserve existing behavior and avoid mandatory exporter dependencies.

#### Scenario: Disabled hooks are no-op

r[valence_hyperion_integration.observability.wiring.disabled]
- GIVEN observability hooks are disabled
- WHEN existing Valence tests run
- THEN no profiler/exporter dependency is required
- AND subsystem behavior and public packet output remain unchanged.

### Requirement: Observability tests

r[valence_hyperion_integration.observability.tests] Observability work MUST include positive and negative tests for disabled hooks, enabled labels, redaction, unknown metrics, exporter failure, and overhead checks when overhead is claimed.

#### Scenario: Exporter failure does not stop server logic

r[valence_hyperion_integration.observability.tests.exporter_failure]
- GIVEN an optional exporter returns an error
- WHEN a hook emits an observability record
- THEN the error is reported according to policy
- AND core server behavior continues unless policy explicitly requires fail-closed behavior.

### Requirement: Observability validation

r[valence_hyperion_integration.observability.validation] Observability work MUST record tests, plugin-disabled regressions, smoke trace/export checks, overhead checks if claimed, and Cairn gates before archive.

#### Scenario: Observability closeout is reviewable

r[valence_hyperion_integration.observability.validation.log]
- GIVEN observability work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show classification tests, redaction tests, disabled-mode regressions, exporter failure fixtures, smoke trace output, overhead evidence if claimed, and Cairn validation.
