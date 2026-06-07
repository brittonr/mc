# mc-compatibility Change Spec: Scenario live probe capability registry

## Requirements

### Requirement: Scenario live capability contract

r[mc_compatibility.scenario_live_probe_capabilities.contract] The runner SHOULD define an explicit scenario live-probe capability registry before future targeted packet live promotions rely on scenario selection.

#### Scenario: Capability entries name scope and non-claims

r[mc_compatibility.scenario_live_probe_capabilities.contract.scope]
- GIVEN a scenario can produce or cannot produce a targeted packet live signal
- WHEN maintainers inspect the capability registry
- THEN each entry names scenario id, packet row ids, capability kind, backend/client path, evidence mode, required signals, required non-claims, and optional blocker reason
- AND registry entries do not claim live promotion without separate receipt and checker evidence.

### Requirement: Pure capability registry core

r[mc_compatibility.scenario_live_probe_capabilities.core] Capability lookup and validation MUST be pure deterministic logic over in-memory registry data.

#### Scenario: Registry validation has no side effects

r[mc_compatibility.scenario_live_probe_capabilities.core.pure]
- GIVEN static capability definitions and known scenario/packet row inputs
- WHEN registry validation runs
- THEN it returns success or diagnostics without reading files, writing files, spawning commands, inspecting environment, using clocks, or performing network access.

### Requirement: Seeded capability entries

r[mc_compatibility.scenario_live_probe_capabilities.seed] The registry SHOULD seed entries for currently known targeted packet live candidates and explicit blockers.

#### Scenario: Blocked live paths are visible

r[mc_compatibility.scenario_live_probe_capabilities.seed.blockers]
- GIVEN a targeted packet row lacks a deterministic owned-local live path
- WHEN the registry is queried
- THEN it can report an explicit blocker entry instead of implying that fixture evidence is live promotion evidence.

### Requirement: Capability checker integration

r[mc_compatibility.scenario_live_probe_capabilities.checker] Scenario manifest or focused checker coverage MUST fail closed when capability registry entries drift from known scenarios, packet rows, evidence modes, or non-claim requirements.

#### Scenario: Invalid capability rows fail closed

r[mc_compatibility.scenario_live_probe_capabilities.checker.rejects]
- GIVEN a capability entry names an unknown scenario, unknown packet row, unsupported evidence mode, empty required signals, duplicate scenario/row pair, or missing non-claims
- WHEN registry validation runs through tests or checker coverage
- THEN it fails with explicit diagnostics.

### Requirement: Capability registry tests

r[mc_compatibility.scenario_live_probe_capabilities.tests] The change MUST include positive and negative tests for capability registry lookup and validation.

#### Scenario: Valid and invalid capability fixtures are covered

r[mc_compatibility.scenario_live_probe_capabilities.tests.coverage]
- GIVEN valid capability entries and invalid fixtures for duplicates, unknown rows, unknown scenarios, unsupported modes, empty signals, and missing non-claims
- WHEN registry tests run
- THEN valid entries pass and invalid entries fail with useful diagnostics.

### Requirement: Capability registry documentation

r[mc_compatibility.scenario_live_probe_capabilities.docs] The repository SHOULD document how future live packet rails use the registry for selection and blocker reporting.

#### Scenario: Future live rails have a selection workflow

r[mc_compatibility.scenario_live_probe_capabilities.docs.workflow]
- GIVEN a future targeted packet live rail is proposed
- WHEN maintainers inspect the workflow docs
- THEN they can identify how to add or query capability entries, record blockers, and avoid live-promotion overclaims.

### Requirement: Capability registry validation

r[mc_compatibility.scenario_live_probe_capabilities.validation] The change MUST record scenario tests/manifest checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.scenario_live_probe_capabilities.validation.logs]
- GIVEN capability registry work is complete
- WHEN the change is archived
- THEN reviewable logs show registry tests, scenario manifest or focused checker coverage, relevant runner dry-runs, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.
