# Design: mc-compat checker crate migration

## Context

`tools/checkers/` already establishes the desired pattern: pure checker cores, shared key-value parsing, thin CLI wrappers, migrated binaries, and legacy script compatibility wrappers. The next migration wave should extend that pattern without changing evidence semantics.

## Decisions

### 1. Migrate touched checker families first

**Choice:** Prioritize checkers that are actively modified or that share parsing/reporting logic with existing migrated checkers.

**Rationale:** This reduces risk while satisfying the workspace rule to migrate touched gates before extending validation behavior.

### 2. Keep legacy command surfaces stable

**Choice:** Existing `tools/check_*.rs` files remain as small delegates to the crate binary/core until consumers are explicitly moved.

**Rationale:** Flake checks, docs, and Cairn tasks cite existing command names.

### 3. Pure core, thin CLI

**Choice:** Checker cores parse in-memory evidence strings or modeled file sets and return diagnostics; CLI shells own argument parsing, file reads, stdout/stderr, and exit codes.

**Rationale:** Checker logic becomes easy to test with positive and negative fixtures.

### 4. Document migration inventory

**Choice:** Update the checker crate README with migrated rows, untouched debt rows, owners, next actions, and non-claim impact.

**Rationale:** Reviewers can see which gates are reusable and which remain standalone debt.

## Risks / Trade-offs

- Migrating too many unrelated checkers at once can obscure behavior changes; use small waves.
- Some loose scripts may rely on ad hoc filesystem scans; model those inputs before moving logic into the core.
- Legacy Python gates are debt but should not be churned unless touched or explicitly planned.
