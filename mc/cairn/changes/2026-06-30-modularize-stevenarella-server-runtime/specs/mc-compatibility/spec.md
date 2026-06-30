# mc-compatibility Change Spec: Stevenarella server runtime modules

## Requirements

### Requirement: Stevenarella server modularization inventory

r[mc_compatibility.stevenarella_server_modularization.inventory] Stevenarella server modularization work MUST inventory the current `server/mod.rs` responsibilities, packet handler domains, compat probe state groups, and baseline tests before extraction.

#### Scenario: Server runtime ownership is reviewable

r[mc_compatibility.stevenarella_server_modularization.inventory.reviewable]
- GIVEN Stevenarella server runtime modularization is selected
- WHEN reviewers inspect the inventory
- THEN login/session, world/chunk, entity, inventory/window, block-entity/sign, chat/plugin-message, dispatch, and compat-probe responsibilities are named
- AND baseline validation commands are recorded before core changes.

### Requirement: Stevenarella server module boundaries

r[mc_compatibility.stevenarella_server_modularization.module_boundaries] Stevenarella server packet handling SHOULD expose cohesive modules for login/session, chunks/world, entities, inventory/windows, block entities/signs, chat/plugin messages, and dispatch helpers.

#### Scenario: Packet handler domains have focused owners

r[mc_compatibility.stevenarella_server_modularization.module_boundaries.focused]
- GIVEN a packet handler is reviewed
- WHEN maintainers inspect the server module tree
- THEN the handler belongs to the focused module for its packet/state domain
- AND unrelated packet domains are not reintroduced into one catch-all runtime file.

### Requirement: Stevenarella compat probe state modules

r[mc_compatibility.stevenarella_server_modularization.probe_state] Stevenarella mc-compat probe state SHOULD be grouped into cohesive state modules with pure transition helpers for CTF, inventory, survival, combat/projectile, and sign/dimension behavior.

#### Scenario: Probe decisions are testable without client I/O

r[mc_compatibility.stevenarella_server_modularization.probe_state.testable]
- GIVEN explicit probe input state and an observed packet or tick
- WHEN a probe transition helper evaluates the next action
- THEN tests can verify the decision without network sockets, renderer state, ECS mutation, packet writes, or filesystem access
- AND the `Server` shell remains responsible for side effects.

### Requirement: Stevenarella server parity

r[mc_compatibility.stevenarella_server_modularization.parity] Stevenarella server modularization MUST preserve packet dispatch behavior, compat milestone/event vocabulary, environment variable contracts, receipt non-claims, and default non-instrumented client behavior.

#### Scenario: Existing compatibility rails observe stable output

r[mc_compatibility.stevenarella_server_modularization.parity.stable]
- GIVEN a supported pre-refactor packet, scenario probe, or default client path
- WHEN the modularized server runtime processes the same input
- THEN packet state updates, probe milestones, typed event IDs, and default instrumentation absence remain equivalent
- AND no new gameplay, protocol, or public-server claim is introduced.

### Requirement: Stevenarella server modularization tests

r[mc_compatibility.stevenarella_server_modularization.tests] The change MUST include positive tests for representative extracted handlers/probe transitions and negative tests for malformed packet/probe inputs, invalid state transitions, missing windows/entities, and disabled probes.

#### Scenario: Valid extracted paths pass

r[mc_compatibility.stevenarella_server_modularization.tests.positive]
- GIVEN valid representative server packet or probe inputs
- WHEN extracted modules process them
- THEN tests prove the expected state transition, packet action, or milestone decision is produced.

#### Scenario: Invalid extracted paths fail closed

r[mc_compatibility.stevenarella_server_modularization.tests.negative]
- GIVEN malformed packet data, missing entity/window state, disabled probes, or invalid probe transition inputs
- WHEN extracted modules process them
- THEN tests prove the inputs are ignored, rejected, or diagnosed without panic or stale state promotion.

### Requirement: Stevenarella server modularization validation

r[mc_compatibility.stevenarella_server_modularization.validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Server modularization closeout is reviewable

r[mc_compatibility.stevenarella_server_modularization.validation.logs]
- GIVEN Stevenarella server modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change tests, affected dry-runs, positive and negative regression coverage, Cairn gates, and Cairn validation passing.
