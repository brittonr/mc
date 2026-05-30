# Proposal: Public server authorized safety rail

## Why

Public-server safety is fail-closed today. Any broader claim requires explicit authorization, owner, bounds, telemetry, and a review checkpoint before traffic.

## What Changes

- Add a bounded `public-server-authorized-safety` row for one explicitly authorized public or non-loopback target envelope with owner, written authorization reference, bounds, telemetry, and abort criteria.
- Define normalized metrics: target owner, authorization artifact, target scope, client count, duration, traffic limits, telemetry, abort criteria, redaction policy, and human checkpoint decision.
- Require evidence standard: human/oracle authorization checkpoint before live run plus deterministic receipt checks that reject missing fields.
- Add fixture/runner/checker work: preflight refuses public targets unless authorization and bounds fields are present; live rail records telemetry and redacted evidence only.
- Reject overclaims and bad evidence: missing owner, missing written authorization, missing bounds, missing telemetry, missing checkpoint, secrets in logs, or production readiness overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: production/network safety.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: third-party target safety without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing.
