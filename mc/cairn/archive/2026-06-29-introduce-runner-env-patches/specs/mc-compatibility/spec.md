# mc-compatibility Change Spec: Composable runner environment patches

## Requirements

### Requirement: Env patch core

r[mc_compatibility.runner_modularity.env_patch_core] Runner environment derivation SHOULD return deterministic `EnvPatch` data before any process `Command` is mutated.

#### Scenario: Env derivation is pure

r[mc_compatibility.runner_modularity.env_patch_core.pure]
- GIVEN scenario, backend, client index, session, and runtime config inputs
- WHEN environment derivation runs
- THEN it returns env patch data without mutating a process command
- AND the patch records enough source context for diagnostics.

### Requirement: Env patch shell application

r[mc_compatibility.runner_modularity.env_patch_shell] The runner shell MUST apply validated env patches to `Command` instances without duplicating scenario env policy.

#### Scenario: Shell applies computed patch

r[mc_compatibility.runner_modularity.env_patch_shell.apply]
- GIVEN an env patch has been computed and validated
- WHEN the client or backend shell prepares a process command
- THEN it applies the patch entries to the command
- AND it does not recompute scenario env policy in the shell.

### Requirement: Env patch positive tests

r[mc_compatibility.runner_modularity.env_patch_positive_tests] The change MUST include positive tests for env patch composition and representative env output across inventory, survival, combat, projectile, CTF, reconnect, and MCP scenarios.

#### Scenario: Supported env patches compose

r[mc_compatibility.runner_modularity.env_patch_positive_tests.coverage]
- GIVEN supported scenario env fragments
- WHEN patches are composed for representative clients and backends
- THEN the resulting env map contains the expected keys and values.

### Requirement: Env patch negative tests

r[mc_compatibility.runner_modularity.env_patch_negative_tests] The change MUST include negative tests for conflicting keys, malformed keys, missing required session values, and backend-incompatible env fragments.

#### Scenario: Invalid env composition fails closed

r[mc_compatibility.runner_modularity.env_patch_negative_tests.fail_closed]
- GIVEN env fragments conflict or omit required inputs
- WHEN patch composition runs
- THEN it returns an actionable diagnostic before process launch.

### Requirement: Env patch validation

r[mc_compatibility.runner_modularity.env_patch_validation] The change MUST record runner tests, env-patch checks, dry-run smoke checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.env_patch_validation.logs]
- GIVEN env patch extraction is complete
- WHEN the change is closed
- THEN reviewable logs show env parity, positive and negative fixtures, dry-run smoke checks, and Cairn validation passing.
