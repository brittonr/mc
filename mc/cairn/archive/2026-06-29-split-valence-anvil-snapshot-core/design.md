# Design: Valence Anvil snapshot split

## Context

The snapshot module is a format boundary and a filesystem boundary. The split should make pure region/chunk decisions testable while preserving existing public behavior.

## Decisions

### 1. Split model, planning, and shell

**Choice:** Create owners for snapshot model types, region/chunk lookup planning, parse validation, cache policy, directory/filesystem shell, and Bevy integration.

**Rationale:** Format decisions and side effects have different failure modes.

### 2. Extract pure lookup and validation

**Choice:** Region coordinate calculation, chunk selection, missing/corrupt classification, and snapshot update planning should be pure over explicit inputs.

**Rationale:** Boundary behavior can be tested without region files.

### 3. Preserve public APIs first

**Choice:** Existing constructors, load behavior, missing-region behavior, and Bevy integration remain stable through adapters if modules move.

**Rationale:** This is not a format expansion.
