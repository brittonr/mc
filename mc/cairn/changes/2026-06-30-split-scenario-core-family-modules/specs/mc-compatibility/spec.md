# mc-compatibility Change Spec: Scenario family modules

## Requirements

### Requirement: Scenario core family inventory

r[mc_compatibility.scenario_core_family_modules.inventory] Scenario core modularization work MUST inventory scenario families, manifest/generated ownership, live capability contracts, duplicated surfaces, and baseline validation before extraction.

#### Scenario: Scenario family ownership is reviewable

r[mc_compatibility.scenario_core_family_modules.inventory.reviewable]
- GIVEN scenario core modularization is selected
- WHEN reviewers inspect the inventory
- THEN CTF, inventory, survival, combat/projectile/equipment, negative, MCP, and targeted-packet live capability responsibilities are named
- AND generated and hand-authored scenario surfaces are identified with baseline checks.

### Requirement: Scenario family module boundaries

r[mc_compatibility.scenario_core_family_modules.family_boundaries] Scenario behavior SHOULD be split into focused family modules for CTF, inventory, survival, combat/projectile/equipment, negative rails, MCP, and targeted-packet live capability contracts behind a stable central façade.

#### Scenario: Scenario behavior has a focused owner

r[mc_compatibility.scenario_core_family_modules.family_boundaries.focused]
- GIVEN a scenario behavior or contract is reviewed
- WHEN maintainers inspect the scenario module tree
- THEN the behavior belongs to its focused family module
- AND unrelated scenario families are not reintroduced into one catch-all scenario core file.

### Requirement: Scenario manifest parity

r[mc_compatibility.scenario_core_family_modules.manifest_parity] Scenario family extraction MUST preserve parity with `compat/config/scenario-manifest.ncl` and generated surfaces for scenario names, aliases, milestones, forbidden patterns, run strategies, receipt expectations, wrapper metadata, and live capability rows.

#### Scenario: Generated and hand-authored scenario data agree

r[mc_compatibility.scenario_core_family_modules.manifest_parity.fresh]
- GIVEN scenario metadata is split across family modules and generated surfaces
- WHEN generated-surface and scenario validation checks run
- THEN every maintained scenario has matching names, aliases, milestones, forbidden patterns, receipt expectations, and wrapper metadata
- AND stale or missing generated rows fail clearly.

### Requirement: Scenario live capability contracts

r[mc_compatibility.scenario_core_family_modules.live_capabilities] Targeted packet and live capability contracts MUST remain fail-closed with explicit required signals, required non-claims, blocker reasons when blocked, backend/client paths, and validation helpers.

#### Scenario: Live capability rows do not overclaim

r[mc_compatibility.scenario_core_family_modules.live_capabilities.non_overclaiming]
- GIVEN a targeted packet or live capability row is reviewed
- WHEN static validation evaluates the row
- THEN required signals, non-claims, backend/client paths, and blocker reasons are present where required
- AND broad compatibility, semantic equivalence, public-server safety, and production-readiness claims remain absent.

### Requirement: Scenario family module tests

r[mc_compatibility.scenario_core_family_modules.tests] The change MUST include positive tests for representative family lookup and behavior plus negative tests for duplicate aliases, missing manifest rows, invalid live capability rows, unsupported env intents, and stale generated surfaces.

#### Scenario: Valid family metadata passes

r[mc_compatibility.scenario_core_family_modules.tests.positive]
- GIVEN valid representative scenario family metadata
- WHEN lookup, behavior, manifest parity, and live capability checks run
- THEN tests prove the expected scenario API and validation output are produced.

#### Scenario: Invalid family metadata fails clearly

r[mc_compatibility.scenario_core_family_modules.tests.negative]
- GIVEN duplicate aliases, missing rows, unsupported env intents, invalid live capability contracts, or stale generated surfaces
- WHEN scenario validation runs
- THEN tests prove diagnostics name the invalid scenario surface and prevent stale metadata promotion.

### Requirement: Scenario family module validation

r[mc_compatibility.scenario_core_family_modules.validation] The change MUST record scenario tests, generated-surface checks, representative maintained dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Scenario family closeout is reviewable

r[mc_compatibility.scenario_core_family_modules.validation.logs]
- GIVEN scenario family modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change scenario tests, generated-surface checks, representative dry-runs, positive and negative regression coverage, Cairn gates, and Cairn validation passing.
