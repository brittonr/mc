# mc-compatibility Change Spec: Stevenarella server probe split

## Requirements

### Requirement: Stevenarella server responsibility inventory

r[mc_compatibility.stevenarella_server_probe_split.inventory] Stevenarella server probe work MUST inventory server module responsibilities, probe families, side effects, and shared helpers before extraction.

#### Scenario: Probe ownership is classified

r[mc_compatibility.stevenarella_server_probe_split.inventory.classified]
- GIVEN `clients/stevenarella/src/server/mod.rs` is selected for modularization
- WHEN reviewers inspect the inventory
- THEN general protocol handling, connection state, world/entity updates, inventory/window handling, compatibility probe decisions, environment parsing, and milestone logging are classified
- AND probe families that can move independently are named.

### Requirement: Stevenarella server module boundaries

r[mc_compatibility.stevenarella_server_probe_split.boundaries] The Stevenarella server split SHOULD define module boundaries for general server state, probe cores, probe shells, inventory/window helpers, block-entity/sign helpers, and environment/config inputs.

#### Scenario: Probe modules do not own packet IO

r[mc_compatibility.stevenarella_server_probe_split.boundaries.packet_io]
- GIVEN a probe family is extracted
- WHEN reviewers inspect the module boundary
- THEN pure probe logic is separate from packet decoding/encoding and connection mutation
- AND shell handlers remain responsible for interacting with the protocol connection and world state.

### Requirement: Pure Stevenarella probe cores

r[mc_compatibility.stevenarella_server_probe_split.pure_probes] Compatibility probe decisions MUST be pure deterministic state machines over explicit inputs wherever practical.

#### Scenario: Probe core returns actions

r[mc_compatibility.stevenarella_server_probe_split.pure_probes.actions]
- GIVEN a probe core receives explicit tick, session, inventory, world, entity, dimension, or packet-observation inputs
- WHEN it evaluates the next probe step
- THEN it returns an action, expected milestone, state update, or no-op
- AND it does not read environment variables, write logs, mutate global state, write packets, or access renderer/resources.

### Requirement: Stevenarella probe shell wiring

r[mc_compatibility.stevenarella_server_probe_split.shell_wiring] Packet-handler shells MUST call probe cores and translate returned actions into existing packet writes, state mutations, and milestone logs.

#### Scenario: Shell remains thin

r[mc_compatibility.stevenarella_server_probe_split.shell_wiring.thin]
- GIVEN a packet handler drives a compatibility probe
- WHEN probe shell wiring runs
- THEN raw packet/world inputs are converted into core inputs, core outputs are applied, and diagnostics are logged as before
- AND scenario-specific decision tables are not duplicated in the shell.

### Requirement: Stevenarella probe compatibility preservation

r[mc_compatibility.stevenarella_server_probe_split.compatibility] The server probe split MUST preserve env var names, milestone text, fixture constants, packet action order, and evidence non-claim boundaries unless another Cairn changes them.

#### Scenario: Existing rail behavior stays stable

r[mc_compatibility.stevenarella_server_probe_split.compatibility.stable]
- GIVEN an existing mc-compat scenario drives Stevenarella after the split
- WHEN selected dry-run or focused rail checks run
- THEN the same probe env vars are honored, the same client actions are sent, and the same milestone text appears
- AND no full-client, full-survival, full-CTF, or public-server claim is added.

### Requirement: Stevenarella probe tests

r[mc_compatibility.stevenarella_server_probe_split.tests] The probe split MUST include positive action tests and negative fail-closed tests for each migrated probe family.

#### Scenario: Valid probe fixtures pass

r[mc_compatibility.stevenarella_server_probe_split.tests.positive]
- GIVEN valid probe state, inventory/window state, packet observations, session values, and fixture positions
- WHEN migrated probe cores are tested
- THEN the expected actions and milestones match pre-split behavior.

#### Scenario: Invalid probe fixtures fail closed

r[mc_compatibility.stevenarella_server_probe_split.tests.negative]
- GIVEN malformed env/config input, missing fixture state, out-of-order packets, invalid window IDs, stale sign/block-entity data, missing dimension bounds, or server rejection evidence
- WHEN migrated probe cores and shells are tested
- THEN diagnostics or no-op/rejected outcomes are produced deterministically
- AND no false success milestone is emitted.

### Requirement: Stevenarella probe split validation

r[mc_compatibility.stevenarella_server_probe_split.validation] Stevenarella probe split work MUST record focused Stevenarella tests, selected mc-compat checks, Cairn gates, and task-evidence checks before archive.

#### Scenario: Probe split closeout is reviewable

r[mc_compatibility.stevenarella_server_probe_split.validation.log]
- GIVEN the Stevenarella server probe split is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative probe tests, selected component checks through the mc devshell, selected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
