# Proposal: Partition durable evidence directories

## Why

`docs/evidence/` is the correct durable evidence location, but a flat directory with many receipts, logs, manifests, generated indexes, and oracle notes becomes hard to browse and audit. A documented partitioning scheme would improve reviewability while keeping existing evidence checks able to resolve cited artifacts.

## What Changes

- Define a partition scheme for durable evidence, such as dated run logs, manifests, generated indexes, oracle notes, and archive-only artifacts.
- Add migration rules for existing evidence without breaking current Cairn task/spec citations.
- Update evidence manifest refresh/check logic if paths or manifest adjacency change.
- Add an index or generated map so reviewers can find evidence by scenario/change/date.

## Impact

- **Files**: `docs/evidence/`, `.b3` manifests, generated evidence indexes, evidence manifest refresh/check tools if needed, Cairn task references if migrated, docs/architecture, and Cairn artifacts.
- **Testing**: evidence manifest refresh/check, task evidence gate, generated index freshness, path migration fixtures, and Cairn validation/gates.
- **Non-claims**: this reorganizes durable evidence only; it does not alter evidence content, scenario pass criteria, or compatibility claims.
