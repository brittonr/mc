# Design: Expose plugin diagnostics registry

## Context

Hyperion gameplay composition has default groups, mode-only plugins, preset planning, feature disables, replacements, custom plugin intents, and dependency checks. Tests can inspect specific resources or plugin flags, but there is no single structured registry that explains composition decisions.

## Decisions

### 1. Registry records decisions, not live behavior claims

**Choice:** The registry records composition-time facts: selected mode, included default gameplay, installed/disabled/replaced features, custom slots, dependency reasons, and known contract metadata.

**Rationale:** Diagnostics should explain how the app was built without claiming runtime correctness or full gameplay parity.

### 2. Builders populate complete diagnostics

**Choice:** Fallible preset/app builders populate diagnostics from the validated plan and shell install results. Direct plugin paths may populate best-effort local metadata where practical.

**Rationale:** Builders have the complete plan; direct Bevy plugin additions are less structured and should not imply full composition provenance.

### 3. Diagnostics are test- and receipt-friendly

**Choice:** Provide deterministic inspection helpers and optional receipt rendering that require no live server/proxy startup.

**Rationale:** Cairn evidence and unit tests need stable metadata quickly.

### 4. Stale diagnostics fail tests

**Choice:** Tests compare registry entries against installed plugin flags/resources and planner outputs for selected cases.

**Rationale:** Diagnostics must not become stale documentation.

## Risks / Trade-offs

- Direct plugin additions may not have enough context to record why a plugin was installed; mark provenance as direct/best-effort.
- Overly detailed diagnostics could freeze private internals; keep default fields at phase/feature/contract level.
- Registry population must not create hidden plugin dependencies or side effects.
