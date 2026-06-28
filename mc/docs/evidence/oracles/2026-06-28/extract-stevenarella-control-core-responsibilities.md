# Stevenarella control-core responsibility checkpoint

## Question

What responsibilities were present before extracting Stevenarella control-core facts, and what boundary does this change claim?

## Inspected evidence

- `clients/stevenarella/src/control.rs`: command data, JSON/value validation, normalization, key/look/mouse payload checks, resource-pack and sign-editor guards, response shapes, and parser tests.
- `clients/stevenarella/src/mcp.rs`: MCP transport, JSON-RPC tool schema/dispatch, queue sends, and response serialization.
- `clients/stevenarella/src/main.rs`: game-state command application, connection/player checks, capture queueing, network packet sends, and logging.
- `docs/evidence/run-logs/2026-06-28/extract-stevenarella-control-core-baseline-tests.run.log`: baseline `cargo test control` and `cargo test mcp` both passed before core edits.

## Decision

The extracted control core owns pure command validation, normalization, command facts, and response classification. MCP transport, game mutation, capture queues, packet sends, and logging remain shell responsibilities. This is an architecture/control-schema preservation change only.

## Owner

`clients/stevenarella/` under the parent mc repository.

## Next action

Use focused control/MCP tests, affected dry-runs, Cairn gates, and Cairn validation as closeout evidence. Do not promote broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claims from this checkpoint.
