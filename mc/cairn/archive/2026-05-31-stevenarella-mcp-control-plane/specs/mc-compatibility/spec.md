# Delta: Stevenarella MCP control plane

## Requirements

### Requirement: Control command contract

r[mc_compatibility.stevenarella_mcp_control.contract] Stevenarella MCP control MUST define a bounded, typed command contract before exposing automation tools.

#### Scenario: Command scope is explicit

r[mc_compatibility.stevenarella_mcp_control.contract.scope]
- GIVEN MCP control work starts
- WHEN the command contract is reviewed
- THEN it names status, connect, disconnect, key, look, mouse, use-item, attack, and chat as the initial supported actions
- AND it states that headless rendering, frame capture, public-server authorization, load testing, and semantic compatibility remain non-claims.

#### Scenario: Invalid commands fail closed

r[mc_compatibility.stevenarella_mcp_control.contract.invalid_commands]
- GIVEN an MCP request contains an unknown key name, unknown mouse button, malformed address, missing required field, or unsupported action
- WHEN command validation runs
- THEN it returns a structured error without mutating Stevenarella game state.

### Requirement: MCP transport safety

r[mc_compatibility.stevenarella_mcp_control.transport] Stevenarella MCP transport MUST be native-only and safe by default.

#### Scenario: Stdio remains JSON-RPC clean

r[mc_compatibility.stevenarella_mcp_control.transport.stdio_clean]
- GIVEN Stevenarella starts with `--mcp-stdio`
- WHEN ordinary client logs are emitted
- THEN stdout carries only MCP JSON-RPC bytes
- AND logs remain available through stderr where safe or through `client.log`.

#### Scenario: TCP bind fails closed

r[mc_compatibility.stevenarella_mcp_control.transport.tcp_auth]
- GIVEN Stevenarella is asked to bind MCP on a non-loopback address without an explicit token environment variable
- WHEN startup validates MCP options
- THEN startup rejects the bind before accepting control requests.

### Requirement: Main-thread command ownership

r[mc_compatibility.stevenarella_mcp_control.main_thread_queue] MCP control MUST preserve winit, GL, `Game`, and `Server` main-thread ownership.

#### Scenario: Worker thread only enqueues

r[mc_compatibility.stevenarella_mcp_control.main_thread_queue.worker_enqueues]
- GIVEN an MCP worker receives a valid command
- WHEN the command is accepted
- THEN the worker only enqueues a typed `ControlCommand`
- AND it does not directly mutate `Game`, `Server`, winit window state, or GL state.

#### Scenario: Commands drain at deterministic boundary

r[mc_compatibility.stevenarella_mcp_control.main_thread_queue.drain_boundary]
- GIVEN queued control commands exist
- WHEN the main loop enters the configured per-frame drain point
- THEN commands are applied before the server tick for that frame
- AND responses identify whether the command was applied, rejected, or deferred.

### Requirement: MCP tool behavior

r[mc_compatibility.stevenarella_mcp_control.tools] MCP tools MUST reuse Stevenarella internal control methods rather than host OS input synthesis.

#### Scenario: Movement uses internal key state

r[mc_compatibility.stevenarella_mcp_control.tools.key_state]
- GIVEN the client is connected and a player entity exists
- WHEN MCP sends a key command for a supported Stevenarella movement key
- THEN `Server::key_press` updates the corresponding `PlayerMovement` key state.

#### Scenario: Look updates player rotation

r[mc_compatibility.stevenarella_mcp_control.tools.look]
- GIVEN the client is connected and a player entity exists
- WHEN MCP sends a bounded look delta
- THEN the player rotation is updated with the same pitch limits as physical mouse input.

#### Scenario: Chat uses protocol packet path

r[mc_compatibility.stevenarella_mcp_control.tools.chat]
- GIVEN the client is connected
- WHEN MCP sends a chat message or slash command
- THEN Stevenarella sends it through the protocol `ChatMessage` serverbound path
- AND oversized or malformed messages are rejected before packet write.

### Requirement: Control validation evidence

r[mc_compatibility.stevenarella_mcp_control.validation] The MCP control plane MUST have positive and negative tests before any runner depends on it.

#### Scenario: Validation covers happy and sad paths

r[mc_compatibility.stevenarella_mcp_control.validation.tests]
- GIVEN MCP control implementation is complete
- WHEN focused tests run
- THEN they cover valid command parsing and application
- AND they cover invalid key names, invalid button names, disconnected operations, stdout contamination, and unsafe bind attempts.

### Requirement: Control evidence artifacts

r[mc_compatibility.stevenarella_mcp_control.artifacts] Review-critical MCP control evidence MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.stevenarella_mcp_control.artifacts.reviewable]
- GIVEN the control plane Cairn is ready to archive
- WHEN reviewers inspect the parent repo
- THEN focused test output, command-shape evidence, Cairn gate output, validation output, and BLAKE3 manifests are present under `docs/evidence/`.
