# Python gate Rust migration oracle — 2026-05-31

## Question

How should the post-review instruction to move Python gates to Rust be interpreted for the current workstream, and which remaining Python files are intentionally left unmigrated after the maintained evidence freshness gates were moved to Rust?

## Inspected evidence

- Current maintained evidence freshness gates now have Rust implementations:
  - `tools/check_acceptance_matrix.rs`
  - `tools/check_current_evidence_bundle.rs`
  - `tools/check_survival_coverage_matrix.rs`
  - `tools/check_evidence_manifests.rs`
- The Python counterparts for those gates were removed:
  - `tools/check_acceptance_matrix.py`
  - `tools/check_current_evidence_bundle.py`
  - `tools/check_survival_coverage_matrix.py`
  - `tools/check_evidence_manifests.py`
- Current migration validation is recorded in `docs/evidence/protocol-763-python-gate-rust-migration-2026-05-31.run.log` and runs only the Rust maintained gates plus Cairn validation.
- Recursive tracked-Python inventory commands recorded from `/home/brittonr/git/mc`: `git ls-files *.py` and `git ls-files *.py :!:target/** :!:valence/** :!:stevenarella/**`.
- All tracked Python files in `mc/` are checker/gate files; no additional tracked Python files exist outside this list.
- Remaining Python checker inventory after this migration:
  - `tools/check_armor_modifier_matrix.py`
  - `tools/check_ctf_rule_ledger.py`
  - `tools/check_death_respawn_lifecycle.py`
  - `tools/check_equipment_slot_item_matrix.py`
  - `tools/check_inventory_semantics_matrix.py`
  - `tools/check_load_network_safety.py`
  - `tools/check_projectile_travel_collision.py`
  - `tools/check_protocol_coverage_ledger.py`
  - `tools/check_survival_reference_parity.py`
  - `tools/check_vanilla_combat_parity.py`
- Historical evidence manifests still cite historical Python checkers for archived rows, including `tools/check_survival_reference_parity.py`, `tools/check_protocol_coverage_ledger.py`, and `tools/check_load_network_safety.py`.
- Final manifest validation uses `target/check-evidence-manifests` after refreshing the migration `.b3`; the stable final count after adding the Nix build log is `140 manifests, 670 entries, 71 receipts scanned`.
- Nix source-closure validation is recorded in `docs/evidence/protocol-763-python-gate-rust-migration-nix-build-2026-05-31.run.log`; it captures stdout/stderr plus `exit_status=0` for `nix build --no-update-lock-file .#checks.x86_64-linux.mc-compat-evidence-manifests --no-link -L`, and captures a forced rebuild output check with `--rebuild`.

## Decision

For this post-review fix, treat `move any python gate to rust` as applying to the Python gates that are part of the current evidence freshness/validation path and to any Python gate touched by this survival-crafting-table workstream. Those gates are now Rust-only.

The remaining Python files are explicitly classified as legacy domain checkers, not part of this post-review gate migration. They remain in-tree to preserve historical archived evidence and to avoid silently weakening old row-specific checks. They must not be edited or extended as Python. When one of those domains is next touched, migrate that checker to Rust or Steel as part of that domain change before promoting new evidence.

The historical manifest `docs/evidence/protocol-763-survival-coverage-reference-parity-sync-2026-05-28.b3` was updated only for the current mutable survival coverage checker path that now has a Rust replacement. Historical rows that still cite legacy Python checkers remain unchanged unless their domain checker is migrated with its own validation evidence.

## Owner

Maintainer / next agent touching the affected domain checker.

## Next action

Do not claim all Python checkers are migrated. For each remaining Python checker, create or use the domain-specific OpenSpec change, port the checker to Rust or Steel, run positive and negative fixtures, refresh BLAKE3 manifests, and record row-specific evidence before deleting the Python file.
