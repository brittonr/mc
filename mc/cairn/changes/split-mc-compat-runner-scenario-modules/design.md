# Design: Split mc-compat-runner scenario modules

## Context

`tools/mc-compat-runner` now has a clear scenario behavior/spec abstraction, but the abstraction lives in the same compilation unit as command parsing, backend startup, client process orchestration, log evaluation, receipt writing, and tests. The next maintenance step is a structural split that keeps the functional core testable without changing compatibility semantics.

## Decisions

### 1. Extract pure scenario core first

**Choice:** Move `Scenario`, `ScenarioSpec`, `ScenarioBehaviorKind`, behavior lookup, static specs, and spec validation into scenario-focused modules before touching orchestration code.

**Rationale:** This keeps the risky semantic surface small and lets tests prove the extracted module still exposes the same contract.

### 2. Keep imperative orchestration in `main.rs`

**Choice:** Leave command-line parsing, process spawning, environment mutation, filesystem reads/writes, and receipt emission in the main shell or clearly named shell modules.

**Rationale:** The repo requires functional-core / imperative-shell separation. Scenario validation should be testable without starting servers or clients.

### 3. Preserve public and evidence surfaces

**Choice:** Treat canonical scenario names, aliases, emitted receipts, manifest rows, dry-run logs, and non-claim flags as compatibility surfaces that must not drift during the split.

**Rationale:** The refactor is structural. Any user-visible or evidence-visible change needs its own explicit compatibility change.

### 4. Add fail-closed module tests

**Choice:** Keep or expand tests that build invalid scenario specs in memory and assert duplicate names, missing aliases, missing milestones, and unsupported hook/default combinations fail before evidence is produced.

**Rationale:** The module boundary should make negative tests cheaper and more precise than integration-only coverage.

## Risks / Trade-offs

- Moving code can accidentally widen visibility; keep module APIs small and intentional.
- Rust module splits can make test fixtures awkward; prefer pure fixture constructors over process-level mocks.
- Large mechanical moves can obscure semantic changes; commit the split separately from behavior additions.