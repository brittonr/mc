# Design: Define Valence admin permission ergonomics

## Context

Hyperion has group components, command permission derives, and command tree refresh on group changes. Valence has its own command graph, parsers, modifiers, executables, and scopes. A Valence integration should extend the existing command model rather than introduce a parallel command framework.

## Decisions

### 1. Use Valence command scopes

**Choice:** Permission checks integrate with `valence_command` scopes or a small optional layer around them.

**Rationale:** Command visibility and execution should stay in one command system.

### 2. Pure permission evaluator

**Choice:** Permission decisions are pure functions over command metadata, player roles/scopes, and context. ECS systems gather data and update command trees.

**Rationale:** Allowed/denied behavior needs simple fixture coverage.

### 3. Optional storage boundary

**Choice:** If persistence is added, keep storage behind a trait or adapter and test invalid/missing rows separately from permission logic.

**Rationale:** Servers may have their own auth and moderation storage.

### 4. Denials are explicit

**Choice:** Denied command execution and hidden command-tree entries produce clear user-facing and diagnostic behavior.

**Rationale:** Silent denials are hard to debug and can confuse clients.

## Risks / Trade-offs

- Permissions can become policy-heavy; keep core roles/scopes minimal.
- Command tree refresh can be expensive; measure before optimizing.
- Storage choices are server-specific; keep persistence optional.
