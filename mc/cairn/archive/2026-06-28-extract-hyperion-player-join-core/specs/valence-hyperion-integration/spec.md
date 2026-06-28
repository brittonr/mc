# valence-hyperion-integration Change Spec: Hyperion player join core

## Requirements

### Requirement: Hyperion player join scope

r[valence_hyperion_integration.hyperion_player_join.scope] Hyperion player-join modularity work MUST be scoped as Hyperion-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion player-join ownership is explicit

r[valence_hyperion_integration.hyperion_player_join.scope.owned]
- GIVEN Hyperion player-join work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local
- AND it does not claim Valence adoption or mc-compat evidence.

### Requirement: Hyperion player join core

r[valence_hyperion_integration.hyperion_player_join.core] Hyperion player-join egress SHOULD expose pure cores for initial packet selection, packet ordering, state summaries, chunk/view facts, and diagnostics.

#### Scenario: Join plan is testable without runtime

r[valence_hyperion_integration.hyperion_player_join.core.testable]
- GIVEN player, world, chunk, and connection summaries
- WHEN the Hyperion player-join core processes them
- THEN the join plan can be tested without ECS app, network/proxy state, tracing, scheduling, or packet sends.

### Requirement: Hyperion player join shell boundary

r[valence_hyperion_integration.hyperion_player_join.shell_boundary] Hyperion player-join extraction MUST keep ECS reads, packet sends, network/proxy state, tracing, scheduling, and runtime side effects outside pure join cores.

#### Scenario: Join side effects remain in shell

r[valence_hyperion_integration.hyperion_player_join.shell_boundary.effects]
- GIVEN the player-join core returns a join plan
- WHEN the Hyperion egress shell applies that plan
- THEN only the shell reads ECS state, sends packets, touches network/proxy state, records traces, or wires schedules.

### Requirement: Hyperion player join parity

r[valence_hyperion_integration.hyperion_player_join.parity] Hyperion player-join extraction MUST preserve Hyperion join behavior, packet order, public APIs, performance-sensitive boundaries, and non-claims.

#### Scenario: Hyperion join behavior remains stable

r[valence_hyperion_integration.hyperion_player_join.parity.stable]
- GIVEN a supported pre-refactor Hyperion player-join input
- WHEN the extracted join core and shell process the same input
- THEN packet order, join output, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion player join tests

r[valence_hyperion_integration.hyperion_player_join.tests] The change MUST include positive and negative tests for valid join plans, missing player state, invalid chunk/view facts, packet-order regressions, and rejected join inputs.

#### Scenario: Player-join fixtures cover success and failure

r[valence_hyperion_integration.hyperion_player_join.tests.coverage]
- GIVEN representative valid and invalid Hyperion player-join inputs
- WHEN join tests run
- THEN supported join plans pass and malformed join inputs fail closed.

### Requirement: Hyperion player join validation

r[valence_hyperion_integration.hyperion_player_join.validation] The change MUST record focused Hyperion tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_player_join.validation.logs]
- GIVEN Hyperion player-join extraction is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tests plus Cairn gates passing.
