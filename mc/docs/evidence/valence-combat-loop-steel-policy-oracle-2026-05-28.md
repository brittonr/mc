# Valence combat-loop Steel policy oracle checkpoint — 2026-05-28

## Scope

This checkpoint resolves review ambiguity for archived change `cairn/archive/2026-05-28-valence-combat-loop-steel-policy`.

## Question 1: Does Valence currently execute Steel in a restricted sandbox?

- **Inspected evidence**: `valence/examples/ctf.rs` uses `load_arrow_policy_snapshot_from_path`, `normalize_arrow_policy_module`, forbidden-token checks, literal string/f32 export parsing, and a required `damage-linear` policy shape.
- **Decision**: No. This is a restricted Steel-compatible literal normalizer, not full Steel evaluator execution. Spec/design/tasks were narrowed to avoid claiming sandboxed Steel execution.
- **Owner**: agent.
- **Next action**: final scope is the restricted Steel-compatible literal subset; real Steel evaluator execution is out of scope for this change.

## Question 2: Are milestone/evidence emission lines present in Valence code?

- **Inspected evidence**:
  - publish success milestone in `valence/examples/ctf.rs` around lines 463–473: `MC-COMPAT-MILESTONE steel_arrow_policy_publish` includes source, generation, policy, base damage, velocity multiplier, and max damage.
  - rejection milestone around lines 478–485: `MC-COMPAT-MILESTONE steel_arrow_policy_reject` includes source, active generation, and redacted diagnostics.
  - combat-event projectile use/hit milestones around lines 1855–1923 include policy, generation, clamped, damage, and victim health delta.
  - projectile-interaction use/hit milestones around lines 1987–2042 include policy, generation, clamped, damage, and victim health delta.
  - nested-source diff copied into `docs/evidence/valence-combat-loop-steel-policy-nested-diff-2026-05-28.md` makes Valence commit `2663ed7` reviewable without opening the nested repo.
- **Decision**: Code emission paths exist. Live-equivalent receipt `docs/evidence/valence-combat-loop-steel-policy-live-equivalent-2026-05-28.receipt.json` records non-default damage, both call-site ids, generation, clamped flag, and victim health deltas.
- **Owner**: agent.
- **Next action**: no remaining archive action; keep BLAKE3 evidence manifests current for later evidence edits.

## Question 3: Are snapshot diffing and range-invalid decision-output coverage complete?

- **Inspected evidence**: `cargo test --manifest-path valence/Cargo.toml --example ctf` now reports nine tests, including `snapshot_diff_reports_changed_policy_fields`, `range_invalid_decision_output_is_rejected`, and `non_default_policy_changes_both_projectile_call_site_health_deltas`.
- **Decision**: Snapshot diffing and range-invalid decision-output proof are now covered by focused tests and the core gate log.
- **Owner**: agent.
- **Next action**: no remaining archive action; core gate receipt is preserved under `docs/evidence/valence-combat-loop-steel-policy-core-gate-2026-05-28.run.log`.

## Question 4: Is archived combat integration complete?

- **Inspected evidence**: both `projectile_probe_hit` in `handle_combat_events` and the `handle_projectile_events` interaction path route damage through `projectile_probe_damage_decision()` in `valence/examples/ctf.rs`; call-site inventory is in `docs/evidence/valence-combat-loop-steel-policy-call-sites-2026-05-28.tsv`; nested commit diff is preserved in `docs/evidence/valence-combat-loop-steel-policy-nested-diff-2026-05-28.md`.
- **Decision**: Code wiring exists. Live-equivalent protocol-763 evidence is preserved in `docs/evidence/valence-combat-loop-steel-policy-live-equivalent-2026-05-28.receipt.json`; checker validation ties it to the call-site list and inventory.
- **Owner**: agent.
- **Next action**: none for this archived change; future live-protocol validation remains a separate non-claim.
