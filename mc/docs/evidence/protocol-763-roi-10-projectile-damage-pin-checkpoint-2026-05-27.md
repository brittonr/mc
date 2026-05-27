# Protocol-763 ROI 10 projectile damage pin checkpoint — 2026-05-27

## Oracle checkpoint

- Question: What dependency should the next projectile damage attribution proof use so ROI 08 does not repeat the `VALENCE_REV=HEAD` review gap?
- Inspected evidence:
  - `git -C valence rev-parse HEAD` returned `e5d18ad04010d92881267ac1ea43922ae91821f5`.
  - `git -C valence log -1 --oneline` returned `e5d18ad ctf: add projectile hit compat probe`.
  - `git -C valence status --short --untracked-files=all` returned no lines at checkpoint time.
  - `valence/examples/ctf.rs` contains projectile probe instrumentation for `projectile_loadout`, `projectile_use`, and `projectile_hit`.
  - `tools/mc-compat-runner/src/main.rs` currently has unordered milestone checks for `ProjectileDamageAttribution`, so runner hardening is still required before promotion.
- Decision owner: agent; maintainer should approve any later promotion after reviewing live receipt/log evidence.
- Decision: use the clean local nested `valence/` checkout pinned to exact commit `e5d18ad04010d92881267ac1ea43922ae91821f5` for the next proof. Do not promote evidence gathered with `VALENCE_REV=HEAD`.
- Next action: make `ProjectileDamageAttribution` fail unless the receipt proves ordered client projectile use/swing, server projectile use/hit for the same attacker/victim, and subsequent client health update.

## Promotion guard

Projectile damage attribution remains a non-claim until dry-run, tests, live receipt, run log, and BLAKE3 manifests prove the pinned and causally ordered contract.
