# Protocol-763 ROI 10 dependency checkpoint review evidence — 2026-05-27

## Purpose

Close the post-review WARN that the ROI 10 dependency checkpoint was cited from archived Cairn tasks but not independently reviewable in the supplied diff.

## Verified artifacts

- Checkpoint: `docs/evidence/protocol-763-roi-10-projectile-damage-pin-checkpoint-2026-05-27.md`
- Checkpoint sidecar: `docs/evidence/protocol-763-roi-10-projectile-damage-pin-checkpoint-2026-05-27.b3`
- Verification log: `docs/evidence/protocol-763-roi-10-dependency-checkpoint-review-2026-05-27.run.log`

## Required checkpoint fields

| Requirement | Evidence in checkpoint |
| --- | --- |
| Valence commit | `e5d18ad04010d92881267ac1ea43922ae91821f5` |
| Valence subject | `e5d18ad ctf: add projectile hit compat probe` |
| Clean worktree status | `git -C valence status --short --untracked-files=all` returned no lines |
| Decision owner | `Decision owner: agent` |
| Decision | use clean local nested `valence/` checkout pinned to exact commit |
| HEAD ban | do not promote evidence gathered with `VALENCE_REV=HEAD` |
| Next action | harden `ProjectileDamageAttribution` for ordered proof |
| Instrumentation | `projectile_loadout`, `projectile_use`, and `projectile_hit` |

## Verification result

The verification log ran `b3sum --check` on the checkpoint sidecar and checked each required field, the current nested Valence revision, clean Valence status, and instrumentation markers. Result: `dependency checkpoint verification ok`.
