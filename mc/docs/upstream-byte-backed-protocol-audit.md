# Upstream byte-backed protocol audit

Change: `upstream-byte-backed-protocol-path`

## Decision summary

Valence should own a stable, safe subset of Hyperion's byte-backed protocol path:

- shared packet-body ownership (`bytes::Bytes`) for decoded raw payloads;
- bounded byte fields sliced from a packet body after range and length checks;
- bounded UTF-8 fields sliced from a packet body after UTF-8 and Minecraft UTF-16 character-count checks;
- deterministic in-memory framing checks for complete, split, compressed, malformed, zero-length, oversized, and invalid-compression packet frames;
- a compatibility shim that keeps the existing Valence direct-mode `ReceivedPacket { id, body: Bytes, .. }` surface while allowing the network reader to pass through a byte-backed frame.

This is a Valence-owned port/reference implementation. No Hyperion code is copied, and the default Valence networking mode remains unchanged.

## Required capabilities

| Capability | Valence target | Required invariant |
| --- | --- | --- |
| Raw packet body | `valence_protocol::ByteBackedPacketBody`, `decode::ByteBackedPacketFrame` | Body excludes the leading packet ID, owns shared bytes, and can outlive decoder buffers. |
| Validated byte field | `valence_protocol::ByteBackedBytes` | Constructors check owner range and maximum byte length before exposing bytes. |
| Validated text field | `valence_protocol::ByteBackedStr` | Constructors check owner range, UTF-8 validity, and Minecraft UTF-16 character-count bounds before exposing `&str`. |
| Framing core | `valence_protocol::decode` helpers | Length VarInt parsing, zero-length rejection, packet-size limits, split-frame state, compression threshold checks, and packet-ID splitting are deterministic over in-memory buffers. |
| Direct-mode event-loop shim | `valence_server::client::ReceivedPacket`, `valence_network::packet_io` | Existing `body: Bytes` remains available while ingress builds packets from a byte-backed frame. |

## Hyperion integration inventory

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/Cargo.toml` workspace dependencies on `TestingPlant/valence` branch `feat-bytes` and `valence_bytes` | reference | `upstream-byte-backed-protocol-path` | Confirms Hyperion relies on forked byte-oriented Valence crates and a separate `valence_bytes` crate. | `docs/upstream-byte-backed-protocol-audit.md` | Dependency shape only; no fork code imported. | Audit doc and protocol tests. | Does not prove Hyperion binary compatibility or replace the fork. |
| `hyperion/crates/hyperion/src/net/decoder.rs` (`BorrowedPacketFrame`, `Either<Bytes, RawPacket>`, `DecodeBytes`) | port | `upstream-byte-backed-protocol-path` | The stable idea is shared ownership for decoded packet bodies; Valence ports this as `ByteBackedPacketFrame` over `bytes::Bytes`. | `servers/valence/crates/valence_protocol/src/decode.rs` | Safe Rust only; no `RawPacket` ownership escape; no libdeflater API commitment. | Positive complete/split/compressed decode tests and negative malformed-input tests. | Does not add Hyperion's `DecodeBytes` trait or static packet-event generation. |
| `hyperion/crates/packet-channel/src/lib.rs` (`RawPacket`, fragment channel, unsafe shared fragments) | reference | `upstream-byte-backed-protocol-path` | Identifies the reason Hyperion needs byte-backed ownership across ECS ingress. | none | Nightly features and unsafe fragment internals are not adopted. Valence keeps Tokio/direct-mode shells. | Audit doc and direct-mode regression checks. | Does not port Hyperion packet-channel, proxy runtime, or SPSC fragment storage. |
| `hyperion/crates/hyperion/src/ingress/decode.rs` (ECS event batching from packet frames) | reference | `upstream-byte-backed-protocol-path` | Informs migration risks for event-loop packet surfaces. | `servers/valence/crates/valence_network/src/packet_io.rs`, `servers/valence/crates/valence_server/src/client.rs` | Valence migrates only the direct-mode received-packet construction behind a compatibility shim. | `valence_network` compile/test and mc-compat dry-run logs. | Does not change Bevy schedule semantics or add Hyperion packet events. |
| `hyperion` uses of `valence_bytes::{Utf8Bytes,CowBytes}` | port | `upstream-byte-backed-protocol-path` | Shows needed validated UTF-8 and raw byte-field ergonomics. | `servers/valence/crates/valence_protocol/src/byte_backed.rs` | Constructors validate before public exposure; fields own shared `Bytes` slices. | Positive valid text/byte tests and negative invalid UTF/oversized/range tests. | Does not add a standalone `valence_bytes` crate or promise zero allocation for every generated packet field. |
| Hyperion proxy transport, mTLS/Iroh/rkyv, `hyperion-proxy` cache/egress runtime | reject | `upstream-byte-backed-protocol-path` | Runtime replacement is outside this change and forbidden by the integration boundary. | none | No runtime, transport, proxy, or scheduler code is copied. | Non-goal recorded here and in the Cairn proposal. | No production-scale, public-server safety, Hyperion compatibility, or proxy wire-compatibility claim. |

## Affected Valence surfaces

- `valence_protocol` adds byte-backed packet body/field types and decoder fixtures.
- `valence_server::client::ReceivedPacket` gains conversion/accessor shims while preserving public fields.
- `valence_network::packet_io` constructs received packets through the byte-backed frame path.

## Migration risks and mitigations

- **Public API complexity:** new types are narrow and named around byte-backed ownership; existing owned decode APIs remain.
- **Invalid client data exposure:** constructors reject invalid UTF-8, oversized fields, invalid ranges, zero-length packets, oversized packet lengths, and invalid compression thresholds before public events are emitted.
- **Ownership regressions:** fields own `Bytes` slices, so they can outlive the source packet body without stale references.
- **Runtime churn:** socket I/O, Tokio tasks, channel backpressure, and direct-mode delivery stay in their existing shells.

## Non-goals

This change does not port Hyperion's packet-channel crate, proxy runtime, mTLS/Iroh/rkyv transport, generated packet-event system, full `DecodeBytes` trait surface, or nightly-only implementation details. It does not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, full survival correctness, or Hyperion wire/runtime compatibility.
