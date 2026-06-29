# mc-compatibility Change Spec: Stevenarella server modules

## Requirements

### Requirement: Stevenarella server module boundaries

r[mc_compatibility.stevenarella_modularity.server_boundaries] Stevenarella server session code SHOULD be split into cohesive modules for session lifecycle, packet dispatch, world and dimension state, chunks, entities, inventory/window behavior, plugin messages, and compat-probe shells.

#### Scenario: Packet-family ownership is explicit

r[mc_compatibility.stevenarella_modularity.server_boundaries.ownership]
- GIVEN a server packet or session responsibility is reviewed
- WHEN maintainers inspect the Stevenarella server module tree
- THEN the responsibility is owned by a focused module rather than by unrelated code in the root server module
- AND the root server module exposes only the shell API needed to coordinate those modules.

### Requirement: Stevenarella handler functional cores

r[mc_compatibility.stevenarella_modularity.handler_functional_core] Non-trivial Stevenarella packet-handler decisions SHOULD live in pure deterministic cores that take explicit inputs and return decisions or state updates for the shell to apply.

#### Scenario: Handler decision is testable without a live session

r[mc_compatibility.stevenarella_modularity.handler_functional_core.testable]
- GIVEN handler logic decides how to update world, dimension, entity, inventory, plugin-message, or compat-probe state
- WHEN that logic is extracted
- THEN the decision core can be tested with in-memory inputs
- AND connection I/O, packet writes, ECS/world mutation, and logging remain in the imperative shell.

### Requirement: Stevenarella server modularization parity

r[mc_compatibility.stevenarella_modularity.server_parity] Stevenarella server modularization MUST preserve the public `Server` API, protocol behavior, compat milestone vocabulary, typed-event hooks, MCP/control boundaries, and evidence non-claims.

#### Scenario: Existing client behavior remains stable

r[mc_compatibility.stevenarella_modularity.server_parity.stable]
- GIVEN a supported pre-refactor Stevenarella server session or mc-compat probe input
- WHEN the modularized server session processes the same input
- THEN packet handling, milestone output, typed-event hooks, and non-claim boundaries remain equivalent
- AND the refactor does not promote new compatibility evidence.

### Requirement: Stevenarella server positive tests

r[mc_compatibility.stevenarella_modularity.server_positive_tests] The change MUST include positive tests for representative packet-family decisions, dispatch routing, dimension/world updates, inventory/window behavior, plugin-message routing, and compat-probe scheduling.

#### Scenario: Supported handler paths pass

r[mc_compatibility.stevenarella_modularity.server_positive_tests.coverage]
- GIVEN representative supported packet and probe inputs
- WHEN extracted server cores or routing helpers process them
- THEN tests prove they produce the expected decisions, updates, or shell actions.

### Requirement: Stevenarella server negative tests

r[mc_compatibility.stevenarella_modularity.server_negative_tests] The change MUST include negative tests for unsupported packets, malformed state summaries, invalid inventory/window actions, unknown plugin channels, missing dimension data, and disabled compat probes.

#### Scenario: Invalid handler paths fail closed

r[mc_compatibility.stevenarella_modularity.server_negative_tests.fail_closed]
- GIVEN invalid packet, state, inventory, plugin-message, dimension, or probe inputs
- WHEN extracted server cores or routing helpers process them
- THEN tests prove the inputs are rejected, ignored, or contained according to the existing behavior without panicking or corrupting state.

### Requirement: Stevenarella server modularization validation

r[mc_compatibility.stevenarella_modularity.server_validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_modularity.server_validation.logs]
- GIVEN Stevenarella server modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative module tests plus affected dry-runs and Cairn gates passing.
