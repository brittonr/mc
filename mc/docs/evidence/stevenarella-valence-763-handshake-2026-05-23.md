# Stevenarella → Valence protocol 763 handshake probe

Date (UTC): 2026-05-23

## Summary

A narrow Stevenarella discovery patch gets past the previous hard stop against current Valence main:

- Valence checkout: `c5140b7`
- Valence advertised Minecraft version/protocol: `1.20.1` / `763`
- Stevenarella base checkout: `815ac88`
- Stevenarella patch commit: `ff04368`
- Stevenarella fork remote: `git@github.com:brittonr/stevenarella.git` (`master`)
- Stevenarella patch: advertise protocol `763`, accept `1.20.1`, and route packet-id translation through the existing `1.18.2` / protocol `758` translator.
- Result: headless Stevenarella reached `Detected server protocol version 763` against live Valence `parkour`.

This is **not** full 1.20.1 protocol support. It only proves the first handshake/status boundary no longer panics on `unsupported protocol version: 763`.

## Live probe

Command shape:

```sh
timeout 20 xvfb-run ... stevenarella \
  --server 127.0.0.1:25565 \
  --username compatbot \
  --default-protocol-version 763
```

Observed client log excerpt:

```text
[main.rs:254][INFO] Starting steven
[main.rs:99][INFO] Detected server protocol version 763
```

The process timed out after the bounded 20 second window (`exit_code: 124`), matching the existing smoke-harness success classification when a success pattern is observed before timeout.

## Verification

- Unit tests: `cargo test -p steven_protocol protocol::versions::tests -- --nocapture` → pass (`2 passed`)
- Headless live probe: matched `Detected server protocol version 763`
- Client log BLAKE3: `ea08578b1ff24a0e7e21e26518fa7330586ea13b5b8808412d50a7d0bba4aff7`
- Receipt: `docs/evidence/stevenarella-valence-763-handshake-2026-05-23.receipt.json`
- Receipt BLAKE3: `6ac94c57ee0d7056e86119e52316c8e0b0e6b4d178f74e202609e757b70d6285`

## Non-claims

This evidence intentionally does **not** claim:

- Full current-Valence client compatibility.
- Correct protocol `763` packet layouts.
- Semantic/gameplay correctness.
- That aliasing protocol `763` to protocol `758` translation is more than a discovery shim.
