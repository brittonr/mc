# Stevenarella / Valence protocol 763 instrumented login milestones — 2026-05-23

Receipt BLAKE3: `c4ef10483b6a84f4fa5ddfdea6a9cf1b1d94d5ce279fd7ad13796ef6284c5689`

## Summary

This slice converted the prior bounded runtime smoke into explicit Stevenarella milestone logs and found a concrete next boundary.

Against Valence `ctf` on protocol `763`, Stevenarella fork commit `10e4562` reached:

1. `Detected server protocol version 763`
2. `MC-COMPAT-MILESTONE connect_start`
3. `MC-COMPAT-MILESTONE tcp_connected`
4. `MC-COMPAT-MILESTONE handshake_sent next=login`
5. `MC-COMPAT-MILESTONE login_start_sent`
6. `MC-COMPAT-MILESTONE login_compression threshold=256`
7. `MC-COMPAT-MILESTONE login_success state=play protocol=763 username=... properties=0`

It did **not** reach `join_game`, `first_chunk_data`, or `render_tick_with_player`.
The first observed runtime failure after login success was:

```text
UnexpectedEof while reading the first play-state packet after login_success
```

## Stevenarella fork updates

Commit `10e4562` adds narrow protocol-763 login/milestone support:

- Sends protocol-763 login start with the 1.19+ optional UUID presence field set to false.
- Parses protocol-763 login success with the 1.19+ property array.
- Logs milestone markers at connection, login, join-game, first chunk, and render-with-player seams.
- Adds protocol translation regression tests for the login start and login success wire IDs.

## Verification

Commands run:

```sh
nix develop path:/home/brittonr/git/mc -c bash -lc 'cd /home/brittonr/git/mc/stevenarella && cargo fmt --check && CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo test -p steven_protocol protocol::versions::tests -- --nocapture && CARGO_TARGET_DIR=/tmp/stevenarella-target2 cargo check -p stevenarella'
```

Result: `cargo fmt --check` passed, protocol versions tests reported `12 passed`, and `cargo check -p stevenarella` passed.

Live probe:

```sh
nix develop path:/home/brittonr/git/mc -c bash -lc 'cd /home/brittonr/git/mc/stevenarella && timeout 90s xvfb-run -a cargo run --release -- --network-debug --server 127.0.0.1:25565 --username steve763ms'
```

Result: `exit=101` after `login_success`, with `UnexpectedEof` before `join_game`.

Backtrace probe with `RUST_BACKTRACE=1` reproduced the same failure.

## Non-claims

- Does not prove in-world gameplay success.
- Does not prove chunk/world/render-loop correctness.
- Does not prove full Minecraft 1.20.1 / protocol 763 support.
- Does not prove full current Valence client compatibility.
- Does not prove semantic parser correctness for the first play-state packet.

## Next boundary

The next high-ROI boundary is the first play-state packet immediately after protocol-763 login success. The probe should identify whether the `UnexpectedEof` is a packet-shape mismatch in the current first play packet or a missing post-login transition packet/ack.
