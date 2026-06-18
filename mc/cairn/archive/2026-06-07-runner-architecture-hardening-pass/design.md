## Context

Scenario modules, live capability registry, targeted packet checkers, backend runtime traits, protocol I/O helpers, evidence matcher traits, and checker framework traits now exist. A hardening pass should pick one small remaining seam and prove behavior parity before more live rails are added.

## Goals / Non-Goals

Goals:
- Select one bounded runner or checker seam with duplicated branching or implicit string behavior.
- Extract pure deterministic decision logic that can be tested in memory.
- Keep CLI, filesystem, process, environment, clock, network, and receipt-writing side effects in thin shells.
- Preserve public scenario/checker/evidence output unless a separate Cairn expands it.

Non-goals:
- Adding new live evidence rows.
- Changing receipt schemas, accepted scenario names, milestone IDs, non-claim flags, backend names, or checker row semantics.
- Broad refactors that touch unrelated child repos.

## Design

1. Baseline current runner/checker behavior with focused tests and dry-runs.
2. Choose one small seam, such as a remaining ad hoc matcher branch, backend branch, checker parser duplicate, or targeted-packet row extension branch.
3. Define pure core inputs/outputs and named constants for non-obvious values.
4. Move side effects into shell code that only reads files, invokes processes, writes receipts, or formats diagnostics.
5. Add positive parity fixtures and negative fail-closed fixtures for unknown names, malformed inputs, missing evidence, stale revisions, and broad overclaims.
6. Record logs and manifests under `docs/evidence/`.

## Risks

- Refactor drift could change receipt-visible fields. Mitigate with before/after parity fixtures.
- The selected seam could be too broad. Keep the scope to one exemplar and open follow-up Cairns for additional migrations.

## Validation

- Run focused baseline tests before refactor.
- Run parity and negative tests after refactor.
- Run relevant runner dry-runs or checker self-tests.
- Run evidence-manifest/task-evidence checks plus Cairn gates and validation.
