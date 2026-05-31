# Delta: Harden mc-compat harness coverage gates

## Requirements

### Requirement: Task evidence run logs are explicit

r[mc_compatibility.harness_coverage.task_evidence.run_log_status] The active Cairn task-evidence gate MUST reject completed tasks whose cited run logs lack explicit successful exit-status evidence.

#### Scenario: Completed task cites successful run output

r[mc_compatibility.harness_coverage.task_evidence.run_log_status.success]
- GIVEN a completed active Cairn task cites `docs/evidence/*.run.log`
- WHEN the task-evidence gate validates the task
- THEN the cited run log contains at least one `exit_status=` line
- AND every `exit_status=` line in that run log resolves to `0`.

#### Scenario: Missing or failed status fails closed

r[mc_compatibility.harness_coverage.task_evidence.run_log_status.rejects]
- GIVEN a completed active Cairn task cites a run log with no `exit_status=` line or a nonzero exit status
- WHEN the task-evidence gate validates the task
- THEN the gate fails and names the offending run log.

### Requirement: Task evidence paths are reviewable

r[mc_compatibility.harness_coverage.task_evidence.reviewable_paths] Completed Cairn tasks MUST NOT rely on transient build outputs or nested child-repo paths as review-critical evidence.

#### Scenario: Parent evidence copy is required

r[mc_compatibility.harness_coverage.task_evidence.reviewable_paths.parent_copy]
- GIVEN a completed active Cairn task references review-critical artifacts
- WHEN the task-evidence gate validates the task
- THEN review-critical artifact paths are parent-repo `docs/evidence/` paths
- AND path-like references rooted at `target/`, `stevenarella/`, `valence/`, `hyperion/`, or `Leafish/` are rejected.

### Requirement: Cited manifests cover cited run logs

r[mc_compatibility.harness_coverage.task_evidence.manifest_pairing] Completed Cairn tasks that cite `.b3` sidecars MUST cite a sidecar that covers each cited run log.

#### Scenario: Run log is in the manifest

r[mc_compatibility.harness_coverage.task_evidence.manifest_pairing.run_log]
- GIVEN a completed active Cairn task cites `docs/evidence/foo.run.log` and `docs/evidence/foo.b3`
- WHEN the task-evidence gate validates the task
- THEN at least one cited `.b3` manifest contains the cited run-log path
- OR the task contains an inline BLAKE3 digest for the run-log evidence.

### Requirement: Harness hardening evidence is durable

r[mc_compatibility.harness_coverage.validation] The harness hardening MUST include deterministic positive and negative fixtures plus repo-local validation evidence before archive.

#### Scenario: Validation covers happy and sad paths

r[mc_compatibility.harness_coverage.validation.fixtures]
- GIVEN the task-evidence gate is hardened
- WHEN checker self-tests run
- THEN they include positive completed-task fixtures
- AND negative fixtures for missing evidence label, missing docs evidence path, missing run log, missing BLAKE3, missing artifact, missing exit status, failed exit status, unrelated manifest, `target/` artifact path, and nested child-repo artifact path.

#### Scenario: Validation output is reviewable

r[mc_compatibility.harness_coverage.validation.reviewable]
- GIVEN the harness hardening is complete
- WHEN the change is archived
- THEN repo-local logs record checker self-tests, active task-evidence gate, evidence manifest check, Cairn gates, and Cairn validation under `docs/evidence/` with BLAKE3 sidecars.
