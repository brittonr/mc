# Design: Add factions/clans social core

## Context

Factions/clans is best split into a persistent social foundation plus later gameplay layers. The social foundation owns group identity, membership, roles, permissions, chat/presence, and persistence snapshots. Claims, raids, diplomacy, and economy should depend on this foundation but must not be smuggled into the first implementation.

Hyperion is ECS-driven with game-specific code in `events/bedwars` and shared engine crates under `crates/`. The social core should use Bevy for runtime ownership and Hyperion command/permission/text surfaces for shell integration, while keeping rule decisions pure and deterministic.

## Decisions

### 1. Scope the first layer to social identity and membership

**Choice:** Start with factions/clans identity, roster membership, invite/join/leave/kick/transfer/disband lifecycle, roles, permissions, chat routing, presence summaries, and persistence snapshots.

**Rationale:** Every later factions feature needs a trustworthy roster and authority model. Building claims or raids first would duplicate membership checks and make negative cases harder to prove.

### 2. Use pure cores for lifecycle and authorization

**Choice:** Model lifecycle transitions and permission checks as pure functions over explicit inputs: actor identity, target identity, current roster snapshot, requested action, role policy, pending invites, and named config.

**Rationale:** Clan creation, ownership transfer, role edits, kicks, and disbands are high-risk authorization paths. Pure cores make valid and invalid transitions testable without Bevy `World`, sockets, commands, clocks, logging, or persistence IO.

### 3. Keep Bevy shell ownership explicit

**Choice:** Runtime state uses Bevy components/resources/events where useful: clan membership markers on players, clan registry resources, role policy resources, lifecycle events, audit events, and named system sets/run conditions for command handling, core evaluation, chat/presence update, and persistence flush.

**Rationale:** The shell should be boring and reviewable: gather command/chat/network facts, call pure cores, apply returned mutations, emit feedback, and schedule persistence.

### 4. Treat permissions as capabilities, not rank numbers

**Choice:** Roles grant named capabilities such as invite, kick, promote, demote, edit display, transfer ownership, disband, speak in officer chat, and manage persistence-visible notes. Numeric hierarchy, if any, is represented by named policy fields rather than hidden rank arithmetic.

**Rationale:** Capability checks are easier to audit than magic-number rank thresholds and support custom clan roles without special-case logic.

### 5. Make identity validation deterministic

**Choice:** Clan names, tags, colors, descriptions, and display forms are validated by pure normalization rules with named length, character, uniqueness, reserved-word, and collision policies.

**Rationale:** Identity text touches chat, scoreboards, logs, and persistence. Deterministic validation prevents spoofing, ambiguous tags, and stale display collisions.

### 6. Define persistence as snapshots plus audit records

**Choice:** Persistence boundaries serialize normalized clan snapshots and append reviewable lifecycle/audit records. Loading fails closed on malformed snapshots, duplicate identifiers, impossible owners, dangling invites, invalid roles, or unsupported schema revisions.

**Rationale:** Durable clan state needs recovery behavior before claims or economy can depend on it. The pure loader/validator should reject corrupt state without mutating live ECS state.

## Risks / Trade-offs

- A social core without land claims may feel incomplete, but it gives later claim/raid Cairns a safe authority substrate.
- Capability-based roles are slightly more verbose than rank numbers, but they reduce hidden authorization bugs.
- Persistence implementation may be local-file, database, or fixture-backed later; this Cairn should define deterministic snapshot contracts without claiming production storage.
- Chat/presence integration can accidentally imply scoreboard UI parity. Keep UI and packet-family coverage as explicit non-claims unless separate evidence proves them.
