## Context

Status-effect packet coverage is distinct from combat damage and survival gameplay. The live proof should record packet observation and server correlation only for one configured effect transition.

## Goals / Non-Goals

Goals:
- Define a deterministic status-effect contract with one target entity, one effect, amplifier, duration, and optional remove step.
- Add or select an isolated rail that records client apply/remove observations and Valence server correlation.
- Promote only `entity-status-effect-packets` when checker-backed live evidence passes.

Non-goals:
- Proving all effects, stacking, particles/UI rendering, gameplay modifiers, combat balancing, survival parity, public-server safety, production readiness, or full protocol 763 compatibility.

## Design

1. Define pure contract data for target identity, effect id/name, amplifier, duration, packet rows, server correlation, and non-claims.
2. Add a rail or deterministic fixture that applies the configured effect and optionally removes it while preserving existing scenario semantics.
3. Normalize live evidence into the shared targeted-packet KV schema plus row-specific effect metrics.
4. Extend or reuse checker coverage so valid effect evidence passes and wrong entity/effect/duration, missing apply/remove correlation, stale digest, and broad effect/modifier overclaims fail closed.
5. Update matrix/current-bundle/packet-inventory docs only when evidence passes.

## Risks

- Client-side logs may not expose enough status-effect packet detail; if so, use a blocker rather than promoting the row.
- Effect application could accidentally alter combat or survival rows. Keep the rail isolated and name non-claims explicitly.

## Validation

- Run baseline targeted packet, matrix, and current-bundle checks before runner edits.
- Run focused runner tests or dry-runs for the status-effect rail.
- Run positive and negative targeted-packet live-evidence checker tests.
- Run evidence-manifest/task-evidence checks plus Cairn gates and validation.
