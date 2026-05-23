# Stevenarella ↔ Valence protocol 763 packet boundary (2026-05-23)

## Summary

Focused protocol-763 probing against current Valence shows the existing Stevenarella 763 shim gets through the login packet IDs that are still compatible with the aliased 758 table, then hits the first real packet-boundary mismatch at the first play packet.

## Evidence

- Server: current Valence `ctf` example, `ConnectionMode::Offline`, protocol 763.
- Client trace: direct protocol probe sent handshake + `LoginHelloC2s` for protocol 763.
- Trace BLAKE3: `a32eef028ce239e13fed174820045664908cb73205ed667a0c8d97b8b522becf` (`/tmp/valence-763-offline-login-trace.json`).
- Stevenarella shim commit: `ff043683ad775443355c0671d8edd53ed3dc722e` on `git@github.com:brittonr/stevenarella.git` (`master`).

First captured packets:

| seq | state | direction | wire id | Valence 763 semantic | Stevenarella 758 alias semantic | Result |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | login | clientbound | `0x03` | `SetCompressionS2C` | `SetInitialCompression` | reused safely |
| 1 | login | clientbound | `0x02` | `LoginSuccessS2C` | `LoginSuccess_UUID` | reused at packet-id boundary |
| 2 | play | clientbound | `0x28` | `GameJoinS2C` | `TradeList_WithRestock` | **first mismatch** |

The concrete boundary is `play/clientbound/0x28`: Valence 763 names it `GameJoinS2CPacket` (`valence_generated/extracted/packets.json` id 40), while Stevenarella’s 763 shim currently routes through `v1_18_2.rs`, where `0x28 => TradeList_WithRestock`.

## Devshell/flake repair

`flake.nix` now carries the probe requirements in both the wrapped `mc-compat-runner` environment and the default devshell: cargo/rustc, `xvfb-run`/`xauth`, OpenSSL include/lib paths, fontconfig/freetype/expat pkg-config paths, X11/Wayland/GL library paths, software GL/X11 environment, empty `RUSTC_WRAPPER`, CMake policy compatibility, and `b3sum`.

Validated commands:

```text
nix develop path:/home/brittonr/git/mc -c bash -lc 'command -v cargo; command -v xvfb-run; pkg-config --exists openssl fontconfig freetype2 expat; test -d "$OPENSSL_INCLUDE_DIR"; test -d "$OPENSSL_LIB_DIR"'
nix build path:/home/brittonr/git/mc#mc-compat-runner --no-link -L
```

## Next patch

Add a real protocol-763 clientbound play mapping table rather than aliasing all play packets to 758. The first minimal target is mapping `play/clientbound/0x28` to the join-game/GameJoin structure and checking the 1.20.1 shape delta before advancing to later packets.

Receipt BLAKE3: `4346c573628d93972f034e4b6b3dcb22fd7df548d1b74ec6d7214b58b06c217e`
