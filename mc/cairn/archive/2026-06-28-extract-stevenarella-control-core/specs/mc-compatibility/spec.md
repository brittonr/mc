# mc-compatibility Change Spec: Stevenarella control core

## Requirements

### Requirement: Stevenarella control core

r[mc_compatibility.stevenarella_control.control_core] Stevenarella control command validation, normalization, response classification, key/look/mouse payload checks, and command capability facts SHOULD be pure over explicit command inputs.

#### Scenario: Control command decision is testable without game state

r[mc_compatibility.stevenarella_control.control_core.testable]
- GIVEN a control command input
- WHEN the control core validates or classifies it
- THEN the result can be tested without MCP transport, game mutation, capture queues, network sends, or logging.

### Requirement: Stevenarella control shell boundary

r[mc_compatibility.stevenarella_control.shell_boundary] Control-core extraction MUST keep MCP transport, game-state mutation, capture queues, network sends, and logging outside pure control cores.

#### Scenario: Control side effects remain in shell

r[mc_compatibility.stevenarella_control.shell_boundary.effects]
- GIVEN the control core returns a command decision
- WHEN MCP or game shell applies that decision
- THEN only the shell mutates game state, sends packets, queues capture, handles transport, or logs diagnostics.

### Requirement: Stevenarella control parity

r[mc_compatibility.stevenarella_control.parity] Control-core extraction MUST preserve JSON/control schema, response vocabulary, validation behavior, command names, and evidence non-claims.

#### Scenario: Control behavior remains stable

r[mc_compatibility.stevenarella_control.parity.stable]
- GIVEN a supported pre-refactor control command
- WHEN the extracted control core and shell process the same input
- THEN schema behavior, validation result, command name, response vocabulary, and non-claim boundary remain equivalent.

### Requirement: Stevenarella control positive tests

r[mc_compatibility.stevenarella_control.positive_tests] The change MUST include positive tests for status, connect, disconnect, key, look, mouse, use, attack, chat, resource-pack, sign-editor, and capture command validation.

#### Scenario: Supported control paths pass

r[mc_compatibility.stevenarella_control.positive_tests.coverage]
- GIVEN representative supported control commands
- WHEN extracted control cores process them
- THEN tests prove the expected validation and classification outcomes are produced.

### Requirement: Stevenarella control negative tests

r[mc_compatibility.stevenarella_control.negative_tests] The change MUST include negative tests for malformed commands, invalid keys, out-of-range look values, invalid mouse deltas, missing payloads, unsupported commands, and schema mismatches.

#### Scenario: Invalid control paths fail closed

r[mc_compatibility.stevenarella_control.negative_tests.fail_closed]
- GIVEN invalid control commands
- WHEN extracted control cores process them
- THEN tests prove the inputs are rejected or diagnosed according to current behavior.

### Requirement: Stevenarella control validation

r[mc_compatibility.stevenarella_control.validation] The change MUST record focused control/MCP tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_control.validation.logs]
- GIVEN control-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative control tests plus affected checks and Cairn gates passing.
