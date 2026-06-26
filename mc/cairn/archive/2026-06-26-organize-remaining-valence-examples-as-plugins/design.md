# Design: Organize remaining Valence examples as Bevy plugins

## Context

Valence presents itself as a Bevy ECS-powered Minecraft server framework. Examples are the clearest documentation for that model, but several still register systems directly in `main`. Example-local plugins make the imperative shell obvious while preserving pure gameplay cores and keeping examples opt-in binaries.

## Decisions

### 1. Select examples by schedule value

**Choice:** Start with examples that have multiple systems, event readers, state resources, or ordering-sensitive behavior rather than tiny single-system demos.

**Rationale:** Plugin extraction should improve reviewability and composition, not add ceremony to trivial examples.

### 2. Use common phase intent with local types

**Choice:** Each extracted example plugin defines local `SystemSet`s for input, rule evaluation, world mutation, presentation, and cleanup when those phases exist.

**Rationale:** Local sets avoid cross-example coupling while keeping schedule reviews consistent.

### 3. Keep pure decisions outside ECS shells

**Choice:** Gameplay calculations, command parsing decisions, board updates, parkour scoring decisions, and combat cooldown policy stay in pure helpers where practical.

**Rationale:** Bevy systems should be thin adapters over testable logic.

### 4. Preserve example identity

**Choice:** Binary names, commands, env/CLI inputs, and documented example behavior remain stable.

**Rationale:** This is an organization change, not a user-facing gameplay change.

## Risks / Trade-offs

- Too many tiny plugins can make examples harder to read; only extract where the schedule boundary is useful.
- Reordering systems can change behavior; schedule hygiene and focused example tests are required.
- Public example plugin names may look like stable APIs; documentation must state example support boundaries.
