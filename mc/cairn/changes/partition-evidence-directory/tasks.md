# Tasks

- [ ] [serial] Define durable evidence partitions, allowed artifact classes, and path patterns. r[repository_layout.evidence_partition.scheme]
- [ ] [depends:scheme] Inventory existing `docs/evidence/` artifacts and classify each as stay-flat, migrate-now, migrate-later, generated index, manifest, or oracle note. r[repository_layout.evidence_partition.inventory]
- [ ] [depends:inventory] Update evidence manifest/task-evidence tooling or docs for partitioned paths while preserving existing citations. r[repository_layout.evidence_partition.manifest_rules]
- [ ] [depends:manifest_rules] Add or refresh an evidence index that maps changes/scenarios/dates to durable artifacts and manifests. r[repository_layout.evidence_partition.index]
- [ ] [depends:index] Add migration fixtures or checks for stale manifests, broken task citations, missing index rows, and path escapes. r[repository_layout.evidence_partition.guards]
- [ ] [depends:guards] Run evidence manifest checks, task evidence gate, index freshness checks, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.evidence_partition.validation]
