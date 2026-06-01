# Delta: Inventory stack split merge rail

## Requirements

### Requirement: Contract

r[mc_compatibility.inventory_stack_split_merge.contract] The `inventory stack split/merge` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.inventory_stack_split_merge.contract.scope]
- GIVEN `inventory-stack-split-merge` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured item stack split into two stacks and merged back under one window/state-id sequence
- AND it states that all inventory transactions, drag actions, creative mode, all windows, all item lifecycle correctness, broad protocol coverage, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.inventory_stack_split_merge.checker] A deterministic checker MUST compare normalized metrics before `inventory stack split/merge` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.inventory_stack_split_merge.checker.rejects]
- GIVEN evidence is missing or mismatches initial slot/item/count, split action, carried stack count, destination slot/count, merge action, final slot counts, state id, and server click-slot correlation
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.inventory_stack_split_merge.evidence_standard] `inventory stack split/merge` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.inventory_stack_split_merge.evidence_standard.enforced]
- GIVEN the row requires live Valence receipt with client and server inventory metrics plus checker negative fixtures for wrong counts/state ids
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.inventory_stack_split_merge.rail] The harness MUST expose `inventory-stack-split-merge` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.inventory_stack_split_merge.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `inventory-stack-split-merge` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.inventory_stack_split_merge.artifacts] Review-critical `inventory stack split/merge` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.inventory_stack_split_merge.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.inventory_stack_split_merge.matrix] Acceptance matrix and current bundle MUST promote only the configured `inventory stack split/merge` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.inventory_stack_split_merge.matrix.nonclaims]
- GIVEN `inventory stack split/merge` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND all inventory transactions, drag actions, creative mode, all windows, all item lifecycle correctness, broad protocol coverage, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.inventory_stack_split_merge.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.inventory_stack_split_merge.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
