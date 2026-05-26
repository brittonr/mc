# Design: Reconnect while holding or touching flag compatibility rail

## Context

This package comes from the 2026-05-25 ROI ranking after protocol-763 Valence CTF evidence had landed through scoring, BLUE/RED soaks, inventory/drop/pickup/block-place/click/open-container, and two-client combat/damage. The goal is to make the next compatibility claims receipt-backed without repeating saturated evidence.

## Decisions

### 1. Reuse continuous-server reconnect shape

**Choice:** Build on the existing reconnect scenario instead of creating an unrelated harness.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 2. Bind CTF state explicitly

**Choice:** Require flag-owner/reset/score non-claim fields so the receipt proves state coherence rather than just relogin.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 3. Keep usernames deterministic

**Choice:** Use Minecraft-safe usernames and fixed session ordering for server/client correlation.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

## Risks / Trade-offs

- Disconnect timing can leave ambiguous server state unless milestones are explicit.
- Existing Valence CTF may not log enough flag owner/reset information without narrow instrumentation.
