# Protocol-763 survival child-revision oracle checkpoint — 2026-05-28

## Question

Does the promoted survival break/place/pickup evidence use committed Valence and Stevenarella child-repo revisions, especially Stevenarella `9921e68`, even though the receipt schema records `client.dir` but not `client.git_rev`?

## Inspected evidence

- `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.receipt.json` records `valence.rev: "1fac05a"`, `valence.worktree: "/tmp/valence-compat-survival-1fac05a"`, `valence.example: "survival_compat"`, `client.dir: "stevenarella"`, and `client.log_path: "/tmp/mc-compat-client.compatbot.1779986969825.log"`.
- `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.run.log` records the run output for the survival rail and shows the committed-client rerun reached the required survival probe milestones and wrote `target/mc-compat-survival-break-place-pickup/survival-break-place-pickup-live-committed.json`.
- Pueue status for task `59` recorded the command: `VALENCE_WORKTREE=/tmp/valence-compat-survival-1fac05a VALENCE_TARGET_DIR=/tmp/valence-compat-survival-target-1fac05a VALENCE_REV=1fac05a CLIENT_TIMEOUT=180 MC_COMPAT_SURVIVAL_BREAK_PLACE_PICKUP_RECEIPT=target/mc-compat-survival-break-place-pickup/survival-break-place-pickup-live-committed.json nix run --no-update-lock-file .#mc-compat-valence-survival-break-place-pickup -- --run --client-dir stevenarella --valence-repo valence`.
- `git -C stevenarella rev-parse HEAD` returned `9921e686f56270cb5810c1f6187d19b051ecc236`.
- `git -C stevenarella status --short --branch` returned `## master...fork/master`, with no dirty paths after the run and push.
- `git -C /tmp/valence-compat-survival-1fac05a rev-parse HEAD` returned `1fac05a6d012f27b83d88d83c59e5ab320a78164`.
- `git -C /tmp/valence-compat-survival-1fac05a status --short --branch` returned `## HEAD (no branch)`, with no dirty paths.
- `git -C valence rev-parse HEAD` returned `7d13a242742347a05c9752501880a2e986819ae7`, which contains the fixture commit `1fac05a`; `git -C valence status --short --branch` returned `## main...fork/main`, with no dirty paths.
- `git -C stevenarella log --oneline -2` returned `9921e68 add survival compatibility probe` and `28b3113 ignore pi agent metadata`.
- `git -C /tmp/valence-compat-survival-1fac05a log --oneline -2` returned `1fac05a add survival compatibility fixture for mc rails` and `2bf11c5 ignore pi agent metadata`.

## Decision

Accept the survival evidence row as a bounded compatibility receipt tied to committed child revisions: parent `fff4386`, Valence `1fac05a`, and Stevenarella `9921e68`.

The Stevenarella revision is established by this oracle checkpoint plus clean child-repo status, not by the receipt schema itself. Future evidence should not rely on this workaround when the runner can record child-repo revisions directly.

## Owner

Agent records this checkpoint now; maintainer owns any decision to promote the receipt schema change from follow-up to required gate.

## Next action

Add `client.git_rev`, `client.git_status`, and `valence.git_rev_resolved` fields to `mc-compat-runner` receipts before the next new child-repo evidence rail is promoted. Until then, any promoted row that cites a child repo revision missing from the receipt must include an oracle checkpoint with this same question/evidence/decision/owner/next-action shape.
