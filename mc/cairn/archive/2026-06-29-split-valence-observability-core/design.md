# Design: Valence observability core split

## Context

Observability code bridges deterministic classification and side-effecting Bevy/export behavior. The split should preserve schedule and metric behavior while making redaction and classification directly testable.

## Decisions

### 1. Split taxonomy, classification, and shells

**Choice:** Create owners for config, metric taxonomy, label/redaction handling, packet classification, export planning, and Bevy event systems.

**Rationale:** These concerns have distinct invariants and evidence risks.

### 2. Keep redaction pure

**Choice:** Label/redaction decisions and packet ID classifications should be pure over explicit inputs.

**Rationale:** Safety behavior must be easy to test and review.

### 3. Preserve schedule behavior

**Choice:** Bevy plugin schedule wiring remains equivalent unless another Cairn changes schedule contracts.

**Rationale:** Observability phase events are schedule-sensitive.
