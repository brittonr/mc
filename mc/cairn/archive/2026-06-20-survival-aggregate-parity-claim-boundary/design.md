# Design: Survival aggregate parity claim boundary

## Context

The current `mc-compat-full-survival-gate` proves required row slots are present, but docs still separate row-scoped reference parity from aggregate survival parity. This change formalizes that boundary.

## Decisions

### 1. Separate row coverage from aggregate parity

**Choice:** The aggregate gate requires both all named rows and explicit aggregate evidence docs before any full survival claim can pass.

**Rationale:** Row coverage is necessary but not sufficient for broad semantic parity.

### 2. Fail closed on broad wording

**Choice:** The checker rejects phrases that claim full survival compatibility or broad vanilla parity unless aggregate evidence mode is explicitly enabled by a passing bundle.

**Rationale:** Documentation drift is the highest risk when many rows are covered.

### 3. Treat breadth packages as prerequisites

**Choice:** The gate names survival breadth families as pending prerequisites until their Cairns produce receipts and matrix rows.

**Rationale:** The active queue should show why aggregate parity remains blocked.
