# Design: Introduce scenario behavior traits

## Context

Scenario behavior is mostly data, but a few scenarios need special hooks such as dynamic projectile health needles, multi-client expectations, MCP control, restart persistence, or probe environment setup. The design should avoid a trait object per row while still providing a single review surface.

## Decisions

### 1. Keep `Scenario` as the stable identity

**Choice:** Preserve the enum for CLI/config parsing, receipt serialization, equality checks, and manifest references.

**Rationale:** Scenario names are public compatibility contract inputs. Refactoring internals must not rename or remove them.

### 2. Use static `ScenarioSpec` rows for simple behavior

**Choice:** Represent simple names, aliases, client milestones, server milestones, and forbidden patterns as static data keyed by `Scenario`.

**Rationale:** Most scenario behavior is declarative; static data is easier to audit than repeated matches.

### 3. Use `ScenarioBehavior` only for exceptional hooks

**Choice:** Provide trait methods for optional dynamic matcher inputs, probe environment setup, derived log enrichment, and multi-client behavior. Default methods do nothing.

**Rationale:** Trait abstraction is useful where behavior varies; pure data remains pure data.

### 4. Require full parity coverage before migration

**Choice:** Add tests that enumerate all existing scenarios and compare parse aliases, canonical names, required milestones, forbidden patterns, and special hooks to fixtures.

**Rationale:** This change is structural. Any scenario behavior drift must be caught before archive.

## Risks / Trade-offs

- Over-abstracting scenario rows can hide the matrix of compatibility claims; keep specs static and reviewable.
- Static scenario specs may need generated-manifest alignment if the manifest remains generated elsewhere.
- Exceptional behavior must remain explicit rather than embedded in string-name conditionals.