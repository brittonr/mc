# Delta: CTF score limit win condition rail

## Requirements

### Requirement: Contract

r[mc_compatibility.ctf_score_limit_win_condition.contract] The `score limit / win condition` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.ctf_score_limit_win_condition.contract.scope]
- GIVEN `ctf-score-limit-win-condition` work starts
- WHEN the evidence contract is reviewed
- THEN it names one bounded match reaching a configured score limit and emitting the configured win/end state exactly once
- AND it states that all match settings, overtime/tiebreakers, scoreboard UI parity, all scoring races, production gameplay readiness, and full CTF correctness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.ctf_score_limit_win_condition.checker] A deterministic checker MUST compare normalized metrics before the `score limit / win condition` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.ctf_score_limit_win_condition.checker.rejects]
- GIVEN evidence is missing or mismatches score limit, team scores before final capture, final capture actor/team, win team, end-state milestone, duplicate-win guard, and post-win forbidden score changes
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.ctf_score_limit_win_condition.checker.standard]
- GIVEN the row requires live CTF receipt with score-limit contract, no duplicate win, and matrix/current-bundle non-claims preserved
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.ctf_score_limit_win_condition.rail] The harness MUST expose a `ctf-score-limit-win-condition` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.ctf_score_limit_win_condition.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `ctf-score-limit-win-condition` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.ctf_score_limit_win_condition.evidence] `score limit / win condition` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.ctf_score_limit_win_condition.evidence.reviewable]
- GIVEN the `score limit / win condition` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.ctf_score_limit_win_condition.matrix] Acceptance matrix and current-bundle docs MUST promote only the `score limit / win condition` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.ctf_score_limit_win_condition.matrix.nonclaims]
- GIVEN `score limit / win condition` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `score limit / win condition` row is marked covered
- AND all match settings, overtime/tiebreakers, scoreboard UI parity, all scoring races, production gameplay readiness, and full CTF correctness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.ctf_score_limit_win_condition.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.ctf_score_limit_win_condition.validation.log]
- GIVEN the `score limit / win condition` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
