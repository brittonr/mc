# Profile skin cache scope and Hyperion boundary

## Question

What profile/skin cache behavior is safe to add to Valence for `add-profile-skin-cache-rate-limiter` without changing authentication defaults or copying Hyperion runtime/storage assumptions?

## Inspected evidence

| Source | Classification | Owner/change | Decision | Safety notes | Evidence/non-claims |
| --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion/src/common/util/mojang.rs` (`ApiProvider`, `MojangClient`) | `port` | `add-profile-skin-cache-rate-limiter` | Reimplemented the provider URL, response parsing, and request-budget concepts as Valence-owned typed config and pure fixed-window admission logic. | No Hyperion code copied; no default third-party provider installed; no tokio semaphore refill loop imported into Valence core. | Covered by `profile_cache` unit tests; no Hyperion compatibility, provider uptime, or production-scale claim. |
| `hyperion/crates/hyperion/src/simulation/skin.rs` (`PlayerSkin::from_uuid`) | `reference` | `add-profile-skin-cache-rate-limiter` | Used only to identify the data shape Valence already exposes as signed `textures` profile properties. | Valence stores `valence_protocol::profile::Property` values and keeps parsing/cache decisions independent from network I/O. | No skin rendering correctness, vanilla parity, or broad player-list behavior claim. |
| `hyperion/crates/hyperion/src/storage/db.rs` (`LocalDb`, `SkinHandler`) | `reject` for direct adoption | `add-profile-skin-cache-rate-limiter` | Did not adopt Heed/rkyv storage, hard-coded `db/heed.mdb`, unsafe unchecked archive access, or fixed map sizes. | Valence adds adapter traits plus optional in-memory shell; applications own persistent paths/backends explicitly. | No persistent backend claim and no default storage path claim. |
| `hyperion/events/bedwars/src/skin.rs` (`SkinPlugin`) | `reference` | `add-profile-skin-cache-rate-limiter` | Used only to confirm skins flow through signed `textures` properties and player-list updates. | No Bedwars gameplay, plugin behavior, or packet choreography copied into Valence. | No CTF/Bedwars correctness or Hyperion gameplay compatibility claim. |
| `servers/valence/crates/valence_network/src/connect.rs` | Valence target | `add-profile-skin-cache-rate-limiter` | Online login, BungeeCord, and Velocity authentication/profile acquisition remain unchanged. The new cache helper is optional and inert unless applications call it. | Session-server authentication is still performed by existing login code and callbacks. | No auth bypass, Mojang/Velocity rule bypass, or public-server safety claim. |
| `servers/valence/crates/valence_server/src/client.rs` (`Properties`) | Valence target | `add-profile-skin-cache-rate-limiter` | Cache payload mirrors existing profile properties, including optional signed `textures`. | No default mutation of spawned clients or player-list entries. | No broad skin display or semantic equivalence claim. |

## Decision

Implement a Valence-owned optional `valence_network::profile_cache` module. The module contains pure deterministic cores for configuration validation, JSON profile parsing, cache freshness/staleness decisions, and fixed-window rate-limit admission. HTTP and storage are thin adapter traits with a reqwest provider shell and in-memory store shell; persistent paths and provider endpoints are explicit application configuration.

## Authentication boundary

Cached data is limited to profile UUID, username, and profile properties such as signed `textures` values. The cache does not store shared secrets, verify tokens, auth digests, session-server proof, Velocity forwarding signatures, Bungee forwarding payloads beyond profile properties, raw socket data, or HTTP credentials. Valence online authentication, proxy forwarding validation, and login callbacks stay on the existing path.

## Non-goals and non-claims

This change does not select Mojang, matdoes.dev, or any third-party provider by default. It does not add a persistent cache backend, hard-code a storage path, bypass Mojang/Velocity authentication, prove provider availability, claim skin rendering correctness, claim broad Minecraft compatibility, claim Hyperion compatibility, claim public-server safety, or claim production readiness.

## Next action

Use focused `valence_network profile_cache` tests and Valence dry-run smoke output as implementation evidence, then run Cairn gates, task-evidence validation, and Cairn validation before sync/archive.
