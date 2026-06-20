# Delta: Survival aggregate parity claim boundary

## Requirements

### Requirement: Aggregate survival claim boundary contract

r[mc_compatibility.survival_aggregate_parity_claim_boundary.contract] The repo MUST define an aggregate survival claim boundary before any documentation can claim full survival compatibility or broad vanilla survival parity.

#### Scenario: Boundary names prerequisites

r[mc_compatibility.survival_aggregate_parity_claim_boundary.contract.scope]
- GIVEN row-scoped survival evidence exists
- WHEN the aggregate claim boundary is reviewed
- THEN it names required survival row families, required paired evidence artifacts, manifest freshness requirements, and allowed claim vocabulary
- AND it states that row-scoped coverage alone is not full survival compatibility or broad vanilla parity.

### Requirement: Aggregate survival boundary checker

r[mc_compatibility.survival_aggregate_parity_claim_boundary.checker] A deterministic checker MUST reject aggregate survival claims unless every prerequisite row and evidence artifact passes.

#### Scenario: Broad overclaim fails closed

r[mc_compatibility.survival_aggregate_parity_claim_boundary.checker.rejects]
- GIVEN docs claim full survival compatibility or broad vanilla parity while any prerequisite row, comparator output, evidence manifest, child revision, or aggregate evidence bundle is missing or stale
- WHEN the checker evaluates the docs
- THEN it fails and names the missing prerequisite or overclaiming text.

### Requirement: Aggregate gate wiring

r[mc_compatibility.survival_aggregate_parity_claim_boundary.gate] The aggregate boundary checker MUST be available as a focused verification gate without changing row-scoped survival evidence semantics.

#### Scenario: Existing row gates remain unchanged

r[mc_compatibility.survival_aggregate_parity_claim_boundary.gate.isolated]
- GIVEN existing survival row gates pass
- WHEN the aggregate gate is added
- THEN row-scoped reference-parity labels remain unchanged
- AND aggregate full-survival wording is checked by the new boundary gate.

### Requirement: Aggregate boundary docs

r[mc_compatibility.survival_aggregate_parity_claim_boundary.docs] Survival matrix, acceptance matrix, and current bundle docs MUST point broad survival claims at the aggregate boundary gate.

#### Scenario: Current non-claims remain explicit

r[mc_compatibility.survival_aggregate_parity_claim_boundary.docs.nonclaims]
- GIVEN the aggregate boundary docs are updated
- WHEN reviewers inspect current evidence
- THEN full survival compatibility and broad vanilla parity remain explicit non-claims until the aggregate gate passes with required evidence.

### Requirement: Aggregate boundary evidence

r[mc_compatibility.survival_aggregate_parity_claim_boundary.evidence] Gate output and manifest evidence for the aggregate boundary MUST be copied under `docs/evidence/` before archive.

#### Scenario: Evidence is reviewable

r[mc_compatibility.survival_aggregate_parity_claim_boundary.evidence.reviewable]
- GIVEN the aggregate boundary checker is added
- WHEN the change is ready for review
- THEN checker output, manifest evidence, and task evidence logs are present under `docs/evidence/`.

### Requirement: Aggregate boundary validation

r[mc_compatibility.survival_aggregate_parity_claim_boundary.validation] The change MUST record checker, focused flake check, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_aggregate_parity_claim_boundary.validation.log]
- GIVEN the change is completed
- WHEN it is archived
- THEN repo-local evidence logs record checker self-tests, focused flake checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
