# Design: Valence compatibility fixture modules

## Context

Valence compatibility fixtures are runnable examples, not core default gameplay. They still need strong schedule and non-claim boundaries because mc-compat receipts use them as local evidence fixtures. Modularization should keep the examples launchable while extracting pure fixture rules and Bevy systems.

## Decisions

### 1. Keep examples as thin app shells

**Choice:** `ctf.rs` and `survival_compat.rs` should primarily build the Valence app, register explicit fixture plugins, and wire configuration sources.

**Rationale:** Reviewers can see the launch surface quickly, while fixture modules own the behavior.

### 2. Split pure fixture contracts from Bevy shells

**Choice:** Keep deterministic rules, score/flag/inventory/survival contracts, and config parsing in pure modules. Bevy systems and resources adapt those decisions to `App`, `World`, events, and packets.

**Rationale:** Pure fixture logic can have positive and negative tests without running a server.

### 3. Preserve schedule hygiene contracts

**Choice:** Update or preserve schedule contract declarations and schedule hygiene expectations as modules move.

**Rationale:** Plugin order and explicit opt-in behavior are part of the fixture safety boundary.

### 4. Avoid broad Valence claims

**Choice:** Keep all extracted modules under examples/fixture ownership unless a separate Valence API Cairn promotes reusable code into crates.

**Rationale:** The fixtures provide bounded compatibility evidence, not default server behavior.

## Risks / Trade-offs

- Moving systems can change Bevy schedule order accidentally; schedule hygiene must be part of validation.
- Extracting fixture code into a shared crate can make bounded fixture behavior look like a public API; document ownership if a crate is created.
- Some constants intentionally mirror runner/client contracts; keep generated scenario contract checks in sync.
