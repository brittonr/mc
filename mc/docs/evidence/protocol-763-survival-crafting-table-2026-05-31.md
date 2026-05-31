# Protocol-763 survival crafting table evidence — 2026-05-31

## Scope

This evidence promotes only the bounded `survival-crafting-table` row: one owned local client opens one crafting table at `4,64,0`, sends two `OakPlanks` inputs to slots `1` and `4`, observes `minecraft:stick` / `Stick x4` in result slot `0`, collects the result, and observes the crafted stack in inventory slot `36`.

It does not claim full crafting coverage, recipe-book behavior, shift-click/drag/split semantics, all recipe variants, furnace/hunger/mob/redstone/biome/dimension/world-persistence behavior, broad survival compatibility, broad vanilla parity, production readiness, or unbounded load/reconnect safety.

## Artifacts

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Paper receipt | `docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.receipt.json` | `710f64a04451a62604d17a78cc84f3e2db84ec3d7034b7feaa149b1e8af57a15` |
| Paper client log | `docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.client.log` | `3b73de5a559f9665510e7d540f3923c234aa502b90e3e3c903e21e75f47fa0f2` |
| Paper server log | `docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.server.log` | `92b355166cf9028cc3c477f6b5c854a696a27085bbd8e804f45ac10f3da73adf` |
| Paper run log | `docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.run.log` | `1115c9231f2889e8ce8b38f90850d796ce89e534889d77e7f618fea00cc28dfa` |
| Valence receipt | `docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.receipt.json` | `59a44542ccae0bb2af696227b79c4bbc3e7dc696bc026a44cd14c04e6d0e0c61` |
| Valence client log | `docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.client.log` | `155602432ba17b7a7e5ca221f44c3bc8ff4767787076bd1949a3238b7aa39efd` |
| Valence server log | `docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.server.log` | `0cba929c15ddee5bfd00183e668421c2dd0090e33d9632e09297ed75514a537d` |
| Valence run log | `docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.run.log` | `d7b88550ab38b5a236a847e7017422985920a9d04b8047b90e5319bfb3538ee2` |
| Checker log | `docs/evidence/protocol-763-survival-crafting-table-2026-05-31.checker.log` | `9c1ab8ce22b40dc25bb9136e9364fbdec018235394d4ce1b3fbaa1f636075bc5` |
| Validation log | `docs/evidence/protocol-763-survival-crafting-table-validation-2026-05-31.run.log` | `22caafb551667aa870a97eff74485ce57698c10368ff823267cb2b9aa2017546` |
| Paper fixture JAR | `docs/evidence/mc-compat-paper-survival-crafting-fixture-2026-05-31.jar` | `2caca95d561df91e1c21a29580f023437e048d218531b7ebe54d9fb389f7375b` |
| Valence live source patch | `docs/evidence/protocol-763-survival-crafting-table-valence-live-source-2026-05-31.patch` | `f4679fcaff8883a458ae3f7b27b764cb88288ecaa49570c451ad1e9ae4f0267b` |
| Stevenarella live source patch | `docs/evidence/protocol-763-survival-crafting-table-client-live-source-2026-05-31.patch` | `fe6a051c653d8173369302c9e01b151e110d329772f2e030ccf355954d371c47` |
| BLAKE3 manifest | `docs/evidence/protocol-763-survival-crafting-table-2026-05-31.b3` | manifest |

## Live commands

Paper reference fixture:

```sh
MC_COMPAT_IGNORE_DECODE_ERRORS=1 \
SERVER_PROTOCOL=763 \
SERVER_VERSION=1.20.1 \
SERVER_NAME=mc-compat-survival-crafting-paper-20260531 \
PAPER_PLUGIN_JAR=docs/evidence/mc-compat-paper-survival-crafting-fixture-2026-05-31.jar \
CLIENT_TIMEOUT=90 \
SMOKE_RECEIPT=target/mc-compat-survival-crafting-table-receipts/paper-survival-crafting-table-2026-05-31.receipt.json \
nix run --no-update-lock-file .#mc-compat-smoke -- \
  --run --keep-server --server-backend paper \
  --scenario survival-crafting-table \
  --receipt target/mc-compat-survival-crafting-table-receipts/paper-survival-crafting-table-2026-05-31.receipt.json
```

Valence fixture:

```sh
MC_COMPAT_IGNORE_DECODE_ERRORS=1 \
SERVER_PROTOCOL=763 \
SERVER_VERSION=1.20.1 \
VALENCE_REPO=/home/brittonr/git/mc/valence \
VALENCE_REV=3359f855e9fa01f3c924b84adaf10727ea0a67b5 \
VALENCE_EXAMPLE=survival_compat \
VALENCE_WORKTREE=/tmp/valence-compat-survival-crafting-3359f855e9fa01f3c924b84adaf10727ea0a67b5-live \
VALENCE_TARGET_DIR=/tmp/valence-compat-survival-crafting-target \
CLIENT_TIMEOUT=60 \
SMOKE_RECEIPT=target/mc-compat-survival-crafting-table-receipts/valence-survival-crafting-table-2026-05-31.receipt.json \
nix run --no-update-lock-file .#mc-compat-smoke -- \
  --run --server-backend valence \
  --scenario survival-crafting-table \
  --receipt target/mc-compat-survival-crafting-table-receipts/valence-survival-crafting-table-2026-05-31.receipt.json
```

Pair checker:

```sh
CARGO_TARGET_DIR=/tmp/mc-cargo-script-target ./tools/check_survival_crafting_table.rs \
  --reference-receipt docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.receipt.json \
  --reference-client-log docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.client.log \
  --reference-server-log docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.server.log \
  --valence-receipt docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.receipt.json \
  --valence-client-log docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.client.log \
  --valence-server-log docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.server.log
```

Checker output: `survival crafting table contract ok: 72 metrics`.

Validation output records checker self-tests, paired comparator, Valence dry-run wrapper, acceptance/current-bundle/survival-coverage checks, evidence manifest check (`evidence manifests ok: 137 manifests, 644 entries, 71 receipts scanned`), task gate `valid=true`, and Cairn validation `valid=true`.

## Result

Both receipts have `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, empty missing milestone lists, and empty forbidden match lists.

Observed paired metrics include:

- `client.crafting.open.position=4,64,0`
- `client.crafting.input_a.slot=1`, `item=OakPlanks`, `count=1`
- `client.crafting.input_b.slot=4`, `item=OakPlanks`, `count=1`
- `client.crafting.result.slot=0`, `item=Stick`, `count=4`, `recipe=minecraft:stick`
- `client.crafting.collect.slot=0`, `item=Stick`, `count=4`
- `client.crafting.inventory.slot=36`, `item=Stick`, `count=4`
- `server.crafting.open.position=4,64,0`
- `server.crafting.input_a.slot=1`, `item=OakPlanks`, `count=1`
- `server.crafting.input_b.slot=4`, `item=OakPlanks`, `count=1`
- `server.crafting.result.slot=0`, `item=Stick`, `count=4`, `recipe=minecraft:stick`
- `server.crafting.collect.slot=0`, `item=Stick`, `count=4`, `inventory_slot=36`

Recorded child revisions:

- Valence fixture receipt: `3359f855e9fa01f3c924b84adaf10727ea0a67b5` (`git_rev_requested` equals `git_rev_resolved`).
- Stevenarella client receipt: `4d1b1554650bd91924f7ce99c9dab69a91142edc` with clean client status.
- Paper fixture source is tracked in this parent change; generated plugin JAR BLAKE3 is `2caca95d561df91e1c21a29580f023437e048d218531b7ebe54d9fb389f7375b`.

## Non-claims

This row is not evidence for full crafting coverage, recipe-book behavior, shift-click/drag/split semantics, all recipe variants, all-container behavior, furnace persistence, hunger/food, mob drops, redstone, biome/dimension behavior, world persistence, broad survival compatibility, broad vanilla parity, production readiness, or unbounded reconnect/load safety.
