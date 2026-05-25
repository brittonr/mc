# Stevenarella ⇄ Valence protocol-763 inventory interaction receipt (2026-05-25)

## Scope

Bounded single-client Stevenarella probe against the local Valence `ctf` example, protocol `763` / Minecraft `1.20.1`, after RED team selection.

This slice exercises a new gameplay/protocol seam beyond CTF scoring: inventory slot updates, hotbar selection, a serverbound drop-item action, a serverbound block-placement action, Valence-side drop-item event correlation, and Valence-side block-placement event correlation. It does **not** claim full inventory correctness, pickup semantics, full CTF correctness, broad Minecraft compatibility, unbounded soak, or production load safety.

## Command

```sh
cd /home/brittonr/git/mc
nix run .#mc-compat-smoke -- --stop || true
if [ -e /tmp/valence-compat-763/.git ]; then
  git -C /home/brittonr/git/mc/valence worktree remove --force /tmp/valence-compat-763 || rm -rf /tmp/valence-compat-763
fi
rm -rf target/mc-compat-block-place target-mc-compat-block-place-live.log
MC_COMPAT_INVENTORY_RECEIPT=target/mc-compat-block-place/block-place.json \
  VALENCE_REV=HEAD CLIENT_TIMEOUT=180 \
  nix run .#mc-compat-valence-ctf-inventory-interaction > target-mc-compat-block-place-live.log 2>&1
```

Important pitfall: Valence worktree `.git` is a file, not a directory; stale worktree cleanup must test `-e`, not `-d`, or the runner can reuse an older detached Valence checkout.

## Code under test

- Stevenarella fork: `ca45a2e` (`master...fork/master`)
- Valence fork: `9e21a61` (`main...fork/main`)
- Parent harness/docs: this evidence commit

## Receipt

- Path: `target/mc-compat-block-place/block-place.json`
- Schema: `mc.compat.scenario.receipt.v2`
- BLAKE3: `9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122`
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
- `inventory_block_place_sent`

Observed server correlation:

- `server_username_seen`
- `server_inventory_hotbar_select`
- `server_inventory_drop`
- `server_block_place`

Valence log excerpt:

```text
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0 source=team_inventory_setup
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0
MC-COMPAT-MILESTONE inventory_drop_item username=compatbot from_slot=36 item=WoodenSword count=1
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=1
MC-COMPAT-MILESTONE block_place_item username=compatbot item=RedWool from_slot=37 block=RedWool at=-40,65,0
```

Stevenarella log excerpt:

```text
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=36 item=id=777 count=1
MC-COMPAT-MILESTONE inventory_probe_slot36_nonempty count=1 item_id=777
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=37 item=id=194 count=64
MC-COMPAT-MILESTONE inventory_probe_slot37_stack count=64 item_id=194
MC-COMPAT-MILESTONE inventory_probe_select_hotbar_slot slot=0
MC-COMPAT-MILESTONE inventory_probe_drop_item_sent status=drop_item slot=36 sequence=77
MC-COMPAT-MILESTONE inventory_probe_select_wool_hotbar_slot slot=1
MC-COMPAT-MILESTONE inventory_probe_place_block_sent hand=main location=-40,64,0 face=up sequence=88
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=2 slot=37 item=id=194 count=63
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

The gate verifies the maintained app shape and receipt fields, including `server_inventory_drop` and `server_block_place`, without starting the live Valence/Stevenarella scenario.
