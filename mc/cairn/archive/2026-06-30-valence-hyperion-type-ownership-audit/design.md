# Design: Valence/Hyperion type ownership audit

## Context

A bridge between Valence and Hyperion crosses several sensitive boundaries: protocol packet models, player/session identifiers, chunks and dimensions, entity state, chat and commands, proxy routing, and gameplay events. Some surfaces are already well-owned by Valence. Others are Hyperion runtime details that should not become public Valence API. The audit exists to make these boundaries explicit before implementation.

## Decisions

### Valence owns public protocol and framework types

**Choice:** Treat Valence protocol, registry, packet composition, layer/chunk, command, and public framework abstractions as the canonical external API unless a later accepted change proves a different ownership model.

**Rationale:** Valence's role is the modular framework and protocol foundation. Public users should not need Hyperion internals to build a Valence server.

### Hyperion owns runtime-local types

**Choice:** Treat Hyperion ECS components, proxy transport internals, game-server scheduling, Bedwars/game-mode state, and high-scale runtime helpers as Hyperion-owned unless a specific small source is audited for adoption or porting.

**Rationale:** Hyperion optimizes around its runtime architecture and current toolchain. Directly exposing those details through Valence would create unnecessary coupling and stable-Rust risk.

### Adapter DTOs stay narrow and explicit

**Choice:** Introduce adapter-owned summaries only when neither side's canonical type should cross the boundary directly. DTOs should describe intent, not mirror entire internal worlds.

**Rationale:** Intent summaries such as join plans, movement facts, route intents, and chat route requests are easier to test and evolve than broad shared state structs.

### Fail closed on conversion ambiguity

**Choice:** Conversions reject missing dimensions, invalid entity identifiers, unsupported protocol assumptions, malformed packet bytes, stale player/session mappings, invalid routes, and lossy field mappings unless a documented default is explicit.

**Rationale:** Ambiguous conversions can leak packets to wrong clients, corrupt state, or create compatibility overclaims.

### Fixtures precede bridge wiring

**Choice:** Positive and negative ownership/conversion fixtures must exist before a bridge slice depends on a mapped type family.

**Rationale:** Bridge shell bugs are easier to diagnose when conversion behavior is already deterministic and independently tested.

## Risks / Trade-offs

- Too many adapter DTOs can become a parallel framework. The matrix should reject broad mirrors and prefer narrow intent summaries.
- Reusing Valence types everywhere may be inconvenient for Hyperion internals. The audit should distinguish public API ownership from internal runtime ownership.
- Existing archived evidence may not match current code. The audit should record source revisions or mark evidence as historical only.
