# Proposal: Introduce Minecraft protocol I/O traits

## Why

The compatibility runner has protocol helpers for packet writing, strings, and VarInt read/write that are tied directly to `TcpStream` and `Vec<u8>`. That makes wire-format logic harder to test in isolation and mixes pure encoding decisions with network I/O. Extension traits over `Read` and `Write` can make protocol helpers reusable and testable with in-memory cursors.

## What Changes

- Add bounded `McRead` and `McWrite` extension traits for VarInt, string, and packet helpers used by the runner.
- Keep wire bytes, error messages, maximum VarInt length, and status-query behavior unchanged.
- Move pure encoding/decoding logic into functions testable with `Cursor<Vec<u8>>`; keep `TcpStream` only in the network shell.
- Add positive and negative tests for VarInt round trips, packet framing, string encoding, EOF, and too-long VarInts.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs` or a small runner protocol module, plus focused protocol I/O tests.
- **Testing**: in-memory protocol helper tests, status-query regression tests, negative malformed input fixtures, and Cairn validation/gates.