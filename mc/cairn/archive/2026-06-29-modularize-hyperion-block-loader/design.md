# Design: Hyperion block loader modules

## Context

The Hyperion block loader is a runtime data boundary. The modularization should make parse and validation logic deterministic while leaving IO and runtime mutation in shells.

## Decisions

### 1. Keep ownership Hyperion-local

**Choice:** Treat this as Hyperion-owned block-loader work unless a separate Valence integration Cairn classifies a source for use.

**Rationale:** Parent planning must not imply copied Valence behavior.

### 2. Split parse, validation, and application

**Choice:** Extract pure parsing summaries, validation decisions, palette/section plans, and storage update plans.

**Rationale:** Block-loader correctness can be tested without runtime IO.

### 3. Preserve performance boundaries

**Choice:** Avoid introducing allocation-heavy abstractions in hot paths without measurement.

**Rationale:** Hyperion's block systems are performance-sensitive.
