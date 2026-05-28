# Design: Projectile travel and collision proof

## State strategy

Represent projectile proof rows as ordered states: use, spawn, visible travel, collision/hit or miss, server attribution, and client consequence. A row can claim only the states it observes. Existing damage attribution can be reused as input, not as proof of travel.

## Correlation strategy

Correlate attacker, victim or target block, projectile sequence, entity id where available, server event, and client observation. If client-visible travel cannot be observed reliably, keep travel/collision as a non-claim and prove only server-side behavior.

## Variant strategy

Weapon variants should be representative rows with explicit item names and expected behavior. Bow, crossbow, and trident rows must not be generalized to all projectile weapons unless the matrix says so.

## Risks

- Stevenarella may not expose enough client telemetry for travel/collision. The change should stop at a server-only or infeasible checkpoint rather than overclaim.
- Timing/order evidence can be ambiguous. Require deterministic sequence markers where possible.
