# Protocol-763 ROI 09 projectile damage demotion checkpoint — 2026-05-27

## Oracle checkpoint

- Question: Should ROI 08 projectile damage attribution remain in maintained acceptance evidence?
- Inspected evidence:
  - `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.run.log` records Valence worktree `HEAD e5d18ad` and depends on server-side `projectile_use` / `projectile_hit` emitted outside the parent repo diff.
  - `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.receipt.json` lists `projectile_damage_update`, `server_projectile_use`, and `server_projectile_hit`, but only as observed milestone names.
  - The live run log can show `update_health health=17.0` before the local client `projectile_probe_use_item_sent` / `projectile_probe_swing_sent` lines, so milestone presence is not enough to prove causal ordering.
  - `cairn/archive/2026-05-27-roi-08-projectile-damage-attribution/tasks.md` used `VALENCE_REV=HEAD`, not a pinned Valence dependency recorded in repo-local review evidence.
- Decision owner: agent; maintainer should approve re-promotion only after pinned Valence and causality proof are added.
- Decision: demote ROI 08 from accepted matrix/bundle/residual covered rails. Preserve artifacts as experimental audit evidence only.
- Next action: re-scope projectile damage attribution as a fresh proof that pins or includes the Valence server instrumentation and requires a causally ordered client/server receipt.

## Required proof before re-promotion

1. Pin or include the Valence commit that provides `projectile_use` / `projectile_hit` instrumentation.
2. Record a repo-local checkpoint for whether that dependency is landed, pinned, reproducible, and maintained.
3. Strengthen the runner so the client damage milestone is ordered after projectile action/server attribution instead of matched anywhere in combined logs.
4. Rerun dry-run and live evidence; update BLAKE3 manifests and matrix/bundle only after the proof passes.
