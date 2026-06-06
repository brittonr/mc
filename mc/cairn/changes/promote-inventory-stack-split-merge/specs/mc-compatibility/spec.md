# Delta: Promote inventory stack split/merge evidence

## Requirements

### Requirement: Stack split/merge promotion contract

r[mc_compatibility.inventory_stack_split_merge_promotion.contract] The `inventory-stack-split-merge` row MUST define a bounded promotion contract before any matrix or current-bundle coverage is claimed.

#### Scenario: Promotion scope is exact

r[mc_compatibility.inventory_stack_split_merge_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one deterministic actor, item, source slot, destination slot, initial count, split action, carried count, merge action, final counts, state-id sequence, Valence server correlation, child revisions, and comparator/checker metrics
- AND drag transactions, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Stack split/merge checker

r[mc_compatibility.inventory_stack_split_merge_promotion.checker] A deterministic Rust checker MUST validate normalized stack split/merge evidence before promotion.

#### Scenario: Valid row evidence passes

r[mc_compatibility.inventory_stack_split_merge_promotion.checker.valid]
- GIVEN a receipt or normalized KV record names `inventory-stack-split-merge`, clean child revisions, the configured actor/item/source slot/destination slot/counts, state-id sequence, and Valence server `ClickSlot` correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak row evidence fails closed

r[mc_compatibility.inventory_stack_split_merge_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale or unknown child revisions, omits state-id data, mismatches source/destination slot counts, records the wrong item, lacks server `ClickSlot` correlation, contains Valence-only unreviewable target output, or claims broad inventory coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated stack split/merge rail

r[mc_compatibility.inventory_stack_split_merge_promotion.rail] The harness MUST expose an isolated `inventory-stack-split-merge` rail without changing existing inventory, survival, CTF, protocol, combat, or negative-live semantics.

#### Scenario: Existing rows stay separate

r[mc_compatibility.inventory_stack_split_merge_promotion.rail.isolated]
- GIVEN existing maintained inventory rows cover drop, pickup, player-inventory click, open-container click, and block placement/use-item-on-block
- WHEN the stack split/merge rail is added
- THEN existing scenario milestones and non-claims remain unchanged
- AND the new row records its own client and server milestones for split, merge, state-id, slot counts, and server correlation.

### Requirement: Reviewable stack split/merge artifacts

r[mc_compatibility.inventory_stack_split_merge_promotion.artifacts] Review-critical stack split/merge artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.inventory_stack_split_merge_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized comparator or KV inputs, checker output, BLAKE3 manifests, child revisions, and any oracle limitation checkpoint are present under `docs/evidence/`.

### Requirement: Narrow stack split/merge matrix promotion

r[mc_compatibility.inventory_stack_split_merge_promotion.matrix] Acceptance matrix and current-bundle docs MUST promote only the configured `inventory-stack-split-merge` row after checker and evidence gates pass.

#### Scenario: Broader inventory remains a non-claim

r[mc_compatibility.inventory_stack_split_merge_promotion.matrix.nonclaims]
- GIVEN stack split/merge evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured stack split/merge row is marked covered
- AND drag transactions, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Stack split/merge validation evidence

r[mc_compatibility.inventory_stack_split_merge_promotion.validation] The change MUST record checker, runner, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.inventory_stack_split_merge_promotion.validation.log]
- GIVEN the stack split/merge row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
