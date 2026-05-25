# Stevenarella ⇄ Valence protocol-763 inventory interaction receipt (2026-05-25)

## Scope

Bounded single-client Stevenarella probe against the local Valence `ctf` example, protocol `763` / Minecraft `1.20.1`, after RED team selection.

This slice exercises a new gameplay/protocol seam beyond CTF scoring: inventory slot updates, hotbar selection, a serverbound drop-item action, and Valence-side drop-item event correlation. It does **not** claim full inventory correctness, pickup semantics, full CTF correctness, broad Minecraft compatibility, unbounded soak, or production load safety.

## Command

```sh
cd /home/brittonr/git/mc
nix run .#mc-compat-smoke -- --stop || true
if [ -e /tmp/valence-compat-763/.git ]; then
  git -C /home/brittonr/git/mc/valence worktree remove --force /tmp/valence-compat-763 || rm -rf /tmp/valence-compat-763
fi
rm -rf target/mc-compat-inventory-drop target-mc-compat-inventory-drop-live.log
MC_COMPAT_INVENTORY_RECEIPT=target/mc-compat-inventory-drop/inventory-drop-server.json \
  VALENCE_REV=HEAD CLIENT_TIMEOUT=120 \
  nix run .#mc-compat-valence-ctf-inventory-interaction > target-mc-compat-inventory-drop-live.log 2>&1
```

Important pitfall: Valence worktree `.git` is a file, not a directory; stale worktree cleanup must test `-e`, not `-d`, or the runner can reuse an older detached Valence checkout.

## Code under test

- Stevenarella fork: `f0e89e4` (`master...fork/master`)
- Valence fork: `f03f336` (`main...fork/main`)
- Parent harness/docs: this evidence commit

## Receipt

- Path: `target/mc-compat-inventory-drop/inventory-drop-server.json`
- Schema: `mc.compat.scenario.receipt.v2`
- BLAKE3: `4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46`
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

Observed server correlation:

- `server_username_seen`
- `server_inventory_hotbar_select`
- `server_inventory_drop`

Valence log excerpt:

```text
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0 source=team_inventory_setup
MC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0
MC-COMPAT-MILESTONE inventory_drop_item username=compatbot from_slot=36 item=WoodenSword count=1
```

Stevenarella log excerpt:

```text
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=36 item=id=777 count=1
MC-COMPAT-MILESTONE inventory_probe_slot36_nonempty count=1 item_id=777
MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=37 item=id=194 count=64
MC-COMPAT-MILESTONE inventory_probe_slot37_stack count=64 item_id=194
MC-COMPAT-MILESTONE inventory_probe_select_hotbar_slot slot=0
MC-COMPAT-MILESTONE inventory_probe_drop_item_sent status=drop_item slot=36 sequence=77
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

The gate verifies the maintained app shape and receipt fields, including `server_inventory_drop`, without starting the live Valence/Stevenarella scenario.
