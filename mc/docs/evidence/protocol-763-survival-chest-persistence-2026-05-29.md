# Protocol-763 survival chest persistence evidence — 2026-05-29

## Scope

This evidence promotes only the bounded `survival-chest-persistence` row: one owned local client opens one chest at `8,64,0`, stores one `Dirt` item in slot `0`, closes, reconnects once as the same username, reopens the same chest, and observes the same item/count persisted in the slot.

It does not claim full survival compatibility, all-container behavior, server restart/world persistence, broad vanilla parity, production readiness, or unbounded reconnect/load safety.

## Artifacts

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Paper receipt | `docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.receipt.json` | `3dd16d3d15f47793505e97a088408d039c6cd45a73f288c7301c5e4f3f4851cf` |
| Paper client log | `docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.client.log` | `95c4c14d65f2efbf717882a423ce24844c9011345a5cef04037b1b2af5dae0c9` |
| Paper server log | `docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.server.log` | `888a673a4217b9d06178fcfcc981ecd0dcd4ebbca0f74363d50d2567a51077cb` |
| Paper run log | `docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.run.log` | `9f232bd99129b45f412230af05648e44cea1de520b4856193f125ba80d7b9f0f` |
| Valence receipt | `docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.receipt.json` | `6c77d94ae2512b7e14f4ebb1a3419c6cba29d56cedc7a3a821ac53336d58b086` |
| Valence client log | `docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.client.log` | `d609f490ecd64859210283d79d4dbbbd710958a4081684354aaf4fc97dcc948a` |
| Valence server log | `docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.server.log` | `e8a883098c723e6338f33de76263b2e18a89acc2639dc06915ddd594432bb66e` |
| Valence run log | `docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.run.log` | `b5b0d9e0cbd50964fcbfd555a22671fc85d60fe42b1ea902ee5101502bdb2ff4` |
| Checker log | `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.checker.log` | recorded in `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.b3` |
| Cairn validation log | `docs/evidence/protocol-763-survival-chest-persistence-validation-2026-05-29.run.log` | recorded in `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.b3` |
| BLAKE3 manifest | `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.b3` | manifest |

## Live commands

Paper reference fixture:

```sh
MC_COMPAT_IGNORE_DECODE_ERRORS=1 \
SERVER_PROTOCOL=763 \
SERVER_VERSION=1.20.1 \
PAPER_PLUGIN_JAR=target/mc-compat-survival-reference-parity/mc-compat-paper-survival-fixture.jar \
CLIENT_TIMEOUT=45 \
SMOKE_RECEIPT=target/mc-compat-survival-chest-persistence/paper-survival-chest-persistence-keep-2026-05-29.receipt.json \
nix run --no-update-lock-file .#mc-compat-smoke -- \
  --run --keep-server --server-backend paper \
  --scenario survival-chest-persistence \
  --receipt target/mc-compat-survival-chest-persistence/paper-survival-chest-persistence-keep-2026-05-29.receipt.json
```

Valence fixture:

```sh
MC_COMPAT_IGNORE_DECODE_ERRORS=1 \
SERVER_PROTOCOL=763 \
SERVER_VERSION=1.20.1 \
VALENCE_REPO=/home/brittonr/git/mc/valence \
VALENCE_REV=5a0fdfed94656b0308b03b7ca74ba7bc214bb01e \
VALENCE_EXAMPLE=survival_compat \
VALENCE_WORKTREE=/tmp/valence-compat-survival-chest-5a0fdfe \
VALENCE_TARGET_DIR=/tmp/valence-compat-survival-chest-target \
CLIENT_TIMEOUT=25 \
SMOKE_RECEIPT=target/mc-compat-survival-chest-persistence/valence-survival-chest-persistence-2026-05-29.receipt.json \
nix run --no-update-lock-file .#mc-compat-smoke -- \
  --run --server-backend valence \
  --scenario survival-chest-persistence \
  --receipt target/mc-compat-survival-chest-persistence/valence-survival-chest-persistence-2026-05-29.receipt.json
```

Pair checker:

```sh
./tools/check_survival_chest_persistence.rs \
  --reference-receipt docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.receipt.json \
  --reference-client-log docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.client.log \
  --reference-server-log docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.server.log \
  --valence-receipt docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.receipt.json \
  --valence-client-log docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.client.log \
  --valence-server-log docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.server.log
```

Checker output: `survival chest persistence contract ok: 48 metrics`.

Cairn validation output records `valid=true`, `issues=[]`, `change_issues=[]`, and `spec_issues=[]`.

## Result

Both receipts have `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, `server.passed=true`, empty missing milestone lists, and empty forbidden match lists.

Observed paired metrics include:

- `client.chest.open.position=8,64,0`
- `client.chest.store.slot=0`, `item=Dirt`, `count=1`
- `client.chest.close.window=1`
- `client.chest.reconnect.session=1`
- `client.chest.reopen.position=8,64,0`
- `client.chest.persisted.slot=0`, `item=Dirt`, `count=1`
- `server.chest.open/store/close/reopen/persisted` milestones for the same fixed chest slot/item/count.

Recorded child revisions:

- Valence fixture receipt: `5a0fdfed94656b0308b03b7ca74ba7bc214bb01e`.
- Stevenarella client receipt: `9fced11170764e4e24e7b8ca0d7999eb24c4f9fb`.
- Paper fixture source is tracked in this parent change; generated plugin JAR BLAKE3 is `fb1b8aeb4009adaa33bd1915c4987046a5289ce97bc0e0d589af9fc5d78c344f`.

## Non-claims

This row is not evidence for all containers, multi-slot transactions, shift-click/drag/split semantics, server restart persistence, world persistence across process restart, crafting/furnace/hunger/mob/redstone/biome/dimension behavior, broad survival compatibility, broad vanilla parity, production readiness, or unbounded reconnect/load safety.
