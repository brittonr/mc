# Delta: CTF invalid pickup ownership rail

## Requirements

### Requirement: Contract

r[mc_compatibility.ctf_invalid_pickup_ownership.contract] The `invalid flag pickup/ownership` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.ctf_invalid_pickup_ownership.contract.scope]
- GIVEN `ctf-invalid-pickup-ownership` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured invalid flag pickup attempt by the wrong team or invalid owner state with no ownership transfer and no score
- AND it states that all invalid actions, all flag permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.ctf_invalid_pickup_ownership.checker] A deterministic checker MUST compare normalized metrics before the `invalid flag pickup/ownership` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.ctf_invalid_pickup_ownership.checker.rejects]
- GIVEN evidence is missing or mismatches player team, flag identity, pre-owner state, invalid pickup action, post-owner state, score counters, forbidden capture/score patterns, and containment outcome
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.ctf_invalid_pickup_ownership.checker.standard]
- GIVEN the row requires live Valence CTF receipt with negative containment checker and BLAKE3-backed logs
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.ctf_invalid_pickup_ownership.rail] The harness MUST expose a `ctf-invalid-pickup-ownership` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.ctf_invalid_pickup_ownership.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `ctf-invalid-pickup-ownership` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.ctf_invalid_pickup_ownership.evidence] `invalid flag pickup/ownership` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.ctf_invalid_pickup_ownership.evidence.reviewable]
- GIVEN the `invalid flag pickup/ownership` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.ctf_invalid_pickup_ownership.matrix] Acceptance matrix and current-bundle docs MUST promote only the `invalid flag pickup/ownership` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.ctf_invalid_pickup_ownership.matrix.nonclaims]
- GIVEN `invalid flag pickup/ownership` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `invalid flag pickup/ownership` row is marked covered
- AND all invalid actions, all flag permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.ctf_invalid_pickup_ownership.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.ctf_invalid_pickup_ownership.validation.log]
- GIVEN the `invalid flag pickup/ownership` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
