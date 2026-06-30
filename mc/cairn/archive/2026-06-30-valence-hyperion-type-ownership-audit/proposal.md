# Proposal: Audit Valence/Hyperion type ownership

## Why

Before bridge code can safely connect Valence and Hyperion, the projects need a clear answer to a simple question: which side owns each type, protocol fact, state summary, and conversion boundary? Without that answer, an implementation slice can accidentally duplicate protocol models, leak Hyperion runtime internals into Valence APIs, or make adapters depend on unstable implementation details.

The merge path should make Valence the canonical public framework/protocol surface, keep Hyperion internals Hyperion-owned, and use narrow adapter DTOs only where evidence shows they are needed.

## What Changes

- Inventory Valence protocol, networking, packet composition, layer/chunk, entity/player, command/chat, and optional proxy surfaces relevant to a bridge.
- Inventory Hyperion game-server, proxy, packet, join, movement, broadcast, chunk egress, command/chat, and game-mode surfaces relevant to a bridge.
- Classify each inspected Hyperion source as adopt, port, reference, or reject with safety notes and non-claims.
- Define a type ownership matrix that names canonical Valence types, Hyperion-only types, adapter-owned DTOs, and rejected shared abstractions.
- Define conversion contracts, failure semantics, positive fixtures, negative fixtures, and validation evidence before implementation uses these types.

## Impact

- **Files**: Cairn artifacts, an ownership inventory/matrix under `docs/evidence/` or `docs/`, possible fixture definitions, and no production code unless a later task explicitly adds tests around pure conversion helpers.
- **Testing**: Cairn gates, validation, fixture checks if helper code is added, and evidence-manifest checks for promoted ownership evidence.
- **Non-claims**: this does not implement the bridge, change public APIs, merge workspaces, make Valence consume Hyperion runtime types, or claim Hyperion compatibility.
