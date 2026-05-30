# Valence CTF baseline oracle — 2026-05-30

## Question

Can reviewers verify the prior session claim that the Valence CTF example baseline passed 12/12 before starting `ctf-score-limit-win-condition` work?

## Inspected evidence

- Command log: `docs/evidence/valence-ctf-baseline-2026-05-30.run.log`
- BLAKE3: `7fd4d3042fe24816c933cc6383bae0e99304f8b96eb19576001295f2c79ceb22`
- Command: `cd valence && nix develop --no-update-lock-file -c cargo test --example ctf -- --nocapture`
- Result excerpt: `test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- Exit status: `exit_status=0`

## Decision

The 12/12 baseline claim is verified by repo-local command output and is only a pre-change baseline for the next active row. It is not additional evidence for the archived `ctf-invalid-return-drop` row.

## Owner

Agent maintaining the mc Cairn drain.

## Next action

Use this baseline as the starting point for `cairn/changes/ctf-score-limit-win-condition`; rerun the same Valence CTF example tests after that row changes Valence CTF logic.
