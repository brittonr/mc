# Design: Stevenarella game shell modules

## Context

The client entry module is both the executable shell and the owner of several reusable decisions. The first refactor should keep public startup behavior stable while moving reusable decisions out of the root.

## Decisions

### 1. Split startup, control, and event shells

**Choice:** Create focused owners for CLI-derived options, capture startup, MCP control application, connection orchestration, tick orchestration, and window events.

**Rationale:** Each surface has separate side effects and tests.

### 2. Extract pure action planning

**Choice:** Control commands and startup options should resolve to explicit action/response plans before the shell mutates game state.

**Rationale:** MCP and startup behavior can be tested without a renderer, window, or network session.

### 3. Preserve compatibility vocabulary

**Choice:** Existing MCP response messages, capture queue behavior, sign-editor checks, pitch bounds, and connection status responses remain stable.

**Rationale:** Runner MCP checks consume those strings and outcomes.

## Risks / Trade-offs

- Window event handling touches many game fields; extract pure decisions before moving side-effect code.
- Some tests may need shared fixtures for `Game` state summaries rather than full `Game` construction.
