## Why

The targeted packet checker now has a live-evidence mode, but future packet rows need a shared KV schema so live promotions fail closed consistently across packets. A schema-focused Cairn reduces duplicated ad hoc keys before more live rails are added.

## What Changes

- Define a normalized targeted packet live evidence KV schema for row id, packet row, live status, scenario, backend/client path, receipt path/digest/currentness, revision metadata, row-specific metrics, and non-claims.
- Add a pure schema validator or checker helper that live packet rows can reuse.
- Add positive and negative fixtures for missing required keys, stale digests, wrong packet rows, weak revision metadata, and broad overclaims.
- Document migration guidance for existing and future targeted packet live rails.

## Impact

- **Files**: `tools/check_targeted_packet_promotions.rs` or shared checker modules, `docs/evidence/**`, `README.md` or evidence workflow docs if needed, Cairn specs/tasks.
- **Testing**: Checker self-tests, targeted packet checks, evidence-manifest/task-evidence checks, Cairn gates and validation.
