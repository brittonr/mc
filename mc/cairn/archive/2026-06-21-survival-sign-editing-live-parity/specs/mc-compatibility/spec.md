# Delta: Survival sign editing live parity

## Requirements

### Requirement: Sign editing live contract

r[mc_compatibility.survival_sign_editing_live_parity.contract] The `survival-sign-editing-live-parity` row MUST define a bounded live sign editing contract before promotion.

#### Scenario: Contract names finite sign editing scope

r[mc_compatibility.survival_sign_editing_live_parity.contract.scope]
- GIVEN sign editing live work starts
- WHEN the contract is reviewed
- THEN it names sign position, side, initial text, submitted text payload, client open/update milestones, server acceptance metric, post-update observation, and normalized comparison fields
- AND all sign UI behavior, all sign variants, all formatting, arbitrary NBT, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Sign editing live checker

r[mc_compatibility.survival_sign_editing_live_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence sign editing metrics before promotion.

#### Scenario: Weak sign editing evidence fails closed

r[mc_compatibility.survival_sign_editing_live_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits open/update metrics, mismatches text, position, or side, reports stale child revisions, or claims all sign UI behavior
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid sign editing metric.

### Requirement: Isolated sign editing rail

r[mc_compatibility.survival_sign_editing_live_parity.rail] The harness MUST expose an isolated sign editing rail without changing sign persistence or targeted packet fixture rows.

#### Scenario: Existing sign rows remain unchanged

r[mc_compatibility.survival_sign_editing_live_parity.rail.isolated]
- GIVEN sign persistence and sign editor fixture rows already exist
- WHEN the live sign editing rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own live editing metrics.

### Requirement: Reviewable sign editing receipts

r[mc_compatibility.survival_sign_editing_live_parity.receipts] Paired sign editing receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_sign_editing_live_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow sign editing promotion

r[mc_compatibility.survival_sign_editing_live_parity.promotion] Matrix and bundle docs MUST promote only the bounded sign editing row after paired evidence passes.

#### Scenario: Broader sign behavior remains a non-claim

r[mc_compatibility.survival_sign_editing_live_parity.promotion.nonclaims]
- GIVEN paired sign editing evidence passes
- WHEN docs are updated
- THEN only the configured sign editing row is marked covered
- AND all sign UI behavior, all sign variants, formatting breadth, arbitrary NBT, all block entities, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Sign editing validation evidence

r[mc_compatibility.survival_sign_editing_live_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_sign_editing_live_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
