# mc-compatibility Change Spec: Stevenarella player entity split

## Requirements

### Requirement: Stevenarella player boundaries

r[mc_compatibility.stevenarella_player.player_boundaries] Stevenarella player entity code SHOULD expose cohesive boundaries for player construction, model state, rendering, movement, collision, and ECS system wiring.

#### Scenario: Player responsibility has one owner

r[mc_compatibility.stevenarella_player.player_boundaries.ownership]
- GIVEN a player entity responsibility is reviewed
- WHEN maintainers inspect player modules
- THEN the responsibility is owned by a focused module
- AND unrelated rendering, movement, collision, and ECS concerns are not reintroduced into one module.

### Requirement: Stevenarella player core

r[mc_compatibility.stevenarella_player.player_core] Player movement, collision, model-part visibility, and local/remote state decisions SHOULD be pure over explicit inputs.

#### Scenario: Player decision is testable without renderer

r[mc_compatibility.stevenarella_player.player_core.testable]
- GIVEN player state, movement, collision, or model summaries
- WHEN the player core processes them
- THEN the decision can be tested without renderer, resource manager, network, or live ECS side effects.

### Requirement: Stevenarella player parity

r[mc_compatibility.stevenarella_player.parity] Player entity splitting MUST preserve existing player behavior, model visibility, movement/collision semantics, public APIs, and evidence non-claims.

#### Scenario: Player behavior remains stable

r[mc_compatibility.stevenarella_player.parity.stable]
- GIVEN a supported pre-refactor player input
- WHEN the split player modules process the same input
- THEN player state, visibility, movement, collision, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Stevenarella player positive tests

r[mc_compatibility.stevenarella_player.positive_tests] The change MUST include positive tests for local/remote creation facts, model visibility, movement updates, collision decisions, and renderer-shell plans.

#### Scenario: Supported player paths pass

r[mc_compatibility.stevenarella_player.positive_tests.coverage]
- GIVEN representative supported player inputs
- WHEN extracted player cores process them
- THEN tests prove the expected state, movement, collision, or render plans are produced.

### Requirement: Stevenarella player negative tests

r[mc_compatibility.stevenarella_player.negative_tests] The change MUST include negative tests for invalid movement input, collision edge cases, missing model resources, disabled model parts, missing ECS components, and empty entity sets.

#### Scenario: Invalid player paths fail closed

r[mc_compatibility.stevenarella_player.negative_tests.fail_closed]
- GIVEN invalid player inputs
- WHEN extracted player cores or shells process them
- THEN tests prove the inputs are rejected, ignored, clamped, or contained according to current behavior.

### Requirement: Stevenarella player validation

r[mc_compatibility.stevenarella_player.validation] The change MUST record focused Stevenarella entity/player tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_player.validation.logs]
- GIVEN player entity splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative player tests plus affected checks and Cairn gates passing.
