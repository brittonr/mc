# Delta: Death respawn reconnect during death rail

## Requirements

### Requirement: Contract

r[mc_compatibility.death_respawn_reconnect_during_death.contract] The `reconnect during death` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.death_respawn_reconnect_during_death.contract.scope]
- GIVEN `death-respawn-reconnect-during-death` work starts
- WHEN the evidence contract is reviewed
- THEN it names one death event followed by disconnect before respawn, reconnect, and coherent dead/respawnable or respawned state according to the fixture policy
- AND it states that all reconnect timings, crash recovery, multi-client reconnect races, full death/respawn lifecycle, production readiness, and unbounded reconnect safety remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.death_respawn_reconnect_during_death.checker] A deterministic checker MUST compare normalized metrics before the `reconnect during death` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.death_respawn_reconnect_during_death.checker.rejects]
- GIVEN evidence is missing or mismatches death milestone, disconnect point, reconnect username/session, server retained death state, client post-reconnect state, respawn action, and final health/playable state
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.death_respawn_reconnect_during_death.checker.standard]
- GIVEN the row requires two-session live receipt with state-machine checker and forbidden stale/alive-state checks
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.death_respawn_reconnect_during_death.rail] The harness MUST expose a `death-respawn-reconnect-during-death` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.death_respawn_reconnect_during_death.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `death-respawn-reconnect-during-death` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.death_respawn_reconnect_during_death.evidence] `reconnect during death` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.death_respawn_reconnect_during_death.evidence.reviewable]
- GIVEN the `reconnect during death` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.death_respawn_reconnect_during_death.matrix] Acceptance matrix and current-bundle docs MUST promote only the `reconnect during death` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.death_respawn_reconnect_during_death.matrix.nonclaims]
- GIVEN `reconnect during death` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `reconnect during death` row is marked covered
- AND all reconnect timings, crash recovery, multi-client reconnect races, full death/respawn lifecycle, production readiness, and unbounded reconnect safety remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.death_respawn_reconnect_during_death.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.death_respawn_reconnect_during_death.validation.log]
- GIVEN the `reconnect during death` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
