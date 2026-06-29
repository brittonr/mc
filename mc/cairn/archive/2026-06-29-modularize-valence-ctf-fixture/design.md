# Design: Valence CTF fixture modules

## Context

The CTF example is both a runnable Valence fixture and a compatibility evidence source. It already has some pure `fixture_core` logic, but much of the CTF-specific behavior still lives in the example shell. This change extends the pure-core / imperative-shell split without changing the bounded fixture contract.

## Decisions

### 1. Split by gameplay/probe family

**Choice:** Extract modules for runtime config, arena/setup, team assignment, flag ownership, scoring/win conditions, inventory probes, combat/projectile probes, and milestone formatting.

**Rationale:** Each family changes for different reasons and should have independent tests.

### 2. Keep Bevy systems as shells

**Choice:** Bevy systems gather ECS state, call pure decision functions, then apply returned commands, mutations, events, and logs.

**Rationale:** CTF rules can be tested without a running server, while side effects remain explicit.

### 3. Treat probes as bounded contracts

**Choice:** Probe modules must name their env flags, expected milestones, and non-claims at the module boundary.

**Rationale:** Refactoring must keep evidence boundaries visible and prevent gameplay overclaims.

### 4. Preserve schedule contracts

**Choice:** Existing gameplay schedule and source plugin contract checks remain authoritative. Module extraction may move constants but must not weaken the contract.

**Rationale:** The fixture is reviewable because schedule ownership is explicit.

## Risks / Trade-offs

- Moving Bevy-heavy code can create borrow churn; extract pure decisions first and move shells second.
- Some temporary adapter functions may be needed during module moves; remove them before archive when possible.
- Fixture evidence names are externally consumed by runner tests and Cairn evidence; keep strings stable unless a separate change authorizes a schema move.
