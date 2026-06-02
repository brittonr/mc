# Proposal: Production readiness envelope gate

## Why

Production readiness remains a non-claim. Public-server, WAN, adversarial, telemetry, authorization, and load bounds must be complete before any production-ready wording is allowed.

## What Changes

- Add `production-readiness-envelope` as a row-scoped Cairn for an aggregate production-readiness gate requiring owned/public/WAN/adversarial safety rows, telemetry, authorization, redaction, abort criteria, and evidence manifests.
- Define normalized metrics: target scope, authorization, owner, client count, duration, perturbation settings, adversarial model, telemetry, abort criteria, redaction status, and row evidence paths.
- Require evidence standard: aggregate checker plus human/oracle checkpoints for public/adversarial rows and deterministic fail-closed fixtures.
- Reject bad evidence and overclaims: missing authorization, missing telemetry, missing oracle checkpoint, missing redaction, unbounded load, public target without approval, or production-readiness overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: production readiness until every envelope row passes, public third-party safety without authorization, unbounded load, WAN robustness, adversarial robustness, and security certification.
