# Protocol-763 current evidence bundle

## Scope

Current-head index for the maintained Stevenarella ⇄ Valence CTF protocol-763 compatibility evidence set. This bundle summarizes the acceptance matrix and gives operators one stable checklist for what is covered and what remains a non-claim.

- Matrix: `docs/evidence/protocol-763-acceptance-matrix.md`
- Matrix checker: `python3 tools/check_acceptance_matrix.py`
- Bundle checker: `python3 tools/check_current_evidence_bundle.py`
- Evidence manifest checker: `python3 tools/check_evidence_manifests.py`
- Latest parent checkout before this bundle refresh: `4d29f46 scope pinned projectile damage proof`
- Child commits used for the current-head representative refresh: Valence `e5d18ad`, Stevenarella `616ee72`

## Evidence rows

| Seam | Maintained command | BLAKE3 |
| --- | --- | --- |
| RED/BLUE scoring soak | `nix run .#mc-compat-valence-ctf-600s-soak`; `nix run .#mc-compat-valence-ctf-blue-600s-soak` | `b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de` |
| Inventory/drop | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46` |
| Block placement / use-item-on-block | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122` |
| Pickup semantics | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `bcac4aab63857cf0d3b6dd148455324e7f0368dd3e57cfd26841ae7fc1b5ffe8` |
| Player-inventory click/container click | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` |
| Open-container semantics | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3` |
| Two-client combat/damage | `nix run .#mc-compat-valence-ctf-combat-damage` | `b67962dd5d4fe7242b69fd7c879390e80e13528475d55d7feb5305289f762ac8` |
| Flag-carrier death/return | `nix run .#mc-compat-valence-ctf-flag-carrier-death-return` | `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4` |
| Reconnect flag-state | `nix run .#mc-compat-valence-ctf-reconnect-flag-state` | `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3` |
| Latency/jitter tolerance | `nix run .#mc-compat-valence-ctf-latency-jitter-inventory` | `a4a407fb1ac3aceae06faeacb794891ff8411c8ac86470c651c89b37b6c7f33d` |
| Combat knockback | `nix run .#mc-compat-valence-ctf-combat-knockback` | `a5d0ba5ea6155a99b58f245a03195da05b4925d7bd151b5b3f67503ae7a4cf09` |
| Armor equipment mitigation | `nix run .#mc-compat-valence-ctf-armor-equipment-mitigation` | `176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765` |
| Equipment update observation | `nix run .#mc-compat-valence-ctf-equipment-update-observation` | `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d` |
| Projectile use/loadout rail | `nix run .#mc-compat-valence-ctf-projectile-hit` | `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b` |
| Projectile damage attribution | `nix run .#mc-compat-valence-ctf-projectile-damage-attribution` | `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529` |

## Inventory semantics matrix checkpoint

The maintained inventory rows are validated as a five-row bounded matrix in `docs/evidence/protocol-763-inventory-semantics-matrix-2026-05-27.md`. Covered rows are drop, pickup, player-inventory click, open-container click, and block placement/use-item-on-block. Full inventory semantics remains a non-claim; stale state-id rejection, invalid slots, malformed clicks, stack splitting/merging, drag transactions, and all-window coverage are not promoted.

## Death/respawn lifecycle checkpoint

The maintained `Flag-carrier death/return` row is also validated as a bounded lifecycle row in `docs/evidence/protocol-763-death-respawn-lifecycle-2026-05-27.md`. The row covers one flag-carrier death, respawn request, restored health, server flag return/reset, and no unexpected score/capture. No all death/drop/recovery permutations are claimed; full death/respawn lifecycle remains a non-claim.

## Representative current-head live refresh

The maintained matrix rows above stay historical so their BLAKE3-backed receipt hashes do not move silently. A fresh representative current-head live run was added for projectile row freshness instead:

- Seam: projectile hit rail.
- Maintained command: `VALENCE_REV=HEAD VALENCE_WORKTREE=/tmp/valence-compat-current-head-projectile-refresh VALENCE_TARGET_DIR=/tmp/valence-compat-current-head-projectile-refresh-target CLIENT_TIMEOUT=300 MC_COMPAT_PROJECTILE_HIT_RECEIPT=target/mc-compat-current-head-live-refresh/projectile-hit-current-head.json nix run --no-update-lock-file .#mc-compat-valence-ctf-projectile-hit -- --run`.
- Source receipt: `target/mc-compat-current-head-live-refresh/projectile-hit-current-head.json`.
- Reviewable receipt copy: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.receipt.json`.
- Reviewable run log copy: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.run.log`.
- BLAKE3: `756b6f732e71ae370808b2a653d1310baa88875f2c3345a1c87444fcffb51c6c`.
- Run log BLAKE3: `05429930472e764a6a2b140ce9c0a7652552659210b4bb1407d93d0d2cd7fada`.
- Payload commits at run time: parent `a2dddea`, Valence `e5d18ad`, Stevenarella `616ee72`.
- Receipt outcome: `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, no missing client/server milestones, `triage.suggested_boundary=none`.
- Scoped non-claims remain: no full projectile physics, projectile travel/collision simulation, all-weapons, enchantment/status-effect, production-load, broad protocol, or full CTF/combat correctness claim.

## Pinned projectile damage attribution refresh

ROI 10 re-promotes projectile damage attribution with pinned dependency and causal receipt proof:

- Maintained command: `nix run .#mc-compat-valence-ctf-projectile-damage-attribution`.
- Source receipt: `target/roi-10-live/projectile-damage-pinned-live.receipt.json`.
- Reviewable receipt copy: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json`.
- Reviewable run log copy: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.run.log`.
- Reviewable Valence/client logs: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.valence.log`, `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbota.log`, `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbotb.log`.
- BLAKE3: `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529`.
- Payload commits at run time: parent `4d29f46` plus current runner diff, Valence `e5d18ad`, Stevenarella `616ee72`.
- Receipt outcome: `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, `server.passed=true`, `projectile_damage_causality.passed=true`, missing steps `[]`, order violations `[]`.
- Scoped non-claims remain: no full projectile physics, projectile travel/collision simulation, all-weapons, exact vanilla damage parity, enchantment/status-effect, production-load, broad protocol, or full CTF/combat correctness claim.

## Current maintained checks

```sh
python3 tools/check_acceptance_matrix.py
python3 tools/check_current_evidence_bundle.py
python3 tools/check_load_network_safety.py
python3 tools/check_death_respawn_lifecycle.py
python3 tools/check_inventory_semantics_matrix.py
nix develop --no-update-lock-file -c python3 tools/check_evidence_manifests.py
nix run --no-update-lock-file .#cairn -- validate --root .
```

## Evidence freshness promotion gate

Before adding or replacing a maintained evidence row, run the freshness gate from `/home/brittonr/git/mc` and copy the output under `docs/evidence/`:

```sh
python3 tools/check_acceptance_matrix.py --self-test
python3 tools/check_acceptance_matrix.py
python3 tools/check_current_evidence_bundle.py --self-test
python3 tools/check_current_evidence_bundle.py
nix develop --no-update-lock-file -c python3 tools/check_evidence_manifests.py --self-test
nix develop --no-update-lock-file -c python3 tools/check_evidence_manifests.py
nix run --no-update-lock-file .#cairn -- validate --root .
```

Rows that cite live receipts should point at tracked `docs/evidence/*.receipt.json` copies and BLAKE3 manifests. Historical `target/` rows require an explicit evidence/oracle note that records the original digest, inspected evidence, decision, owner, and next action; the RED/BLUE scoring soak row is the current historical exception, recorded at `docs/evidence/protocol-763-red-blue-soak-historical-oracle-2026-05-27.md`.

Reviewable copied receipts for matrix rows are indexed at `docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.md` with manifest `docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.b3`.

## Production/load/network safety envelope

The runner receipt surface includes a `load_network_safety` block that records owned-local or explicit authorization, client/duration/reconnect/network bounds, telemetry readiness, and fail-closed diagnostics. Evidence: `docs/evidence/protocol-763-load-network-safety-2026-05-27.md`.

Broader production/public/WAN/unbounded safety remains a non-claim unless a future authorized bounded envelope has live telemetry, BLAKE3-backed evidence, and an updated matrix/bundle row.

## Non-claims

This bundle still does not claim full Minecraft compatibility, full CTF correctness, projectile travel/collision simulation, all projectile weapon variants, all equipment slots/items, all armor loadouts, enchantment/status-effect semantics, exact vanilla knockback/damage/mitigation balancing, production readiness, public-server load safety, or unbounded soak/reconnect/latency safety.
