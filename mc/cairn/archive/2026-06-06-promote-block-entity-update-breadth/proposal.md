# Proposal: Promote one additional block-entity update breadth row

## Why

The sign block-entity row is valuable but should not stand in for all block entities. A second bounded non-sign block entity, such as a chest or skull update, would make the block-entity packet-family evidence less sign-specific while still avoiding broad NBT claims.

## What Changes

- Add one bounded non-sign block-entity update row with a configured block entity kind, position, and payload metric.
- Require client observation and server fixture correlation for that exact payload.
- Promote only the configured non-sign block-entity update row, keeping all block entities, arbitrary NBT, persistence breadth, full protocol coverage, and production readiness as non-claims.

## Impact

- **Files**: Valence/Paper or fixture instrumentation, Stevenarella observation code if needed, runner metadata, checker, evidence docs/manifests, packet inventory/current bundle, and Cairn specs/tasks.
- **Testing**: checker positive/negative fixtures, focused fixture tests, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
