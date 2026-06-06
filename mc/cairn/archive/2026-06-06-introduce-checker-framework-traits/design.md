# Design: Introduce checker framework traits

## Context

Evidence checkers are review-critical but currently independent single-file programs. A shared framework should improve consistency without hiding each checker’s specific contract. It should be possible to test validation logic entirely in memory.

## Decisions

### 1. Define a minimal `Checker` shell contract

**Choice:** A checker implementation exposes `name`, `parse_args`, `run`, and `self_test` methods. The generic main shell handles success/failure exit formatting.

**Rationale:** This removes boilerplate while preserving explicit per-checker argument and validation logic.

### 2. Define `KeyValueEvidence` for parsed records

**Choice:** Common key/value parsing returns a deterministic map with duplicate-key, empty-key, and malformed-line diagnostics. Validation code reads through a trait with `value` and helper methods.

**Rationale:** Several checkers already implement this pattern with slight drift.

### 3. Keep validation cores checker-owned

**Choice:** The framework provides helpers for `require_exact`, `require_present`, token require/reject, clean revision checks, overclaim rejection, and fixture assertion. It does not encode row-specific semantics.

**Rationale:** Shared mechanics should not blur evidence contracts.

### 4. Migrate incrementally

**Choice:** Start with two highly similar inventory evidence checkers before broad adoption.

**Rationale:** The first migration can prove CLI/self-test/output parity and refine the trait boundary before touching every checker.

## Risks / Trade-offs

- A framework can make one-off checkers harder to read if it becomes too clever; keep helpers small and explicit.
- Migrating every checker at once would risk evidence-gate churn; use exemplar migration first.
- Common exit formatting must preserve current logs expected by tasks and docs.