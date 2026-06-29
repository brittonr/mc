# Design: Composable runner config patches

## Context

The runner already has test seams for `Config::from_sources`, but the implementation still mutates a fully constructed `Config` while parsing each source. This change separates source parsing, patch composition, final validation, and side-effecting file/environment access.

## Decisions

### 1. Model partial updates explicitly

**Choice:** Add a `ConfigPatch` type with optional fields for every configurable value and a `ConfigSource` label that records where the patch came from.

**Rationale:** A patch can be compared, composed, and tested without constructing a `Command`, reading environment variables, or touching the filesystem.

### 2. Preserve existing source order as data

**Choice:** Build an ordered list of config sources that represents the current precedence rules, then fold patches into defaults through a pure merge function.

**Rationale:** Precedence becomes reviewable in one place and future config sources can be inserted without interleaving mutation with parsing.

### 3. Keep shell code thin

**Choice:** File reads, Steel evaluation, environment lookup, and CLI argument collection remain shell responsibilities. The core receives raw source text or key/value inputs and returns patches plus diagnostics.

**Rationale:** Tests should exercise precedence and validation with in-memory fixtures.

### 4. Validate after resolution

**Choice:** Cross-field checks such as unsafe paths, invalid mode combinations, missing option values, and safety bounds run after patch application, with source-aware diagnostics where possible.

**Rationale:** Validation should see the same final configuration the runner will execute, while still explaining which source introduced the problem.

## Risks / Trade-offs

- The first extraction may introduce adapter functions while call sites migrate; remove adapters before archive when they no longer protect parity.
- Some CLI flags trigger mode-specific behavior while parsing today; preserve those effects with explicit patch semantics before reshaping syntax.
- Error message wording should remain stable where tests or docs depend on it; any unavoidable diagnostic change needs focused fixture updates.
