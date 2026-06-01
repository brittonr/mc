# Proposal: Protocol command recipe advancement family coverage rail

## Why

Command, recipe, and advancement packet families remain mostly non-claims. Existing high-risk raw parser fixtures are narrow; a family rail can promote additional reviewed rows only when semantic fixtures and live receipts exist.

## What Changes

- Add a bounded `protocol-command-recipe-advancement-family` row for selected command, recipe, or advancement packet rows with reviewed mapping/parser fixtures and bounded live evidence for the selected feature.
- Define normalized metrics: packet family, wire id, semantic fixture id, parser fixture result, malformed fixture status, live scenario feature, receipt path, and digest.
- Require evidence standard: protocol ledger rows require no fallback alias, parser fixture, live feature receipt, owner, next action, and explicit semantic non-claims for raw paths.
- Add fixture/runner/checker work: protocol fixtures cover selected command/recipe/advancement payloads and live fixture exercises corresponding command tree, recipe visibility, or advancement update observation.
- Reject overclaims and bad evidence: raw-only semantic overclaim, fallback alias, missing parser fixture, missing live feature receipt, malformed acceptance without oracle, or all-feature claim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: broad protocol coverage.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all commands, all recipes, all advancements, recipe-book semantics, command execution semantics, full protocol-763 compatibility, and production readiness.
