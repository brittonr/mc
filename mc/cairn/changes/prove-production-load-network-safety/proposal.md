# Proposal: Production load and network safety proof

## Summary

Create a proof package for production/load/network claims that are explicitly outside the current bounded local compatibility receipts.

## Motivation

Existing Valence/Stevenarella receipts are owned-local, bounded, and scenario-specific. They do not prove public-server safety, production load behavior, unbounded soak, unbounded reconnect, WAN latency, adversarial network behavior, or packet-loss tolerance beyond recorded fixtures. These claims need a separate authorization and safety envelope.

## Scope

- Define an owned-target authorization and safety policy for load/network experiments.
- Define load, soak, reconnect, latency, jitter, and packet-loss envelopes with explicit upper bounds.
- Add deterministic dry-run checks and bounded live receipts for each promoted envelope.
- Require fail-closed behavior for missing authorization, public targets, unbounded parameters, or missing telemetry.

## Out of scope

- Any public-server stress test without explicit authorization.
- Treating current local receipts as production readiness evidence.
- Broad gameplay correctness.
