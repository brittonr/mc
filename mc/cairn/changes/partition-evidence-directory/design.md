# Design: Partition durable evidence directories

## Context

Evidence is intentionally durable under `docs/evidence/`, but a flat directory makes it hard to understand which artifacts are receipts, logs, manifests, generated indexes, or human oracle notes. Any partition must preserve review links and BLAKE3 validation.

## Decisions

### 1. Define partitions before moving files

**Choice:** Specify allowed evidence categories and directory patterns before migration.

**Rationale:** Reviewers need a stable mental model and tools need deterministic path rules.

### 2. Preserve existing citations until safely migrated

**Choice:** Existing cited paths may remain or be moved only with manifest/task-reference updates in the same change.

**Rationale:** Broken citations invalidate Cairn closeout evidence.

### 3. Keep manifests reviewable

**Choice:** `.b3` manifests must remain close enough to artifacts or indexed clearly enough that evidence checks and reviewers can resolve them.

**Rationale:** Digest verification is part of the evidence contract.

### 4. Add an evidence index

**Choice:** Generate or maintain an index that maps scenario/change/date to key receipt/log/manifest artifacts.

**Rationale:** Partitioning improves navigation only if discovery remains easy.

## Risks / Trade-offs

- Moving many artifacts can cause noisy diffs; mitigate by starting with new-evidence partition rules and migrating old evidence incrementally.
- Path changes can stale manifests; mitigate with manifest refresh and task-evidence gates.
- Too many directories can be harder than flat layout; mitigate with a small category set.
