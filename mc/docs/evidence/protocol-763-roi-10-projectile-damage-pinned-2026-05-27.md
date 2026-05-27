# Protocol-763 ROI 10 pinned projectile damage attribution — 2026-05-27

## Scope

This is a bounded Stevenarella ⇄ Valence CTF protocol-763 projectile damage attribution rail. It re-promotes projectile damage attribution after ROI 08 was demoted, but only for the local two-client fixture and the pinned Valence instrumentation commit.

## Command

```sh
CLIENT_TIMEOUT=60 \
VALENCE_WORKTREE=/tmp/valence-compat-projectile-damage-pinned-2026-05-27 \
VALENCE_TARGET_DIR=/tmp/valence-compat-projectile-damage-pinned-2026-05-27-target \
MC_COMPAT_PROJECTILE_DAMAGE_RECEIPT=target/roi-10-live/projectile-damage-pinned-live.receipt.json \
nix run --no-update-lock-file .#mc-compat-valence-ctf-projectile-damage-attribution -- --run
```

Dry-run gate:

```sh
nix build .#checks.x86_64-linux.mc-compat-valence-ctf-projectile-damage-attribution-dry-run --no-link -L --no-update-lock-file --option builders ''
```

## Pinned dependency

- Valence repo: `valence/`
- Valence revision used by receipt: `e5d18ad04010d92881267ac1ea43922ae91821f5`
- Valence subject: `ctf: add projectile hit compat probe`
- Dependency checkpoint: `docs/evidence/protocol-763-roi-10-projectile-damage-pin-checkpoint-2026-05-27.md`
- The wrapper and runner reject promoted projectile damage evidence gathered with `VALENCE_REV=HEAD`.

## Outcome

- Receipt: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json`
- Run log: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.run.log`
- Valence server log: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.valence.log`
- Client log, victim `compatbota`: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbota.log`
- Client log, attacker `compatbotb`: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbotb.log`
- Dry-run receipt/log: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.dry-run.receipt.json`, `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.dry-run.log`
- Receipt digest: `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529`

The live receipt reports `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, `server.passed=true`, and `projectile_damage_causality.passed=true`.

## Causal proof boundary

The receipt does not accept unordered milestone presence. Its `projectile_damage_causality` block records:

- attacker: `compatbotb`
- victim: `compatbota`
- observed steps:
  - `attacker_client_projectile_use_sent`
  - `attacker_client_projectile_swing_sent`
  - `server_projectile_use_attacker_victim`
  - `server_projectile_hit_attacker_victim_health_delta`
  - `victim_client_damage_update`
- missing steps: `[]`
- order violations: `[]`

The reviewable logs show the attacker client sent projectile use/swing, the pinned Valence server recorded `projectile_use` and later `projectile_hit` for the same attacker/victim pair with `victim_health_after=14.0`, and the victim client observed `update_health health=14.0`.

## Non-claims

This evidence does not claim full projectile physics, projectile travel/collision simulation, all projectile weapon variants, exact vanilla damage parity, enchantments/status effects, production PvP readiness, full combat correctness, broad Minecraft compatibility, or full CTF correctness.
