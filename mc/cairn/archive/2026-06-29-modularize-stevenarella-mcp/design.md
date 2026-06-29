# Design: Stevenarella MCP modules

## Context

MCP is an explicit compatibility instrumentation surface. The current single module contains both pure JSON-RPC request semantics and side-effecting transport/runtime code. The desired shape is a pure MCP protocol core with thin transport/tool shells.

## Decisions

### 1. Separate protocol core from transport runtime

**Choice:** Move JSON-RPC parsing, method dispatch, result/error rendering, tool/resource metadata, and auth decisions into pure modules. Keep socket/stdio loops, threads, channel waits, and runtime shutdown in transport shells.

**Rationale:** MCP request behavior can be tested with in-memory JSON lines and explicit tool adapters.

### 2. Keep adapters narrow

**Choice:** Control and capture modules expose small adapter traits or functions that convert routed MCP calls into queue/capture shell operations.

**Rationale:** The core should not know about renderer readback or game control internals beyond stable adapter outcomes.

### 3. Preserve stable vocabulary

**Choice:** Tool names, resource URIs, JSON-RPC error codes, auth token semantics, and response object shapes remain stable.

**Rationale:** The mc-compat runner and evidence fixtures consume those names.

### 4. Keep opt-in safety visible

**Choice:** Process startup must continue to create MCP queues/endpoints only when explicit MCP flags opt in.

**Rationale:** MCP control surfaces must remain absent by default.

## Risks / Trade-offs

- Transport loops are side-effect-heavy; extract pure routing first, then move runtime code.
- Capture tool tests may need fake adapters to avoid renderer dependencies.
- Auth diagnostics are safety relevant; preserve exact failure reasons where current tests rely on them.
