# Design: Introduce a Valence proxy broadcast backend

## Context

Hyperion's docs describe a game server that emits compact broadcast intents to one or more proxies. The proxy owns socket fanout and regional delivery based on player chunk positions and subscriptions. Valence already has direct client networking and packet-writing APIs; the integration should be additive and optional.

## Decisions

### 1. Preserve direct mode

**Choice:** Add proxy mode behind an explicit feature/plugin or backend setting. Direct Valence networking remains the default path.

**Rationale:** Valence users should not inherit proxy operational complexity unless they opt in.

### 2. Define a stable message contract before transport work

**Choice:** Specify proxy messages and delivery semantics first, then choose or adapt an encoding/transport.

**Rationale:** The contract is the reviewable integration boundary. Transport details can evolve without changing Valence game logic.

### 3. Keep routing decisions pure

**Choice:** Put route selection in a deterministic core that consumes player positions, subscriptions, exclusions, and broadcast intents, returning delivery plans. The network shell owns sockets, encoding, retries, and task lifetimes.

**Rationale:** Routing correctness can be tested without standing up proxies or Minecraft clients.

### 4. Fail closed on proxy state drift

**Choice:** Stale stream IDs, unknown channel IDs, malformed proxy messages, and invalid player-position updates must produce diagnostics and no silent broadening of packet visibility.

**Rationale:** Proxy mode must not leak packets to the wrong clients when proxy state diverges.

## Risks / Trade-offs

- Proxy mode adds operational complexity; mitigate with direct mode as default and clear docs.
- Regional visibility bugs can be subtle; mitigate with pure routing fixtures and negative tests.
- Hyperion's current implementation uses nightly and unsafe pieces in places; port concepts, not code, unless an audit approves specific code paths.
