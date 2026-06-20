# Design: Extract mc-compat harness planning core

## Context

The runner already has some pure scenario core, but much of the orchestration logic still lives next to process and filesystem operations. A planning core should answer "what should happen" with data. The shell should only perform the plan and report results.

## Decisions

### 1. Introduce explicit plan structs

**Choice:** Represent server plan, client session plan, receipt plan, artifact plan, and cleanup plan as deterministic structs derived from config and scenario metadata.

**Rationale:** Tests can assert full orchestration intent without standing up external services.

### 2. Keep side effects in the shell

**Choice:** The pure core performs no filesystem reads, process execution, environment mutation, Docker calls, sleeps, clocks, or network probes.

**Rationale:** The functional core remains fast, deterministic, and mock-free.

### 3. Preserve current public behavior first

**Choice:** The extraction starts with parity tests and keeps CLI parsing, default ports, scenario names, receipt paths, and non-claims unchanged.

**Rationale:** Architecture work must not change compatibility evidence semantics.

### 4. Model negative paths as plans

**Choice:** Invalid config, unsafe public-server inputs, missing receipt destinations, incompatible matrix flags, and cleanup hazards should produce structured plan diagnostics before shell execution.

**Rationale:** Fail-closed behavior should be testable without relying on live command failures.

## Risks / Trade-offs

- Refactoring a large runner can accidentally change CLI behavior; mitigate with baseline tests before edits and post-refactor parity tests.
- Plan structs can become too broad; mitigate by keeping each plan focused on one side-effect boundary.
- Some existing helpers may need temporary adapters; mitigate by migrating one boundary at a time.
