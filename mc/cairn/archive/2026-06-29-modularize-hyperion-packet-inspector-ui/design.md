# Design: Hyperion packet inspector UI modularization

## Context

Packet inspector UI code mixes state decisions with rendering/event shells. The split should keep UI behavior stable while allowing filter and selection behavior to be tested without terminal rendering.

## Decisions

### 1. Keep scope Hyperion tool-owned

**Choice:** Treat packet-inspector modularity as Hyperion tool work.

**Rationale:** Tool UI code is not Valence-owned and does not imply compatibility evidence.

### 2. Extract pure UI state decisions

**Choice:** Filtering, selection, sorting/grouping, viewport state, and render-model generation should be pure over explicit packet-list inputs.

**Rationale:** UI behavior can be tested without the terminal/UI framework.

### 3. Keep rendering in shells

**Choice:** Terminal drawing, event loop integration, IO, and logging remain in shell modules.

**Rationale:** Rendering side effects should be separate from packet-list semantics.
