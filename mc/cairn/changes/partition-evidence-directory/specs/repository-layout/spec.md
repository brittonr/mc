# repository-layout Change Spec: Evidence directory partitioning

## Requirements

### Requirement: Evidence partition scheme

r[repository_layout.evidence_partition.scheme] Durable evidence under `docs/evidence/` SHOULD follow a documented partition scheme for receipts, run logs, manifests, generated indexes, oracle notes, and archived/historical artifacts.

#### Scenario: Evidence path class is clear

r[repository_layout.evidence_partition.scheme.clear]
- GIVEN a durable evidence artifact is created or promoted
- WHEN its path is reviewed
- THEN the path category identifies whether it is a receipt, run log, BLAKE3 manifest, generated index, oracle note, or archived artifact
- AND transient outputs are not mixed into durable evidence partitions.

### Requirement: Existing evidence inventory

r[repository_layout.evidence_partition.inventory] Existing `docs/evidence/` artifacts MUST be inventoried before bulk movement or partition migration.

#### Scenario: Existing citation is protected

r[repository_layout.evidence_partition.inventory.citation]
- GIVEN an existing Cairn task, spec, or evidence note cites an artifact
- WHEN evidence partition migration is planned
- THEN the artifact is marked stay-flat, migrate-now, migrate-later, or historical
- AND migration includes any required citation and manifest updates.

### Requirement: Manifest rules for partitioned evidence

r[repository_layout.evidence_partition.manifest_rules] Evidence manifest tooling MUST resolve partitioned durable evidence paths and reject stale, missing, target-only, result-only, or path-escaping entries.

#### Scenario: Partitioned manifest validates

r[repository_layout.evidence_partition.manifest_rules.valid]
- GIVEN a `.b3` manifest references artifacts in approved evidence partitions
- WHEN evidence manifest validation runs
- THEN digest rows resolve within durable evidence paths
- AND stale digests, missing files, path escapes, and transient-only paths fail.

### Requirement: Evidence index

r[repository_layout.evidence_partition.index] The repository SHOULD provide a reviewable evidence index mapping changes, scenarios, dates, and artifact classes to durable evidence paths.

#### Scenario: Reviewer finds scenario evidence

r[repository_layout.evidence_partition.index.findable]
- GIVEN a reviewer wants evidence for a maintained scenario or Cairn change
- WHEN they inspect the evidence index
- THEN they can find the relevant receipt, run log, manifest, and oracle-note paths when those artifacts exist.

### Requirement: Evidence partition guards

r[repository_layout.evidence_partition.guards] Partition migration MUST include guards or fixtures for stale manifests, broken citations, missing index rows, and unsafe paths.

#### Scenario: Broken citation fails

r[repository_layout.evidence_partition.guards.broken]
- GIVEN a task or index row points to a moved or missing evidence artifact
- WHEN validation runs
- THEN the broken path is reported
- AND archive is blocked until the path or manifest is corrected.

### Requirement: Evidence partition validation

r[repository_layout.evidence_partition.validation] Evidence partitioning MUST record evidence manifest checks, task evidence validation, index freshness checks, and Cairn gates before archive.

#### Scenario: Partition closeout is reviewable

r[repository_layout.evidence_partition.validation.log]
- GIVEN evidence directories have been partitioned or partition rules added
- WHEN the change is archived
- THEN reviewable logs show manifest validation, task evidence validation, index freshness checks, Cairn proposal/design/tasks gates, and Cairn validation.
