# Design: Survival hunger and health-cycle parity

## Context

The promoted `survival-hunger-food` row covers one Bread consumption. This change defines a separate finite health-cycle row for exhaustion, regeneration, and starvation-adjacent behavior.

## Decisions

### 1. Use controlled fixture state

**Choice:** The fixture sets explicit starting health, food, saturation, exhaustion trigger, and tick checkpoints.

**Rationale:** Deterministic state is required to compare health-cycle behavior across Paper and Valence.

### 2. Compare state transitions, not wall-clock timing

**Choice:** The checker compares named checkpoints and normalized food/health/saturation values rather than absolute runtime duration.

**Rationale:** Checkpoints are stable evidence; timing breadth needs separate tolerance work.

### 3. Preserve broad hunger non-claims

**Choice:** All foods, all exhaustion sources, potion effects, and natural gameplay breadth remain non-claims.

**Rationale:** The row proves one controlled cycle only.
