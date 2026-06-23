# Valence proxy broadcast contract

## Boundary

The proxy broadcast backend is an opt-in Valence-owned contract in the networking layer. Direct Valence networking remains the default. Applications must explicitly enable the optional backend before any proxy delivery plan is produced.

## Server-to-proxy messages

- `UpdatePlayerPosition` / `UpdatePlayerPositions`: update known active stream chunk positions. Unknown, inactive, or out-of-range positions are rejected.
- `AddChannel` / `RemoveChannel`: register and remove channel identifiers. Duplicate channel registration and unknown-channel removal are rejected. Removing a channel also removes subscriptions to that channel.
- `SubscribeChannelPackets`: deliver channel-owned packet bytes to active subscribed streams, with an optional excluded stream. Unknown channels, unknown/inactive exclusions, empty payloads, and queue-budget violations are rejected.
- `SetReceiveBroadcasts`: marks a known active stream eligible for global and local broadcasts.
- `BroadcastGlobal`: delivers payloads to active broadcast-enabled streams except an optional excluded stream.
- `BroadcastLocal`: delivers payloads to active broadcast-enabled streams within a chunk-radius square around a center chunk, excluding an optional stream.
- `BroadcastChannel`: delivers payloads to active streams subscribed to a known channel, excluding an optional stream.
- `Unicast`: delivers payloads to one known active stream.
- `Shutdown`: marks a known active stream as shutting down; new outbound deliveries are no longer planned for it.

## Proxy-to-server messages

- `ProxyReady`: no state mutation; records that transport readiness can be represented without creating streams.
- `PlayerConnect`: adds a new active stream. Duplicate connects are rejected.
- `PlayerDisconnect`: removes a known stream. Unknown streams are rejected.
- `PlayerPackets`: validates serverbound bytes for a known stream and configured queue budget without mutating state.
- `RequestSubscribeChannelPackets`: subscribes a known active stream to known channels. Unknown streams, inactive streams, and unknown channels are rejected.

## Ordering and routing

Delivery plans preserve input message order. Recipients inside each message are deterministic by stream identifier. Route validation happens before delivery for each message, and invalid global state such as duplicate streams, duplicate channels, stale subscriptions, or invalid positions fails closed before producing deliveries.

## Backpressure and malformed-message policy

The default payload budget uses Valence protocol maximum packet size. Callers can provide a stricter `ProxyBackpressurePolicy`. Empty payloads are malformed. Payloads larger than the configured budget are rejected with a structured diagnostic and no delivery for that message.

## Stream lifecycle

Only active streams receive new outbound deliveries. A stream marked `ShuttingDown` remains known so in-flight proxy-to-server packets can be tolerated until `PlayerDisconnect`, but new outbound route planning skips it.

## Non-claims

This contract does not claim Hyperion wire compatibility, rkyv compatibility, transport implementation, mTLS/Iroh support, production-scale readiness, public-server safety, broad Minecraft compatibility, semantic equivalence, or default Valence behavior changes.
