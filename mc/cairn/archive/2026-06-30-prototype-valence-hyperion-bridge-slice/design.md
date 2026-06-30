# Design: Prototype Valence/Hyperion bridge slice

## Context

A full Valence/Hyperion merge crosses too many domains at once: protocol ownership, runtime scheduling, proxy transport, chunk egress, player movement, commands/chat, gameplay plugins, and evidence policy. A bridge slice should validate only the smallest loop that users can observe while preserving existing defaults.

The slice should reuse prior archived work where it remains current: packet compose core, cached chunk egress, proxy broadcast routing, and gameplay plugin boundaries. It must still verify current source facts before claiming any behavior.

## Decisions

### Start as optional prototype scope

**Choice:** Implement the first bridge as an optional plugin, example, or fixture harness, not as Valence core behavior or Hyperion runtime replacement.

**Rationale:** Optional scope lets the adapter prove interoperability without forcing operational or API commitments on existing users.

### Make bridge decisions pure

**Choice:** Model join planning, initial chunk delivery planning, movement mapping, and chat/broadcast routing as pure deterministic cores over explicit summaries.

**Rationale:** The bridge should be testable without live clients, proxies, sockets, Bevy schedules, filesystem state, clocks, or logging.

### Keep shells thin and owner-specific

**Choice:** Valence shell code gathers Valence state and applies Valence mutations or packet sends. Hyperion/proxy shell code remains separate and applies only adapter-approved intents. Neither shell owns the other's runtime internals.

**Rationale:** This follows functional-core/imperative-shell boundaries and prevents hidden runtime coupling.

### Fail closed on stale or incomplete state

**Choice:** Missing join facts, invalid chunk or dimension facts, stale sessions, malformed movement, unauthorized chat routes, closed clients, and invalid broadcast targets return diagnostics and no delivery plan.

**Rationale:** A bridge must not leak packets, move the wrong entity, corrupt chunk visibility, or broaden chat recipients when state diverges.

### Evidence limits claims

**Choice:** Closeout evidence should claim only the exercised bridge slice and plugin-disabled regressions. Production scale, Hyperion compatibility, Bedwars behavior, broad proxy readiness, and vanilla parity remain non-claims.

**Rationale:** A minimal bridge proves a seam, not the full merged product.

## Risks / Trade-offs

- A prototype may need temporary adapter DTOs that should not become public API. Mark them crate-private or experimental unless the type-ownership audit promotes them.
- Smoke tests can pass while hiding scale issues. Do not cite them as production capacity evidence.
- Prior archived integration code may have drifted. Run current baseline checks before relying on archived behavior.
