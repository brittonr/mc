# Design: Add an optional anti-cheat statistics plugin

## Context

Hyperion's `hyperion-stats` README describes parallel running statistics for anti-cheat signals. The local crate uses nightly SIMD features, so Valence should start with a stable pure core and optional plugin wiring. Detection policy and moderation actions should remain outside the first integration.

## Decisions

### 1. Metrics before enforcement

**Choice:** The plugin records and exposes metrics or observations. It does not kick, ban, or mutate player state by default.

**Rationale:** Statistical signals need tuning and false-positive review.

### 2. Stable pure core first

**Choice:** Implement rolling/counting statistics with stable Rust first. SIMD or unsafe optimizations require a later audit.

**Rationale:** Correctness and portability matter more than optimization at the first integration step.

### 3. Explicit sampling inputs

**Choice:** Adapters opt into concrete event streams and sample windows; the core receives explicit samples and clock/tick values from the shell.

**Rationale:** Metrics should be deterministic in tests and not depend on implicit global time.

### 4. Document data retention

**Choice:** The plugin must document what per-player metrics are retained and for how long, even if no persistent storage is added.

**Rationale:** Anti-cheat telemetry can become sensitive operational data.

## Risks / Trade-offs

- Metrics can be misused as enforcement; mitigate with no default actions.
- Stable scalar code may be slower than Hyperion's SIMD path; optimize only after benchmarks and audit.
- False positives are likely without gameplay context; docs must explain limits.
