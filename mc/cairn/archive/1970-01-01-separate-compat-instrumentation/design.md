# Design: Separate compatibility instrumentation from core client/server code

## Context

The compatibility harness depends on targeted client/server observations and probes. Those hooks should be explicit, opt-in, and testable, so normal component logic remains understandable and scenario instrumentation remains reviewable.

## Decisions

### 1. Inventory instrumentation first

**Choice:** List each client probe, capture hook, MCP surface, server fixture milestone, and scenario-specific env toggle before moving code.

**Rationale:** Evidence rails depend on these names and behaviors; migration must avoid accidental evidence loss.

### 2. Prefer opt-in modules or features

**Choice:** Compat-only code should live in named modules/features or harness-only entrypoints, with core logic calling pure interfaces where necessary.

**Rationale:** Explicit boundaries make it clear what is evidence instrumentation versus product behavior.

### 3. Keep event names stable or migrate with receipts

**Choice:** Typed-event/milestone names should remain stable unless changed with fixture updates and evidence checker migration.

**Rationale:** Evidence gates rely on exact event vocabulary.

### 4. Test both enabled and disabled states

**Choice:** Add positive tests showing instrumentation emits required events when enabled and negative tests showing disabled paths do not trigger harness-only behavior.

**Rationale:** Instrumentation boundaries need both sides covered.

## Risks / Trade-offs

- Feature gates can complicate build matrices; mitigate with one documented compat feature or harness profile.
- Moving hooks can break evidence checks; mitigate with event parity tests.
- Over-isolating instrumentation can duplicate protocol logic; mitigate by sharing pure event/probe planning code.
