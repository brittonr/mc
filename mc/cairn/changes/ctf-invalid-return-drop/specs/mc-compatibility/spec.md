# Delta: CTF invalid return/drop rail

## Requirements

### Requirement: Contract

r[mc_compatibility.ctf_invalid_return_drop.contract] The `invalid flag return/drop` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.ctf_invalid_return_drop.contract.scope]
- GIVEN `ctf-invalid-return-drop` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured invalid flag return or drop attempt with unchanged flag state and no unexpected score
- AND it states that all invalid return/drop permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.ctf_invalid_return_drop.checker] A deterministic checker MUST compare normalized metrics before the `invalid flag return/drop` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.ctf_invalid_return_drop.checker.rejects]
- GIVEN evidence is missing or mismatches flag identity, actor team, pre-state, invalid return/drop action, post-state, score counters, forbidden transitions, and server containment milestone
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.ctf_invalid_return_drop.checker.standard]
- GIVEN the row requires live negative CTF receipt with client/server attempted-action evidence and forbidden-transition scan
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.ctf_invalid_return_drop.rail] The harness MUST expose a `ctf-invalid-return-drop` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.ctf_invalid_return_drop.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `ctf-invalid-return-drop` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.ctf_invalid_return_drop.evidence] `invalid flag return/drop` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.ctf_invalid_return_drop.evidence.reviewable]
- GIVEN the `invalid flag return/drop` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.ctf_invalid_return_drop.matrix] Acceptance matrix and current-bundle docs MUST promote only the `invalid flag return/drop` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.ctf_invalid_return_drop.matrix.nonclaims]
- GIVEN `invalid flag return/drop` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `invalid flag return/drop` row is marked covered
- AND all invalid return/drop permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.ctf_invalid_return_drop.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.ctf_invalid_return_drop.validation.log]
- GIVEN the `invalid flag return/drop` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
