# Design: Separate gameplay config sources from plugin behavior

## Context

Current gameplay examples use process environment reads as a quick fixture-control mechanism. That couples gameplay systems to process-global state and makes config refresh happen as hidden I/O during ordinary schedules. Composable plugins need typed config values whose source is explicit and whose ownership can be global, defaulted, or arena-scoped.

## Decisions

### 1. Treat config parsing as a pure core

**Choice:** Parse and validate config from explicit input values into typed config structs without reading environment variables, files, clocks, or ECS state.

**Rationale:** Tests should exercise valid, missing, malformed, and boundary config without mutating the process environment.

### 2. Move source reads into adapter plugins or startup shells

**Choice:** Env/CLI/file-backed configuration, if used, is read by a source adapter that writes typed config resources or arena components. Gameplay plugins consume those typed values only.

**Rationale:** This preserves fixture convenience while making side effects visible and replaceable.

### 3. Make reload explicit

**Choice:** Runtime config reloads occur through documented events, source plugin systems, or explicit test helpers instead of every gameplay phase reading process state.

**Rationale:** Hidden process-state reads make multi-arena tests flaky and complicate ordering.

### 4. Preserve compatibility toggles through adapters

**Choice:** Existing env names and CLI flags can remain as source-adapter contracts while gameplay systems stop reading them directly.

**Rationale:** Compatibility receipts and manual example workflows should remain comparable.

## Risks / Trade-offs

- Splitting config source from gameplay behavior may add a small amount of boilerplate to examples.
- Existing tests that rely on environment mutation must migrate to pure config values or scoped source fixtures.
- If reload semantics are too broad, config changes can still affect unrelated arenas; reload events must carry scope where needed.
