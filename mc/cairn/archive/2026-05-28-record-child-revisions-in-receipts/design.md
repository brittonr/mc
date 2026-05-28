# Design: Record child revisions in compatibility receipts

## Data model

Add a child revision section to the scenario receipt:

- `client.git_rev`: full Stevenarella commit hash resolved from `--client-dir`.
- `client.git_status`: clean/dirty/unavailable summary for the client checkout.
- `valence.git_rev_requested`: configured `VALENCE_REV` string.
- `valence.git_rev_resolved`: full commit hash used by the Valence worktree.
- `valence.git_status`: clean/dirty/unavailable summary for the Valence worktree.

## Implementation shape

Keep git probing in the runner shell/CLI layer. Pure receipt evaluation logic should consume plain strings/booleans passed in from I/O helpers.

Dirty status should fail closed for promoted live evidence unless a dedicated oracle checkpoint explains why the dirty state is intended.

## Validation

- Positive unit tests for clean child revision capture.
- Negative unit tests for unavailable/dirty child repo cases.
- Dry-run shape test with deterministic placeholder values.
- Live survival receipt refresh can remove the oracle workaround once machine fields exist.
