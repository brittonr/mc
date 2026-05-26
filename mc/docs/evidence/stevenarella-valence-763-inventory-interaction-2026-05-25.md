# Stevenarella ⇄ Valence protocol-763 inventory interaction receipt (2026-05-25)

## Scope

Bounded single-client Stevenarella probe against the local Valence `ctf` example, protocol `763` / Minecraft `1.20.1`, after RED team selection.

This slice exercises a gameplay/protocol seam beyond CTF scoring: inventory slot updates, hotbar selection, a serverbound drop-item action, a client-observed pickup animation, a serverbound validated click-slot action, a serverbound block-placement action, Valence-side drop-item event correlation, Valence-side pickup correlation, Valence-side click-slot event correlation, and Valence-side block-placement event correlation. It does **not** claim full inventory correctness, natural item-entity physics, full CTF correctness, broad Minecraft compatibility, unbounded soak, or production load safety.

## Command

```sh
cd /home/brittonr/git/mc
nix run .#mc-compat-smoke -- --stop || true
if [ -e /tmp/valence-compat-763/.git ]; then
  git -C /home/brittonr/git/mc/valence worktree remove --force /tmp/valence-compat-763 || rm -rf /tmp/valence-compat-763
fi
rm -rf target/mc-compat-click target-mc-compat-click-live.log
MC_COMPAT_RECEIPT_PATH=target/mc-compat-click/click.json \
MC_COMPAT_TIMEOUT_SECS=90 \
nix run .#mc-compat-valence-ctf-inventory-interaction -- --receipt target/mc-compat-click/click.json \
  > target-mc-compat-click-live.log 2>&1
```

Important pitfall: Valence worktree `.git` is a file, not a directory; stale worktree cleanup must test `-e`, not `-d`, or the runner can reuse an older detached Valence checkout.

## Code under test

- Stevenarella fork: `72ab57e` (`master...fork/master`)
- Valence fork: `7d2afee` (`main...fork/main`)
- Parent harness/docs: this evidence commit

## Receipt

- Path: `target/mc-compat-click/click.json`
- Schema: `mc.compat.scenario.receipt.v2`
- BLAKE3: `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d`
- Status: `pass`
- Client exit classification: `timeout-success-evidence` (`exit_code=124` after bounded evidence window)

Observed client milestones:

- `protocol_detected`
- `join_game`
- `render_tick`
- `team_red`
- `inventory_slot_update`
- `inventory_sword_slot`
- `inventory_wool_slot`
- `inventory_drop_sent`
- `inventory_pickup_seen`
- `inventory_click_sent`
- `inventory_block_place_sent`

Observed server correlation:

- `server_username_seen`
- `server_inventory_hotbar_select`
- `server_inventory_drop`
- `server_inventory_pickup`
- `server_inventory_click`
- `server_block_place`

Valence log excerpt:

```text
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0 source=team_inventory_setup
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0
MC-COMPAT-MILESTONE inventory_drop_item username=compatbot from_slot=36 item=WoodenSword count=1
MC-COMPAT-MILESTONE inventory_pickup_item username=compatbot from_slot=36 item=WoodenSword count=1 collected_entity_id=7630036 collector_entity_id=3
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=1
MC-COMPAT-MILESTONE block_place_item username=compatbot item=RedWool from_slot=37 block=RedWool at=-40,65,0
MC-COMPAT-MILESTONE inventory_click_slot username=compatbot window=0 slot=37 button=0 mode=Click carried_item=RedWool count=63 slot_changes=1
```

Stevenarella log excerpt:

```text
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=36 item=id=777 count=1
MC-COMPAT-MILESTONE inventory_probe_slot36_nonempty count=1 item_id=777
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=37 item=id=194 count=64
MC-COMPAT-MILESTONE inventory_probe_slot37_stack count=64 item_id=194
MC-COMPAT-MILESTONE inventory_probe_select_hotbar_slot slot=0
MC-COMPAT-MILESTONE inventory_probe_drop_item_sent status=drop_item slot=36 sequence=77
MC-COMPAT-MILESTONE inventory_probe_collect_item collected_entity_id=7630036 collector_entity_id=3 count=1
MC-COMPAT-MILESTONE inventory_probe_select_wool_hotbar_slot slot=1
MC-COMPAT-MILESTONE inventory_probe_place_block_sent hand=main location=-40,64,0 face=up sequence=88
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=2 slot=37 item=id=194 count=63
MC-COMPAT-MILESTONE inventory_probe_click_slot_sent window=0 slot=37 state_id=2 button=0 mode=click carried_item=RedWool count=63
```

## Hygiene scan

Checked combined live runner/client/server logs for:

- `panic`: 0
- `unexpected_eof`: 0
- `protocol_mismatch`: 0
- `decode_error`: 0
- `disconnect`: 0
- `parser`: 0

## Deterministic gate

Focused dry-run gate:

```sh
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-inventory-interaction-dry-run --no-link -L
```

The gate verifies the maintained app shape and receipt fields, including `inventory_pickup_seen`, `inventory_click_sent`, `server_inventory_drop`, `server_inventory_pickup`, `server_inventory_click`, and `server_block_place`, without starting the live Valence/Stevenarella scenario.
