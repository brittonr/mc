# Delta: Survival container and block-entity breadth parity

## Requirements

### Requirement: Container block-entity breadth contract

r[mc_compatibility.survival_container_block_entity_breadth_parity.contract] The `survival-container-block-entity-breadth-parity` row MUST define a bounded container and block-entity matrix before promotion.

#### Scenario: Contract names finite storage scope

r[mc_compatibility.survival_container_block_entity_breadth_parity.contract.scope]
- GIVEN container/block-entity breadth work starts
- WHEN the contract is reviewed
- THEN it names exact container kinds, positions, transfer action or rejection, item metadata fields, non-sign block-entity payloads, and normalized comparison metrics
- AND all-container behavior, arbitrary NBT parity, all block entities, sign editing UI parity, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Container block-entity breadth checker

r[mc_compatibility.survival_container_block_entity_breadth_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence container/block-entity metrics before promotion.

#### Scenario: Weak storage evidence fails closed

r[mc_compatibility.survival_container_block_entity_breadth_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits container kind, omits transfer or payload metrics, mismatches configured metadata, reports stale child revisions, or claims all-container or arbitrary-NBT parity
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid storage or payload metric.

### Requirement: Isolated container block-entity rail

r[mc_compatibility.survival_container_block_entity_breadth_parity.rail] The harness MUST expose an isolated container/block-entity breadth rail without changing existing chest or sign persistence rows.

#### Scenario: Existing storage rows remain unchanged

r[mc_compatibility.survival_container_block_entity_breadth_parity.rail.isolated]
- GIVEN chest persistence and sign block-entity rows are promoted
- WHEN the container/block-entity breadth rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own container and payload metrics.

### Requirement: Reviewable container block-entity receipts

r[mc_compatibility.survival_container_block_entity_breadth_parity.receipts] Paired container/block-entity breadth receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_container_block_entity_breadth_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow container block-entity promotion

r[mc_compatibility.survival_container_block_entity_breadth_parity.promotion] Matrix and bundle docs MUST promote only the bounded container/block-entity row after paired evidence passes.

#### Scenario: Broader storage remains a non-claim

r[mc_compatibility.survival_container_block_entity_breadth_parity.promotion.nonclaims]
- GIVEN paired container/block-entity evidence passes
- WHEN docs are updated
- THEN only the configured container/block-entity row is marked covered
- AND all containers, arbitrary NBT, all block entities, sign editing UI, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Container block-entity validation evidence

r[mc_compatibility.survival_container_block_entity_breadth_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_container_block_entity_breadth_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
