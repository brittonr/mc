# mc-compatibility Change Spec: Typed-event readiness

## Requirements

### Requirement: Typed-event readiness contract

r[mc_compatibility.typed_event_readiness.contract] Maintained scenarios SHOULD migrate from substring fallback to typed-event-ready status when client milestones, server milestones, forbidden patterns, and receipt timeline evidence have typed-event equivalents or explicit derivation rules.

#### Scenario: Row-level readiness is complete

r[mc_compatibility.typed_event_readiness.contract.complete]
- GIVEN a scenario row is marked `typed-event-ready`
- WHEN its milestone and forbidden-pattern surfaces are inspected
- THEN each required client milestone, server milestone, and forbidden pattern is backed by a typed event or named derivation rule
- AND the row records no new compatibility claim solely because typed events exist.

### Requirement: Typed-event readiness checker

r[mc_compatibility.typed_event_readiness.checker] The scenario-manifest checker MUST evaluate typed-event readiness and fallback waivers from pure in-memory scenario and fixture data.

#### Scenario: Incomplete readiness fails closed

r[mc_compatibility.typed_event_readiness.checker.negative]
- GIVEN a scenario row is marked `typed-event-ready`
- WHEN a required client typed event, server typed event, forbidden-pattern mapping, or derivation rule is missing
- THEN the checker fails with a diagnostic naming the scenario and missing typed-event surface.

### Requirement: Controlled migration

r[mc_compatibility.typed_event_readiness.migration] Eligible scenario rows MAY move to `typed-event-ready` only after parity fixtures prove existing milestone IDs, forbidden IDs, receipt scenario names, and non-claim wording remain stable.

#### Scenario: Migrated row preserves evidence semantics

r[mc_compatibility.typed_event_readiness.migration.parity]
- GIVEN a row moves from substring fallback to typed-event-ready
- WHEN dry-run or receipt-shape validation runs
- THEN required/observed/missing milestone calculations match the pre-migration contract
- AND receipt non-claims remain unchanged.

### Requirement: Typed-event-first tests

r[mc_compatibility.typed_event_readiness.tests] The runner MUST include positive and negative tests proving typed-event-ready rows prefer typed-event evidence and fail closed before substring fallback can hide missing structured events.

#### Scenario: Missing typed events are not masked

r[mc_compatibility.typed_event_readiness.tests.fail_closed]
- GIVEN a typed-event-ready fixture omits a required typed event but includes the legacy substring
- WHEN the typed-event oracle evaluates the fixture
- THEN the fixture fails with a structured missing-event diagnostic.

### Requirement: Typed-event readiness documentation

r[mc_compatibility.typed_event_readiness.docs] README and evidence docs MUST explain typed-event-ready status, substring fallback waivers, and reviewer expectations for migrated rows.

#### Scenario: Reviewer can distinguish migration states

r[mc_compatibility.typed_event_readiness.docs.review]
- GIVEN a reviewer inspects scenario manifest output
- WHEN a row is typed-event-ready or waiver-backed fallback
- THEN the row's validation basis and next action are visible without reading live logs.

### Requirement: Typed-event readiness validation

r[mc_compatibility.typed_event_readiness.validation] The change MUST record focused runner tests, typed-event oracle fixtures, scenario-manifest checks, affected dry-run checks, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves typed-event migration safety

r[mc_compatibility.typed_event_readiness.validation.log]
- GIVEN rows are migrated or waiver-backed
- WHEN the change is archived
- THEN reviewable logs show typed-event positive and negative fixtures, manifest migration-state checks, Cairn proposal/design/tasks gates, and Cairn validation.
