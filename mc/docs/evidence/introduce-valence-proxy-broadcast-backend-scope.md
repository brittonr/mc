# Valence proxy broadcast backend scope

## Question

What Hyperion proxy concepts can safely inform an optional Valence proxy broadcast backend without changing Valence direct networking defaults?

## Inspected evidence

| source_path | classification | owner | reason | valence_target | safety_notes | evidence required before archive | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/README.md` proxy architecture diagrams and server/proxy transport notes | reference | `introduce-valence-proxy-broadcast-backend` | Describes the high-level split between game-server simulation and proxy fanout, but no code is copied. | `servers/valence/crates/valence_network/src/proxy_broadcast.rs`, `servers/valence/crates/valence_network/README.md` | Reference-only; no nightly, unsafe, transport, or mTLS code enters Valence. | Scope note, route/contract unit tests, direct-mode regression, Cairn gates. | No Hyperion runtime compatibility, production-scale capacity, public-server safety, or default behavior change. |
| `hyperion/crates/hyperion-proto/src/server_to_proxy.rs` | port | `introduce-valence-proxy-broadcast-backend` | Message vocabulary for unicast, global/local/channel broadcast, subscription, player position, and shutdown is reimplemented in Valence-owned stable types. | `valence_network::proxy_broadcast` | Stable Rust only; data is owned `Vec<u8>`/IDs, no rkyv transport dependency, no copied implementation. | Positive and negative proxy message validation tests. | No wire compatibility with Hyperion rkyv frames or mTLS/Iroh transports. |
| `hyperion/crates/hyperion-proto/src/proxy_to_server.rs` | port | `introduce-valence-proxy-broadcast-backend` | Proxy-to-server lifecycle vocabulary informs fail-closed validation for ready/connect/disconnect/packets/subscription requests. | `valence_network::proxy_broadcast` | Stable Rust only; validation is pure over explicit state snapshots. | Malformed/unknown stream and unknown channel rejection tests. | No claim that Valence accepts Hyperion proxy binaries. |
| `hyperion/crates/hyperion/src/net/mod.rs` `Compose` and `IoBuf` broadcast builders | reference | `introduce-valence-proxy-broadcast-backend` | Builder names and fanout concepts inform Valence route planning, but the implementation is not copied. | `valence_network::proxy_broadcast`, existing `valence_server::packet_compose` | Hyperion runtime, thread-local buffers, proxy egress channels, compression, and unsafe/nightly-adjacent choices stay out of Valence. | Routing core fixtures and optional backend smoke tests. | No replacement of Valence network runtime or packet encoder. |
| `hyperion/crates/hyperion/src/egress/channel.rs` | reference | `introduce-valence-proxy-broadcast-backend` | Channel-position/subscription shape informs contract coverage only. | `valence_network::proxy_broadcast` | Bedwars/gameplay channel semantics are not adopted. | Channel broadcast and stale subscription tests. | No broad gameplay, CTF, or Bedwars semantics claim. |
| `servers/valence/crates/valence_server/src/packet_compose.rs` | reference | `introduce-valence-proxy-broadcast-backend` | Existing backend-neutral direct-mode planner proves Valence already has direct route planning; proxy work should compose conceptually without changing it. | `valence_network::proxy_broadcast` | Direct-mode API remains untouched unless tests prove no behavior change. | Baseline and post-change `packet_compose` tests. | Existing compose API still does not claim proxy transport by itself. |
| `servers/valence/crates/valence_network/src/lib.rs` `NetworkPlugin` and `NetworkSettings` | port | `introduce-valence-proxy-broadcast-backend` | Optional plugin/resource wiring belongs in the networking crate while `NetworkPlugin` stays direct-mode by default. | `valence_network::proxy_broadcast` | New backend plugin is opt-in and no-op unless added explicitly; default `NetworkSettings::default()` remains unchanged. | Valence-network unit tests and direct-mode dry-run/regression evidence. | No production backend transport or default proxy mode claim. |

## Decision

Port the message vocabulary and pure routing/validation concepts into a Valence-owned `valence_network::proxy_broadcast` module. Keep transport, sockets, encoding, retry loops, TLS/Iroh, load balancing, and Hyperion runtime behavior outside the current scope. The implementation is a deterministic contract/routing core plus a thin optional Bevy plugin shell that records opt-in state; Valence direct networking remains the default path.

## Non-goals

- Do not merge Hyperion runtime, proxy transport, mTLS/Iroh setup, rkyv frame encoding, ECS scheduling, or Bedwars/gameplay behavior into Valence.
- Do not make proxy mode default or change `NetworkPlugin`, `NetworkSettings::default()`, login, direct packet writes, or direct client flushing.
- Do not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF/survival correctness, Hyperion binary compatibility, or large-scale capacity.

## Next action

Implement the optional Valence-owned proxy broadcast contract and pure routing core, then verify it with positive/negative unit tests, direct-mode regressions, selected dry runs, and Cairn gates before archive.
