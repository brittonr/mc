# Proposal: Modularize Stevenarella MCP transport and tools

## Why

`clients/stevenarella/src/mcp.rs` owns JSON-RPC framing, transport startup, TCP auth, stdio/TCP serving, control command queues, tool/resource registry, capture tool adapters, error rendering, and tests in one large module. This makes MCP safety boundaries hard to audit and couples pure request handling to transport side effects.

## What Changes

- Split MCP code into focused modules for JSON-RPC request/response handling, auth validation, transport runtime, control command queues, tool/resource registry, and capture-tool adapters.
- Keep pure request routing and response rendering separate from threads, sockets, stdio, channels, capture waits, and shutdown behavior.
- Preserve existing endpoint opt-in behavior, auth semantics, tool/resource names, JSON-RPC error codes, response shapes, capture/control behavior, and non-claims.
- Add positive and negative tests for request routing, auth, tool calls, resources, capture output modes, transport options, and malformed JSON-RPC inputs.

## Impact

- **Files**: `clients/stevenarella/src/mcp.rs`, new `clients/stevenarella/src/mcp/*` modules, MCP/control/capture tests, docs if instrumentation boundaries move, and Cairn artifacts.
- **Testing**: baseline MCP tests, focused positive and negative module tests, affected mc-compat MCP dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: MCP architecture and safety-boundary clarity only; this does not add new game control capabilities or compatibility evidence.
