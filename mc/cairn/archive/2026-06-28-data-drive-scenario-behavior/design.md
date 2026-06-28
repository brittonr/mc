# Design: Data-drive mc-compat scenario behavior

## Context

The runner already treats `SCENARIO_SPECS` as a central catalog for names and milestones, but behavior remains split across multiple match statements. A more composable model should make scenario rows carry the facts consumed by planning, env derivation, evidence, and receipts.

## Decisions

### 1. Extend scenario metadata incrementally

**Choice:** Add structured metadata for behavior facts in stages instead of replacing every specialized match at once.

**Rationale:** Some rails have special logic; incremental migration keeps risk bounded.

### 2. Prefer generated/static data over ad hoc matches

**Choice:** Facts such as run strategy, typed-event edges, env intents, evidence selector flags, and non-claims should be data fields or generated tables where possible.

**Rationale:** A new scenario should be auditable by reviewing one metadata row and generated parity checks.

### 3. Keep escape hatches explicit

**Choice:** Specialized behavior may remain as named handler hooks referenced by metadata.

**Rationale:** Declarative data should not force unreadable encodings for genuinely custom behavior.

### 4. Validate scenario rows fail closed

**Choice:** Static validation must reject missing required facts, unknown env intents, invalid graph edges, duplicate aliases, and unsupported handler references.

**Rationale:** Metadata only improves composability if incomplete rows cannot silently pass.

## Risks / Trade-offs

- Data structures can grow too broad; keep fields grouped by consumer responsibility.
- Some existing behavior will need temporary adapters; retire them only after parity tests pass.
- Generated surfaces can drift; preserve freshness checks before archive.
