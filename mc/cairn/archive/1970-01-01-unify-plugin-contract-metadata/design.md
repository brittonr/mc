# Design: Unify plugin contract metadata

## Context

Valence has `GameplayPluginContract` and `GameplayPluginContracts` in example-local helper code. Hyperion has `GameplayFeatureInventory`, `DefaultGameplayPlugins`, preset planning metadata, and mode/plugin tests. These surfaces answer similar questions but differ in field names, visibility, dependency representation, and non-claim recording.

## Decisions

### 1. Define a vocabulary, not a shared dependency

**Choice:** Align on a minimal contract vocabulary and provide engine-local adapter types rather than introducing a hard Valence↔Hyperion crate dependency.

**Rationale:** The parent workspace intentionally keeps component boundaries independent. Metadata should be comparable without coupling release units.

### 2. Keep contract metadata explicit and inspectable

**Choice:** Each contract entry records plugin id, install mode, scope model, schedule labels, phase order, dependencies, owned resources, owned events, compatibility boundaries, and non-claims.

**Rationale:** These fields are the stable review surface for plugin composition and evidence.

### 3. Treat stale metadata as a test failure

**Choice:** Contract tests should compare declared metadata with installed resources/events/schedules where practical, and should fail when plugin metadata is absent or stale.

**Rationale:** Metadata that drifts from actual wiring is worse than no metadata.

### 4. Preserve private implementation details

**Choice:** Contracts expose phase-level and ownership-level facts, while private subphases and internal system ordering stay private unless deliberately promoted.

**Rationale:** The goal is composability and reviewability without freezing every implementation detail.

## Risks / Trade-offs

- Some resources/events are type-level and may need string identifiers in metadata; tests should validate representative installed facts rather than every private type.
- Cross-engine vocabulary can become too broad; keep fields minimal and add optional extension fields only when evidence requires them.
- Existing accepted specs already cover Valence gameplay contracts; this change should refine and align rather than churn prior scope.
