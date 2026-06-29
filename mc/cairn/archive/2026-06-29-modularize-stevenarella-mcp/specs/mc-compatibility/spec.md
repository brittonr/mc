# mc-compatibility Change Spec: Stevenarella MCP modules

## Requirements

### Requirement: Stevenarella MCP module boundaries

r[mc_compatibility.stevenarella_mcp.module_boundaries] Stevenarella MCP code SHOULD expose cohesive module boundaries for JSON-RPC protocol handling, auth validation, transport runtime, control queue adaptation, tool and resource registry, and capture-tool adaptation.

#### Scenario: MCP responsibility has one owner

r[mc_compatibility.stevenarella_mcp.module_boundaries.ownership]
- GIVEN an MCP responsibility is reviewed
- WHEN maintainers inspect the MCP module tree
- THEN the responsibility is owned by a focused module
- AND unrelated transport, tool, auth, and capture concerns are not reintroduced into one root module.

### Requirement: Stevenarella MCP protocol core

r[mc_compatibility.stevenarella_mcp.protocol_core] MCP request routing and JSON-RPC response rendering SHOULD be pure over in-memory request values, explicit auth state, and explicit tool adapter outcomes.

#### Scenario: MCP request routing is testable without transport

r[mc_compatibility.stevenarella_mcp.protocol_core.testable]
- GIVEN an MCP JSON-RPC request line and explicit adapter outcomes
- WHEN the protocol core handles the request
- THEN the result can be tested without stdio, TCP sockets, threads, capture waits, game state, or channel side effects
- AND transport/runtime shells remain responsible for those side effects.

### Requirement: Stevenarella MCP parity

r[mc_compatibility.stevenarella_mcp.parity] MCP modularization MUST preserve endpoint opt-in behavior, auth semantics, tool and resource names, JSON-RPC error codes, response shapes, capture and control outcomes, and evidence non-claims.

#### Scenario: MCP public surface remains stable

r[mc_compatibility.stevenarella_mcp.parity.stable]
- GIVEN a supported pre-refactor MCP request or startup option
- WHEN the modularized MCP surface processes the same input
- THEN the endpoint behavior, auth result, JSON-RPC response, tool/resource vocabulary, and non-claim boundaries remain equivalent
- AND no new game-control capability is enabled by default.

### Requirement: Stevenarella MCP positive tests

r[mc_compatibility.stevenarella_mcp.positive_tests] The change MUST include positive tests for tools/list, resources/list, control calls, capture calls, resource reads, auth success, stdio options, and TCP endpoint validation.

#### Scenario: Supported MCP paths pass

r[mc_compatibility.stevenarella_mcp.positive_tests.coverage]
- GIVEN representative supported MCP requests and startup inputs
- WHEN extracted MCP modules process them
- THEN tests prove the expected responses, adapter calls, and validation outcomes are produced.

### Requirement: Stevenarella MCP negative tests

r[mc_compatibility.stevenarella_mcp.negative_tests] The change MUST include negative tests for malformed JSON, unknown methods, missing tools, unauthorized requests, invalid capture arguments, closed queues, empty token env names, and invalid TCP auth.

#### Scenario: Invalid MCP paths fail closed

r[mc_compatibility.stevenarella_mcp.negative_tests.fail_closed]
- GIVEN invalid or unauthorized MCP requests or startup inputs
- WHEN extracted MCP modules process them
- THEN tests prove the inputs are rejected with the expected JSON-RPC error, validation diagnostic, or containment outcome.

### Requirement: Stevenarella MCP validation

r[mc_compatibility.stevenarella_mcp.validation] The change MUST record focused Stevenarella MCP tests, affected mc-compat MCP dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_mcp.validation.logs]
- GIVEN MCP modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative MCP tests plus affected dry-runs and Cairn gates passing.
