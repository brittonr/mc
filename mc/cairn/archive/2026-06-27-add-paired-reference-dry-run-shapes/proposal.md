# Proposal: Add paired-reference dry-run shapes

## Why

The paired combat reference scenarios rely on reviewable Paper/Valence comparator receipts, but their scenario manifest entries still have no deterministic dry-run shape check. That is acceptable for the original live evidence, but it leaves generated scenario surfaces with an explicit gap and prevents ordinary dry-run validation from catching receipt-shape drift.

A bounded dry-run shape should cover the reference-comparator receipt contract without claiming new vanilla parity or replacing live paired receipts.

## What Changes

- Define deterministic dry-run receipt shapes for `vanilla-combat-reference-parity` and `vanilla-combat-armor-reference-parity`.
- Add a pure shape validator that checks required paired-reference fields, metric names, tolerance fields, backend labels, and non-claim text.
- Add positive and negative fixtures for missing Paper/reference fields, missing Valence fields, missing tolerance, wrong backend labels, and overbroad parity claims.
- Wire focused dry-run checks into the scenario manifest and generated indexes.
- Refresh docs/evidence so dry-run shape evidence remains distinct from live paired comparator evidence.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, runner dry-run shape code, generated scenario surfaces, reference parity checker or fixture tests, evidence docs/manifests, Cairn specs/tasks.
- **Testing**: pure dry-run shape fixture tests, scenario manifest checks, generated-surface freshness, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: deterministic dry-run shape coverage does not produce Paper or Valence live evidence, does not promote vanilla parity, and does not broaden combat, armor, projectile, CTF, public-server, or production-readiness claims.
