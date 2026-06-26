# Design: Add event-loop phase SystemSets

## Context

The event loop is Valence's bridge from network packets to ECS gameplay. Raw `PacketEvent` remains a low-level access point, while typed events are increasingly useful for gameplay semantics. Named phase sets can make this bridge explicit without removing raw access.

## Decisions

### 1. Inventory current event-loop order

**Choice:** Record schedule labels, raw packet event production, typed adapter systems, domain consumers, diagnostics, and cleanup systems before adding sets.

**Rationale:** Event-loop ordering is compatibility-sensitive.

### 2. Phase sets describe ownership

**Choice:** Define sets for raw packet observation, typed adapter emission, domain consumption, diagnostics, and cleanup only where they have concrete systems.

**Rationale:** Sets should reflect ordering ownership, not speculative architecture.

### 3. Raw access remains available

**Choice:** `PacketEvent` stays readable according to the existing event-loop contract; typed adapters document what semantics they own.

**Rationale:** Valence promises direct protocol access for low-level users.

### 4. Ambiguity is tested

**Choice:** Schedule tests cover missing sets, ambiguous ordering, duplicate adapter emission, and disabled plugin configurations.

**Rationale:** Phase sets are useful only if regressions fail clearly.

## Risks / Trade-offs

- Adding sets can accidentally imply stronger ordering guarantees than intended; document private boundaries.
- Moving systems between sets may affect typed event timing; focused tests and schedule hygiene are required.
- Too many phase sets can make the schedule noisy; start with concrete event-loop ownership phases.
