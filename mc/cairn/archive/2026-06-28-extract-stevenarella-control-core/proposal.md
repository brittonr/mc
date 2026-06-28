# Proposal: Extract Stevenarella control command core

## Why

`clients/stevenarella/src/control.rs` defines control command data, response shapes, validation, and tests used by MCP and game-shell code. Control semantics should be a small pure core so MCP transport and game-state shells can share the same validation and response contract.

## What Changes

- Extract pure control command validation, normalization, response classification, key/look/mouse payload checks, and command capability facts.
- Keep MCP transport, game-state mutation, capture queues, network sends, and logging in shells.
- Preserve JSON/control schema, response vocabulary, validation behavior, command names, and non-claims.

## Impact

- **Files**: `clients/stevenarella/src/control.rs`, control submodules, MCP/game-shell call sites, focused tests, and Cairn artifacts.
- **Testing**: baseline control/MCP tests, positive and negative control-core tests, affected dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: control architecture only; no new MCP/game-control capability is promoted.
