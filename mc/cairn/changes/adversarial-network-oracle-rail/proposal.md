# Proposal: Adversarial network oracle rail

## Why

Adversarial-network safety remains an oracle-required non-claim. A Cairn should define the approval, threat model, mutation bounds, telemetry, and abort criteria before any adversarial test is attempted.

## What Changes

- Add a bounded `adversarial-network-oracle` row for one explicitly approved adversarial-network model with bounded packet mutation, target ownership, telemetry, and human/oracle decision record.
- Define normalized metrics: threat model id, mutation types, packet bounds, target ownership, authorization, telemetry, abort criteria, observed containment, and oracle decision.
- Require evidence standard: human/oracle checkpoint plus deterministic evidence; no live adversarial claim without approval and bounded model.
- Add fixture/runner/checker work: test harness simulates or applies bounded packet mutation only after approval, then records containment and abort metrics.
- Reject overclaims and bad evidence: missing oracle approval, missing threat model, missing target ownership, unbounded mutation, missing telemetry, or security overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: production/network safety.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: general malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded adversarial robustness, and full protocol security.
