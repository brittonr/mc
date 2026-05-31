# Delta: Refresh RED BLUE scoring soak live evidence

## Requirements

### Requirement: Contract

r[mc_compatibility.red_blue_scoring_soak_live_refresh.contract] The `RED/BLUE scoring soak freshness` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.red_blue_scoring_soak_live_refresh.contract.scope]
- GIVEN `red-blue-scoring-soak-live-refresh` work starts
- WHEN the evidence contract is reviewed
- THEN it names one fresh live rerun of the maintained RED and BLUE scoring soak rails with copied receipts, run logs, and BLAKE3 manifests
- AND it states that full CTF correctness, production load, public-server safety, unbounded soak, broad Minecraft compatibility, and unrelated CTF rule rows remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.red_blue_scoring_soak_live_refresh.checker] A deterministic checker MUST compare normalized metrics before `RED/BLUE scoring soak freshness` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.red_blue_scoring_soak_live_refresh.checker.rejects]
- GIVEN evidence is missing or mismatches scenario status, RED score milestone, BLUE score milestone, server score path milestones, missing milestone lists, forbidden score/capture patterns, child revisions, receipt digests, and run-log digests
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.red_blue_scoring_soak_live_refresh.evidence_standard] `RED/BLUE scoring soak freshness` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.red_blue_scoring_soak_live_refresh.evidence_standard.enforced]
- GIVEN the row requires live Valence receipts/logs copied under docs/evidence with BLAKE3 manifests plus acceptance matrix/current bundle hash updates
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.red_blue_scoring_soak_live_refresh.rail] The harness MUST expose `red-blue-scoring-soak-live-refresh` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.red_blue_scoring_soak_live_refresh.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `red-blue-scoring-soak-live-refresh` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.red_blue_scoring_soak_live_refresh.artifacts] Review-critical `RED/BLUE scoring soak freshness` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.red_blue_scoring_soak_live_refresh.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.red_blue_scoring_soak_live_refresh.matrix] Acceptance matrix and current bundle MUST promote only the configured `RED/BLUE scoring soak freshness` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.red_blue_scoring_soak_live_refresh.matrix.nonclaims]
- GIVEN `RED/BLUE scoring soak freshness` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND full CTF correctness, production load, public-server safety, unbounded soak, broad Minecraft compatibility, and unrelated CTF rule rows remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.red_blue_scoring_soak_live_refresh.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.red_blue_scoring_soak_live_refresh.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
