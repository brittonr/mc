# Stevenarella → Valence protocol 763 inventory probe (2026-05-23)

## Scope

Bounded single-client headless Stevenarella probe against Valence `ctf` after the prior protocol 763 team-selection/combat/death/respawn work.

This records a narrow inventory decode/observation slice after joining RED team. It does not claim full inventory behavior.

## Code under test

- Stevenarella fork commit: `73d6d4b stevenarella: add 763 ctf inventory probe`
- Parent mc base before this evidence: `da8d9cf mc: record 763 ctf respawn probe`
- Valence checkout: `c5140b7 valence: add parkour smoke receipts`

## Probe command shape

Valence `ctf` was started from `/home/brittonr/git/mc/valence` in the mc Nix devshell.

Stevenarella was run headlessly with:

```sh
MC_COMPAT_ACTIVE_PROBE=1 \
MC_COMPAT_TEAM_PROBE=1 \
MC_COMPAT_TEAM_PROBE_TEAM=red \
MC_COMPAT_INVENTORY_PROBE=1 \
xvfb-run -a cargo run --release -- \
  --server 127.0.0.1:25565 \
  --username InvProbeRed \
  --default-protocol-version 763
```

The client was bounded with `timeout 180s`; status `124` is the expected timeout after the observed milestones.

## Observed milestones

The probe log observed:

- `Detected server protocol version 763`
- `MC-COMPAT-MILESTONE login_success state=play protocol=763 username=InvProbeRed properties=0`
- `MC-COMPAT-MILESTONE join_game_763_shape dimension_type=minecraft:overworld world=minecraft:overworld portal_cooldown=0`
- `MC-COMPAT-MILESTONE render_tick_with_player`
- `Received chat message: You are on team RED!`
- `MC-COMPAT-MILESTONE inventory_probe_current_hotbar_slot slot=0`
- `MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=36 item=id=777 count=1`
- `MC-COMPAT-MILESTONE inventory_probe_slot36_nonempty count=1 item_id=777`
- `MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=37 item=id=194 count=64`
- `MC-COMPAT-MILESTONE inventory_probe_slot37_stack count=64 item_id=194`

Valence `ctf.rs` assigns slot `36` to `WoodenSword` count `1` and slot `37` to team wool count `64` when the player selects a team. The probe therefore proves the bounded Stevenarella fork decoded and observed those Valence team-inventory slot updates for protocol 763 in this scenario.

## Runtime failure scan

The bounded probe log contained zero occurrences of:

- `UnexpectedEof`
- `FromUtf8Error`
- `failed to read packet`
- `Bad packet`
- `panic`
- `disconnect`

## Artifacts

- Receipt: `docs/evidence/stevenarella-valence-763-inventory-2026-05-23.receipt.json`
- Receipt BLAKE3: `711b379d839c472cc675c2191a103203682c26664f423fba453b2ace56ebed4f`
- Source temp log BLAKE3: `7b5098e6e27a9f6c7da01bbe04f237a81faa1c24f9b3080a57822b0f5b8aa7c1`
- Source temp status BLAKE3: `16d78a954cb775e521c46341a98c2c6a187ea74a6d018de5efbae328941e87e6`

## Non-claims

This evidence does **not** prove:

- full inventory semantics,
- inventory click/drag/drop behavior,
- item pickup/drop semantics,
- equipment correctness beyond this bounded observation,
- flag capture/scoring/objective semantics,
- reconnect/session behavior,
- stable gameplay or long soak,
- full Minecraft 1.20.1 compatibility,
- complete protocol 763 coverage.
