# valence-hyperion-integration Change Spec: Hyperion inventory core

## Requirements

### Requirement: Hyperion inventory scope

r[valence_hyperion_integration.hyperion_inventory.scope] Hyperion inventory modularity work MUST be scoped as Hyperion-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion ownership is explicit

r[valence_hyperion_integration.hyperion_inventory.scope.owned]
- GIVEN Hyperion inventory modularity work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local
- AND it does not claim Valence adoption or compatibility evidence.

### Requirement: Hyperion inventory core

r[valence_hyperion_integration.hyperion_inventory.core] Hyperion inventory simulation SHOULD expose pure cores for inventory transitions, slot validation, transaction outcomes, and packet-facing summaries.

#### Scenario: Inventory transition is testable without runtime

r[valence_hyperion_integration.hyperion_inventory.core.testable]
- GIVEN inventory state and transaction summaries
- WHEN the Hyperion inventory core processes them
- THEN the result can be tested without Bevy runtime, network IO, proxy state, or tracing side effects.

### Requirement: Hyperion inventory shell boundary

r[valence_hyperion_integration.hyperion_inventory.shell_boundary] Hyperion inventory extraction MUST keep ECS mutation, packet emission, scheduling, tracing, and network/proxy side effects outside pure inventory cores.

#### Scenario: Inventory side effects remain in shell

r[valence_hyperion_integration.hyperion_inventory.shell_boundary.effects]
- GIVEN the inventory core returns a transition decision
- WHEN the Hyperion shell applies that decision
- THEN only the shell mutates ECS state, emits packets, wires schedules, records traces, or touches network/proxy state.

### Requirement: Hyperion inventory parity

r[valence_hyperion_integration.hyperion_inventory.parity] Hyperion inventory extraction MUST preserve Hyperion public APIs, simulation behavior, packet-facing behavior, performance-sensitive boundaries, and non-claims.

#### Scenario: Hyperion inventory behavior remains stable

r[valence_hyperion_integration.hyperion_inventory.parity.stable]
- GIVEN a supported pre-refactor Hyperion inventory input
- WHEN the extracted core and shell process the same input
- THEN inventory state, packet-facing output, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion inventory tests

r[valence_hyperion_integration.hyperion_inventory.tests] The change MUST include positive and negative tests for valid transactions, invalid slots, malformed packets, empty inventories, boundary stack sizes, and rejected transitions.

#### Scenario: Inventory fixtures cover success and failure

r[valence_hyperion_integration.hyperion_inventory.tests.coverage]
- GIVEN representative valid and invalid Hyperion inventory inputs
- WHEN inventory tests run
- THEN they prove supported transitions pass and malformed transitions fail closed.

### Requirement: Hyperion inventory validation

r[valence_hyperion_integration.hyperion_inventory.validation] The change MUST record focused Hyperion tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_inventory.validation.logs]
- GIVEN Hyperion inventory extraction is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tests plus Cairn gates passing.
