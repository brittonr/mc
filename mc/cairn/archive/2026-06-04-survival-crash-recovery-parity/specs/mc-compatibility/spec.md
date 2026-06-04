# Delta: Bounded survival crash-recovery parity

## Requirements

### Requirement: Survival crash-recovery contract

r[mc_compatibility.survival_crash_recovery_parity.contract] The `survival-crash-recovery-parity` row MUST define a bounded paired-reference evidence contract before any crash-recovery survival evidence is promoted.

#### Scenario: Contract names one crash-recovered block mutation

r[mc_compatibility.survival_crash_recovery_parity.contract.scope]
- GIVEN the crash-recovery row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic actor, block, position, isolated storage scope, ungraceful stop method, backend restart, reconnect, post-crash observation, Paper/reference backend, Valence backend, child revisions, and comparator metrics
- AND long-term durability, arbitrary crash consistency, multi-chunk persistence, all block entities, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Crash row parity checker

r[mc_compatibility.survival_crash_recovery_parity.checker] A deterministic Rust checker MUST compare normalized paired Paper/reference and Valence crash-recovery metrics before the row is promoted.

#### Scenario: Valid crash-recovery paired evidence passes

r[mc_compatibility.survival_crash_recovery_parity.checker.valid]
- GIVEN Paper and Valence records name `survival-crash-recovery-parity`, clean child revisions, `Dirt`, position `24,64,0`, forced stop, isolated storage, crash-recovery restart, reconnect, post-crash observation, and server recovery state
- WHEN the checker compares the records
- THEN it passes only if every configured metric is present and equal across Paper and Valence.

#### Scenario: Weak crash-recovery evidence fails closed

r[mc_compatibility.survival_crash_recovery_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, uses an unknown row, omits a configured crash metric, reports a stale required revision, lacks child revision metadata, uses a graceful shutdown metric, or mismatches the post-crash block state
- WHEN the checker compares the records
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated crash-recovery runner rail

r[mc_compatibility.survival_crash_recovery_parity.rail] The harness MUST expose an isolated `survival-crash-recovery-parity` rail without broadening existing survival or graceful world-persistence semantics.

#### Scenario: Graceful and crash rows stay separate

r[mc_compatibility.survival_crash_recovery_parity.rail.isolated]
- GIVEN existing survival rows and the `survival-world-persistence-restart` graceful row are maintained separately
- WHEN the crash-recovery rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the crash row records its own explicit client and server milestones for forced stop, crash-recovery restart, reconnect, and post-crash observation.

### Requirement: Crash-reference and Valence fixtures

r[mc_compatibility.survival_crash_recovery_parity.fixtures] Paper/reference and Valence fixtures MUST record comparable crash-recovery server metrics for the configured mutation.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.survival_crash_recovery_parity.fixtures.comparable]
- GIVEN the configured crash-recovery interaction runs against Paper/reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for actor, block, position, isolated storage, ungraceful stop, backend restart, reconnect, post-crash observation, backend identity, and row id
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable crash-recovery paired receipts

r[mc_compatibility.survival_crash_recovery_parity.receipts] Paired crash-recovery evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.survival_crash_recovery_parity.receipts.reviewable]
- GIVEN the crash-recovery row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper/reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow crash-recovery promotion

r[mc_compatibility.survival_crash_recovery_parity.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `survival-crash-recovery-parity` row after the paired comparator passes.

#### Scenario: Broad durability remains a non-claim

r[mc_compatibility.survival_crash_recovery_parity.promotion.nonclaims]
- GIVEN paired crash-recovery evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured crash-recovery row is marked covered
- AND long-term durability, arbitrary crash consistency, multi-chunk persistence, all block entities, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Crash-recovery validation and archive evidence

r[mc_compatibility.survival_crash_recovery_parity.validation] The change MUST record checker, paired comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.survival_crash_recovery_parity.validation.log]
- GIVEN the crash-recovery row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
