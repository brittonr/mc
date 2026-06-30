# Valence/Hyperion integration roadmap evidence — 2026-06-30

## Scope

This document closes the docs-only `valence-hyperion-integration-roadmap` Cairn. It is governance evidence for bounded Valence/Hyperion convergence. It does not implement bridge code, merge repositories, replace Valence defaults, change Hyperion defaults, prove Hyperion compatibility, prove production scale, or claim vanilla parity.

## Source checkpoint

- Root: `/home/brittonr/git/mc`
- Valence source-tree checkpoint: parent repository commit `69057be95d5108d7f2612d40a7666f68fa938caa` for `mc/servers/valence`.
- Valence dirty state at inspection: dirty; local edits existed in `servers/valence/crates/valence_network/src/profile_cache.rs` and `servers/valence/crates/valence_network/src/proxy_broadcast.rs` before this roadmap evidence was written.
- Hyperion checkpoint: `jj` working copy commit `e1cc02f36b5d9f6e26ab1d58eda1d452697d34bd`, parent `a70294d8`.
- Hyperion dirty state at inspection: dirty in Bedwars event files (`events/bedwars/src/lib.rs`, `main.rs`, `plugin/attack.rs`, `plugin/chat.rs`); this roadmap treats those as unrelated pre-existing work and makes no Bedwars integration claim.

## Inventory

### Accepted specs inspected

| Spec | Role in roadmap |
| --- | --- |
| `cairn/specs/valence-hyperion-integration/spec.md` | Canonical boundary requirements: source classification, forbidden core merges, optional-plugin boundary, review gate, and archived integration tracks. |
| `cairn/specs/valence-bevy-ecs/spec.md` | Accepted Bevy/ECS ownership, scheduling, plugin, and typed-event patterns that future optional Valence plugins must respect. |
| `cairn/specs/mc-compatibility/spec.md` | Compatibility and evidence non-claim boundaries for any future smoke or reference evidence. |
| `cairn/specs/repository-layout/spec.md` | Workspace ownership and nested-repo boundaries for parent-owned Valence/Stevenarella trees and nested Hyperion. |

### Boundary and architecture docs inspected

| Source | Roadmap fact |
| --- | --- |
| `docs/hyperion-integration-boundaries.md` | Every Hyperion source used by Valence work must be classified as `adopt`, `port`, `reference`, or `reject`; forbidden core merges stay rejected or reference-only. |
| `hyperion/README.md` | Hyperion is an ECS-driven Minecraft game engine optimized for large events, custom mechanics, and a horizontally scaled proxy layer. |
| `hyperion/docs/architecture/introduction.md` | Hyperion uses plugin-first ECS architecture; mechanics such as combat are plugin-owned rather than vanilla defaults. |
| `hyperion/docs/architecture/game-server.md` | Hyperion game-server ingress/egress, packet channel, and tick-loop behavior are runtime-owned and not Valence API. |
| `hyperion/docs/architecture/proxy.md` | Hyperion proxy owns regional broadcast, channel broadcast, global broadcast, and unicast routing concepts. |
| `servers/valence/README.md` | Valence is a modular Rust framework for Minecraft servers; vanilla mechanics and dedicated executables are optional plugins. |
| `servers/valence/crates/README.md` | Valence crates are exported through the main Valence crate and are the intended third-party interface. |

### Archived integration Cairns reconciled

| Archive | Reuse / non-duplication decision |
| --- | --- |
| `2026-06-22-guard-hyperion-integration-boundaries` | Reuse its boundary checklist; do not duplicate classification rules. |
| `2026-06-23-add-valence-packet-compose-api` and `2026-06-28-extract-valence-packet-compose-core` | Treat packet composition as Valence-owned; bridge work may call documented pure composition surfaces but must not create a parallel packet framework. |
| `2026-06-23-add-cached-chunk-egress-pipeline` | Treat cached chunk egress as Valence-owned implementation evidence; bridge planning may reference chunk/view summaries instead of duplicating egress cache internals. |
| `2026-06-23-introduce-valence-proxy-broadcast-backend` | Reuse optional proxy-broadcast concepts and direct-mode non-claims; future bridge work must preserve default-disabled behavior. |
| `2026-06-23-upstream-byte-backed-protocol-path` | Keep byte-backed protocol behavior Valence/protocol-owned; adapters may validate bytes but must not replace protocol encoding ownership. |
| `2026-06-28-extract-hyperion-player-join-core` | Hyperion-local player-join extraction is historical source evidence only unless the type audit classifies a specific concept. |
| `2026-06-28-extract-hyperion-bot-packet-core` | Hyperion-local bot packet utilities remain nested-repo/tool-owned; bridge work may use them only as classified reference evidence. |
| `2026-06-28-extract-hyperion-inventory-core` | Hyperion inventory remains Hyperion-owned unless a later optional plugin ports a narrow Valence-owned concept. |
| `2026-06-29-modularize-hyperion-block-loader`, `2026-06-29-modularize-hyperion-simulation-shell`, `2026-06-29-modularize-hyperion-packet-inspector-ui` | These are Hyperion-local modularity evidence and do not grant Valence adoption rights. |
| `2026-06-25` / `2026-06-26` Valence Bevy/plugin archives | Future gameplay/plugin slices must follow accepted Valence plugin, system-set, arena-scope, and config-source boundaries. |

### Active Cairns reconciled

| Active change | Roadmap classification |
| --- | --- |
| `valence-hyperion-integration-roadmap` | This governance roadmap; prerequisite for ownership and bridge implementation. |
| `valence-hyperion-type-ownership-audit` | Required next dependency; must publish type ownership, conversion contracts, and fixture plans before bridge implementation depends on shared concepts. |
| `prototype-valence-hyperion-bridge-slice` | Blocked until roadmap and type-ownership evidence are complete; must stay optional/default-disabled. |
| `build-battle-creative-contest-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |
| `coop-summit-climb-game-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |
| `duels-kitpvp-arena-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |
| `factions-claims-raid-loop` | Hyperion-owned Factions territory/raid layer; depends on social core. |
| `factions-clans-social-core` | Hyperion-owned Factions/Clans social foundation. |
| `factions-diplomacy-economy-progression` | Hyperion-owned Factions diplomacy/economy/progression layer; depends on social and territory cores. |
| `lifesteal-smp-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |
| `murder-mystery-social-deduction-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |
| `parkour-time-trial-mode` | Hyperion-owned game-mode implementation; Valence examples remain reference-only. |
| `skyblock-oneblock-island-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |
| `skywars-elimination-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |
| `survival-games-uhc-elimination-mode` | Hyperion-owned game-mode implementation; no Valence integration scope. |

## Ownership roadmap

| Responsibility | Owner | Boundary |
| --- | --- | --- |
| Public protocol encoding/decoding, packet structs, registries, identifiers, generated protocol data | Valence | Use Valence crates as canonical API. Do not mirror Hyperion protocol models as public Valence API. |
| Valence networking, direct client connection lifecycle, session/profile cache, packet IO | Valence | Bridge/proxy modes are optional adapters and must preserve direct-mode defaults. |
| Valence layers, chunks, entities, commands, text, events, plugins, schedules | Valence | Optional plugins may add behavior only through accepted Valence APIs and system sets. |
| Hyperion runtime, game-server tick loop, proxy transport, packet channels, simulation state, Bedwars/game modes | Hyperion | Reference, port, or reject only unless a later audit proves a tiny adoptable component. |
| Adapter DTOs for join facts, chunk/view facts, movement facts, route intents, diagnostics | Adapter-owned | Narrow intent summaries only; no broad shared world or runtime mirror. |
| Evidence receipts, manifests, Cairn task citations | Parent mc workspace | Review-critical bytes live under `docs/evidence/` with BLAKE3 manifests. |

## Dependency sequence

1. **Roadmap and boundary inventory**: complete this document and keep non-claims explicit.
2. **Type ownership audit**: publish Valence and Hyperion inventories, classify inspected Hyperion sources, define ownership/conversion contracts, and list positive/negative fixture plans.
3. **Optional bridge slice**: implement only a default-disabled plugin/example/fixture harness after the audit. The first slice may cover join/chunk planning, movement mapping, chat, and broadcast route planning.
4. **Optional backend and gameplay plugins**: add proxy, observability, command, GUI, anti-cheat, world snapshot, or gameplay plugin work only through separate accepted Cairns and plugin-disabled regressions.
5. **Consolidation decisions**: consider workspace/package convergence only after compatibility, maintenance, owner, release, and default-behavior evidence exists.

## Stop conditions

Future integration work must stop or remain active when any of these apply:

- a required source lacks adopt/port/reference/reject classification;
- a proposed change copies Bedwars-specific, runtime-replacement, custom-combat-core, unsafe-heavy, or nightly-only Hyperion code into Valence core;
- default Valence behavior changes without an accepted requirement and direct-mode regression evidence;
- fixture coverage lacks both positive and negative cases for new adapters;
- source revisions or evidence are stale and not labeled historical;
- the change implies production-scale, Hyperion-compatibility, vanilla-parity, public-server safety, or default-behavior claims without evidence.

## Decision-record template for future Cairns

| Field | Required value |
| --- | --- |
| Source path / revision | Exact Hyperion path, docs path, or archive evidence plus source checkpoint. |
| Classification | `adopt`, `port`, `reference`, or `reject`. |
| Owner | Valence, Hyperion, adapter, docs/evidence, or rejected. |
| Target | Valence crate/plugin/example, adapter module, Hyperion-local module, docs-only target, or `none`. |
| Safety notes | Stable Rust, unsafe/nightly, runtime/threading, allocation, API, and default-behavior concerns. |
| Evidence needed | Positive tests, negative tests, direct-mode regressions, smoke receipts, manifests, or oracle notes. |
| Non-claims | Unsupported production-scale, vanilla-parity, Hyperion-compatibility, default-behavior, safety, and Bedwars claims. |
| Decision | Adopt/port/reference/reject rationale and next action. |

## Closeout non-claims

This roadmap is documentation and governance evidence. It does not prove bridge functionality, does not update accepted specs until Cairn sync runs, does not resolve active Hyperion game-mode implementation tasks, does not validate dirty Valence or Hyperion working-copy edits, and does not authorize direct Hyperion runtime imports into Valence core.
