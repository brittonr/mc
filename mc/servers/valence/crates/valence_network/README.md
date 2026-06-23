# `valence_network`

The plugin responsible for accepting connections and spawning clients.

This covers everything in the "handshaking", "status" and "login" stages of the protocol, before the main "play" stage begins. Support for proxies like [Velocity] and [BungeeCord] are implemented here.

Valence users can choose not to include `valence_network` in their project. This could be useful for testing or using Valence as an integrated server in a client.

## Optional proxy broadcast backend

`valence_network::proxy_broadcast` provides a Valence-owned proxy-mode contract for applications that want to experiment with server-to-proxy packet fanout. The default `NetworkPlugin` and `NetworkSettings::default()` behavior is unchanged: direct client networking remains the normal path, and proxy broadcast routing is disabled unless an application explicitly adds and enables the optional backend resource/plugin.

The core is pure and deterministic over explicit state snapshots: server-to-proxy and proxy-to-server messages, stream lifecycle, player chunk positions, channel subscriptions, exclusions, shutdown, and backpressure policy are validated before delivery plans are produced. Unknown streams, unknown channels, stale subscriptions, malformed payloads, invalid positions, inactive streams, and queue-budget violations fail closed with structured diagnostics.

The plugin shell only initializes disabled backend state. Transport ownership remains outside this module: sockets, TLS/Iroh setup, retries, encoding compatibility, load balancing, and task lifetimes belong to application-owned adapters or future accepted scope.

Non-claims: this helper is not enabled by default, is not wire-compatible with Hyperion proxy binaries, does not port Hyperion runtime/mTLS/Iroh/rkyv transport code, does not prove production-scale readiness or public-server safety, and does not claim broad Minecraft compatibility or Hyperion compatibility.

[Velocity]: https://papermc.io/software/velocity
[BungeeCord]: https://github.com/SpigotMC/BungeeCord
