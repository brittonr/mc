# mc-compatibility Change Spec: Block-entity update breadth promotion

## Requirements

### Requirement: Block-entity update breadth contract

r[mc_compatibility.block_entity_update_breadth_promotion.contract] The `block-entity-update-breadth` row MUST define a bounded non-sign block-entity promotion contract before coverage is claimed.

#### Scenario: Contract names one non-sign payload

r[mc_compatibility.block_entity_update_breadth_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor or fixture, non-sign block entity kind, position, normalized payload metric, packet row, backend evidence, child revisions if live, and checker metrics
- AND all block entities, arbitrary NBT parity, persistence breadth, sign editing, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity update breadth checker

r[mc_compatibility.block_entity_update_breadth_promotion.checker] A deterministic Rust checker MUST validate normalized non-sign block-entity update evidence before promotion.

#### Scenario: Valid non-sign block-entity evidence passes

r[mc_compatibility.block_entity_update_breadth_promotion.checker.valid]
- GIVEN normalized evidence names `block-entity-update-breadth`, the configured kind, position, payload metric, packet row, backend evidence, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak non-sign block-entity evidence fails closed

r[mc_compatibility.block_entity_update_breadth_promotion.checker.rejects]
- GIVEN evidence is missing the row id, names the wrong kind, position, packet row, or payload, omits backend evidence, lacks required revision metadata, or claims arbitrary NBT/all-block-entity coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, unexpected, or mismatched metric.

### Requirement: Block-entity update breadth rail

r[mc_compatibility.block_entity_update_breadth_promotion.rail] The harness MUST expose or select an isolated non-sign block-entity update rail without changing existing sign persistence, survival, inventory, CTF, combat, or network semantics.

#### Scenario: Sign and non-sign rows stay separate

r[mc_compatibility.block_entity_update_breadth_promotion.rail.isolated]
- GIVEN sign block-entity evidence already exists
- WHEN non-sign block-entity evidence is added
- THEN sign claims remain unchanged
- AND the non-sign row records its own fixture and checker output.

### Requirement: Block-entity update breadth artifacts

r[mc_compatibility.block_entity_update_breadth_promotion.artifacts] Review-critical non-sign block-entity artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and payload source

r[mc_compatibility.block_entity_update_breadth_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts or fixtures, logs, normalized inputs, checker output, BLAKE3 manifests, revision metadata, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow block-entity update breadth matrix promotion

r[mc_compatibility.block_entity_update_breadth_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured non-sign block-entity row after checker and evidence gates pass.

#### Scenario: Broader block-entity coverage remains a non-claim

r[mc_compatibility.block_entity_update_breadth_promotion.matrix.nonclaims]
- GIVEN non-sign block-entity evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND arbitrary NBT, all block entities, persistence breadth, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity update breadth validation evidence

r[mc_compatibility.block_entity_update_breadth_promotion.validation] The change MUST record checker, fixture or runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.block_entity_update_breadth_promotion.validation.log]
- GIVEN the non-sign block-entity row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, fixture/runner checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
