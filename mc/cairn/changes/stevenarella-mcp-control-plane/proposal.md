# Proposal: Stevenarella MCP control plane

## Why

Stevenarella can currently be driven only through OS window input, hard-coded `MC_COMPAT_*` probes, or fixture-specific command paths. That makes compatibility work brittle: the runner cannot ask the client for state, issue generic movement/chat/click actions, or coordinate visual capture through a stable automation API.

## What Changes

- Add a native-only Stevenarella MCP control plane with a deterministic command queue from MCP transport into the existing winit/render thread.
- Expose bounded MCP tools for status, connect/disconnect, movement key state, look deltas, mouse actions, right-click/use, attack, and chat.
- Keep all game, server, winit, and GL mutation on the main thread; MCP worker threads may only parse requests and enqueue commands.
- Make stdio MCP safe by keeping JSON-RPC stdout clean and moving client logs to stderr/file-only mode when MCP stdio is active.
- Add positive and negative command parsing tests plus an in-process fake-command drain test before exposing the tool surface to runners.

## Impact

- **Files**: `stevenarella/src/main.rs`, new Stevenarella control/MCP modules, `stevenarella/src/server/mod.rs`, logging configuration, tests, and parent `mc` docs/evidence when promoted.
- **Validation**: Stevenarella focused tests, command parser positive/negative fixtures, Cairn gates, and parent `mc` validation.
- **Non-claims**: no headless EGL/OSMesa support, no public-server automation authorization, no external stress/load tooling, no semantic compatibility claim from merely issuing commands, and no frame artifact claim unless the capture Cairn is also completed.
