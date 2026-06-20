# mc-compatibility Change Spec: Failure evidence bundles

## Requirements

### Requirement: Failure-bundle contract

r[mc_compatibility.failure_evidence_bundles.contract] Failed or blocked mc-compat runs SHOULD produce a bounded diagnostic bundle that records scenario, backend, command summary, first failure, artifact paths, artifact BLAKE3 digests, and explicit non-claims.

#### Scenario: Failure bundle is diagnostic only

r[mc_compatibility.failure_evidence_bundles.contract.nonclaim]
- GIVEN a compatibility rail fails or is blocked
- WHEN a failure bundle is written
- THEN the bundle records a failed or blocked outcome
- AND it does not claim scenario success, gameplay parity, full protocol compatibility, public-server safety, production readiness, or semantic equivalence.

### Requirement: Failure-bundle validator

r[mc_compatibility.failure_evidence_bundles.validator] The repository MUST include a pure validator for failure-bundle shape, digest format, artifact path policy, outcome status, and non-claim presence.

#### Scenario: Invalid failure bundle fails closed

r[mc_compatibility.failure_evidence_bundles.validator.negative]
- GIVEN a bundle has missing artifacts, path escapes, malformed BLAKE3 digests, missing nonclaims, or a success-labeled outcome
- WHEN validation runs
- THEN it fails with diagnostics naming each invalid field
- AND the bundle cannot be cited as review evidence.

### Requirement: Runner failure bundle emission

r[mc_compatibility.failure_evidence_bundles.runner] Runner failure paths SHOULD collect available receipt, client log, server log, typed-event log, stderr, and command-summary metadata into a failure bundle while preserving the original failing exit status.

#### Scenario: Original failure remains visible

r[mc_compatibility.failure_evidence_bundles.runner.exit]
- GIVEN a dry-run or live rail fails
- WHEN the runner writes a failure bundle
- THEN the command still exits as failed
- AND the bundle names the first failure without rewriting it as a passing receipt.

### Requirement: Failure-bundle documentation

r[mc_compatibility.failure_evidence_bundles.docs] Documentation MUST explain when failure bundles should be copied into `docs/evidence/`, how their BLAKE3 identities are recorded, and why they remain non-claiming diagnostic artifacts.

#### Scenario: Reviewer can reproduce artifact identity

r[mc_compatibility.failure_evidence_bundles.docs.review]
- GIVEN a failure bundle is cited in Cairn tasks or evidence notes
- WHEN a reviewer inspects the cited files
- THEN the bundle and each critical artifact path resolve under reviewable evidence storage
- AND BLAKE3 digests identify the cited bytes.

### Requirement: Failure-bundle validation evidence

r[mc_compatibility.failure_evidence_bundles.validation] The change MUST record failure-bundle positive and negative fixtures, runner failure-path tests, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves diagnostic bundle safety

r[mc_compatibility.failure_evidence_bundles.validation.log]
- GIVEN failure bundle support is implemented
- WHEN the change is archived
- THEN reviewable logs show validator fixtures, fail-only outcome rejection, path and digest rejection, Cairn proposal/design/tasks gates, and Cairn validation.
