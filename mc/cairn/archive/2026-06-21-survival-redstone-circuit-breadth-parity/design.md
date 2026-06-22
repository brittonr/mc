# Design: Survival redstone circuit breadth parity

## Context

The promoted `survival-redstone-toggle` row covers one lever and lamp. This change defines a separate row for one configured circuit path with propagation checkpoints.

## Decisions

### 1. Use one finite circuit fixture

**Choice:** The row names one input, one dust/repeater path, one output, and one stateful/mechanical component when configured.

**Rationale:** Redstone breadth is large; a finite circuit keeps evidence reviewable.

### 2. Normalize tick checkpoints

**Choice:** The checker compares named powered-state checkpoints and bounded tick sequence labels rather than open-ended timing.

**Rationale:** The row needs enough ordering evidence to detect mismatches without claiming all tick-order semantics.

### 3. Preserve broad redstone non-claims

**Choice:** General circuits, clocks, farms, and component breadth remain out of scope.

**Rationale:** Each component family should get its own fixture if promoted later.
