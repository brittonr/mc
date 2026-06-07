# Design: Promote targeted packet rows to live parity

## Context

Eight packet areas now have deterministic fixture evidence and non-overclaiming receipts. Those rows are useful for parser/checker coverage, but fixture evidence alone does not prove live behavior across Paper, Valence, and Stevenarella. Live promotion should be selective and evidence-driven to avoid turning fixture rows into broad compatibility claims.

## Decisions

### 1. Select a small packet subset before implementation

**Choice:** Record the packet rows selected for live parity with a short rationale, expected live signal, and explicit non-claims before adding runner behavior.

**Rationale:** Some packet rows are easier to exercise live than others. Selection prevents a broad change from becoming an unbounded compatibility sweep.

### 2. Use runner scenarios as the live evidence boundary

**Choice:** Prefer `mc-compat-runner` scenarios or scenario behavior hooks that can exercise selected packets and emit normal receipts/logs.

**Rationale:** The runner already owns compatibility evidence, backend startup, client orchestration, and non-claim fields. Reusing it keeps evidence comparable with prior rows.

### 3. Require matrix promotion to be evidence-gated

**Choice:** Update acceptance matrix rows only after logs and receipts demonstrate the selected packet behavior live. Leave unexercised packet rows at their existing fixture-bounded status.

**Rationale:** The packet inventory is a claim surface. Promotion must follow evidence, not intent.

### 4. Add negative checker coverage for overclaims

**Choice:** Extend targeted packet checks so missing live logs, mismatched packet rows, stale receipts, or full-protocol overclaims are rejected.

**Rationale:** The checker should guard the same mistakes this promotion path is designed to avoid.

## Risks / Trade-offs

- Some packet behaviors may require richer client automation than current scenarios provide; blocked rows should stay fixture-bounded with a recorded reason.
- Live Paper checks can be slower and noisier than deterministic fixtures; receipts must include enough context for review.
- Promoting too many rows in one change can blur evidence. Keep the first live batch intentionally small.