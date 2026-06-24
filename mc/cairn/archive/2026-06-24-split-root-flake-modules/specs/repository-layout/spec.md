# repository-layout Change Spec: Root flake module split

## Requirements

### Requirement: Public flake interface remains stable

r[repository_layout.flake_module_split.public_interface] Splitting the root flake MUST preserve existing public package, app, check, and dev-shell output names unless a separate accepted change explicitly renames them.

#### Scenario: Output inventory matches

r[repository_layout.flake_module_split.public_interface.inventory]
- GIVEN the root flake is split into local modules
- WHEN the public output inventory is compared against the pre-split inventory
- THEN existing package, app, check, and dev-shell names remain present
- AND any intentionally new or removed output is tied to a separate reviewed change.

### Requirement: Package module boundaries

r[repository_layout.flake_module_split.package_modules] Package definitions and shared package helper data SHOULD live in focused local Nix modules with explicit inputs.

#### Scenario: Package module is explicit

r[repository_layout.flake_module_split.package_modules.explicit]
- GIVEN package definitions move out of `flake.nix`
- WHEN reviewers inspect the imported package module
- THEN required inputs such as `pkgs`, `lib`, source paths, tool lists, and shared constants are passed explicitly
- AND package behavior remains equivalent to the root-flake definition it replaced.

### Requirement: App and check module boundaries

r[repository_layout.flake_module_split.app_check_modules] App wrappers and check definitions SHOULD be factored into focused local modules without changing command shapes or evidence semantics.

#### Scenario: Wrapper command shape is stable

r[repository_layout.flake_module_split.app_check_modules.dry_run]
- GIVEN app wrappers or checks move into imported modules
- WHEN selected dry-run app wrappers and focused checks execute
- THEN command names, default arguments, receipt paths, backend defaults, and non-claim text match the pre-split behavior.

### Requirement: Dev shell module boundary

r[repository_layout.flake_module_split.devshell_module] Dev shell definitions MAY move into a focused local module only if documented environment variables, native dependencies, and GUI/client dependencies remain stable.

#### Scenario: Dev shell contract remains documented

r[repository_layout.flake_module_split.devshell_module.contract]
- GIVEN dev shell definitions are imported from a local module
- WHEN developers enter the shell or inspect dry-run documentation
- THEN expected cargo, Nickel, Cairn, Octet, GUI, Xvfb, OpenSSL, and Docker-client support remains available or explicitly documented as unchanged.

### Requirement: Flake split parity checks

r[repository_layout.flake_module_split.parity_checks] The repository MUST include focused checks that prove the flake split preserved output inventory and selected wrapper behavior.

#### Scenario: Missing output fails fast

r[repository_layout.flake_module_split.parity_checks.missing]
- GIVEN a local module accidentally omits a previously available public output
- WHEN the parity check runs
- THEN the missing output name is reported
- AND the check fails before archive.

### Requirement: Flake split validation

r[repository_layout.flake_module_split.validation] The flake split MUST record focused Nix evaluation, selected dry-run/check output, Cairn gates, and Cairn validation before archive.

#### Scenario: Refactor closeout is reviewable

r[repository_layout.flake_module_split.validation.log]
- GIVEN the flake has been split into local modules
- WHEN the change is archived
- THEN reviewable logs show public output inventory parity, selected wrapper dry-runs, selected check builds, Cairn proposal/design/tasks gates, and Cairn validation.
