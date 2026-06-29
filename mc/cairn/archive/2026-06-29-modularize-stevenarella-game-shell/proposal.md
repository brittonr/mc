# Proposal: Modularize Stevenarella game shell

## Why

`clients/stevenarella/src/main.rs` combines CLI/options, game construction, MCP control application, capture startup, connection lifecycle, tick orchestration, window event handling, and tests. That makes the main client shell difficult to extend safely because pure control decisions sit beside renderer/window/network side effects.

## What Changes

- Split the game shell into focused modules for startup options, game lifecycle, MCP control application, capture startup, connection orchestration, ticking, and window event handling.
- Extract pure control and startup decisions into deterministic cores that return explicit shell actions.
- Keep renderer calls, network connection changes, window operations, filesystem setup, and process startup in thin shells.
- Preserve existing CLI flags, MCP response vocabulary, capture behavior, connection behavior, window behavior, and non-claims.
- Add positive and negative tests for extracted control and startup decisions.

## Impact

- **Files**: `clients/stevenarella/src/main.rs`, new game-shell modules, MCP/control/capture tests, affected dry-runs, and Cairn artifacts.
- **Testing**: baseline Stevenarella checks, positive and negative game-shell tests, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: client shell architecture only; no new gameplay, rendering, MCP, or compatibility claim is promoted.
