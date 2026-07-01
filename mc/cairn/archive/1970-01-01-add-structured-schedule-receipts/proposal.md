# Proposal: Add structured schedule receipts

## Why

Valence schedule tests currently rely on DOT graph strings for many plugin-set and system assertions. That catches regressions, but the evidence is brittle, hard to diff, and not ideal for review-critical Cairn receipts. We need a deterministic structured schedule receipt surface that records the schedule facts reviewers care about without making large graph dumps mandatory.

## What Changes

- Define a Valence schedule receipt schema for selected schedules, plugin configuration, expected sets/systems, absent facts, and ambiguity settings.
- Keep Bevy schedule inspection in the shell and normalize facts through a small deterministic core.
- Update selected schedule hygiene tests to emit or validate structured receipts alongside existing focused assertions.
- Preserve DOT graph dumps as optional diagnostic evidence, not the primary review contract.

## Impact

- **Files**: Valence schedule test helpers or tools, selected core plugin schedule tests, docs/evidence receipt examples, and Cairn evidence manifests.
- **Testing**: positive valid-schedule receipt tests, negative unknown schedule/missing set/unexpected system tests, receipt determinism checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this does not require every Bevy schedule to be serialized or every private system to become a public ordering contract.
