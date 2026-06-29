# valence-hyperion-integration Change Spec: Hyperion simulation shell

## Requirements

### Requirement: Hyperion simulation scope

r[valence_hyperion_integration.hyperion_simulation.scope] Hyperion simulation shell modularity work MUST be scoped as Hyperion-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion simulation ownership is explicit

r[valence_hyperion_integration.hyperion_simulation.scope.owned]
- GIVEN Hyperion simulation modularity work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local
- AND it does not claim Valence adoption, production-scale evidence, or mc-compat evidence.

### Requirement: Hyperion simulation boundaries

r[valence_hyperion_integration.hyperion_simulation.boundaries] Hyperion simulation code SHOULD expose cohesive boundaries for system registration, simulation state orchestration, packet-facing adapters, domain coordination, diagnostics, and pure simulation decisions.

#### Scenario: Simulation responsibility has one owner

r[valence_hyperion_integration.hyperion_simulation.boundaries.ownership]
- GIVEN a Hyperion simulation responsibility is reviewed
- WHEN maintainers inspect simulation modules
- THEN the responsibility is owned by a focused Hyperion module.

### Requirement: Hyperion simulation shell boundary

r[valence_hyperion_integration.hyperion_simulation.shell_boundary] Hyperion simulation modularization MUST keep ECS mutation, packet emission, network/proxy integration, tracing, scheduling, and hot-path side effects outside pure simulation cores.

#### Scenario: Simulation side effects remain in shell

r[valence_hyperion_integration.hyperion_simulation.shell_boundary.effects]
- GIVEN a pure simulation core returns a decision
- WHEN the Hyperion shell applies that decision
- THEN only the shell mutates ECS state, emits packets, integrates network/proxy state, records traces, or wires schedules.

### Requirement: Hyperion simulation parity

r[valence_hyperion_integration.hyperion_simulation.parity] Hyperion simulation modularization MUST preserve Hyperion public APIs, simulation behavior, performance-sensitive boundaries, and non-claims.

#### Scenario: Hyperion simulation behavior remains stable

r[valence_hyperion_integration.hyperion_simulation.parity.stable]
- GIVEN a supported pre-refactor Hyperion simulation input
- WHEN the modularized simulation shell processes the same input
- THEN public API behavior, simulation output, performance-sensitive boundaries, and non-claim boundaries remain equivalent.

### Requirement: Hyperion simulation tests

r[valence_hyperion_integration.hyperion_simulation.tests] The change MUST include positive and negative tests for simulation state transitions, missing state, invalid packet-facing summaries, empty worlds, and rejected update plans.

#### Scenario: Simulation fixtures cover success and failure

r[valence_hyperion_integration.hyperion_simulation.tests.coverage]
- GIVEN representative valid and invalid Hyperion simulation inputs
- WHEN simulation tests run
- THEN supported transitions pass and malformed transitions fail closed.

### Requirement: Hyperion simulation validation

r[valence_hyperion_integration.hyperion_simulation.validation] The change MUST record focused Hyperion tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_simulation.validation.logs]
- GIVEN Hyperion simulation modularization is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tests plus Cairn gates passing.
