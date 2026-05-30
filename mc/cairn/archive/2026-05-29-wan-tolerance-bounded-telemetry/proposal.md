# Proposal: WAN tolerance bounded telemetry rail

## Why

WAN tolerance and packet-loss tolerance remain non-claims. A bounded local perturbation rail with telemetry is needed before any WAN-like tolerance can be promoted.

## What Changes

- Add a bounded `wan-tolerance-bounded-telemetry` row for one authorized owned-local perturbation envelope with configured delay, jitter, packet loss, timeout, duration, client count, and telemetry.
- Define normalized metrics: target ownership, authorization, delay, jitter, loss, timeout, duration, client count, reconnect count, telemetry samples, pass/fail criteria, and abort reason.
- Require evidence standard: fail-closed preflight plus live telemetry receipt and human/oracle checkpoint if tooling or target scope changes.
- Add fixture/runner/checker work: runner applies approved local perturbation tooling or fails closed before traffic, then records telemetry and scenario milestones.
- Reject overclaims and bad evidence: missing authorization, missing perturbation parameters, unavailable tooling without fail-closed receipt, missing telemetry, public target, or production readiness overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: production/network safety.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: public-server safety, internet-path safety, adversarial network safety, production readiness, unbounded soak/reconnect safety, and third-party target safety.
