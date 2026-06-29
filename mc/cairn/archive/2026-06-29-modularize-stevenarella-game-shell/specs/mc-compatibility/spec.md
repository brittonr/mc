# mc-compatibility Change Spec: Stevenarella game shell

## Requirements

### Requirement: Stevenarella game shell boundaries

r[mc_compatibility.stevenarella_game.shell_boundaries] Stevenarella game startup and runtime shell code SHOULD expose cohesive boundaries for startup options, game lifecycle, MCP control application, capture startup, connection orchestration, ticking, and window events.

#### Scenario: Game shell responsibility has one owner

r[mc_compatibility.stevenarella_game.shell_boundaries.ownership]
- GIVEN a client shell responsibility is reviewed
- WHEN maintainers inspect the game shell modules
- THEN the responsibility is owned by a focused module
- AND unrelated startup, control, capture, connection, tick, and window concerns are not reintroduced into one root module.

### Requirement: Stevenarella control core

r[mc_compatibility.stevenarella_game.control_core] Non-trivial MCP control and startup decisions SHOULD be pure over explicit state summaries and return explicit shell actions or responses.

#### Scenario: Control decision is testable without live game state

r[mc_compatibility.stevenarella_game.control_core.testable]
- GIVEN a control command, startup option, or capture startup input
- WHEN the pure game-shell core evaluates it
- THEN the resulting response or shell action can be tested without renderer, window, filesystem, or network side effects.

### Requirement: Stevenarella game shell parity

r[mc_compatibility.stevenarella_game.parity] Game-shell modularization MUST preserve existing CLI flags, MCP response vocabulary, capture behavior, connection behavior, window behavior, and evidence non-claims.

#### Scenario: Game shell behavior remains stable

r[mc_compatibility.stevenarella_game.parity.stable]
- GIVEN a supported pre-refactor startup option, control command, or window event
- WHEN the modularized game shell processes the same input
- THEN the selected action, response message, side-effect boundary, and non-claim behavior remain equivalent.

### Requirement: Stevenarella game shell positive tests

r[mc_compatibility.stevenarella_game.positive_tests] The change MUST include positive tests for status, connect, disconnect, key, look, mouse, use-item, attack, chat, sign-editor, capture, and startup option plans.

#### Scenario: Supported game-shell paths pass

r[mc_compatibility.stevenarella_game.positive_tests.coverage]
- GIVEN representative supported game-shell inputs
- WHEN extracted game-shell cores process them
- THEN tests prove the expected responses or action plans are produced.

### Requirement: Stevenarella game shell negative tests

r[mc_compatibility.stevenarella_game.negative_tests] The change MUST include negative tests for disconnected commands, missing player state, invalid sign editor state, invalid capture request, unavailable queues, invalid startup recording options, and out-of-range look input.

#### Scenario: Invalid game-shell paths fail closed

r[mc_compatibility.stevenarella_game.negative_tests.fail_closed]
- GIVEN invalid game-shell inputs
- WHEN extracted game-shell cores process them
- THEN tests prove the inputs are rejected, deferred, or contained with the expected response before unintended side effects occur.

### Requirement: Stevenarella game shell validation

r[mc_compatibility.stevenarella_game.validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_game.validation.logs]
- GIVEN game-shell modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative game-shell tests plus affected dry-runs and Cairn gates passing.
