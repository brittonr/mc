# Delta: Task evidence closeout gate

## Requirements

### Requirement: Active task evidence closeout

r[mc_compatibility.task_evidence_gate.active_closeout] Completed active Cairn tasks MUST cite durable local evidence before archive.

#### Scenario: Completed active task cites durable evidence

r[mc_compatibility.task_evidence_gate.active_closeout.completed_task]
- GIVEN an active `cairn/changes/*/tasks.md` file contains a checked task
- WHEN the task-evidence gate evaluates the task
- THEN the task contains an evidence-labeled line
- AND the task cites at least one existing `docs/evidence/` artifact
- AND the task cites verification command output as a `docs/evidence/*.run.log` artifact
- AND the task cites either an existing `docs/evidence/*.b3` manifest or an inline BLAKE3 digest

### Requirement: Task evidence gate fails closed

r[mc_compatibility.task_evidence_gate.fail_closed] The task-evidence checker MUST fail closed for checked tasks that omit durable copied evidence, verification output, BLAKE3 evidence, or existing artifact paths.

#### Scenario: Missing evidence field fails

r[mc_compatibility.task_evidence_gate.fail_closed.missing_field]
- GIVEN a checked task lacks an evidence label, copied evidence path, run log path, BLAKE3 manifest or digest, or cites a missing artifact
- WHEN the task-evidence gate runs
- THEN the gate fails with a diagnostic naming the task location and missing field

### Requirement: Task evidence gate workflow

r[mc_compatibility.task_evidence_gate.flake_workflow] The task-evidence gate MUST be available through a focused flake check and included in the maintained compatibility aggregate.

#### Scenario: Operator runs closeout gate locally

r[mc_compatibility.task_evidence_gate.flake_workflow.local]
- GIVEN an operator is preparing to complete or archive a Cairn change
- WHEN they run the focused flake check or maintained aggregate
- THEN the gate self-tests its positive and negative fixtures
- AND the gate scans active Cairn tasks against copied repo evidence

### Requirement: Task evidence validation is recorded

r[mc_compatibility.task_evidence_gate.validation_evidence] Task-evidence gate work MUST record validation output before archive.

#### Scenario: Validation evidence is copied under docs/evidence

r[mc_compatibility.task_evidence_gate.validation_evidence.local]
- GIVEN the task-evidence gate is implemented
- WHEN the change is archived
- THEN checker self-test output, active scan output, flake output, Cairn validation, Cairn gate output, and evidence manifest validation are copied under `docs/evidence/` with BLAKE3 evidence
