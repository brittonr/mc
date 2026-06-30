# Valence/Hyperion type ownership audit evidence — 2026-06-30

## Scope

This document closes the docs-only `valence-hyperion-type-ownership-audit` Cairn. It inventories Valence and Hyperion type families, classifies inspected Hyperion sources, defines ownership/conversion contracts, and records fixture plans for later bridge work. It does not implement the bridge, change public APIs, merge workspaces, make Valence consume Hyperion runtime types, or claim Hyperion compatibility.

## Source checkpoint

- Root: `/home/brittonr/git/mc`
- Valence source-tree checkpoint: parent repository commit `69057be95d5108d7f2612d40a7666f68fa938caa` for `mc/servers/valence`.
- Valence dirty state at inspection: dirty; local edits existed in `servers/valence/crates/valence_network/src/profile_cache.rs` and `servers/valence/crates/valence_network/src/proxy_broadcast.rs` before this audit evidence was written.
- Hyperion checkpoint: `jj` working copy commit `e1cc02f36b5d9f6e26ab1d58eda1d452697d34bd`, parent `a70294d8`.
- Hyperion dirty state at inspection: dirty in Bedwars event files (`events/bedwars/src/lib.rs`, `main.rs`, `plugin/attack.rs`, `plugin/chat.rs`); those files are game-mode-local and not bridge ownership evidence.

## Valence inventory

| Type family | Valence surfaces inspected | Ownership decision | Notes |
| --- | --- | --- | --- |
| Protocol packets and bytes | `servers/valence/crates/valence_protocol/src/lib.rs`, `encode.rs`, `decode.rs`, `raw.rs`, `byte_backed.rs`, `packets/play/*.rs` | Valence-owned canonical public API | Bridge adapters may validate packet bytes or plan packet order, but packet structs and encoding remain Valence-owned. |
| Player/session/profile | `valence_network/src/connect.rs`, `login.rs`, `packet_io.rs`, `profile_cache.rs`, `session_core.rs`, `valence_protocol/src/profile.rs` | Valence-owned for Valence direct connections; adapter-owned summaries for bridge state | Stale sessions fail closed; profile cache changes remain Valence-network implementation details. |
| Proxy broadcast | `valence_network/src/proxy_broadcast.rs` | Optional Valence/adapter backend, default-disabled | Reuse archived optional proxy-broadcast evidence; direct networking remains default. |
| Dimensions, registries, chunks, chunk view | `valence_protocol/src/chunk_pos.rs`, `chunk_section_pos.rs`, `global_pos.rs`, `valence_server/src/chunk_view.rs`, `layer/chunk/*`, `layer/chunk/egress_cache.rs` | Valence-owned | Adapters pass explicit registry/dimension/chunk/view summaries rather than Hyperion world state. |
| Entities and movement | `valence_server/src/client.rs`, `movement.rs`, `layer/entity.rs`, `teleport.rs`, `valence_entity`, `valence_spatial` | Valence-owned | Movement mapper may emit adapter intents only after known session/entity/dimension checks. |
| Commands and chat/text | `valence_command/src/*`, `valence_server/src/client/command.rs`, `message.rs`, `valence_text`, `valence_protocol/src/packets/play/chat_message_s2c.rs`, `game_message_s2c.rs` | Valence-owned | Hyperion command/chat concepts are reference-only unless ported into optional Valence-owned plugins. |
| Bevy schedules/plugins/events | `valence_server/src/event/loop.rs`, `tick_scheduler.rs`, archived `valence-bevy-ecs` specs | Valence-owned | Optional bridge or gameplay code must use accepted Valence plugin/system-set boundaries. |

## Hyperion inventory and classification

| Hyperion source | Classification | Owner / target | Safety notes and evidence needs |
| --- | --- | --- | --- |
| `hyperion/README.md`, `hyperion/docs/architecture/introduction.md` | reference | docs/evidence | Architecture vocabulary only. No code copied; no production-scale claim reused. |
| `hyperion/docs/architecture/game-server.md` | reference | adapter design notes | Join/egress/tick-loop concepts inform bridge planning, but Hyperion runtime loop remains Hyperion-owned. |
| `hyperion/docs/architecture/proxy.md` | reference | optional proxy/broadcast adapter design | Broadcast categories are useful vocabulary; routing implementation must be Valence/adapter-owned and tested. |
| `hyperion/crates/hyperion/src/egress/player_join/core.rs` | reference | bridge join/chunk planner | Existing Hyperion-local core can inform fixture shape; no direct adoption until stable/API audit and tests prove compatibility. |
| `hyperion/crates/hyperion/src/egress/player_join/mod.rs`, `list.rs` | reject for Valence core; reference for adapter facts | none / adapter docs | Runtime shell and Hyperion state ownership must not enter Valence core. |
| `hyperion/crates/hyperion/src/egress/sync_chunks.rs` and `simulation/blocks/chunk/packet.rs` | reference | bridge chunk fixture planning | Treat chunk packet strategy as historical/reference; Valence chunk egress remains canonical. |
| `hyperion/crates/hyperion/src/egress/sync_entity_state.rs`, `simulation/packet_adapters.rs`, `simulation/packet_state.rs` | reference | movement and packet fixture planning | Reuse concepts only as explicit movement/packet summaries; reject broad simulation-state sharing. |
| `hyperion/crates/hyperion/src/net/agnostic/chat.rs`, `hyperion-text` | reference | chat route fixture planning | Chat formatting/routing concepts may inform adapter tests; Valence text/chat APIs remain authoritative. |
| `hyperion/crates/hyperion-proto/src/server_to_proxy.rs`, `proxy_to_server.rs`, `shared.rs` | port/reference | optional proxy DTO contracts | Rkyv proxy messages are Hyperion-owned. Adapter DTOs may port narrow route intent concepts with Valence-owned serialization if implemented. |
| `hyperion/crates/hyperion-proxy/src/*` | reject for Valence core; reference for optional backend | none / optional adapter docs | Proxy runtime, transport, cache, and player management are not Valence core. |
| `hyperion/crates/hyperion-command`, `hyperion-permission`, `hyperion-stats`, `hyperion-gui`, `hyperion-inventory` | reference or future optional-plugin port only | optional Valence plugins, if separately scoped | Do not create parallel Valence framework internals; require separate accepted Cairns and positive/negative plugin-disabled tests. |
| `hyperion/events/bedwars/*` | reject for Valence core | none | Bedwars-specific game logic is a forbidden core merge; dirty local Bedwars edits are unrelated to bridge ownership. |
| `hyperion/crates/geometry`, `bvh-region` | future port candidate only | Valence-owned spatial helper if separately scoped | Any port needs stable Rust review, pure-core tests, negative NaN/zero/boundary fixtures, and no vanilla combat overclaim. |
| Hyperion runtime/scheduler/networking internals | reject for Valence core | none | Runtime replacement, scheduler replacement, networking runtime swap, and unaudited unsafe/nightly-heavy imports are forbidden. |

## Type ownership matrix

| Type family | Canonical owner | Hyperion-only type | Adapter DTO allowed? | Conversion contract |
| --- | --- | --- | --- | --- |
| Protocol packet structs and packet byte buffers | Valence | Hyperion packet builder/intermediate types | Yes, for validated byte slices or packet-intent names only | Adapter inputs must state protocol/version assumption, packet order, byte validity, and lossy policy. Invalid bytes fail closed. |
| Player identity, session, stream/client id | Owner-specific; Valence for Valence clients, Hyperion for Hyperion proxy streams | Hyperion stream ids/proxy player records | Yes, `BridgeSessionKey`/summary in prototype scope | A mapping is valid only when player id, session id, owner, and freshness agree. Stale or missing sessions reject. |
| Dimensions, registries, chunk/view facts | Valence for Valence bridge shell | Hyperion world/chunk state | Yes, explicit `DimensionSummary`, `ChunkViewSummary`, `ChunkPlanFact` | Required registry/dimension/chunk bounds must be present. Missing or incompatible facts reject with no send. |
| Entity ids and movement state | Valence for Valence entities; Hyperion for Hyperion simulation | Hyperion ECS entity/state components | Yes, narrow movement fact/update DTO | A movement update maps exactly one known entity in one known dimension. Invalid coords, malformed rotation, stale entity, or lossy conversion reject. |
| Chat text, channels, broadcast routes | Valence for public text/chat APIs; adapter for bridge route intents | Hyperion proximity/global route internals | Yes, `RouteIntent`/recipient summary | Authorized route, known recipients, explicit exclusions, and deterministic ordering required. Unauthorized, closed, stale, or malformed routes reject. |
| Command graph, parsers, permissions | Valence for Valence command APIs | Hyperion command framework and permissions | Future optional plugin DTO only | Do not introduce parallel command framework. Permission decisions must be pure over explicit context and plugin-disabled by default. |
| Runtime/schedule/proxy transport | Owner-specific; not shared | Hyperion runtime/proxy transport | No broad shared DTO | Use shell boundaries. No Valence public API may depend on Hyperion runtime internals. |
| Gameplay/game-mode state | Hyperion for Hyperion game modes, optional Valence plugins for separately scoped Valence features | Bedwars and custom game-mode internals | No broad shared DTO | Bedwars/game-mode state is rejected for Valence core. Optional plugins require separate accepted specs and fixture evidence. |

## Conversion contracts

### Player/session identifiers

Inputs must include owner, player identifier, session identifier, freshness marker, and target shell. Valid mappings return a bridge-local session summary. Unknown players, stale sessions, mismatched owner, duplicate live mapping, or closed clients return deterministic diagnostics and no mutation.

### Dimension, registry, chunk, and view facts

Inputs must include registry identity, dimension key, min/max build bounds, chunk position, view radius/visibility facts, and packet-order policy. Missing dimensions, incompatible bounds, absent chunk data, unsupported order, or lossy chunk summary reject before packet planning.

### Entity and movement facts

Inputs must include player/session/entity identifiers, position, rotation, velocity, on-ground state, dimension, and freshness. Valid updates map one entity only. NaN/invalid coordinates, malformed rotation, unsupported dimension, stale entity, or precision loss beyond explicit policy reject.

### Packet bytes and packet intents

Inputs must identify packet family, packet order, protocol assumption, and whether bytes are already Valence-encoded. Malformed bytes, unknown packet family, unsupported protocol assumption, or missing packet order reject. Adapter code must not reinterpret Hyperion bytes as Valence packets without validation.

### Chat and broadcast routes

Inputs must include sender, channel/route, candidate recipients, permissions, visibility rule, exclusions, and closed-client state. Valid plans return deterministic recipients and diagnostics. Unauthorized routes, unknown channels, invalid exclusions, hidden-member leaks, stale recipients, or closed clients reject.

## Fixture plan

| Area | Positive fixtures | Negative fixtures |
| --- | --- | --- |
| Session mapping | valid Valence client, valid bridge session, fresh owner-specific mapping | missing session, stale session, duplicate mapping, closed client, owner mismatch |
| Join/chunk plan | valid registry, dimension, chunk/view facts, supported packet order | missing registry, invalid dimension bounds, absent chunk, unsupported packet order, lossy chunk fact |
| Movement | valid position/rotation/velocity for one known entity | NaN coordinate, malformed rotation, stale entity, unknown dimension, lossy conversion, duplicate target |
| Chat/broadcast | authorized route, deterministic recipients, explicit exclusions | unauthorized route, unknown channel, invalid exclusion, hidden membership leak, stale recipient, closed client |
| Packet bytes | valid Valence-encoded bytes with known packet family | malformed bytes, unsupported protocol, unknown family, missing order |
| Disabled behavior | bridge disabled returns direct-mode/no-op plan and leaves Valence path unchanged | disabled mode rejects bridge-only mutation, send, or compatibility claim |

## Non-claims

This audit is a prerequisite document. It does not add tests or production code, does not validate the dirty Valence or Hyperion working-copy changes, does not prove bridge behavior, does not prove production-scale routing, does not claim full Hyperion compatibility, does not claim Bedwars behavior, and does not authorize copying Hyperion runtime or game-mode internals into Valence core.
