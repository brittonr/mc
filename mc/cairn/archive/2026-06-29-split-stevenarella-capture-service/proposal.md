# Proposal: Split Stevenarella capture service responsibilities

## Why

`clients/stevenarella/src/capture.rs` already contains useful pure validation and planning code, but it still combines capture request validation, queueing, framebuffer readback, PNG encoding, artifact persistence, recording cadence, recording state, metadata validation, and tests in one module. The capture surface is consumed by MCP and evidence flows, so clearer boundaries would make it safer to extend.

## What Changes

- Split capture code into focused modules for request validation/planning, queueing, readback normalization, artifact persistence, recording cadence/state, metadata validation, and service shells.
- Keep framebuffer reads, filesystem writes, image encoding, clocks, and channel operations in shells or adapter modules.
- Keep dimension/path/digest/recording decisions pure and testable with in-memory inputs.
- Preserve existing request types, artifact path semantics, BLAKE3 metadata, redaction state, recording bounds, MCP-facing behavior, and non-claims.
- Add positive and negative tests for extracted planning, validation, queue, persistence, readback, and recording cores.

## Impact

- **Files**: `clients/stevenarella/src/capture.rs`, new `clients/stevenarella/src/capture/*` modules, MCP capture adapter tests, and Cairn artifacts.
- **Testing**: baseline capture tests, focused positive and negative capture module tests, affected MCP dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: capture architecture only; this does not change evidence scope or add new rendering/capture guarantees.
