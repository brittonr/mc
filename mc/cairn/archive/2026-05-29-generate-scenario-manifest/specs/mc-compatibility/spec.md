# Delta: Generated scenario manifest

## Requirements

### Requirement: Scenario manifest contract

r[mc_compatibility.scenario_manifest.contract] Scenario metadata MUST have a typed Nickel source of truth before generation or drift checks depend on it.

#### Scenario: Manifest row is complete

r[mc_compatibility.scenario_manifest.contract.row]
- GIVEN a scenario is maintained by the compatibility harness
- WHEN its manifest row is evaluated
- THEN the row declares canonical name, aliases, client milestone requirements, server milestone requirements, forbidden patterns, client count, reconnect/session count, receipt expectations, dry-run wrapper metadata, and migration state.

### Requirement: Manifest validation

r[mc_compatibility.scenario_manifest.validation] The scenario manifest MUST fail closed for malformed, duplicate, or incomplete scenario rows.

#### Scenario: Invalid manifest fails

r[mc_compatibility.scenario_manifest.validation.invalid]
- GIVEN a manifest row has a duplicate name, missing required milestone list, invalid alias, unsupported migration state, or incomplete dry-run metadata
- WHEN manifest validation runs
- THEN validation fails with a diagnostic naming the bad row and field.

### Requirement: Scenario drift checker

r[mc_compatibility.scenario_manifest.drift_checker] A deterministic checker MUST detect drift between the manifest and scenario surfaces in code, Nix checks, and docs.

#### Scenario: Missing surface fails

r[mc_compatibility.scenario_manifest.drift_checker.missing]
- GIVEN a scenario exists in the manifest
- WHEN the checker inspects runner parsing, help text, milestone tables, maintained dry-run checks, README examples, and current bundle rows
- THEN any missing or mismatched surface fails the check unless the manifest row records an explicit exclusion reason.

### Requirement: Generated scenario tables

r[mc_compatibility.scenario_manifest.generated_tables] Generated or validated scenario tables MUST be checked in and consumed by Rust without runtime Nickel evaluation.

#### Scenario: Runtime remains Rust-owned

r[mc_compatibility.scenario_manifest.generated_tables.runtime]
- GIVEN the runner starts
- WHEN it parses scenario metadata
- THEN it uses checked-in Rust or JSON artifacts generated from the manifest
- AND it does not evaluate Nickel at runtime.

### Requirement: Manifest-driven dry-run coverage

r[mc_compatibility.scenario_manifest.dry_run_coverage] Maintained dry-run receipt checks MUST be derived from or checked against the scenario manifest.

#### Scenario: Maintained scenario has receipt check

r[mc_compatibility.scenario_manifest.dry_run_coverage.covered]
- GIVEN a manifest row is marked maintained
- WHEN flake checks run
- THEN the scenario has a dry-run receipt shape check or the row records why dry-run coverage is intentionally absent.

### Requirement: Manifest documentation

r[mc_compatibility.scenario_manifest.docs] README and evidence docs MUST identify the manifest as the source of truth after migration.

#### Scenario: Operator workflow is documented

r[mc_compatibility.scenario_manifest.docs.workflow]
- GIVEN an operator adds or changes a scenario
- WHEN they follow README guidance
- THEN they update the manifest first, regenerate or run drift checks, then update evidence docs only after checks pass.

### Requirement: Manifest validation evidence

r[mc_compatibility.scenario_manifest.validation_evidence] Manifest work MUST record validation output before archive.

#### Scenario: Evidence is local

r[mc_compatibility.scenario_manifest.validation_evidence.local]
- GIVEN the manifest migration is completed
- WHEN the change is archived
- THEN validation fixtures, drift checker output, maintained dry-run output, and Cairn validation logs are copied under `docs/evidence/`.
