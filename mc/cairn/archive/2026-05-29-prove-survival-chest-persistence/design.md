# Design: Survival chest persistence row

## Context

`break/place/pickup` already has paired Paper and Valence reference parity evidence. The next missing survival row with high value and manageable scope is `chest persistence`: a client stores an item in a chest, closes/reconnects/reopens, and observes that the same stack remains in the expected container slot.

## Decisions

### 1. Keep the claim row-scoped

**Choice:** Claim only one deterministic chest interaction: one chest block, one item stack, one chest slot, one close/reconnect/reopen sequence, and exact normalized metrics.

**Rationale:** This proves a useful survival storage seam without implying all containers, all items, server restart persistence, or broad vanilla parity.

### 2. Require paired Paper and Valence receipts

**Choice:** Use the Paper fixture plugin as the reference backend and the Valence `survival_compat` fixture as the implementation backend. Both receipts must be copied under `docs/evidence/` with matching client/server logs.

**Rationale:** The existing survival parity proof established this paired-reference pattern. Chest persistence should not regress to Valence-only evidence.

### 3. Normalize exact metrics before promotion

**Choice:** A checker must compare metrics such as chest open, store click, close, reconnect/reopen, persisted slot item, persisted count, and server-side chest-state milestones.

**Rationale:** Text logs alone are too easy to overclaim. The row should fail closed when required evidence is absent, stale, or mismatched.

### 4. Treat restart/world persistence as separate scope

**Choice:** A reconnect/reopen cycle is required for this row, but server restart/reload persistence remains part of the existing `world persistence` missing row.

**Rationale:** Restart persistence needs a different fixture lifecycle and should not be smuggled into the chest row.

## Risks / Trade-offs

- Paper container interaction may require extra Stevenarella packet support beyond the existing inventory/container probe.
- Valence may need explicit fixture state for the chest slot and reconnect observation.
- The row name includes persistence, so docs must clearly distinguish reconnect/session persistence from server restart/world persistence.
