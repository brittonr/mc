# Design: Stevenarella server session modules

## Context

Stevenarella's `server/mod.rs` currently acts as both the protocol session shell and the owner for many unrelated packet families. The existing `server/probes.rs` module shows the desired pattern: pure scheduling/decision cores with a thin shell that performs ECS mutations, packet writes, and milestone logging.

## Decisions

### 1. Keep `Server` as the shell, not the dumping ground

**Choice:** Preserve `Server` as the session object that owns connection state, world state, ECS access, and packet writes, but move packet-family behavior into focused modules with narrow APIs.

**Rationale:** This keeps public call sites stable while making the internal ownership boundary reviewable.

### 2. Extract by packet family and mutation target

**Choice:** Split handlers by responsibility family: login/session lifecycle, packet dispatch, world/dimension state, chunks, entities, inventory/window interactions, chat/player-list behavior, plugin messages, and compat probes.

**Rationale:** Packet changes usually affect one family. Matching the module boundary to that family reduces accidental coupling.

### 3. Prefer functional cores for handler decisions

**Choice:** For non-trivial handler logic, introduce pure functions that take explicit packet/state summaries and return decisions or updates. The `Server` shell applies those decisions to world/ECS/connection/logging.

**Rationale:** Handler logic can be tested without connecting to Valence, Paper, OpenGL, or a live network socket.

### 4. Keep compat instrumentation explicit

**Choice:** Compat probes and milestone/typed-event vocabulary remain visibly bounded test-harness surfaces. If moved, the module name and tests must still make the instrumentation boundary obvious.

**Rationale:** Refactoring must not turn bounded probes into general gameplay claims.

## Risks / Trade-offs

- Packet handlers may share mutable state today; extract the pure decisions first, then move shells once borrow boundaries are clear.
- Some duplicated helper wrappers may be useful temporarily to maintain behavior; remove or consolidate them before archive.
- Client checks may require the mc devshell for native dependencies; record any blocker and run the smallest relevant component checks.
