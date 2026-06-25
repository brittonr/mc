# Proposal: Split Stevenarella server probes

## Why

`clients/stevenarella/src/server/mod.rs` mixes protocol packet handling, connection state, world updates, inventory/window logic, compatibility probe drivers, fixture environment parsing, and milestone logging in one very large module. This makes client protocol work riskier because compatibility-rail probe logic is interleaved with general server-state handling.

## What Changes

- Inventory the server module responsibilities and isolate compatibility probe code from general packet/state handling.
- Extract pure probe state machines for CTF, survival, inventory, combat, and dimension/biome rails.
- Keep packet reads/writes, connection mutation, resource access, and logging in thin shell handlers.
- Preserve current client behavior, milestone text, env var contracts, and evidence boundaries.
- Add positive probe-action tests and negative malformed/out-of-order/input-missing tests.

## Impact

- **Files**: `clients/stevenarella/src/server/mod.rs`, new `clients/stevenarella/src/server/*` modules, focused tests and fixtures, contract imports if scenario contracts are centralized first.
- **Testing**: Stevenarella server/probe tests through the mc devshell, selected mc-compat dry-runs, and Cairn gates before archive.
