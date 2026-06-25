# Design: Split Stevenarella server probes

## Context

Stevenarella's `server/mod.rs` is an integration-heavy module. Some complexity is inherent to Minecraft protocol state, but compatibility probes should be testable without starting the renderer, network connection, or full world. The split should extract deterministic probe decisions first and leave packet IO as a boring shell.

## Decisions

### 1. Start with probe inventory

Classify code as general protocol handling, world/entity state update, inventory/window handling, compat probe decision, environment parsing, or milestone logging. Move only well-classified clusters.

### 2. Pure probe cores return actions

Probe cores should accept explicit state snapshots and return actions such as key press, chat command, click slot, block interaction, expected milestone, or no-op. They must not read environment variables, log, mutate world state, or write packets.

### 3. Shell handlers own IO and mutation

Packet handlers remain responsible for decoding, connection state mutation, ECS/world updates, and dispatching returned probe actions into existing packet writes/logs.

### 4. Preserve rail semantics

The split is structural. Milestone text, env var names, fixture positions, packet sequences, and receipt-observable behavior stay stable unless another Cairn changes the scenario contract.

## Risks / Trade-offs

- Extracting too much can fight legacy control flow; move one probe family at a time.
- Some probes share inventory helpers; prefer shared pure helpers over cross-module mutable state.
- Tests need fixtures for malformed and out-of-order packets, not only happy-path rail scripts.
