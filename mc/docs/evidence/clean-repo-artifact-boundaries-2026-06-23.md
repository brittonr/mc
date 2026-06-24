# Clean repository artifact boundaries — 2026-06-23

## Scope

This note records the repository-hygiene cleanup for Cairn change `clean-repo-artifact-boundaries`.

## Decisions

- Artifact classes are documented in `docs/architecture.md` and `docs/layout-checklist.md`: durable evidence, generated checked-in output, transient run/build output, and local scratch.
- Durable task/review evidence lives under `docs/evidence/` and is paired with BLAKE3 evidence when cited from Cairn tasks.
- Root `evidence/` is retired. The four historical notes were promoted to `docs/evidence/legacy-*.md`.
- Root live logs were promoted to `docs/evidence/legacy-target-*.log` before the root copies were removed.
- Root `result` and `result-*` symlinks, empty root `config/`, and `tools/__pycache__/` were retired as local/transient artifacts.
- `.gitignore` now targets root transient outputs without hiding `docs/evidence/` logs, receipts, or `.b3` manifests.
- `tools/check_cairn_task_evidence.rs` rejects task citations to target-only, result-only, root-transient, retired root `evidence/`, or missing artifacts.

## Promoted legacy notes

- `docs/evidence/legacy-mc-compat-maintained-entrypoint-600s-soak-2026-05-24.md`
- `docs/evidence/legacy-mc-compat-multi-client-600s-soak-2026-05-24.md`
- `docs/evidence/legacy-mc-compat-multi-client-repeatability-2026-05-24.md`
- `docs/evidence/legacy-mc-compat-strict-decode-maintained-600s-soak-2026-05-24.md`

## Promoted root live logs

- `docs/evidence/legacy-target-mc-compat-block-place-live.log`
- `docs/evidence/legacy-target-mc-compat-click-live.log`
- `docs/evidence/legacy-target-mc-compat-combat-live.log`
- `docs/evidence/legacy-target-mc-compat-inventory-drop-live.log`
- `docs/evidence/legacy-target-mc-compat-inventory-live.log`
- `docs/evidence/legacy-target-mc-compat-open-container-live.log`
- `docs/evidence/legacy-target-mc-compat-pickup-live.log`

## Non-claims

This cleanup improves repository artifact boundaries and Cairn evidence reviewability only. It does not add live compatibility coverage, prove semantic equivalence, prove full CTF or survival correctness, change production readiness, or make any public-server safety claim.
