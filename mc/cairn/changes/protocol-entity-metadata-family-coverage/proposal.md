# Proposal: Protocol entity metadata family coverage rail

## Why

Broad protocol coverage remains blocked because entity metadata variants are only scenario-observed. A packet-family Cairn can add mapping/parser fixtures and live receipts for a bounded metadata subset.

## What Changes

- Add a bounded `protocol-entity-metadata-family` row for a named subset of entity metadata packet shapes with reviewed Stevenarella mapping/parser fixtures and one live scenario receipt touching those shapes.
- Define normalized metrics: wire id, Valence packet name, Stevenarella semantic, parser fixture id, positive payload fixture, malformed rejection fixture where semantic decoding exists, and live receipt evidence path.
- Require evidence standard: protocol ledger row with reviewed mapping, parser-shape fixture, non-fallback status, live receipt, owner, next action, and current-bundle digest.
- Add fixture/runner/checker work: protocol tests encode/decode representative entity metadata payloads and live fixture emits corresponding entity metadata observations.
- Reject overclaims and bad evidence: fallback alias, missing parser fixture, malformed acceptance without oracle, missing live receipt, missing owner/next action, or all-metadata overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: broad protocol coverage.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all entity metadata variants, all entity types, full protocol-763 compatibility, full Minecraft compatibility, and production readiness.
