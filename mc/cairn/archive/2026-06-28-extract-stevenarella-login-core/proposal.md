# Proposal: Extract Stevenarella login functional core

## Why

`Server::connect` currently mixes TCP connection setup, handshake packet writing, compression negotiation, encryption request handling, online/offline login success variants, reader spawning, and final `Server` construction. The login state machine is difficult to test in isolation and repeated packet-shape handling makes protocol-version changes riskier.

## What Changes

- Introduce pure login state and outcome types that normalize supported login packet variants into explicit decisions.
- Keep network I/O, encryption side effects, Mojang session joins, reader spawning, and `Server` construction in a thin imperative shell.
- Preserve online/offline login behavior, compression handling, encryption behavior, disconnect handling, milestone logs, and protocol-version compatibility.
- Add positive and negative tests for offline login, encrypted login, compression-before-success, disconnects, wrong packets, unsupported FML network versions, and malformed login outcomes.

## Impact

- **Files**: `clients/stevenarella/src/server/mod.rs`, new `clients/stevenarella/src/server/login.rs` or session modules, focused tests, affected compat dry-run checks if milestone behavior moves, and Cairn artifacts.
- **Testing**: baseline Stevenarella protocol/server tests before extraction, positive and negative login-core tests, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: login architecture only; this does not add new protocol support, public-server safety evidence, or gameplay parity claims.
