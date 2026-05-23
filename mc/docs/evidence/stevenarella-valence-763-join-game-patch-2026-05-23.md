# Stevenarella / Valence protocol 763 join-game mapping patch — 2026-05-23

Receipt status: `pass`
Receipt BLAKE3: `d1744fb2f787e06579cf1054d17e263c97c925abd3a00d2721c013dbe0de3108`

## What changed
- Stevenarella fork commit: `475c4a0bc1f27b195738d28487eaf8930eda5bd1`
- Fork remote: `git@github.com:brittonr/stevenarella.git` (`master`)
- Protocol `763` now routes through a narrow `v1_20_1` translator.
- Patched boundary: `play/clientbound/0x28` maps to `JoinGame_WorldNames_IsHard_SimDist` instead of the inherited 758 `TradeList_WithRestock` mapping.

## Evidence
- Valence target: protocol `763` / Minecraft `1.20.1`, `ctf` example in offline mode.
- Direct trace artifact: `/tmp/valence-763-after-join-patch-trace.json`
- Direct trace BLAKE3: `491dd0be04aab2d162aa383800864c370a41a498c7d03cde17ca763af53b0dab`
- Stevenarella probe log: `/tmp/stevenarella-763-after-join-patch.log`
- Stevenarella probe log BLAKE3: `db7cdea68abd413bdd96d2c63194403d3657427debccfab8d896b32c5f0a876e`
- Probe classification: timeout-success evidence after matching `Detected server protocol version 763`.

## Packet boundary result
Captured sequence after login:

1. `login/clientbound/0x03` — `SetCompressionS2C`
2. `login/clientbound/0x02` — `LoginSuccessS2C`
3. `play/clientbound/0x28` — `GameJoinS2C` / patched to Stevenarella `JoinGame_WorldNames_IsHard_SimDist`
4. `play/clientbound/0x10` — next unpatched mismatch: Valence `CommandTreeS2CPacket`, but Stevenarella's inherited 758 table interprets `0x10` as `ClearTitles`.

## Verification
- `cargo test -p steven_protocol protocol::versions::tests -- --nocapture`: pass
- `cargo fmt --check`: pass
- `git diff --check`: pass before Stevenarella commit

## Contract
This evidence claims the first proven protocol-763 packet-id mismatch (`0x28`) is now mapped in Stevenarella. It does **not** claim full 1.20.1 client compatibility. The next focused patch is `play/clientbound/0x10` (`CommandTreeS2CPacket` vs inherited `ClearTitles`).
