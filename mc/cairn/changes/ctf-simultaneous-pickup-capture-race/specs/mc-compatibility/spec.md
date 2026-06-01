# Delta: CTF simultaneous pickup capture race rail

## Requirements

### Requirement: Contract

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.contract] The `simultaneous pickup/capture race` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.contract.scope]
- GIVEN `ctf-simultaneous-pickup-capture-race` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured two-client race window with deterministic ordering oracle and exactly one accepted state transition
- AND it states that all race conditions, network adversarial safety, unbounded concurrency, full CTF correctness, production readiness, and broad Minecraft compatibility remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.checker] A deterministic checker MUST compare normalized metrics before the `simultaneous pickup/capture race` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.checker.rejects]
- GIVEN evidence is missing or mismatches client identities, team roles, action timestamps or ordered milestones, accepted transition, rejected transition, final flag state, final score, and race-window bounds
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.checker.standard]
- GIVEN the row requires multi-client live receipt with deterministic ordering metadata and negative forbidden-transition checks
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.rail] The harness MUST expose a `ctf-simultaneous-pickup-capture-race` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `ctf-simultaneous-pickup-capture-race` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.evidence] `simultaneous pickup/capture race` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.evidence.reviewable]
- GIVEN the `simultaneous pickup/capture race` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.matrix] Acceptance matrix and current-bundle docs MUST promote only the `simultaneous pickup/capture race` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.matrix.nonclaims]
- GIVEN `simultaneous pickup/capture race` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `simultaneous pickup/capture race` row is marked covered
- AND all race conditions, network adversarial safety, unbounded concurrency, full CTF correctness, production readiness, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.ctf_simultaneous_pickup_capture_race.validation.log]
- GIVEN the `simultaneous pickup/capture race` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
