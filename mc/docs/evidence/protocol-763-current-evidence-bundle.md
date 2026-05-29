# Protocol-763 current evidence bundle

## Scope

Current-head index for the maintained Stevenarella ⇄ Valence CTF protocol-763 compatibility evidence set. This bundle summarizes the acceptance matrix and gives operators one stable checklist for what is covered and what remains a non-claim.

- Matrix: `docs/evidence/protocol-763-acceptance-matrix.md`
- Matrix checker: `python3 tools/check_acceptance_matrix.py`
- Bundle checker: `python3 tools/check_current_evidence_bundle.py`
- Evidence manifest checker: `python3 tools/check_evidence_manifests.py`
- Latest parent checkout before this bundle refresh: `5d4973d add Paper survival reference fixture`
- Child commits used for the current-head representative refresh: Valence `e5d18ad`, Stevenarella `616ee72`
- Child commits used for the survival reference parity refresh: Valence `7d13a24`, Stevenarella `d758630`; machine-recorded in the paired receipts.

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
| Survival break/place/pickup | Paper+Valence paired `survival-break-place-pickup` receipts | `a88fe547bfe2dd43fff3ac5bd967f0ebf5a3c539403211dd029865293130090b` |
| Survival chest persistence | Paper+Valence paired `survival-chest-persistence` receipts | `3dd16d3d15f47793505e97a088408d039c6cd45a73f288c7301c5e4f3f4851cf` |

## Inventory semantics matrix checkpoint

The maintained inventory rows are validated as a five-row bounded matrix in `docs/evidence/protocol-763-inventory-semantics-matrix-2026-05-27.md`. Covered rows are drop, pickup, player-inventory click, open-container click, and block placement/use-item-on-block. `docs/evidence/protocol-763-negative-live-rails-2026-05-29.md` adds bounded negative containment receipts for stale state-id and invalid slot/window probes, but these do not promote full inventory semantics, malformed-click breadth, stack splitting/merging, drag transactions, or all-window coverage.

## Equipment slot/item matrix checkpoint

The maintained equipment update row is validated as one bounded slot/item matrix row in `docs/evidence/protocol-763-equipment-slot-item-matrix-2026-05-27.md`. Covered row is `main_hand_remote_entity / slot4 / item id 829 / count 1 / non_empty_update` with one remote-spawn-correlated equipment update. All equipment slots/items/permutations remain a non-claim.

## Armor/enchantment/status modifier checkpoint

The maintained armor mitigation row is validated as one bounded modifier row in `docs/evidence/protocol-763-armor-modifier-matrix-2026-05-27.md` with a fresh live receipt/log bundle. Covered row is `armor_loadout_chest_only / DiamondChestplate / enchantment_none / status_effect_none / melee`; all armor loadouts, enchantments, status effects, modifier stacking, and exact vanilla parity remain non-claims.

## Projectile travel/collision checkpoint

The maintained projectile rows are validated as two bounded projectile state rows in `docs/evidence/protocol-763-projectile-travel-collision-2026-05-27.md`: projectile use/loadout and pinned projectile damage attribution. The damage row covers bounded server projectile hit/damage attribution; continuous projectile travel/collision simulation, all projectile weapons, and full projectile physics remain non-claims.

## Survival break/place/pickup checkpoint

The maintained survival parity row is validated by `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md` with paired Paper and Valence receipt/log bundles from committed child revisions. The paired receipts record `client.git_rev`, `client.git_status`, `valence.git_rev_requested`, and `valence.git_rev_resolved`; `docs/evidence/protocol-763-survival-child-revision-oracle-2026-05-28.md` remains as review history. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture with exact join/render, fixed-coordinate block break, pickup/inventory observation, and block placement metrics. Full survival compatibility and vanilla parity remain non-claims.

## Survival chest persistence checkpoint

The maintained chest persistence row is validated by `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture opening chest `8,64,0`, storing one `Dirt` item in slot `0`, closing, reconnecting once, reopening, and observing the same slot/item/count. `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` and `tools/check_survival_coverage_matrix.py` keep crafting, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence as explicit missing rows. `tools/check_survival_chest_persistence.rs` rejects Valence-only evidence and mismatched slot/item/count/position/session metrics, and passes the paired Paper/Valence bundle. Full survival compatibility, all-container behavior, restart/world persistence, and broad vanilla parity remain non-claims.

## Vanilla combat parity checkpoint

Vanilla combat parity is guarded by `docs/evidence/protocol-763-vanilla-combat-parity-2026-05-27.md` and `tools/check_vanilla_combat_parity.py`. No paired reference/Valence receipts exist today, so exact vanilla combat parity remains a non-claim and Valence-only evidence is rejected for parity promotion.

## CTF rule ledger checkpoint

CTF rule scope is guarded by `docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md` and `tools/check_ctf_rule_ledger.py`. Promoted clusters are bounded RED/BLUE scoring soak, flag-carrier death/return, and reconnect flag-state. `docs/evidence/protocol-763-negative-live-rails-2026-05-29.md` adds bounded wrong-score and reconnect-race containment receipts with no forbidden score/capture milestones, but full CTF correctness remains a non-claim.

## Broad protocol coverage checkpoint

Broad coverage is guarded by `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-28.md`, `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv`, `docs/evidence/protocol-763-broad-parser-fixtures-stevenarella-2026-05-28.md`, `docs/evidence/protocol-763-broad-parser-fixture-oracle-2026-05-28.md`, and `tools/check_protocol_coverage_ledger.py`. The ledger indexes 16 bounded seams plus 175 Valence protocol-763 packet rows, promotes only four high-risk parser-fixture-backed packet rows, and blocks full protocol-763 compatibility/full Minecraft compatibility until packet-family mapping/parser fixtures and live receipts exist for every claimed row.

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
python3 tools/check_equipment_slot_item_matrix.py
python3 tools/check_armor_modifier_matrix.py
python3 tools/check_projectile_travel_collision.py
python3 tools/check_vanilla_combat_parity.py
python3 tools/check_ctf_rule_ledger.py
python3 tools/check_protocol_coverage_ledger.py
python3 tools/check_survival_coverage_matrix.py
python3 tools/check_survival_reference_parity.py
./tools/check_survival_chest_persistence.rs --self-test
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

The runner receipt surface includes a `load_network_safety` block that records owned-local or explicit authorization, client/duration/reconnect/network bounds, telemetry readiness, and fail-closed diagnostics. Evidence: `docs/evidence/protocol-763-load-network-safety-2026-05-27.md` and `docs/evidence/protocol-763-production-network-safety-matrix-2026-05-28.md`.

The production/network matrix promotes only bounded owned-local loopback load safety. Broader production readiness, public-server safety, WAN safety, adversarial-network safety, packet-loss tolerance, and unbounded safety remain non-claims unless a future authorized bounded envelope has live telemetry, BLAKE3-backed evidence, and an updated matrix/bundle row.

## Reference parity labels

- `reference-parity-covered`: Survival break/place/pickup and chest persistence only.
- `valence-only-containment`: CTF scoring, inventory, combat, projectile, reconnect, latency/jitter, and load/network rows.
- `explicit-non-claim`: exact vanilla combat parity, broad survival, full Minecraft/CTF/protocol correctness, and survival rows not named as reference-parity covered.

Policy/checkpoint: `docs/evidence/protocol-763-reference-parity-expansion-2026-05-29.md`.

## Non-claims

This bundle still does not claim full Minecraft compatibility, full survival compatibility, broad vanilla parity, full CTF correctness, projectile travel/collision simulation, all projectile weapon variants, all equipment slots/items, all armor loadouts, enchantment/status-effect semantics, exact vanilla knockback/damage/mitigation balancing, all-container behavior, restart/world persistence, crafting/furnace/hunger/mob/redstone/biome/dimension coverage, production readiness, public-server load safety, or unbounded soak/reconnect/latency safety.
