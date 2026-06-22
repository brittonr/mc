# mc-compatibility Change Spec: MCP-controlled smoke typed-event migration

## Requirements

### Requirement: MCP-controlled smoke typed-event readiness

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.readiness] The `mcp-controlled-smoke` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required MCP control sequence, frame artifact identity, forbidden surfaces, and explicit non-claims.

#### Scenario: MCP-controlled smoke row is typed-event-ready

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.readiness.complete]
- GIVEN `mcp-controlled-smoke` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, frame artifact policy, and non-claim scope.

### Requirement: MCP-controlled smoke typed-event gate

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.gate] The runner MUST include MCP-controlled smoke evidence in a typed-event pass/fail gate so missing or invalid structured MCP control events fail before substring fallback can satisfy the row.

#### Scenario: Missing MCP typed evidence fails closed

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.gate.missing]
- GIVEN an MCP-controlled smoke fixture contains legacy substring-compatible transcript text but omits a required frame artifact identity event
- WHEN typed-event validation evaluates `mcp-controlled-smoke`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered MCP control phases fail closed

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.gate.order]
- GIVEN an MCP-controlled smoke fixture contains all required typed control events but puts frame capture before status observation
- WHEN typed-event validation evaluates `mcp-controlled-smoke`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: MCP-controlled smoke migration evidence

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.validation] The migration MUST record reviewable evidence for MCP fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.validation.log]
- GIVEN the MCP-controlled smoke row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the MCP dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
