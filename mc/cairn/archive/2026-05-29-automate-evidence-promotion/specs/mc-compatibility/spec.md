# Delta: Evidence promotion automation

## Requirements

### Requirement: Promotion plan

r[mc_compatibility.evidence_promotion.plan] Evidence promotion MUST be represented as a deterministic plan before filesystem mutation.

#### Scenario: Plan names all artifacts

r[mc_compatibility.evidence_promotion.plan.artifacts]
- GIVEN a rail run is selected for promotion
- WHEN the promotion plan is computed
- THEN the plan names source receipt, run log, client logs, server logs, destination paths, BLAKE3 hashes, child-revision fields, matrix changes, current-bundle changes, validation commands, and explicit non-claims.

### Requirement: Promotion core

r[mc_compatibility.evidence_promotion.core] Promotion planning MUST be implemented by a pure core over in-memory inputs.

#### Scenario: Core has no side effects

r[mc_compatibility.evidence_promotion.core.pure]
- GIVEN receipt summaries, artifact inventories, matrix text, and bundle text
- WHEN the core computes a promotion plan
- THEN it returns the plan or diagnostics without reading files, writing files, executing processes, or inspecting environment variables.

### Requirement: Promotion fixtures

r[mc_compatibility.evidence_promotion.fixtures] Promotion planning MUST include positive and negative fixtures for evidence hygiene failure modes.

#### Scenario: Incomplete promotion fails

r[mc_compatibility.evidence_promotion.fixtures.incomplete]
- GIVEN a promotion input is missing receipt, run log, required client/server log, BLAKE3 hash, child revision evidence, or required non-claim text
- WHEN fixture tests run
- THEN planning fails with a diagnostic naming the missing or unsafe field.

### Requirement: Promotion CLI

r[mc_compatibility.evidence_promotion.cli] The promotion shell MUST default to dry-run and require explicit apply mode for mutation.

#### Scenario: Dry-run does not mutate

r[mc_compatibility.evidence_promotion.cli.dry_run]
- GIVEN promotion planning is invoked without apply mode
- WHEN the command exits successfully
- THEN no files are written and the output lists the exact planned writes.

#### Scenario: Apply writes exact destinations

r[mc_compatibility.evidence_promotion.cli.apply]
- GIVEN a valid promotion plan is approved with explicit apply mode
- WHEN the command mutates files
- THEN it writes only the planned destination files and never force-adds broad directories.

### Requirement: Promotion workflow

r[mc_compatibility.evidence_promotion.workflow] README or evidence guidance MUST document the promotion workflow and required validation gates.

#### Scenario: Operator runs safe path

r[mc_compatibility.evidence_promotion.workflow.docs]
- GIVEN an operator has a new live receipt
- WHEN they follow documented promotion steps
- THEN they run promotion dry-run, apply exact artifacts, and run acceptance, bundle, manifest, and Cairn validation gates before claiming the row.

### Requirement: Matrix and bundle update planning

r[mc_compatibility.evidence_promotion.matrix_bundle] The promotion tool SHOULD plan acceptance matrix and current-bundle row updates for maintained rails.

#### Scenario: Non-claims are preserved

r[mc_compatibility.evidence_promotion.matrix_bundle.nonclaims]
- GIVEN a planned matrix or bundle edit would remove or weaken a required non-claim
- WHEN promotion planning runs
- THEN the plan is rejected unless matching promoted evidence and checker coverage are present.

### Requirement: Promotion validation evidence

r[mc_compatibility.evidence_promotion.validation] Promotion automation MUST record dry-run/apply fixture output and existing evidence gate output before archive.

#### Scenario: Validation is reviewable

r[mc_compatibility.evidence_promotion.validation.local]
- GIVEN promotion automation is completed
- WHEN the change is archived
- THEN test output, checker output, and Cairn validation output are copied under `docs/evidence/`.
