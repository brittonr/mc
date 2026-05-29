# Design: Oracle property and mutation tests

## Context

`evaluate_scenario`, `evaluate_server_scenario`, projectile causality, receipt summary parsing, and safety envelope checks are pure enough to test without launching the world. The harness should exploit that by generating systematic positive and negative cases from each scenario definition.

## Decisions

### 1. Deterministic loops first

**Choice:** Use deterministic table-driven mutation tests in Rust before adding any random/property-test dependency.

**Rationale:** The scenario catalog is finite. A loop that removes each milestone and inserts each forbidden marker is simple, reproducible, and reviewable.

### 2. Positive fixtures are explicit

**Choice:** Each scenario has a canonical passing client fixture and, when server milestones exist, a canonical passing server fixture.

**Rationale:** Mutations need a known-good baseline. The fixture also documents what the oracle expects.

### 3. Negative tests assert diagnostics

**Choice:** Mutations must assert the expected missing milestone, forbidden marker, order violation, or receipt validation error.

**Rationale:** A test that only asserts `!passed` can hide the wrong failure boundary.

### 4. Keep live rails separate

**Choice:** These tests validate oracle behavior only. Live evidence remains required before promoting compatibility claims.

**Rationale:** Unit/property tests are necessary harness hardening, not gameplay evidence.

## Risks / Trade-offs

- Scenario fixtures duplicate some milestone names until the generated manifest exists.
- Mutating string-based fixtures can be brittle; typed event fixtures should replace them after the typed event oracle lands.
- Broad mutation coverage can make tests verbose, but failure diagnostics will be much clearer.
