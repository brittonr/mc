# mc-compatibility Change Spec: Paired-reference dry-run shapes

## Requirements

### Requirement: Paired-reference dry-run inventory

r[mc_compatibility.paired_reference_dry_run_shapes.inventory] The change MUST inventory the current paired-reference scenario manifest entries, live comparator receipt fields, and existing dry-run exclusion rationale before adding shape coverage.

#### Scenario: Dry-run gap is reviewable

r[mc_compatibility.paired_reference_dry_run_shapes.inventory.reviewable]
- GIVEN paired-reference combat scenarios lack deterministic dry-run shape checks
- WHEN reviewers inspect the inventory
- THEN it names the affected scenarios, required live comparator fields, current exclusion rationale, and non-claim boundary.

### Requirement: Deterministic dry-run shape contract

r[mc_compatibility.paired_reference_dry_run_shapes.contract] The selected paired-reference scenarios MUST have deterministic dry-run receipt shapes that record scenario identity, reference backend label, Valence backend label, metric names, tolerance fields, comparison-status placeholder, and dry-run non-claims.

#### Scenario: Shape uses placeholders instead of live evidence

r[mc_compatibility.paired_reference_dry_run_shapes.contract.placeholders]
- GIVEN a selected paired-reference scenario runs in dry-run mode
- WHEN the receipt shape is written
- THEN it uses deterministic placeholder values for backend evidence and source revisions
- AND it does not assert live metric equality or exact vanilla parity.

### Requirement: Pure shape validation

r[mc_compatibility.paired_reference_dry_run_shapes.validation_core] Dry-run shape validation MUST be a pure deterministic core over normalized receipt inputs and MUST include positive and negative fixtures for the paired-reference scenarios.

#### Scenario: Valid paired-reference shapes pass

r[mc_compatibility.paired_reference_dry_run_shapes.validation_core.positive]
- GIVEN a dry-run receipt contains the required reference, Valence, metric, tolerance, scenario, and non-claim fields
- WHEN the shape validator evaluates it
- THEN validation passes with stable diagnostics.

#### Scenario: Weak paired-reference shape fails

r[mc_compatibility.paired_reference_dry_run_shapes.validation_core.negative]
- GIVEN a dry-run receipt lacks reference fields, Valence fields, tolerance fields, allowed backend labels, or dry-run non-claim text
- WHEN the shape validator evaluates it
- THEN validation fails and names the missing or invalid field.

### Requirement: Scenario manifest integration

r[mc_compatibility.paired_reference_dry_run_shapes.integration] Scenario manifest dry-run metadata and generated surfaces MUST expose the new dry-run checks without changing live comparator promotion rules.

#### Scenario: Generated index distinguishes shape from parity

r[mc_compatibility.paired_reference_dry_run_shapes.integration.generated]
- GIVEN generated scenario surfaces are refreshed
- WHEN reviewers inspect the paired-reference rows
- THEN the rows name the deterministic dry-run shape checks
- AND they continue to state that live paired comparator evidence is required for parity promotion.

### Requirement: Dry-run documentation and non-claims

r[mc_compatibility.paired_reference_dry_run_shapes.docs] Evidence docs MUST state that paired-reference dry-run shape coverage is not live Paper/Valence evidence and does not promote vanilla parity.

#### Scenario: Dry-run non-claims are explicit

r[mc_compatibility.paired_reference_dry_run_shapes.docs.nonclaims]
- GIVEN dry-run shape evidence is cited
- WHEN reviewers read the evidence docs
- THEN exact vanilla parity, full combat parity, public-server safety, production readiness, and live comparator success remain explicit non-claims.

### Requirement: Paired-reference dry-run closeout

r[mc_compatibility.paired_reference_dry_run_shapes.closeout] The change MUST record reviewable logs for dry-run shape fixtures, scenario manifest checks, generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.paired_reference_dry_run_shapes.closeout.log]
- GIVEN paired-reference dry-run shapes are implemented
- WHEN reviewers inspect task evidence
- THEN logs show positive and negative shape fixtures, scenario manifest checks, generated-surface freshness, evidence manifest validation, task-evidence validation, Cairn gates, and Cairn validation.
