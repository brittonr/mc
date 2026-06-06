# Design: Promote inventory drag transaction evidence

## Context

The current protocol-763 bundle promotes bounded inventory rows for drop, pickup, player-inventory click, open-container click, block placement/use-item-on-block, and one stack split/merge fixture. It explicitly leaves drag transactions as a non-claim. A prior archived row-contract change described expected drag metrics, but no accepted current evidence row promotes live drag transaction coverage.

## Decisions

### 1. Promote one drag transaction, not drag breadth

**Choice:** Add one `inventory-drag-transactions` promotion row with fixed actor, item, source slot, target slots, drag phase order, carried-stack state, final slot counts, and state-id sequence.

**Rationale:** A single deterministic drag path is reviewable and matches the existing evidence style. It avoids expanding into all drag modes, creative mode, all windows, all click modes, or broad inventory semantics.

### 2. Use a two-target distribution for exact counts

**Choice:** Configure `compatbot` to use `RedWool x64` from source slot `37`, left-drag across target slots `38` and `39`, and verify final counts `source=0`, `slot38=32`, `slot39=32`, `carried=0`.

**Rationale:** Two empty target slots make the distribution exact and avoid remainder-order ambiguity while still exercising drag start/add/end phases instead of ordinary split/merge clicks.

### 3. Keep checker logic pure

**Choice:** Implement the row checker as a pure parser/comparator over in-memory normalized KV or receipt-derived metrics, with an imperative shell only for file reads, argument parsing, and exit status.

**Rationale:** The correctness decision should be testable with positive and negative fixtures without starting Valence, Paper, Stevenarella, or a shell.

### 4. Reuse existing inventory fixture patterns where possible

**Choice:** Prefer extending the existing inventory/CTF fixture instrumentation with named drag milestones and server quick-craft correlation rather than creating a broad new fixture server.

**Rationale:** Existing rails already record owned-local child revisions and server correlation. A small scenario-specific extension minimizes side effects and keeps promoted rows stable.

### 5. Promote only after durable evidence exists

**Choice:** Matrix/current-bundle docs are updated only after the checker, runner tests, live/dry-run evidence, evidence manifests, task evidence, Cairn gates, and Cairn validation pass.

**Rationale:** The current bundle is the operator-facing source of promoted scope. Promotion must not precede durable evidence.

## Risks / Trade-offs

- Stevenarella may not expose a convenient vanilla quick-craft gesture. If so, record a bounded blocker or narrow implementation to the closest protocol-level quick-craft correlation without claiming UI gesture breadth.
- Valence `ClickSlotEvent` may not expose every drag phase field needed. If missing, add explicit fixture logging or narrow the row to observable metrics and keep unobservable fields as non-claims.
- Drag distribution can be remainder-sensitive for more than two targets. The configured two-target split avoids that ambiguity.
- Updating evidence manifests can cascade through nested `.b3` manifests; refresh source-closure hashes before final validation.
