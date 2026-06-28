# Design: Extract mc-compat client-driver functional core

## Context

The client driver has a natural split: decide what evidence a set of client runs proves, then perform the side effects needed to obtain those runs. Today those concerns are interleaved.

## Decisions

### 1. Model client run plans as data

**Choice:** Derive client usernames, indices, session count, timeout, log strategy, restart needs, and dry-run evidence mode through pure plan builders.

**Rationale:** Orchestration decisions can be validated without spawning the client.

### 2. Extract evidence classification core

**Choice:** Combine `SingleClientRun`-like inputs into a pure output/evidence summary that evaluates success patterns, scenario milestones, server correlation, projectile checks, and classification strings.

**Rationale:** The most error-prone logic is deterministic over logs and exit codes.

### 3. Keep live execution as shell

**Choice:** Xvfb, process spawning, timeout handling, log reads/writes, restart transitions, and stdout/stderr stay in shell functions that produce run records.

**Rationale:** Tests for core behavior remain fast and mock-free.

### 4. Preserve receipt compatibility

**Choice:** Existing `ClientRunEvidence` fields and classification labels remain stable unless a separate schema Cairn changes them.

**Rationale:** Downstream receipts and evidence gates depend on current labels.

## Risks / Trade-offs

- Some dry-run evidence construction may still need scenario-specific hooks; keep those hooks explicit.
- Run records may need normalization to avoid leaking shell-only path details into pure tests.
- Existing tests may need fixture helpers; keep helpers deterministic and local.
