# Delta: Bounded survival block-entity persistence parity

## Requirements

### Requirement: Survival block-entity persistence contract

r[mc_compatibility.survival_block_entity_persistence_parity.contract] The `survival-block-entity-persistence-parity` row MUST define a bounded paired-reference evidence contract before any block-entity persistence survival evidence is promoted.

#### Scenario: Contract names one sign block entity payload

r[mc_compatibility.survival_block_entity_persistence_parity.contract.scope]
- GIVEN the block-entity persistence row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic actor, block entity kind, sign text payload, position, restart method, reconnect, post-restart observation, Paper/reference backend, Valence backend, child revisions, and comparator metrics
- AND all block entities, arbitrary NBT parity, sign editing UI semantics, multi-chunk persistence, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity row parity checker

r[mc_compatibility.survival_block_entity_persistence_parity.checker] A deterministic Rust checker MUST compare normalized paired Paper/reference and Valence sign block-entity persistence metrics before the row is promoted.

#### Scenario: Valid block-entity paired evidence passes

r[mc_compatibility.survival_block_entity_persistence_parity.checker.valid]
- GIVEN Paper and Valence records name `survival-block-entity-persistence-parity`, clean child revisions, the configured sign block entity kind, position, text payload, restart method, reconnect, post-restart observation, and server persistence state
- WHEN the checker compares the records
- THEN it passes only if every configured metric is present and equal across Paper and Valence.

#### Scenario: Weak block-entity evidence fails closed

r[mc_compatibility.survival_block_entity_persistence_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, uses an unknown row, omits a configured sign metric, reports a stale required revision, lacks child revision metadata, uses the wrong block entity kind, mismatches the post-restart text payload, or reports the wrong position
- WHEN the checker compares the records
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated block-entity persistence runner rail

r[mc_compatibility.survival_block_entity_persistence_parity.rail] The harness MUST expose an isolated `survival-block-entity-persistence-parity` rail without broadening existing survival, graceful world-persistence, or crash-recovery semantics.

#### Scenario: Existing persistence rows stay separate

r[mc_compatibility.survival_block_entity_persistence_parity.rail.isolated]
- GIVEN existing survival rows, the graceful ordinary-block world-persistence row, and the crash-recovery row are maintained separately
- WHEN the block-entity persistence rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the block-entity row records its own explicit client and server milestones for sign mutation, restart, reconnect, and post-restart sign observation.

### Requirement: Block-entity reference and Valence fixtures

r[mc_compatibility.survival_block_entity_persistence_parity.fixtures] Paper/reference and Valence fixtures MUST record comparable sign block-entity persistence server metrics for the configured payload.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.survival_block_entity_persistence_parity.fixtures.comparable]
- GIVEN the configured sign block-entity interaction runs against Paper/reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for actor, block entity kind, text payload, position, restart method, reconnect, post-restart observation, backend identity, and row id
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable block-entity paired receipts

r[mc_compatibility.survival_block_entity_persistence_parity.receipts] Paired sign block-entity persistence evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.survival_block_entity_persistence_parity.receipts.reviewable]
- GIVEN the block-entity persistence row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper/reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow block-entity promotion

r[mc_compatibility.survival_block_entity_persistence_parity.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `survival-block-entity-persistence-parity` row after the paired comparator passes.

#### Scenario: Broad block-entity parity remains a non-claim

r[mc_compatibility.survival_block_entity_persistence_parity.promotion.nonclaims]
- GIVEN paired sign block-entity evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured sign block-entity row is marked covered
- AND all block entities, arbitrary NBT parity, sign editing UI semantics, multi-chunk persistence, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity validation and archive evidence

r[mc_compatibility.survival_block_entity_persistence_parity.validation] The change MUST record checker, paired comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.survival_block_entity_persistence_parity.validation.log]
- GIVEN the block-entity persistence row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
