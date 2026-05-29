# Design: Negative live rails

## Context

The current evidence set intentionally avoids claiming many invalid-path semantics. For example, inventory docs state that stale state-id rejection and malformed clicks are not live-proven. A negative rail should prove one exact invalid action and observed containment/recovery, not broad adversarial safety.

## Decisions

### 1. Fail-closed injection envelope

**Choice:** Add a shared negative-rail envelope that records target ownership, authorization, action bounds, expected rejection/restoration signal, and abort criteria before any invalid action is sent.

**Rationale:** Negative rails should not become fuzzers or public-server stress tools.

### 2. One invalid behavior per row

**Choice:** Each negative rail claims one precise invalid behavior with exact precondition, injected action, expected server response, and postcondition.

**Rationale:** Reviewable negative evidence needs tighter scope than happy-path smoke evidence.

### 3. Reference parity when fidelity is claimed

**Choice:** If the row claims vanilla-like rejection, require paired Paper/reference and Valence evidence. If it only claims Valence containment, keep vanilla parity false.

**Rationale:** Negative behavior can differ by implementation; the receipt must not overclaim.

### 4. Typed events preferred

**Choice:** Negative rails should emit typed events when the typed event oracle is available, but may start with text milestones plus deterministic checkers.

**Rationale:** This change can proceed before the full typed-event migration, but should align with it.

## Risks / Trade-offs

- Stevenarella may need low-level packet/action hooks that are unsafe if reused outside local fixtures.
- Some invalid paths may disconnect the client; the receipt must distinguish expected disconnect from crash.
- Paper reference fixtures may not expose every server-side rejection signal without plugin instrumentation.
