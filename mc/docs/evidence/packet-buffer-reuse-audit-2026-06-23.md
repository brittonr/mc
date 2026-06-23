# Packet encoder buffer reuse audit and contract

## Question

What packet encoder buffer reuse work is safe for the `optimize-packet-encoder-buffer-reuse` Cairn without changing Valence packet semantics or claiming broad compatibility/performance readiness?

## Inspected evidence

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion/src/net/encoder/mod.rs` (`PacketEncoder`, `append_packet_with_compression`, contiguous `Buf`, scratch/compressor inputs) | reference | `optimize-packet-encoder-buffer-reuse` | Informs the lifecycle shape: reserve/write into reusable storage, decide compression from an explicit threshold, and keep compression scratch separate. No Hyperion code is copied. | `servers/valence/crates/valence_protocol/src/encode.rs` | Hyperion code uses Hyperion `Buf`, `ScratchBuffer`, `libdeflater`, and project-specific packet bundle traits; Valence keeps stable-owned `PacketEncoder`, `PacketWriter`, and `flate2` behavior. | `docs/evidence/packet-buffer-reuse-final-component-tests-2026-06-23.run.log`; `docs/evidence/packet-buffer-reuse-post-bench-2026-06-23.run.log` | No Hyperion wire compatibility, no runtime/proxy adoption, no production-scale claim. |
| `hyperion/crates/hyperion/src/net/mod.rs` (`Compose`, `DataBundle`) and `hyperion/crates/hyperion/src/egress/channel.rs` (`compose.io_buf().encode_packet(...)`) | reference | `optimize-packet-encoder-buffer-reuse` | Shows egress code centralizing packet composition through a reusable encode helper before fanout. Valence keeps existing direct client writes and cached player-list writes. | `servers/valence/crates/valence_player_list/src/systems.rs` | Only the scratch reuse idea is ported; no ECS/runtime, transport, or broadcast semantics are copied. | `docs/evidence/packet-buffer-reuse-final-component-tests-2026-06-23.run.log` | No default route semantics change, no Hyperion compatibility claim. |
| `servers/valence/crates/valence_protocol/src/encode.rs` (`PacketEncoder`, `PacketWriter`) | port | `optimize-packet-encoder-buffer-reuse` | Valence already owned the encode path. The change hardens existing reusable encoder state against partial encode/compression failures and adds an opt-in `ReusablePacketWriter` scratch carrier for repeated compressed writes. | `valence_protocol::encode` | Pure encode decisions stay deterministic over packet length, compression threshold, packet limit, and explicit scratch state. I/O remains outside protocol encoding. Failed writes truncate to the pre-write length and clear compression scratch. | `docs/evidence/packet-buffer-reuse-final-component-tests-2026-06-23.run.log` | No protocol semantic change, no public-server safety claim. |
| `servers/valence/crates/valence_server/src/client.rs` (`Client::flush_packets`) and `servers/valence/crates/valence_server/src/packet_compose.rs` direct flush shell | port | `optimize-packet-encoder-buffer-reuse` | Confirms flush is the imperative shell: it takes encoded bytes and sends them to the connection. Added closed-client coverage proves failed sends do not leave stale bytes queued for reuse. | `valence_server::client` tests | Network I/O is not moved into the protocol core. Closed-client errors remain reported by the connection shell and the existing system removes failed clients. | `docs/evidence/packet-buffer-reuse-final-component-tests-2026-06-23.run.log`; `docs/evidence/packet-buffer-reuse-post-server-tests-2026-06-23.run.log` | No live compatibility or production readiness claim. |
| `servers/valence/benches/packet.rs` packet encode benchmark filter | reference | `optimize-packet-encoder-buffer-reuse` | Provides bounded before/after timing for the existing packet encode workloads named by packet mix and filter. | `servers/valence/benches/packet.rs` | Baseline and post runs use the same `packet::encode` filter and `DIVAN_MAX_TIME=0.2`. Existing `encode_*_compressed` functions set `CompressionThreshold(-1)`, so they are retained as existing workload labels rather than claimed compressed evidence. | `docs/evidence/packet-buffer-reuse-baseline-bench-2026-06-23.run.log`; `docs/evidence/packet-buffer-reuse-post-bench-2026-06-23.run.log` | No broad allocation-reduction claim beyond this bounded benchmark output. |

## Decision

Port a narrow Valence-owned implementation: keep `PacketEncoder` and `PacketWriter` public semantics, add explicit reset/discard behavior for partial encode failures, add `PacketEncodeScratch` plus `ReusablePacketWriter` for callers that encode many compressed packets, and wire that scratch into cached player-list packet updates. Do not add a global pool, thread-local pool, transport rewrite, or Hyperion runtime code.

## Buffer lifecycle and safety contract

- `PacketEncoder` remains per connection/client state. `append_packet` writes into the current buffer; `clear` keeps capacity for the next encode window; `take` transfers queued bytes to the flush shell.
- `PacketEncodeScratch` is caller-owned scratch. Callers may retain it across encode windows. It stores compression scratch only; it does not own socket state, packet ordering, connection identity, or compression threshold policy.
- Compression is enabled when `CompressionThreshold.0 >= 0`, preserving existing Valence behavior. Packets whose encoded length is greater than the threshold use the compressed branch; smaller packets get a zero data-length prefix in compression mode.
- Capacity policy is conservative: keep `BytesMut`/`Vec` capacity on successful clear/reuse, clear scratch length between writes, and discard state by dropping the encoder/scratch. No shrink policy is introduced in this change.
- Reset/discard policy: if packet body encoding, compression, prefix writing, or packet limit validation returns an error, truncate the destination to its pre-write length. For reusable compression scratch, also clear scratch length before reuse.
- Packet limit behavior continues to reject frames exceeding `MAX_PACKET_SIZE` with deterministic `packet exceeds maximum length` diagnostics. The oversized-packet regression verifies a following valid packet has no stale bytes.
- Flush shell behavior remains imperative: `Client::flush_packets` transfers bytes to the connection and reports backend errors. Closed-client failure drains the taken buffer before subsequent empty reuse.
- Raw `write_packet_bytes` remains caller-responsibility and is not upgraded into semantic validation.

## Target workloads and checks

- Target workloads: `valence_protocol` packet encode paths, compressed/uncompressed encode fixtures, cached player-list update packet encoding, direct client flush, direct-mode packet-compose flush, and the existing `packet::encode` divan benchmark filter.
- Compression boundaries: `CompressionThreshold::DEFAULT` no-compression path, `CompressionThreshold(0)` all-valid-packets-compressed test path, and existing packet decode validation after compressed error reuse.
- Protocol semantics unchanged: encoded bytes for a default valid packet still match `PacketWriter`; packet ID/body decode still round trips; direct flush ordering stays in existing packet-compose tests.

## Non-claims

This evidence does not claim broad Minecraft compatibility, vanilla/reference parity, production readiness, public-server safety, Hyperion compatibility, a global allocation reduction, or live gameplay correctness. The selected mc-compat smoke dry-run is command-shape evidence only.
