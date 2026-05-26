# Stevenarella ⇄ Valence protocol-763 inventory interaction receipt (2026-05-25)

## Scope

Bounded single-client Stevenarella probe against the local Valence `ctf` example, protocol `763` / Minecraft `1.20.1`, after RED team selection.

This slice exercises a gameplay/protocol seam beyond CTF scoring: inventory slot updates, hotbar selection, a serverbound drop-item action, a client-observed pickup animation, a serverbound validated player-inventory click-slot action, a Valence-opened non-player container/window, a serverbound validated click inside that open container, a serverbound block-placement action, and Valence-side correlation for all of those events. It does **not** claim full inventory correctness, natural item-entity physics, full CTF correctness, broad Minecraft compatibility, unbounded soak, or production load safety.

## Command

```sh
cd /home/brittonr/git/mc
if [ -e /tmp/valence-compat-763/.git ]; then
  git -C /home/brittonr/git/mc/valence worktree remove --force /tmp/valence-compat-763 || rm -rf /tmp/valence-compat-763
fi
rm -f /tmp/mc-compat-valence.log target-mc-compat-open-container-live.log target/mc-compat-open-container/open-container.json
MC_COMPAT_INVENTORY_RECEIPT=target/mc-compat-open-container/open-container.json \
VALENCE_REV=main \
VALENCE_WORKTREE=/tmp/valence-compat-763 \
VALENCE_TARGET_DIR=/tmp/valence-compat-763-target \
CLIENT_TIMEOUT=160 \
nix run .#mc-compat-valence-ctf-inventory-interaction \
  > target-mc-compat-open-container-live.log 2>&1
```

Important pitfall: Valence worktree `.git` is a file, not a directory; stale worktree cleanup must test `-e`, not `-d`, or the runner can reuse an older detached Valence checkout.

## Code under test

- Stevenarella fork: `b7a48ab` (`master...fork/master`)
- Valence fork: `f82abd3` (`main...fork/main`)
- Parent harness/docs: this evidence commit

## Receipt

- Path: `target/mc-compat-open-container/open-container.json`
- Schema: `mc.compat.scenario.receipt.v2`
- BLAKE3: `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3`
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
- `inventory_open_container_seen`
- `inventory_container_click_sent`
- `inventory_block_place_sent`

Observed server correlation:

- `server_username_seen`
- `server_inventory_hotbar_select`
- `server_inventory_drop`
- `server_inventory_pickup`
- `server_inventory_click`
- `server_inventory_open_container`
- `server_inventory_container_click`
- `server_block_place`

Valence log excerpt:

```text
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0 source=team_inventory_setup
MC-COMPAT-MILESTONE inventory_drop_item username=compatbot from_slot=36 item=WoodenSword count=1
MC-COMPAT-MILESTONE inventory_pickup_item username=compatbot from_slot=36 item=WoodenSword count=1 collected_entity_id=7630036 collector_entity_id=3
MC-COMPAT-MILESTONE block_place_item username=compatbot item=RedWool from_slot=37 block=RedWool at=-40,65,0
MC-COMPAT-MILESTONE inventory_click_slot username=compatbot window=0 slot=37 button=0 mode=Click carried_item=RedWool count=63 slot_changes=1
MC-COMPAT-MILESTONE inventory_open_container username=compatbot kind=Generic3x3 trigger=inventory_click_slot
MC-COMPAT-MILESTONE inventory_click_slot username=compatbot window=1 slot=0 button=0 mode=Click carried_item=Air count=0 slot_changes=1
MC-COMPAT-MILESTONE inventory_container_click username=compatbot window=1 slot=0 button=0 mode=Click carried_item=Air count=0 slot_changes=1
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
MC-COMPAT-MILESTONE inventory_probe_open_container window=1 type=6 title=Inventory
MC-COMPAT-MILESTONE inventory_probe_window_items window=1 state_id=2 slots=9 slot36=missing slot37=missing carried=id=194 count=63
MC-COMPAT-MILESTONE inventory_probe_container_items window=1 state_id=2 slots=9
MC-COMPAT-MILESTONE inventory_probe_container_click_sent window=1 slot=0 state_id=2 button=0 mode=click carried_item=empty slot_item=RedWool count=63
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

The gate verifies the maintained app shape and receipt fields, including `inventory_pickup_seen`, `inventory_click_sent`, `inventory_open_container_seen`, `inventory_container_click_sent`, `server_inventory_drop`, `server_inventory_pickup`, `server_inventory_click`, `server_inventory_open_container`, `server_inventory_container_click`, and `server_block_place`, without starting the live Valence/Stevenarella scenario.
