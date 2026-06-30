# Design: Stevenarella server runtime modules

## Context

The Stevenarella server runtime is the point where network packets become client state, renderer/world mutations, and mc-compat milestones. The current root module combines pure decision logic with packet I/O and large mutable probe state. The extraction should reduce file size without changing packet semantics or milestone evidence.

## Decisions

### 1. Extract by ownership domain

**Choice:** Move handlers and helpers by domain: login/session, world/chunks, entity tracking, inventory/window updates, block entities/signs, chat/plugin messages, and compat probes.

**Rationale:** Packet families change independently; focused modules make review and regression tests local to the affected domain.

### 2. Encapsulate compat probe state

**Choice:** Replace scattered `Server` probe booleans/counters with grouped state structs where each group owns its pure transition helpers.

**Rationale:** Probe state machines become testable without a live network connection, while the `Server` shell remains responsible for packet writes, ECS mutation, and logging.

### 3. Keep dispatch stable during extraction

**Choice:** Preserve packet registration/dispatch names and route extracted handlers through thin methods or helper modules before deeper API cleanup.

**Rationale:** This minimizes risk to generated protocol dispatch and protocol-version compatibility.

### 4. Preserve evidence vocabulary

**Choice:** Do not rename mc-compat environment variables, typed-event IDs, milestone text, or receipt expectations unless a separate compatibility Cairn requires it.

**Rationale:** Existing runner checks and promoted evidence consume those stable strings.

## Risks / Trade-offs

- The file has many implicit dependencies on `Server` fields; initial extraction should prefer small adapter structs over broad public access.
- Moving handler code can accidentally change packet dispatch order; baseline tests and dry-runs are required before and after core changes.
- Some probe decisions already live in `server/probes.rs`; avoid creating a second catch-all by splitting that file as well.
