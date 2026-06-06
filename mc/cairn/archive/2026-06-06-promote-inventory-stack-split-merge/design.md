# Design: Promote inventory stack split/merge evidence

## Context

The current protocol-763 bundle promotes bounded inventory rows for drop, pickup, player-inventory click, open-container click, and block placement/use-item-on-block. It explicitly leaves stack split/merge as a non-claim. A prior archived row-contract change introduced expected metric names in `tools/check_mc_compat_row_contracts.rs`, but no accepted spec requirement or current evidence row promotes live stack split/merge coverage.

## Decisions

### 1. Promote one row, not inventory breadth

**Choice:** Add one `inventory-stack-split-merge` promotion row with fixed actor, item, source slot, destination slot, counts, and state-id sequence.

**Rationale:** A single deterministic row is reviewable and matches the existing evidence style. It avoids expanding into drag transactions, creative mode, all windows, or all click modes.

### 2. Keep checker logic pure

**Choice:** Implement the row checker as a pure parser/comparator over in-memory normalized KV or receipt-derived metrics, with an imperative shell only for file reads, argument parsing, and exit status.

**Rationale:** The correctness decision should be testable with positive and negative fixtures without starting Valence, Paper, Stevenarella, or a shell. This follows the repo rule for functional core plus imperative shell.

### 3. Reuse Valence survival/inventory fixture patterns where possible

**Choice:** Prefer extending existing `survival_compat` or inventory interaction instrumentation with named stack split/merge milestones rather than creating a broad new fixture server.

**Rationale:** Existing rails already record owned-local child revisions and server correlation. A small scenario-specific extension minimizes side effects and keeps old rows stable.

### 4. Promote only after durable evidence exists

**Choice:** Matrix/current-bundle docs are updated only after the checker, runner tests, live/dry-run evidence, evidence manifests, task evidence, Cairn gates, and Cairn validation pass.

**Rationale:** The current bundle is the operator-facing source of promoted scope. Promotion must not precede durable evidence.

## Risks / Trade-offs

- Client automation may not expose exact vanilla half-stack gestures. If so, record a bounded blocker or use a deterministic fixture-level server/client correlation without overclaiming UI gesture breadth.
- Valence `ClickSlotEvent` may not expose every state-id/count field needed. If missing, add explicit fixture logging or narrow the row to observable metrics and keep unobservable fields as non-claims.
- Updating evidence manifests can cascade through nested `.b3` manifests; refresh source-closure hashes before final validation.
