# valence-bevy-ecs Change Spec: Schedule hygiene gates

## Requirements

### Requirement: Schedule hygiene inventory

r[valence_bevy_ecs.schedule_hygiene.inventory] Schedule hygiene work MUST inventory current Valence schedule tooling, named sets, schedule-impacting checks, and evidence gaps before adding gates.

#### Scenario: Schedule tooling is reviewable

r[valence_bevy_ecs.schedule_hygiene.inventory.reviewable]
- GIVEN schedule hygiene work is selected
- WHEN reviewers inspect the inventory
- THEN existing schedule dump tools, named sets, default plugin behavior, ambiguity settings, and validation gaps are recorded
- AND heavy graph evidence is not required for non-schedule-impacting changes.

### Requirement: Schedule evidence policy

r[valence_bevy_ecs.schedule_hygiene.policy] The repository SHOULD define when Bevy schedule evidence is required for new plugins, schedules, system sets, ordering constraints, event-loop phases, and default plugin membership changes.

#### Scenario: Schedule-impacting change triggers evidence

r[valence_bevy_ecs.schedule_hygiene.policy.trigger]
- GIVEN a Cairn changes Bevy plugin registration, schedule labels, system sets, ordering constraints, event-loop phases, or default plugin membership
- WHEN the task plan is reviewed
- THEN the plan includes focused schedule evidence or records why schedule evidence is unnecessary.

### Requirement: Schedule receipts

r[valence_bevy_ecs.schedule_hygiene.receipts] Schedule evidence SHOULD record selected schedule labels, plugin configuration, expected sets/systems, disabled-plugin comparisons when relevant, and command provenance.

#### Scenario: Receipt identifies schedule facts

r[valence_bevy_ecs.schedule_hygiene.receipts.facts]
- GIVEN schedule evidence is produced for a Bevy change
- WHEN reviewers inspect the receipt
- THEN it names the command, schedule label, plugin configuration, expected sets or systems, and disabled/default comparison if applicable
- AND large graph artifacts are optional unless needed for review.

### Requirement: Schedule hygiene tests

r[valence_bevy_ecs.schedule_hygiene.tests] Schedule hygiene gates MUST include positive valid-schedule checks and negative unknown schedule, missing set, unintended default plugin, and ambiguity regression checks where feasible.

#### Scenario: Valid schedule check passes

r[valence_bevy_ecs.schedule_hygiene.tests.positive]
- GIVEN a selected Valence app/plugin configuration is valid
- WHEN the schedule hygiene check runs
- THEN expected schedules, sets, and plugin-owned systems are reported.

#### Scenario: Invalid schedule check fails clearly

r[valence_bevy_ecs.schedule_hygiene.tests.negative]
- GIVEN an unknown schedule, missing expected set, unintended default plugin insertion, or forbidden ambiguity is present in a fixture
- WHEN the schedule hygiene check runs
- THEN it fails with a diagnostic that names the missing or unexpected schedule fact.

### Requirement: Schedule evidence promotion

r[valence_bevy_ecs.schedule_hygiene.evidence] Task-cited schedule artifacts MUST be promoted under reviewable tracked evidence paths and include BLAKE3 manifests when cited by tasks or archive docs.

#### Scenario: Cited schedule evidence is durable

r[valence_bevy_ecs.schedule_hygiene.evidence.durable]
- GIVEN a task cites a schedule dump, schedule check log, or graph artifact
- WHEN evidence validation runs
- THEN the cited artifact is tracked outside transient `target/` output
- AND any manifest digest uses BLAKE3 unless an existing contract requires another algorithm.

### Requirement: Schedule hygiene validation

r[valence_bevy_ecs.schedule_hygiene.validation] Schedule hygiene work MUST record schedule checks, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Schedule hygiene closeout is reviewable

r[valence_bevy_ecs.schedule_hygiene.validation.log]
- GIVEN schedule hygiene work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative schedule hygiene checks, promoted schedule artifacts if cited, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
