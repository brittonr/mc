# Proposal: Prove production and network safety

## Summary

Define the evidence needed before local compatibility rails can claim production readiness, public-server safety, WAN tolerance, or adversarial-network safety.

## Motivation

Current receipts are owned-local, bounded, and loopback-only. They intentionally do not prove production load, public target safety, WAN behavior, or adversarial network resilience. These claims need separate authorization, load bounds, telemetry, and failure-mode evidence.

## Scope

- Add an explicit production/network safety matrix.
- Define authorization and target-scope requirements.
- Add bounded load and WAN/jitter/loss evidence requirements.
- Add adversarial-network negative tests or oracle checkpoints.
- Keep current local rails as non-production evidence until the matrix passes.

## Non-goals

- No stress against public servers.
- No unbounded load tests.
- No production claim from existing loopback receipts alone.
