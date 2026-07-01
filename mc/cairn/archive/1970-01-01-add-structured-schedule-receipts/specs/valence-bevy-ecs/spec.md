# valence-bevy-ecs Change Spec: Structured schedule receipts

## Requirements

### Requirement: Structured schedule receipt inventory

r[valence_bevy_ecs.structured_schedule_receipts.inventory] Structured schedule receipt work MUST inventory current Valence schedule graph checks, schedule evidence policy, selected plugin configurations, DOT graph usage, ambiguity settings, disabled-plugin comparisons, and review-critical schedule facts before changing schedule evidence tooling.

#### Scenario: Schedule evidence baseline is reviewable

r[valence_bevy_ecs.structured_schedule_receipts.inventory.reviewable]
- GIVEN structured schedule receipt work is selected
- WHEN reviewers inspect the inventory
- THEN current core plugin set checks, event-loop phase checks, DOT graph string assertions, default/disabled plugin configurations, and evidence gaps are recorded
- AND private ordering points that remain non-contractual are identified.

### Requirement: Structured schedule receipt schema

r[valence_bevy_ecs.structured_schedule_receipts.schema] Valence schedule receipts SHOULD use a deterministic structured schema for selected schedule labels, plugin configuration, expected facts, observed facts, absent facts, ambiguity settings, diagnostics, command provenance, and optional content hashes.

#### Scenario: Receipt describes selected schedule facts

r[valence_bevy_ecs.structured_schedule_receipts.schema.facts]
- GIVEN schedule evidence is produced for a selected Valence app or plugin configuration
- WHEN reviewers inspect the receipt
- THEN it names the command, schedule label, plugin configuration, expected sets/systems, observed sets/systems where selected, disabled-plugin comparison if relevant, ambiguity mode, and pass/fail diagnostics
- AND large DOT graph artifacts remain optional diagnostic attachments.

### Requirement: Structured schedule receipt collection

r[valence_bevy_ecs.structured_schedule_receipts.collection] Structured schedule receipt collection MUST keep Bevy `App`/`World` inspection, filesystem writes, command execution, and JSON/Markdown rendering in shell code while keeping schedule fact validation deterministic over explicit in-memory inputs.

#### Scenario: Fact checker is pure

r[valence_bevy_ecs.structured_schedule_receipts.collection.pure]
- GIVEN collected schedule facts and expected schedule facts are passed to the receipt checker
- WHEN the checker evaluates them
- THEN it returns deterministic pass/fail rows and diagnostics from explicit inputs
- AND it does not access Bevy `World`, files, environment variables, processes, clocks, logging, or global mutable state.

### Requirement: Structured schedule receipt wiring

r[valence_bevy_ecs.structured_schedule_receipts.wiring] Selected Valence schedule hygiene checks SHOULD produce or validate structured receipts for default and disabled-plugin configurations without changing default plugin behavior.

#### Scenario: Existing schedule checks gain reviewable receipts

r[valence_bevy_ecs.structured_schedule_receipts.wiring.default]
- GIVEN a selected core plugin schedule check runs for the default Valence plugin group
- WHEN structured receipts are enabled or validated
- THEN expected schedule sets, selected systems, resources/events, and plugin configuration facts are recorded in the receipt
- AND default Valence behavior, feature gates, and plugin membership remain compatible.

### Requirement: Structured schedule receipt tests

r[valence_bevy_ecs.structured_schedule_receipts.tests] Structured schedule receipt work MUST include positive valid-schedule receipt tests and negative unknown schedule, missing set, unexpected plugin/system, ambiguity, disabled-plugin, and determinism tests where feasible.

#### Scenario: Valid schedule receipt passes

r[valence_bevy_ecs.structured_schedule_receipts.tests.positive]
- GIVEN a selected Valence app/plugin configuration installs expected schedules and sets
- WHEN the structured receipt check runs
- THEN the receipt reports the expected schedule facts and stable diagnostics.

#### Scenario: Invalid schedule receipt fails clearly

r[valence_bevy_ecs.structured_schedule_receipts.tests.negative]
- GIVEN a receipt fixture references an unknown schedule, missing set, unexpected system, unintended plugin insertion, forbidden ambiguity, or disabled-plugin regression
- WHEN the structured receipt check runs
- THEN it fails with a diagnostic naming the missing or unexpected schedule fact
- AND no large DOT graph comparison is required to understand the failure.

### Requirement: Structured schedule receipt validation

r[valence_bevy_ecs.structured_schedule_receipts.validation] Structured schedule receipt work MUST record focused Valence schedule receipt tests, selected core plugin checks, negative receipt fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Schedule receipt closeout is reviewable

r[valence_bevy_ecs.structured_schedule_receipts.validation.log]
- GIVEN structured schedule receipt work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, schema checks, positive and negative receipt tests, selected schedule/plugin checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
