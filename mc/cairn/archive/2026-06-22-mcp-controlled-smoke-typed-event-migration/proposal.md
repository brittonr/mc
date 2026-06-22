# Proposal: Migrate MCP-controlled smoke to typed-event pass/fail

## Why

`mcp-controlled-smoke` is the maintained MCP-controlled observability row with stdio transcript, tool-call sequence, status/look/input evidence, and frame artifact identity. It remains waiver-backed substring fallback, so missing or misordered MCP control evidence could be obscured by legacy text checks.

Moving this row to typed-event-ready makes the MCP control sequence fail closed through structured events while preserving visual and semantic non-claims.

## What Changes

- Mark `mcp-controlled-smoke` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::McpControlledSmoke` or an equivalent MCP control event row.
- Add positive and negative fixtures for initialize, tools/list, status, look, bounded input, chat or command evidence, frame capture, frame artifact identity, forbidden stderr/stdout surfaces, and ordering.
- Add manifest readiness fixtures for the MCP-controlled smoke row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, MCP-controlled runner/checker code, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused MCP control event fixtures, scenario-manifest checks, generated-surface freshness, MCP dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded MCP-controlled observability row. It does not claim visual regression approval, semantic gameplay equivalence, broad MCP API coverage, public-server safety, production readiness, or screenshot-only correctness.
