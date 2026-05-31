# Protocol-763 Python gate Rust migration — 2026-05-31

## Scope

This post-review fix removes the mutable Python implementations for the current matrix/bundle/survival coverage/evidence-manifest gates and leaves their Rust equivalents as the maintained gates:

| Removed Python gate | Maintained Rust gate |
| --- | --- |
| `tools/check_acceptance_matrix.py` | `tools/check_acceptance_matrix.rs` |
| `tools/check_current_evidence_bundle.py` | `tools/check_current_evidence_bundle.rs` |
| `tools/check_survival_coverage_matrix.py` | `tools/check_survival_coverage_matrix.rs` |
| `tools/check_evidence_manifests.py` | `tools/check_evidence_manifests.rs` |

Historical archive text and historical run logs may still mention the old Python commands as evidence history. Current maintained validation in `docs/evidence/protocol-763-current-evidence-bundle.md` uses the Rust gates. The explicit exemption and inventory checkpoint for remaining legacy Python domain checkers is `docs/evidence/protocol-763-python-gate-rust-migration-oracle-2026-05-31.md`.

## Verification

Validation output: `docs/evidence/protocol-763-python-gate-rust-migration-2026-05-31.run.log`. The run log records the final post-refresh manifest check with `140 manifests, 670 entries, 71 receipts scanned`. Nix source-closure validation output: `docs/evidence/protocol-763-python-gate-rust-migration-nix-build-2026-05-31.run.log`.

The run log records:

- Rust acceptance matrix self-test and repo check.
- Rust current evidence bundle self-test and repo check.
- Rust survival coverage matrix self-test and repo check.
- Rust evidence manifest self-test and repo check.
- Recursive tracked-Python gate/checker inventory.
- Nix `mc-compat-evidence-manifests` build with `exit_status=0`.
- Cairn validation.
- A deterministic check that the four removed Python gate files are absent.

## Non-claims

This does not migrate older unrelated Python checkers such as historical matrix-family checkers. Those exemptions are recorded in `docs/evidence/protocol-763-python-gate-rust-migration-oracle-2026-05-31.md`; remaining Python checkers must be migrated before their domains are edited or promoted again.
