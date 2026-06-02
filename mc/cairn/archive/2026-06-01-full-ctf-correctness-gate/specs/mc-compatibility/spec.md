# Delta: Full CTF correctness aggregate gate

## Requirements

### Requirement: Contract

r[mc_compatibility.full_ctf_correctness_gate.contract] The `full CTF correctness aggregate` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.full_ctf_correctness_gate.contract.scope]
- GIVEN `full-ctf-correctness-gate` work starts
- WHEN the evidence contract is reviewed
- THEN it names an aggregate checker over CTF rule ledger rows requiring every configured rule family to be covered before full CTF correctness can be claimed
- AND it states that full CTF correctness until all rule rows pass, production gameplay readiness, public-server safety, and broad Minecraft compatibility remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.full_ctf_correctness_gate.checker] A deterministic checker MUST compare normalized metrics before `full CTF correctness aggregate` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.full_ctf_correctness_gate.checker.rejects]
- GIVEN evidence is missing or mismatches rule family, status, receipt path, run log path, BLAKE3 manifest, forbidden-transition checks, negative fixture coverage, and current-bundle label
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.full_ctf_correctness_gate.evidence_standard] `full CTF correctness aggregate` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.full_ctf_correctness_gate.evidence_standard.enforced]
- GIVEN the row requires CTF ledger aggregate checker with negative fixtures for missing rule families and premature full-CTF claims
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.full_ctf_correctness_gate.rail] The harness MUST expose `full-ctf-correctness-gate` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.full_ctf_correctness_gate.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `full-ctf-correctness-gate` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.full_ctf_correctness_gate.artifacts] Review-critical `full CTF correctness aggregate` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.full_ctf_correctness_gate.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.full_ctf_correctness_gate.matrix] Acceptance matrix and current bundle MUST promote only the configured `full CTF correctness aggregate` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.full_ctf_correctness_gate.matrix.nonclaims]
- GIVEN `full CTF correctness aggregate` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND full CTF correctness until all rule rows pass, production gameplay readiness, public-server safety, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.full_ctf_correctness_gate.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.full_ctf_correctness_gate.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
