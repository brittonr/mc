# repository-layout Change Spec: Artifact boundary cleanup

## Requirements

### Requirement: Artifact classification

r[repository_layout.artifact_boundaries.classification] The repository MUST classify artifacts as durable evidence, generated checked-in output, transient run/build output, or local scratch before cleanup rules are changed.

#### Scenario: Artifact class is documented

r[repository_layout.artifact_boundaries.classification.documented]
- GIVEN a file is produced by a build, run, evidence promotion, manifest refresh, or local experiment
- WHEN repository artifact rules are reviewed
- THEN the file class is documented
- AND its allowed location, tracking policy, and citation policy are clear.

### Requirement: Durable evidence location

r[repository_layout.artifact_boundaries.evidence_location] Review-critical Cairn evidence MUST live under `docs/evidence/` or another explicitly documented durable evidence path that is visible to repository checks.

#### Scenario: Task-cited evidence is durable

r[repository_layout.artifact_boundaries.evidence_location.task]
- GIVEN a Cairn task cites a run log, receipt, manifest, or review note
- WHEN task-evidence validation runs
- THEN the cited artifact is present under a durable evidence path
- AND target-only, result-only, root-transient, or missing artifacts are rejected.

### Requirement: Root directory ownership

r[repository_layout.artifact_boundaries.root_dirs] Empty or ambiguous root directories SHOULD be removed or documented with owner, purpose, and expected contents.

#### Scenario: Root directory is not ambiguous

r[repository_layout.artifact_boundaries.root_dirs.clear]
- GIVEN a top-level directory such as `config/` or `evidence/` exists
- WHEN a developer inspects the repository layout
- THEN docs or directory contents make its owner and purpose clear
- AND obsolete empty directories are removed or explicitly reserved.

### Requirement: Targeted ignore rules

r[repository_layout.artifact_boundaries.ignore_rules] Ignore rules MUST hide transient build/run outputs without hiding durable evidence, manifests, or source-controlled generated artifacts that checks need to inspect.

#### Scenario: Transient root output is ignored safely

r[repository_layout.artifact_boundaries.ignore_rules.safe]
- GIVEN a local run produces root `result-*`, root live logs, pycache, or other transient output
- WHEN VCS status and evidence checks run
- THEN transient artifacts do not appear as source changes
- AND `docs/evidence/` logs, receipts, `.b3` manifests, and generated checked-in files remain visible.

### Requirement: Evidence citation guard

r[repository_layout.artifact_boundaries.citation_guard] Cairn task evidence gates SHOULD reject citations to target-only, result-only, root-transient, missing, or unmanifested artifacts when durable evidence is required.

#### Scenario: Transient citation fails

r[repository_layout.artifact_boundaries.citation_guard.reject]
- GIVEN a checked Cairn task cites an artifact that exists only under `target/`, a `result-*` path, a root live log, or a missing path
- WHEN the task evidence gate runs
- THEN the citation is reported as invalid
- AND the task cannot be archived until durable evidence is copied and, when required, manifested.

### Requirement: Artifact cleanup validation

r[repository_layout.artifact_boundaries.validation] Artifact cleanup MUST record evidence manifest checks, task evidence gates, artifact-boundary guard output, and Cairn gates before archive.

#### Scenario: Cleanup closeout is reviewable

r[repository_layout.artifact_boundaries.validation.log]
- GIVEN artifact boundaries have been cleaned up
- WHEN the change is archived
- THEN reviewable logs show evidence manifest validation, task evidence validation, artifact-boundary guard output, Cairn proposal/design/tasks gates, and Cairn validation.
