# Design: Retire Hyperion checkout

## Context

`mc/hyperion` is an independent nested checkout and is not parent-tracked. Recent work promoted selected Valence-owned contracts, schedule receipts, and metadata helpers, then added a retirement gate proving deletion was blocked until nested state and live references were handled.

## Decisions

### 1. Preserve nested work as evidence before deletion

**Choice:** Store a durable patch and nested status snapshot under `docs/evidence/archive/2026-07-01/hyperion-checkout-retirement/` before deleting the checkout.

**Rationale:** The parent repo cannot preserve the untracked checkout as a diff. A patch plus status and hashes makes the local state recoverable enough for review without pushing upstream.

### 2. Remove live layout ownership, not historical mentions

**Choice:** Remove active component-registry/layout/agent guidance that treats Hyperion as a current root, while leaving historical archives/evidence and Valence non-claim mentions citation-stable.

**Rationale:** Historical evidence must remain reviewable. The retirement boundary is whether a live checkout is required, not whether the word Hyperion appears in past evidence.

### 3. Retire Hyperion-local accepted obligations

**Choice:** Replace active Hyperion game-mode obligations with a retired-capability statement and add accepted requirements that future work must use an external source snapshot or restore a checkout explicitly.

**Rationale:** Accepted specs should not require local Hyperion commands after the checkout is removed.

### 4. Treat physical deletion as local cleanup

**Choice:** Delete `mc/hyperion` only after backup and validation inputs are in place, and record that deletion is local cleanup because parent tracking count is zero.

**Rationale:** Reviewers can validate the parent-repo effects and evidence; the directory removal itself is not a parent-tracked diff.

## Risks / Trade-offs

- Historical archives and evidence still mention Hyperion. That is intentional and does not imply a live checkout.
- A patch backup preserves local edits but not a complete standalone clone. Recovery still depends on obtaining the recorded base repository history or applying the patch to an equivalent checkout.
- Future Hyperion-derived work needs an explicit source snapshot/reference path instead of relying on a sibling checkout.
