# Delta: Maintained dry-run aggregate catalog

## Requirements

### Requirement: Dry Run Catalog

r[mc_compatibility.roi_05_dry_run_aggregate.dry_run_catalog] The repo MUST catalog all maintained protocol-763 dry-run checks exposed by the flake.

#### Scenario: Dry Run Catalog evidence is required

r[mc_compatibility.roi_05_dry_run_aggregate.dry_run_catalog.scenario]
- GIVEN `Maintained dry-run aggregate catalog` is drained
- WHEN the evidence and checks are reviewed
- THEN `dry_run_catalog` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Dry Run Check

r[mc_compatibility.roi_05_dry_run_aggregate.dry_run_check] A deterministic check MUST validate the dry-run catalog stays aligned with flake and README surfaces.

#### Scenario: Dry Run Check evidence is required

r[mc_compatibility.roi_05_dry_run_aggregate.dry_run_check.scenario]
- GIVEN `Maintained dry-run aggregate catalog` is drained
- WHEN the evidence and checks are reviewed
- THEN `dry_run_check` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant
