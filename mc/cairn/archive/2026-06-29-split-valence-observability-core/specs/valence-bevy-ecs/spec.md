# valence-bevy-ecs Change Spec: Observability core

## Requirements

### Requirement: Observability core boundaries

r[valence_bevy_ecs.observability_core.boundaries] Valence observability code SHOULD expose cohesive boundaries for configuration, metric taxonomy, labels/redaction, packet classification, export planning, and Bevy event shells.

#### Scenario: Observability responsibility has one owner

r[valence_bevy_ecs.observability_core.boundaries.ownership]
- GIVEN an observability responsibility is reviewed
- WHEN maintainers inspect observability modules
- THEN the responsibility is owned by a focused module
- AND unrelated taxonomy, redaction, classification, export, and Bevy shell concerns are not reintroduced into one module.

### Requirement: Observability pure core

r[valence_bevy_ecs.observability_core.core] Observability metric, label, redaction, packet classification, and export-plan decisions SHOULD be pure over explicit inputs.

#### Scenario: Redaction decision is testable without Bevy

r[valence_bevy_ecs.observability_core.core.testable]
- GIVEN packet, label, metric, config, or export summaries
- WHEN the observability core processes them
- THEN the result can be tested without Bevy events, exporter calls, logging, or schedule wiring.

### Requirement: Observability parity

r[valence_bevy_ecs.observability_core.parity] Observability splitting MUST preserve metric names, labels, redaction policy, packet classification, export outcomes, schedule behavior, and evidence non-claims.

#### Scenario: Observability behavior remains stable

r[valence_bevy_ecs.observability_core.parity.stable]
- GIVEN a supported pre-refactor observability input
- WHEN the split observability core and shell process the same input
- THEN metric, label, redaction, classification, export, schedule, and non-claim behavior remain equivalent.

### Requirement: Observability positive tests

r[valence_bevy_ecs.observability_core.positive_tests] The change MUST include positive tests for metric names, label values, redaction, packet ID classes, export plans, and phase event decisions.

#### Scenario: Supported observability paths pass

r[valence_bevy_ecs.observability_core.positive_tests.coverage]
- GIVEN representative supported observability inputs
- WHEN extracted observability cores process them
- THEN tests prove the expected labels, classifications, redactions, and export plans are produced.

### Requirement: Observability negative tests

r[valence_bevy_ecs.observability_core.negative_tests] The change MUST include negative tests for invalid packet IDs, sensitive label leaks, invalid label values, disabled phases, exporter failures, and missing config.

#### Scenario: Invalid observability paths fail closed

r[valence_bevy_ecs.observability_core.negative_tests.fail_closed]
- GIVEN invalid or unsafe observability inputs
- WHEN extracted observability cores or shells process them
- THEN tests prove the inputs are rejected, redacted, or contained according to current behavior.

### Requirement: Observability validation

r[valence_bevy_ecs.observability_core.validation] The change MUST record focused observability/server tests, affected schedule checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_bevy_ecs.observability_core.validation.logs]
- GIVEN observability splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative observability tests plus affected schedule checks and Cairn gates passing.
