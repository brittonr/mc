# Design: Gate Hyperion checkout retirement

## Context

The workspace currently treats `hyperion/` as an independent nested Rust repo with separate `jj` and Git state. Valence now owns selected integration helpers and contract metadata, but the parent repo still records Hyperion as a live reference source in docs, component registry configuration, accepted specs, archives, and evidence.

## Decisions

### 1. Treat deletion as a gated retirement, not cleanup

**Choice:** Require an explicit retirement gate before deleting the local checkout.

**Rationale:** Deletion is destructive local workspace state when the parent repository does not track `mc/hyperion`. A future change needs reviewable proof that the deletion is safe and intentional.

### 2. Block deletion while live references remain

**Choice:** Preserve `hyperion/` whenever accepted specs, docs/config, evidence, or active nested work still depend on it.

**Rationale:** References are not all equivalent, but accepted requirements and component registry entries prove the checkout is still part of the workspace model.

### 3. Keep the gate pure and evidence-backed

**Choice:** The retirement decision is a deterministic audit over explicit inputs: parent tracking, nested repo status, reference inventory, accepted specs, and promoted evidence paths.

**Rationale:** Reviewers should be able to inspect the evidence without relying on implicit local assumptions.

## Risks / Trade-offs

- The reference count is broad and includes historical archive/evidence files. Future deletion work should classify references instead of requiring every historical mention to disappear.
- Keeping the checkout preserves local state but also preserves nested-repo complexity until a future retirement change clears the blockers.
- The parent repo cannot represent physical deletion as a tracked diff unless ownership changes first.
