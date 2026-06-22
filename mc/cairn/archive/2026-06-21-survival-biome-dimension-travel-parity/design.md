# Design: Survival biome and dimension travel parity

## Context

The promoted biome/dimension row records one overworld join-state observation. This change defines a separate travel row for one controlled transition.

## Decisions

### 1. Bound the transition path

**Choice:** The row names one starting environment, one trigger action, one target environment, and one post-transition stabilization observation.

**Rationale:** Dimension mechanics are broad; one transition is reviewable and comparable.

### 2. Compare normalized environment metrics

**Choice:** Records compare environment identifiers, transition state, position bounds, client update, and server fixture state.

**Rationale:** These fields are enough to prove the bounded transition without claiming worldgen parity.

### 3. Keep worldgen and all portals separate

**Choice:** All biomes, all dimensions, portal breadth, Nether/End behavior breadth, and world generation remain non-claims.

**Rationale:** Each needs separate fixtures and acceptance criteria.
