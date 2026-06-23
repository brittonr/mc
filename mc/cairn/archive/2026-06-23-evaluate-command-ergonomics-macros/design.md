# Design: Evaluate command ergonomics macros

## Context

Hyperion uses Clap derive metadata to register Minecraft commands, permissions, parsing, and suggestions. Valence has a graph-based command system with parser and scope registries. A Valence-native helper should generate or build Valence graphs, not route around them.

## Decisions

### 1. Reference-only Hyperion command framework

**Choice:** Hyperion's command ergonomics are design reference only. Valence command graph, parsers, scopes, and handlers remain authoritative.

**Rationale:** Replacing the command internals would conflict with existing Valence APIs.

### 2. Helper output is inspectable

**Choice:** Macros or builders produce command graph structures that can be compared against manually built graphs in tests.

**Rationale:** Ergonomic helpers should not hide command semantics.

### 3. Diagnostics are first-class

**Choice:** Duplicate literals, missing handlers, invalid parser annotations, invalid scopes, and suggestion mismatches produce clear compile-time or build-time diagnostics.

**Rationale:** Command helpers should make misuse difficult.

### 4. Manual construction remains supported

**Choice:** Docs keep manual graph construction as the baseline for advanced commands.

**Rationale:** Macros cannot cover every command shape cleanly.

## Risks / Trade-offs

- Procedural macros can be hard to debug; prefer builder helpers for early prototypes if possible.
- Clap semantics may not map perfectly to Minecraft command graphs; do not force a one-to-one mapping.
- Generated command trees can drift; include graph parity fixtures.
