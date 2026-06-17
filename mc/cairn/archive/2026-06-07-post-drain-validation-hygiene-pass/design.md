## Context

The queue drain completed several targeted packet live-rail and schema changes. The next implementation should start from a clean baseline so later evidence can be attributed to the correct change package.

## Goals / Non-Goals

Goals:
- Produce a reviewable hygiene baseline for accepted specs, active changes, evidence manifests, task-evidence paths, policy compatibility, and matrix/current-bundle consistency.
- Keep remediation deterministic and limited to metadata drift, stale manifests, or broken validation receipts.
- Record explicit blockers rather than silently promoting compatibility coverage.

Non-goals:
- Adding new live rails or scenario behavior.
- Promoting any packet, gameplay, survival, CTF, protocol, public-server, production-readiness, or semantic-equivalence claim.
- Reworking checker or runner architecture unless a gate failure requires a minimal deterministic fix.

## Design

1. Run the repo-pinned Cairn validation, proposal/design/tasks gates for active packages, evidence-manifest checks, and task-evidence checks.
2. Classify diagnostics into deterministic metadata drift, evidence freshness drift, task citation drift, policy/schema drift, or implementation defects.
3. Apply the smallest metadata-only repair when the source of truth is unambiguous, such as refreshing BLAKE3 rows for tracked evidence files.
4. If a diagnostic implies behavior or checker semantics need to change, record a blocker and open a follow-on Cairn instead of broadening this hygiene pass.
5. Promote all review-critical run logs and manifests under `docs/evidence/` with explicit `exit_status=0` lines.

## Risks

- Running checks after new active packages are created can mix baseline and planning diagnostics. Mitigate by naming the checked package set in every run log.
- Manifest refreshes can cascade. Use the deterministic refresh helper and rerun checks to a fixpoint.

## Validation

- Run Cairn validate and gates with the repo-pinned `.#cairn` app.
- Run evidence-manifest and task-evidence checks.
- Run focused matrix/current-bundle sanity checks when evidence metadata changes.
- Record all successful run logs and BLAKE3 manifests under `docs/evidence/`.
