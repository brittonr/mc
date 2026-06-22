# Delta: Survival world multichunk durability parity

## Requirements

### Requirement: Multichunk durability contract

r[mc_compatibility.survival_world_multichunk_durability_parity.contract] The `survival-world-multichunk-durability-parity` row MUST define a bounded multi-chunk storage contract before promotion.

#### Scenario: Contract names finite storage scope

r[mc_compatibility.survival_world_multichunk_durability_parity.contract.scope]
- GIVEN multichunk durability work starts
- WHEN the contract is reviewed
- THEN it names target chunks, block positions, mutations, graceful restart boundary, forced-stop boundary, storage source, post-restart observations, and normalized comparison metrics
- AND long-term durability, arbitrary crash consistency, all chunks, backups, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Multichunk durability checker

r[mc_compatibility.survival_world_multichunk_durability_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence multi-chunk durability metrics before promotion.

#### Scenario: Weak durability evidence fails closed

r[mc_compatibility.survival_world_multichunk_durability_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits chunk coordinates, mismatches post-restart state, reports stale child revisions, relies only on auxiliary markers, or claims arbitrary durability
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid storage metric.

### Requirement: Isolated multichunk durability rail

r[mc_compatibility.survival_world_multichunk_durability_parity.rail] The harness MUST expose an isolated multichunk durability rail without changing existing single-block persistence or crash-recovery rows.

#### Scenario: Existing persistence rows remain unchanged

r[mc_compatibility.survival_world_multichunk_durability_parity.rail.isolated]
- GIVEN single-block world-persistence and crash-recovery rows are promoted
- WHEN the multichunk durability rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own chunk/storage metrics.

### Requirement: Reviewable multichunk durability receipts

r[mc_compatibility.survival_world_multichunk_durability_parity.receipts] Paired multichunk durability receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_world_multichunk_durability_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, storage-source notes, and manifests are present.

### Requirement: Narrow multichunk durability promotion

r[mc_compatibility.survival_world_multichunk_durability_parity.promotion] Matrix and bundle docs MUST promote only the bounded multichunk durability row after paired evidence passes.

#### Scenario: Broader durability remains a non-claim

r[mc_compatibility.survival_world_multichunk_durability_parity.promotion.nonclaims]
- GIVEN paired multichunk durability evidence passes
- WHEN docs are updated
- THEN only the configured multichunk durability row is marked covered
- AND long-term durability, arbitrary crash consistency, all chunks, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Multichunk durability validation evidence

r[mc_compatibility.survival_world_multichunk_durability_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_world_multichunk_durability_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
