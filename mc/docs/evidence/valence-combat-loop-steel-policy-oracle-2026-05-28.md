# Valence combat-loop Steel policy oracle checkpoint — 2026-05-28

## Scope

This checkpoint resolves review ambiguity for partially completed tasks in active change `valence-combat-loop-steel-policy`.

## Question 1: Does Valence currently execute Steel in a restricted sandbox?

- **Inspected evidence**: `valence/examples/ctf.rs` uses `load_arrow_policy_snapshot_from_path`, `normalize_arrow_policy_module`, forbidden-token checks, literal string/f32 export parsing, and a required `damage-linear` policy shape.
- **Decision**: No. This is a restricted Steel-compatible literal normalizer, not full Steel evaluator execution. Spec/design/tasks were narrowed to avoid claiming sandboxed Steel execution.
- **Owner**: agent.
- **Next action**: keep `tasks.md` unchecked item “Replace the restricted literal normalizer with real restricted Steel evaluator execution, or keep the final spec limited to the literal subset before archive.”

## Question 2: Are milestone/evidence emission lines present in Valence code?

- **Inspected evidence**:
  - publish success milestone in `valence/examples/ctf.rs` around lines 463–473: `MC-COMPAT-MILESTONE steel_arrow_policy_publish` includes source, generation, policy, base damage, velocity multiplier, and max damage.
  - rejection milestone around lines 478–485: `MC-COMPAT-MILESTONE steel_arrow_policy_reject` includes source, active generation, and redacted diagnostics.
  - combat-event projectile use/hit milestones around lines 1810–1872 include policy, generation, clamped, damage, and victim health delta.
  - projectile-interaction use/hit milestones around lines 1963–1988 include policy, generation, clamped, damage, and victim health delta.
- **Decision**: Code emission paths exist, but live/live-equivalent receipts are still missing; final evidence tasks remain unchecked.
- **Owner**: agent.
- **Next action**: capture protocol-763 live/live-equivalent receipts before completing evidence tasks or archiving.

## Question 3: Are snapshot diffing and range-invalid decision-output coverage complete?

- **Inspected evidence**: `cargo test --manifest-path valence/Cargo.toml --example ctf` currently reports six tests: default policy, edited policy, invalid candidate preserving previous snapshot, malformed/capability rejection, clamping, and redaction.
- **Decision**: Snapshot diffing is not implemented/evidenced. Range-invalid candidate values are covered, but a distinct range-invalid decision-output test is not evidenced because current evaluation clamps before decision validation.
- **Owner**: agent.
- **Next action**: keep explicit unchecked tasks for snapshot diffing and range-invalid decision-output proof or remove those requirements before archive.

## Question 4: Is active combat integration complete?

- **Inspected evidence**: both `projectile_probe_hit` in `handle_combat_events` and the `handle_projectile_events` interaction path route damage through `projectile_probe_damage_decision()` in `valence/examples/ctf.rs`; call-site inventory is in `docs/evidence/valence-combat-loop-steel-policy-call-sites-2026-05-28.tsv`.
- **Decision**: Code wiring exists, but live/live-equivalent protocol-763 evidence remains open.
- **Owner**: agent.
- **Next action**: run or simulate the protocol-763 projectile scenario with a non-default Steel-compatible policy and preserve receipts under `docs/evidence/`.
