# Design: MCP-controlled smoke typed-event migration

## Scope

The migration is limited to the existing `mcp-controlled-smoke` observability row. It changes the row from waiver-backed substring fallback to typed-event-ready pass/fail while keeping wrapper names, receipt fields, current-bundle row, frame artifact handling, and non-claims stable.

## Functional core

The pure validation core evaluates typed control events from in-memory MCP transcript and receipt evidence. This change adds a validator for the MCP-controlled smoke sequence:

- initialize precedes tools/list
- tools/list precedes status
- status precedes look
- look precedes bounded input or chat action
- bounded input or chat action precedes capture_latest_frame
- capture_latest_frame records an artifact identity with a BLAKE3 digest and reviewable path
- stdout/stderr cleanliness and forbidden surfaces remain explicit checks

Positive fixtures include a complete MCP control event graph. Negative fixtures remove the frame artifact identity and reorder capture before status to verify fail-closed diagnostics.

## Imperative shell

The MCP wrapper, transcript capture, receipt paths, and frame artifact writing remain unchanged. A thin shell converts the existing transcript/receipt evidence into typed control events and calls the pure validator.

## Validation strategy

- Record baseline MCP dry-run, runner, and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run MCP typed-event positive and negative fixtures.
- Run scenario manifest self-test/check/generated-surface checks.
- Run MCP dry-run wrapper check and evidence manifest check.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

The row remains bounded to one owned-local MCP-controlled observability run. It does not claim visual regression approval, semantic gameplay equivalence, broad MCP API behavior, screenshot-only correctness, public-server safety, or production readiness.
