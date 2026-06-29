# Proposal: Modularize Stevenarella server session code

## Why

`clients/stevenarella/src/server/mod.rs` combines connection setup, packet dispatch, world and dimension synchronization, chunk handling, entity updates, inventory/window behavior, plugin messages, and mc-compat probe shells in one large module. That makes protocol changes risky because unrelated concerns share one `impl Server` surface and pure decision logic is harder to test without standing up a live client/server session.

## What Changes

- Split Stevenarella server session responsibilities into cohesive modules such as login/session, packet dispatch, world/dimension handlers, chunk handlers, entity handlers, inventory/window handlers, plugin-message handlers, and compat-probe shells.
- Keep pure decision logic in small cores that return data, while `Server` remains the imperative shell for connection, ECS/world mutation, packet writes, and logging.
- Preserve the public `Server` API, protocol behavior, compat milestone vocabulary, typed-event hooks, MCP/control boundaries, and current evidence non-claims.
- Add positive and negative tests around extracted pure cores and packet-handler routing before removing old monolithic ownership.

## Impact

- **Files**: `clients/stevenarella/src/server/mod.rs`, new `clients/stevenarella/src/server/*` modules, focused client tests, compat runner dry-run checks if probe surfaces move, docs if ownership boundaries are documented, and Cairn artifacts.
- **Testing**: baseline Stevenarella checks before extraction, focused positive and negative module tests after extraction, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: client architecture only; this does not claim broader Minecraft compatibility, public-server readiness, or new gameplay parity.
