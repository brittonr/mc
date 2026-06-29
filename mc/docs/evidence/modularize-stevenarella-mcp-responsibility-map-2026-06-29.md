# Stevenarella MCP responsibility map — 2026-06-29

## Question

Does the modularized Stevenarella MCP surface keep each responsibility in one focused owner while preserving the existing opt-in instrumentation and non-claim boundary?

## Inspected evidence

- Change package: `cairn/changes/modularize-stevenarella-mcp/{proposal.md,design.md,tasks.md,specs/mc-compatibility/spec.md}`.
- Baseline logs: `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-mcp.baseline-stevenarella-mcp-tests.run.log` and `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-mcp.baseline-stevenarella-control-tests.run.log`.
- Post-split logs: `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-mcp.post-dispatcher-mcp-tests.run.log`, `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-mcp.post-dispatcher-control-tests.run.log`, `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-mcp.protocol-core-purity-scan-rerun.run.log`, and `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-mcp.post-dispatcher-mcp-controlled-wrapper-dry-run.run.log`.
- Boundary inventory: `docs/compat-instrumentation-boundary.md`.

## Decision

Use `clients/stevenarella/src/mcp.rs` only as a facade that re-exports the stable `crate::mcp::*` API. The focused owners are:

| Responsibility | Owner |
| --- | --- |
| JSON-RPC parsing, auth-aware routing, pure route effects, and response rendering helpers | `clients/stevenarella/src/mcp/protocol.rs` |
| Tool/resource names, schemas, resource URIs, MIME/content vocabulary, and availability-filtered registry lists | `clients/stevenarella/src/mcp/registry.rs` |
| TCP auth token normalization, exact per-request token checks, and token diagnostic constants | `clients/stevenarella/src/mcp/auth.rs` |
| Side-effecting bridge from pure routed actions to control/capture adapter calls | `clients/stevenarella/src/mcp/dispatcher.rs` |
| Stdio/TCP transport option validation, listener startup, line loops, threads, shutdown flags, and process stdio wiring | `clients/stevenarella/src/mcp/transport.rs` |
| MCP control command channel, bounded per-frame drain, queue-close, timeout, and response-drop outcomes | `clients/stevenarella/src/mcp/control_queue.rs` |
| Capture tool argument validation, one-shot capture queue waits, inline/artifact metadata, and capture queue error mapping | `clients/stevenarella/src/mcp/capture_adapter.rs` |
| Startup opt-in boundary that creates MCP command/capture queues only when CLI MCP flags are present | `clients/stevenarella/src/compat_instrumentation.rs` |

The protocol core routes in-memory requests to `McpProtocolAction` values before any command queue or capture queue side effect runs. The dispatcher shell executes those routed actions against explicit adapters. The transport module owns stdio/TCP I/O and thread lifetimes. Adapter modules own waits on channels and capture service outcomes.

## Non-claims

This map only covers MCP architecture and safety-boundary clarity. It does not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness.

## Owner

Stevenarella MCP compatibility instrumentation (`clients/stevenarella/src/mcp.rs` and `clients/stevenarella/src/mcp/`).

## Next action

Keep new MCP responsibilities in their focused modules. Any future vocabulary change must update `docs/compat-instrumentation-boundary.md`, the MCP scenario dry-run, and task evidence before archive.
