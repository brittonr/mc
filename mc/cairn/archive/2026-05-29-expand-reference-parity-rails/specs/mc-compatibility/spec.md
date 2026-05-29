# Delta: Reference parity expansion

## Requirements

### Requirement: Reference parity policy

r[mc_compatibility.reference_parity.policy] The harness MUST define when a claim requires paired reference-backend evidence instead of Valence-only evidence.

#### Scenario: Claim class is explicit

r[mc_compatibility.reference_parity.policy.classified]
- GIVEN a compatibility row is added or promoted
- WHEN the evidence standard is reviewed
- THEN the row is classified as reference-parity-required, Valence-only containment, or explicit non-claim
- AND the classification is reflected in the matrix and current bundle.

### Requirement: Survival parity inventory

r[mc_compatibility.reference_parity.survival_inventory] Remaining survival coverage rows MUST be inventoried with owner change, evidence standard, and next action.

#### Scenario: Existing chest work is not duplicated

r[mc_compatibility.reference_parity.survival_inventory.chest]
- GIVEN `prove-survival-chest-persistence` is active
- WHEN the reference parity inventory is written
- THEN chest persistence is marked as owned by that change
- AND this change does not promote a competing chest row.

### Requirement: Survival parity comparator

r[mc_compatibility.reference_parity.survival_comparator] Survival reference parity rows MUST compare normalized metrics from paired Paper and Valence artifacts.

#### Scenario: Valence-only survival parity fails

r[mc_compatibility.reference_parity.survival_comparator.valence_only]
- GIVEN a survival row has only Valence evidence
- WHEN the comparator runs for a reference-parity-required claim
- THEN promotion fails and names the missing reference artifact.

### Requirement: Next survival parity row

r[mc_compatibility.reference_parity.next_survival_row] The harness SHOULD add the next highest-ROI survival parity row after chest with deterministic paired evidence.

#### Scenario: Row stays narrow

r[mc_compatibility.reference_parity.next_survival_row.narrow]
- GIVEN the next survival row is implemented
- WHEN evidence is promoted
- THEN only the configured item/block/action metrics for that row are claimed
- AND full survival compatibility remains a non-claim.

### Requirement: Combat parity comparator

r[mc_compatibility.reference_parity.combat_comparator] Vanilla combat parity checks MUST compare bounded normalized metrics with reference-version and tolerance fields.

#### Scenario: Missing tolerance or reference fails

r[mc_compatibility.reference_parity.combat_comparator.rejects]
- GIVEN a combat parity row lacks a Paper/reference receipt, reference version, tolerance, Valence metric, or reference metric
- WHEN the comparator runs
- THEN promotion fails before any vanilla parity claim is recorded.

### Requirement: Combat parity evidence

r[mc_compatibility.reference_parity.combat_evidence] Combat parity work MUST either produce paired reference evidence for a bounded row or keep parity blocked with an oracle checkpoint.

#### Scenario: No evidence keeps non-claim

r[mc_compatibility.reference_parity.combat_evidence.blocked]
- GIVEN paired combat reference evidence is not available
- WHEN the change is reviewed
- THEN exact vanilla combat parity remains a non-claim and the evidence doc names the missing artifacts.

### Requirement: Reference parity matrix labels

r[mc_compatibility.reference_parity.matrix] Acceptance and current-bundle matrices MUST distinguish reference-parity-covered rows from Valence-only rows and non-claims.

#### Scenario: Matrix label prevents overclaim

r[mc_compatibility.reference_parity.matrix.labels]
- GIVEN a row has only Valence evidence
- WHEN the matrix is rendered
- THEN it cannot be labeled as reference parity or vanilla parity.

### Requirement: Reference parity validation

r[mc_compatibility.reference_parity.validation] Reference parity changes MUST record comparator, receipt, manifest, dry-run, and Cairn validation output before archive.

#### Scenario: Validation is local

r[mc_compatibility.reference_parity.validation.local]
- GIVEN a parity row is promoted or explicitly blocked
- WHEN the change is completed
- THEN review-critical artifacts and validation logs are copied under `docs/evidence/`.
