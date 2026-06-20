# Proposal: Migrate mc-compat scenarios to typed-event readiness

## Why

Every current scenario manifest row still records the `substring-fallback` migration state. Substring matching is useful for legacy logs, but it is less precise than typed events and can mask renamed, duplicated, or accidentally broadened milestones. The harness needs a visible migration path where typed-event-ready rows become the default for maintained scenarios.

## What Changes

- Define typed-event readiness criteria for scenario milestones, forbidden patterns, receipt timelines, and fallback waivers.
- Add a manifest/checker gate that rejects new maintained scenarios using substring fallback unless an explicit waiver is present.
- Migrate scenarios to `typed-event-ready` where typed event receipts already prove client/server milestone coverage.
- Keep substring fallback as a reviewed compatibility path for historical or not-yet-migrated rows.

## Impact

- **Files**: scenario manifest, runner typed-event oracle code, scenario manifest checker, receipt tests, README/evidence docs, and Cairn artifacts.
- **Testing**: typed-event oracle positive/negative tests, manifest migration-state fixtures, affected dry-run receipt checks, evidence manifest checks, and Cairn gates/validation.
- **Non-claims**: this change strengthens harness observability and fail-closed matching; it does not by itself promote new protocol, gameplay, survival, combat, or public-server compatibility claims.
