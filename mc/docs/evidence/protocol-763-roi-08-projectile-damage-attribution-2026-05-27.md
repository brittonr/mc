# Protocol-763 ROI 08 projectile damage attribution evidence — demoted

## Status

This evidence is **experimental and demoted** by `roi-09-demote-projectile-damage-claim`. It must not be treated as a maintained accepted projectile damage attribution row.

## Why demoted

Same-family review found two blockers:

1. The server-side `projectile_use` / `projectile_hit` milestones came from Valence `HEAD` at `e5d18ad` without a repo-local dependency checkpoint proving that instrumentation is pinned, landed, reproducible, and reviewable for this rail.
2. The runner/receipt accepted milestone presence. The run log can show client `update_health health=17.0` before client `projectile_probe_use_item_sent` / `projectile_probe_swing_sent`, so the evidence does not prove causal ordering from projectile action to client damage update.

## Original command

```sh
MC_COMPAT_PROJECTILE_DAMAGE_RECEIPT=target/mc-compat-projectile-damage-attribution/projectile-damage-attribution.json \
VALENCE_REV=HEAD \
VALENCE_WORKTREE=/tmp/valence-compat-projectile-damage \
VALENCE_TARGET_DIR=/tmp/valence-compat-projectile-damage-target \
CLIENT_TIMEOUT=300 \
nix run --no-update-lock-file .#mc-compat-valence-ctf-projectile-damage-attribution -- --run
```

## Reviewable artifacts retained for audit

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Feasibility checkpoint | `docs/evidence/protocol-763-roi-08-projectile-damage-feasibility-2026-05-27.md` | `f635054c7d47087db102a676000fd292f5f8df254e66a7b68d8e34e1a5591417` |
| Live receipt | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.receipt.json` | `39b085d43b09c6392e19b0cc74b7d8192d8bf34b4c5351514ad0b94d0d07c603` |
| Live run log | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.run.log` | `c6f0cb7b713b43073a927bf43dbd9e5370c1fdb7d30531c0a3765bfccd5adcd3` |
| Dry-run receipt | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.dry-run.receipt.json` | `631c3cd90500690817ea17098adda4dce378692a6c3d4d85f24865eb5000352d` |
| Dry-run log | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.dry-run.log` | `dc7a7354732efcc4d0dc52fd7c198cb1937a23fbdcbc823fb389f75a75ee2181` |

## Non-claim

These artifacts do not prove projectile damage attribution, projectile travel/collision simulation, all bow/crossbow/trident variants, exact vanilla damage parity, enchantment or status-effect modifiers, production load safety, broad protocol coverage, or full combat correctness.

## Re-promotion requirements

Before this rail can return to the acceptance matrix:

1. Pin or include reviewable Valence code for `projectile_use` / `projectile_hit` server milestones.
2. Record a repo-local checkpoint for the Valence dependency decision.
3. Strengthen the runner/receipt to prove causal ordering between projectile action, server attribution, and client damage update.
4. Rerun dry-run and live evidence with the pinned dependency and updated causality check.
