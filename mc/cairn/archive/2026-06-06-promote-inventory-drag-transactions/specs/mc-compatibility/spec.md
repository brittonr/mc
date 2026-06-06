# Delta: Promote inventory drag transaction evidence

## Requirements

### Requirement: Drag transaction promotion contract

r[mc_compatibility.inventory_drag_transactions_promotion.contract] The `inventory-drag-transactions` row MUST define a bounded promotion contract before any matrix or current-bundle coverage is claimed.

#### Scenario: Promotion scope is exact

r[mc_compatibility.inventory_drag_transactions_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one deterministic actor, item, source slot, target slots, drag phase sequence, carried-stack state, final source and target counts, state-id sequence, Valence server quick-craft correlation, child revisions, and comparator/checker metrics
- AND all drag modes, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, stack split/merge outside its dedicated row, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Drag transaction checker

r[mc_compatibility.inventory_drag_transactions_promotion.checker] A deterministic Rust checker MUST validate normalized drag transaction evidence before promotion.

#### Scenario: Valid row evidence passes

r[mc_compatibility.inventory_drag_transactions_promotion.checker.valid]
- GIVEN a receipt or normalized KV record names `inventory-drag-transactions`, clean child revisions, the configured actor/item/source slot/target slots/final counts, drag start/add/end phase sequence, state-id sequence, and Valence server quick-craft correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak row evidence fails closed

r[mc_compatibility.inventory_drag_transactions_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale or unknown child revisions, omits state-id data, omits or reorders drag phases, mismatches source or target slot counts, records the wrong item, lacks server quick-craft correlation, contains Valence-only unreviewable target output, or claims all drag or broad inventory coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated drag transaction rail

r[mc_compatibility.inventory_drag_transactions_promotion.rail] The harness MUST expose an isolated `inventory-drag-transactions` rail without changing existing inventory, survival, CTF, protocol, combat, stack split/merge, or negative-live semantics.

#### Scenario: Existing rows stay separate

r[mc_compatibility.inventory_drag_transactions_promotion.rail.isolated]
- GIVEN existing maintained inventory rows cover drop, pickup, player-inventory click, open-container click, block placement/use-item-on-block, and one stack split/merge fixture
- WHEN the drag transaction rail is added
- THEN existing scenario milestones and non-claims remain unchanged
- AND the new row records its own client and server milestones for drag phases, state-id, slot counts, carried-stack state, and server correlation.

### Requirement: Reviewable drag transaction artifacts

r[mc_compatibility.inventory_drag_transactions_promotion.artifacts] Review-critical drag transaction artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.inventory_drag_transactions_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized comparator or KV inputs, checker output, BLAKE3 manifests, child revisions, and any oracle limitation checkpoint are present under `docs/evidence/`.

### Requirement: Narrow drag transaction matrix promotion

r[mc_compatibility.inventory_drag_transactions_promotion.matrix] Acceptance matrix and current-bundle docs MUST promote only the configured `inventory-drag-transactions` row after checker and evidence gates pass.

#### Scenario: Broader inventory remains a non-claim

r[mc_compatibility.inventory_drag_transactions_promotion.matrix.nonclaims]
- GIVEN drag transaction evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured drag transaction row is marked covered
- AND all drag modes, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, stack split/merge outside its dedicated row, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Drag transaction validation evidence

r[mc_compatibility.inventory_drag_transactions_promotion.validation] The change MUST record checker, runner, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.inventory_drag_transactions_promotion.validation.log]
- GIVEN the drag transaction row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
