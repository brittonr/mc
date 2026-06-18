## Context

Evidence logs and manifests can drift as checks, specs, and docs evolve. This pass is operational: it refreshes a bounded smoke/evidence baseline and documents blockers, but it does not alter runner behavior or promote compatibility rows.

## Goals / Non-Goals

Goals:
- Define a bounded smoke set before running checks.
- Run selected repo-pinned checks and maintained dry-run apps.
- Promote review-critical logs under `docs/evidence/` with explicit `exit_status=0`.
- Refresh BLAKE3 manifests to a deterministic fixpoint when tracked evidence changes.

Non-goals:
- Adding live rails, changing runner semantics, changing checker contracts, or promoting new acceptance-matrix rows.
- Running unbounded live, public-server, WAN, production, or destructive tests.

## Design

1. Select a smoke set with command names, expected outputs, maximum scope, and non-claims.
2. Run non-mutating checks first: Cairn validation/gates, targeted packet checks, scenario manifest checks, and representative flake dry-runs.
3. Copy or write reviewable logs under `docs/evidence/`; every cited `.run.log` must include explicit successful exit status or a blocker record.
4. Refresh BLAKE3 manifests through the repo helper when evidence files change, then rerun manifest/task-evidence checks to a fixpoint.
5. Record any failed smoke as a blocker with owner and next action rather than converting it to a claim.

## Risks

- Full flake checks can be slow. Keep the default smoke set bounded and queue longer commands with pueue when needed.
- Some dry-runs may require source-closure visibility for new evidence. Ensure cited artifacts live under tracked parent `docs/evidence/` paths.

## Validation

- Run the selected smoke set and record logs.
- Run evidence-manifest and task-evidence checks after manifest refresh.
- Run Cairn gates and validation before archive.
