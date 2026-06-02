# Delta: Inventory drag transaction rail

## Requirements

### Requirement: Contract

r[mc_compatibility.inventory_drag_transactions.contract] The `inventory drag transactions` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.inventory_drag_transactions.contract.scope]
- GIVEN `inventory-drag-transactions` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured drag transaction across a fixed set of slots with exact final item/count distribution
- AND it states that all drag modes, creative inventory, all windows, split/merge outside this row, full inventory semantics, broad protocol coverage, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.inventory_drag_transactions.checker] A deterministic checker MUST compare normalized metrics before `inventory drag transactions` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.inventory_drag_transactions.checker.rejects]
- GIVEN evidence is missing or mismatches window id, state id, drag phase sequence, source stack, target slots, per-slot final counts, carried remainder, and server transaction correlation
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.inventory_drag_transactions.evidence_standard] `inventory drag transactions` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.inventory_drag_transactions.evidence_standard.enforced]
- GIVEN the row requires live receipt and checker fixtures for drag phase order, target slots, and final distribution
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.inventory_drag_transactions.rail] The harness MUST expose `inventory-drag-transactions` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.inventory_drag_transactions.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `inventory-drag-transactions` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.inventory_drag_transactions.artifacts] Review-critical `inventory drag transactions` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.inventory_drag_transactions.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.inventory_drag_transactions.matrix] Acceptance matrix and current bundle MUST promote only the configured `inventory drag transactions` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.inventory_drag_transactions.matrix.nonclaims]
- GIVEN `inventory drag transactions` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND all drag modes, creative inventory, all windows, split/merge outside this row, full inventory semantics, broad protocol coverage, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.inventory_drag_transactions.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.inventory_drag_transactions.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
