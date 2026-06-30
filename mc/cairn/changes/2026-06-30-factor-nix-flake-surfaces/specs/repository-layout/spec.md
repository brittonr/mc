# repository-layout Change Spec: Root Nix flake surface modules

## Requirements

### Requirement: Nix surface modularization inventory

r[repository_layout.nix_surface_modularization.inventory] Root Nix surface modularization work MUST inventory current package, app, check, devshell, generated wrapper metadata, repeated helper patterns, and flake output inventory before splitting files.

#### Scenario: Nix output ownership is reviewable

r[repository_layout.nix_surface_modularization.inventory.reviewable]
- GIVEN root Nix surface modularization is selected
- WHEN reviewers inspect the inventory
- THEN package, app, check, devshell, generated metadata, baseline inventory, and repeated wrapper/check helper responsibilities are named
- AND baseline flake output inventory validation is recorded before core changes.

### Requirement: Nix surface module boundaries

r[repository_layout.nix_surface_modularization.module_boundaries] Root Nix surfaces SHOULD be split into focused modules or helper functions for layout/docs, generated surfaces, evidence, runner/scenario, component behavior, Octet, checker framework, packages, and apps while preserving aggregator entrypoints.

#### Scenario: Nix checks have focused owners

r[repository_layout.nix_surface_modularization.module_boundaries.focused]
- GIVEN a Nix check, package, app, or helper is reviewed
- WHEN maintainers inspect the Nix module tree
- THEN the definition belongs to the focused module for its validation or output family
- AND unrelated flake surfaces are not reintroduced into one long catch-all file.

### Requirement: Nix output parity

r[repository_layout.nix_surface_modularization.output_parity] Nix surface modularization MUST preserve flake output names, app and package main programs, dry-run behavior, baseline output inventory compatibility, and generated scenario wrapper metadata freshness.

#### Scenario: Flake output inventory remains stable

r[repository_layout.nix_surface_modularization.output_parity.stable]
- GIVEN existing package, app, check, and devshell outputs
- WHEN the modularized Nix surfaces evaluate
- THEN the output inventory matches the accepted baseline except for explicitly allowed additions
- AND existing app/package programs and dry-run wrapper behavior remain compatible.

### Requirement: Nix check reviewability

r[repository_layout.nix_surface_modularization.reviewability] Nix helper abstractions SHOULD keep each check's command intent, required inputs, failure diagnostics, and copied `$out` evidence artifacts reviewable.

#### Scenario: Helper output remains inspectable

r[repository_layout.nix_surface_modularization.reviewability.evidence]
- GIVEN a flake check uses a shared helper
- WHEN reviewers inspect the check definition or output
- THEN the command intent, input paths, copied logs, and diagnostics remain visible
- AND helper abstraction does not hide evidence artifacts required by docs or Cairn tasks.

### Requirement: Nix surface modularization tests

r[repository_layout.nix_surface_modularization.tests] The change MUST include positive checks for valid split outputs and negative checks for missing outputs, unexpected outputs, stale generated metadata, missing helper evidence copies, and broken dry-run wrappers where feasible.

#### Scenario: Valid Nix surfaces pass

r[repository_layout.nix_surface_modularization.tests.positive]
- GIVEN split Nix modules with expected outputs
- WHEN focused Nix and inventory checks run
- THEN tests prove packages, apps, checks, devshells, generated metadata, and helper-copied artifacts are available as expected.

#### Scenario: Invalid Nix surfaces fail clearly

r[repository_layout.nix_surface_modularization.tests.negative]
- GIVEN a missing output, unexpected output, stale generated metadata, missing copied check artifact, or broken dry-run wrapper
- WHEN focused Nix and inventory checks run
- THEN tests prove the diagnostic names the stale or missing surface and blocks silent drift.

### Requirement: Nix surface modularization validation

r[repository_layout.nix_surface_modularization.validation] The change MUST record focused Nix eval/build checks, flake output inventory, generated-surface checks when touched, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Nix surface closeout is reviewable

r[repository_layout.nix_surface_modularization.validation.logs]
- GIVEN root Nix surface modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change output inventory, focused Nix checks, generated-surface checks when applicable, positive and negative regression coverage, Cairn gates, and Cairn validation passing.
