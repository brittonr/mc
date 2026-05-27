# Protocol-763 ROI 08 projectile damage attribution evidence — 2026-05-27

## Scope

This evidence covers a bounded Stevenarella ⇄ Valence CTF projectile damage attribution rail. It extends the earlier projectile use/loadout rail by requiring both client-side projectile action and health-update evidence plus Valence server-side projectile use/hit attribution.

## Maintained command

```sh
MC_COMPAT_PROJECTILE_DAMAGE_RECEIPT=target/mc-compat-projectile-damage-attribution/projectile-damage-attribution.json \
VALENCE_REV=HEAD \
VALENCE_WORKTREE=/tmp/valence-compat-projectile-damage \
VALENCE_TARGET_DIR=/tmp/valence-compat-projectile-damage-target \
CLIENT_TIMEOUT=300 \
nix run --no-update-lock-file .#mc-compat-valence-ctf-projectile-damage-attribution -- --run
```

## Reviewable artifacts

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Feasibility checkpoint | `docs/evidence/protocol-763-roi-08-projectile-damage-feasibility-2026-05-27.md` | `f635054c7d47087db102a676000fd292f5f8df254e66a7b68d8e34e1a5591417` |
| Live receipt | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.receipt.json` | `39b085d43b09c6392e19b0cc74b7d8192d8bf34b4c5351514ad0b94d0d07c603` |
| Live run log | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.run.log` | `c6f0cb7b713b43073a927bf43dbd9e5370c1fdb7d30531c0a3765bfccd5adcd3` |
| Dry-run receipt | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.dry-run.receipt.json` | `631c3cd90500690817ea17098adda4dce378692a6c3d4d85f24865eb5000352d` |
| Dry-run log | `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.dry-run.log` | `dc7a7354732efcc4d0dc52fd7c198cb1937a23fbdcbc823fb389f75a75ee2181` |

## Receipt outcome

```text
status=pass mode=run dry_run=false scenario=projectile-damage-attribution scenario.passed=true
client observed: multi_client_count, protocol_detected, join_game, render_tick, team_red, team_blue, remote_player_spawn, projectile_use_sent, projectile_swing_sent, projectile_damage_update
server observed: server_client_a_seen, server_client_b_seen, server_projectile_loadout, server_projectile_use, server_projectile_hit
missing client/server milestones: []
triage.suggested_boundary=none
```

## Non-claims

This rail does not claim full projectile physics, projectile travel/collision simulation, all bow/crossbow/trident variants, exact vanilla damage parity, enchantment or status-effect modifiers, production load safety, broad protocol coverage, or full combat correctness.
