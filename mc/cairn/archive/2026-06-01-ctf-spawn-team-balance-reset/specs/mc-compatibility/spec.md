# Delta: CTF spawn team balance reset rail

## Requirements

### Requirement: Contract

r[mc_compatibility.ctf_spawn_team_balance_reset.contract] The `spawn/team balance/resource reset` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.ctf_spawn_team_balance_reset.contract.scope]
- GIVEN `ctf-spawn-team-balance-reset` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured join/team-selection/reset sequence with bounded team counts, spawn locations, inventory/resource state, and reset milestones
- AND it states that all team balancing algorithms, all maps, all resource loadouts, all reset triggers, production gameplay readiness, and full CTF correctness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.ctf_spawn_team_balance_reset.checker] A deterministic checker MUST compare normalized metrics before the `spawn/team balance/resource reset` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.ctf_spawn_team_balance_reset.checker.rejects]
- GIVEN evidence is missing or mismatches team counts, selected teams, spawn coordinates, initial resources, post-score or post-death reset state, inventory/resource ids, and server correlation ids
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.ctf_spawn_team_balance_reset.checker.standard]
- GIVEN the row requires live CTF receipt with team/spawn/resource matrix row and negative checker for imbalance or stale resources
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.ctf_spawn_team_balance_reset.rail] The harness MUST expose a `ctf-spawn-team-balance-reset` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.ctf_spawn_team_balance_reset.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `ctf-spawn-team-balance-reset` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.ctf_spawn_team_balance_reset.evidence] `spawn/team balance/resource reset` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.ctf_spawn_team_balance_reset.evidence.reviewable]
- GIVEN the `spawn/team balance/resource reset` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.ctf_spawn_team_balance_reset.matrix] Acceptance matrix and current-bundle docs MUST promote only the `spawn/team balance/resource reset` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.ctf_spawn_team_balance_reset.matrix.nonclaims]
- GIVEN `spawn/team balance/resource reset` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `spawn/team balance/resource reset` row is marked covered
- AND all team balancing algorithms, all maps, all resource loadouts, all reset triggers, production gameplay readiness, and full CTF correctness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.ctf_spawn_team_balance_reset.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.ctf_spawn_team_balance_reset.validation.log]
- GIVEN the `spawn/team balance/resource reset` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
