# Design: Add Hyperion-derived load and packet tools

## Context

Hyperion's tools help run bots and inspect packets. Valence already has a packet inspector tree and examples. The integration should consolidate useful diagnostics while keeping tooling separate from runtime APIs and compatibility claims.

## Decisions

### 1. Tooling is repo-owned, not API

**Choice:** Place adapted tools under `tools/` or equivalent repo-owned surfaces, not under the public Valence crate exports.

**Rationale:** Diagnostics should evolve without breaking plugin authors.

### 2. Prefer reproducible configs

**Choice:** Tool runs use checked-in typed config or documented command wrappers with explicit targets, protocols, timeouts, and output paths.

**Rationale:** Evidence needs repeatable invocation and bounded side effects.

### 3. Redact and bound packet captures

**Choice:** Packet tools must document capture scope, redact or omit sensitive data where applicable, and bound output size.

**Rationale:** Diagnostics should not accidentally persist secrets or huge artifacts.

### 4. Separate load evidence from compatibility evidence

**Choice:** Load tools may produce performance or stress evidence, but compatibility claims still require scenario-specific or reference-backed receipts.

**Rationale:** A bot connection count does not prove semantic parity.

## Risks / Trade-offs

- Tooling can become stale if not exercised; mitigate with smoke checks.
- Packet inspection can collect sensitive payloads; mitigate with redaction and explicit output contracts.
- Load tests can be flaky; treat them as evidence with environment metadata rather than universal guarantees.
