# Protocol-763 Python gate Rust migration — 2026-05-31

## Scope

This post-review fix removes the mutable Python implementations for the current matrix/bundle/survival coverage gates and leaves their Rust equivalents as the maintained gates:

| Removed Python gate | Maintained Rust gate |
| --- | --- |
| `tools/check_acceptance_matrix.py` | `tools/check_acceptance_matrix.rs` |
| `tools/check_current_evidence_bundle.py` | `tools/check_current_evidence_bundle.rs` |
| `tools/check_survival_coverage_matrix.py` | `tools/check_survival_coverage_matrix.rs` |

Historical archive text and historical run logs may still mention the old Python commands as evidence history. Current maintained validation in `docs/evidence/protocol-763-current-evidence-bundle.md` uses the Rust gates.

## Verification

Validation output: `docs/evidence/protocol-763-python-gate-rust-migration-2026-05-31.run.log`.

The run log records:

- Rust acceptance matrix self-test and repo check.
- Rust current evidence bundle self-test and repo check.
- Rust survival coverage matrix self-test and repo check.
- Evidence manifest validation.
- Cairn validation.
- A deterministic check that the three removed Python gate files are absent.

## Non-claims

This does not migrate older unrelated Python checkers such as historical matrix-family checkers. It only removes the Python gates touched by the survival crafting table post-review regression and keeps their Rust replacements authoritative.
