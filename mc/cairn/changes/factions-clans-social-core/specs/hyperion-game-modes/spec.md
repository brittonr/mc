# hyperion-game-modes Change Spec: Factions/clans social core

## Requirements

### Requirement: Factions/clans social inventory

r[hyperion_game_modes.factions_clans.social.inventory] Factions/clans social-core work MUST inventory Hyperion event/plugin wiring, command surfaces, permission capabilities, chat/text surfaces, player identity sources, stats or persistence options, and reusable seams before implementing clan state.

#### Scenario: Social baseline is reviewable

r[hyperion_game_modes.factions_clans.social.inventory.reviewable]
- GIVEN factions/clans social-core work is selected
- WHEN reviewers inspect the inventory
- THEN existing event crate patterns, command handlers, permission surfaces, chat/text routing, player identity facts, persistence options, and reusable extension seams are recorded
- AND land claims, raids, diplomacy, economy, public-server moderation, production persistence, Bedwars behavior, Valence behavior, and broad compatibility remain explicit non-claims.

### Requirement: Factions/clans social scope

r[hyperion_game_modes.factions_clans.social.scope] Factions/clans social-core work MUST be scoped as a Hyperion-owned mode or plugin layer with mode-local clan identity, roster state, role policy, named configuration values, and functional-core/imperative-shell boundaries.

#### Scenario: Scope excludes later gameplay layers

r[hyperion_game_modes.factions_clans.social.scope.excludes_later_layers]
- GIVEN the social-core plugin is added to a Hyperion app
- WHEN no separate claim, raid, diplomacy, or economy plugin is enabled
- THEN clan identity, membership, role, chat, presence, audit, and persistence behavior can operate without mutating territory, war, alliance, vault, tax, or raid state
- AND non-clan Hyperion modes remain unchanged.

### Requirement: Clan identity validation

r[hyperion_game_modes.factions_clans.social.identity] Factions/clans social core MUST validate clan names, tags, display text, descriptions, uniqueness, reserved words, and collision policy through deterministic pure rules.

#### Scenario: Valid identity is normalized

r[hyperion_game_modes.factions_clans.social.identity.valid]
- GIVEN a clan creation request with allowed name, tag, display text, description, and no configured collision
- WHEN the identity core validates the request
- THEN it returns a normalized clan identity with stable identifiers and display fields derived from explicit inputs and named configuration values.

#### Scenario: Invalid identity fails closed

r[hyperion_game_modes.factions_clans.social.identity.rejects]
- GIVEN a clan creation or edit request with forbidden characters, reserved words, ambiguous color or formatting, duplicate normalized identity, missing owner, or unsupported text shape
- WHEN the identity core validates the request
- THEN it rejects the request with a deterministic diagnostic
- AND no clan, tag, chat route, scoreboard-like display, or persistence record is created.

### Requirement: Membership lifecycle

r[hyperion_game_modes.factions_clans.social.membership] Factions/clans social core MUST model create, invite, accept, deny, leave, kick, transfer ownership, role change, and disband transitions as deterministic membership decisions.

#### Scenario: Valid lifecycle transition mutates one clan snapshot

r[hyperion_game_modes.factions_clans.social.membership.valid]
- GIVEN an actor has the required capability and the clan snapshot, target player, pending invite, and requested lifecycle action are valid
- WHEN the membership core evaluates the action
- THEN it returns exactly the clan membership, invite, role, ownership, audit, and feedback changes required by that action.

#### Scenario: Invalid lifecycle transition is rejected

r[hyperion_game_modes.factions_clans.social.membership.rejects]
- GIVEN an actor lacks capability, targets a stale invite, attempts self-conflicting ownership, creates an ownerless clan, kicks a missing member, duplicates a request, or acts from the wrong mode
- WHEN the membership core evaluates the action
- THEN the transition is rejected with a deterministic reason
- AND no roster corruption, privilege escalation, duplicate owner, dangling invite, panic, or cross-mode mutation occurs.

### Requirement: Role permissions

r[hyperion_game_modes.factions_clans.social.roles_permissions] Factions/clans social core MUST authorize clan operations through named capabilities rather than hidden rank-number thresholds.

#### Scenario: Capability grants action

r[hyperion_game_modes.factions_clans.social.roles_permissions.allowed]
- GIVEN a clan role policy grants a named capability to the actor's role
- WHEN the actor requests a lifecycle, chat, display, audit, or persistence-visible operation requiring that capability
- THEN the authorization core permits the operation and records the capability that allowed it.

#### Scenario: Missing capability denies action

r[hyperion_game_modes.factions_clans.social.roles_permissions.denied]
- GIVEN the actor's role lacks the required named capability, targets a protected role, or tries to grant capabilities outside policy
- WHEN authorization runs
- THEN the operation is denied without applying the requested mutation
- AND the diagnostic identifies the missing, protected, or invalid capability.

### Requirement: Social shell integration

r[hyperion_game_modes.factions_clans.social.shell] Factions/clans social shell systems MUST gather Bevy, command, chat, identity, and persistence inputs, call pure cores, apply returned mutations, emit feedback, and keep side effects outside the cores.

#### Scenario: Shell applies core-approved mutation

r[hyperion_game_modes.factions_clans.social.shell.applies]
- GIVEN a command or event produces a valid core outcome for a clan-scoped action
- WHEN the shell system runs
- THEN it mutates only the relevant Bevy components, resources, events, feedback messages, audit records, and persistence queue entries returned by the core
- AND packet IO, logging, command parsing, clocks, and storage writes remain shell responsibilities.

#### Scenario: Shell fails closed on stale state

r[hyperion_game_modes.factions_clans.social.shell.rejects_stale]
- GIVEN command input references a disconnected player, missing clan, stale snapshot revision, wrong mode, or malformed identity source
- WHEN the shell system runs
- THEN no unapproved clan mutation is applied
- AND the rejection is observable through feedback or diagnostics.

### Requirement: Clan chat and presence

r[hyperion_game_modes.factions_clans.social.chat_presence] Factions/clans social core SHOULD expose clan-scoped chat and presence summaries without broadening global chat, proximity chat, scoreboard UI, or packet-family claims.

#### Scenario: Clan chat routes to members

r[hyperion_game_modes.factions_clans.social.chat_presence.routes]
- GIVEN a live clan member sends a clan-scoped chat message that passes validation
- WHEN chat routing runs
- THEN only current authorized clan recipients and configured audit sinks receive the clan message
- AND global, proximity, admin, and spectator chat behavior remains unchanged unless separately scoped.

#### Scenario: Invalid chat route is contained

r[hyperion_game_modes.factions_clans.social.chat_presence.rejects]
- GIVEN a non-member, muted actor, stale member, malformed target clan, invalid message, or disabled chat route attempts clan chat
- WHEN chat routing runs
- THEN no clan message is delivered to unauthorized recipients
- AND diagnostics do not leak hidden roster or moderation state beyond configured feedback.

### Requirement: Social persistence snapshots

r[hyperion_game_modes.factions_clans.social.persistence] Factions/clans social persistence MUST define deterministic clan snapshot and audit contracts for identity, rosters, roles, pending invites, schema revisions, and recovery behavior.

#### Scenario: Valid snapshot round trips

r[hyperion_game_modes.factions_clans.social.persistence.round_trip]
- GIVEN a valid clan snapshot with identity, owner, members, roles, pending invites, and audit metadata
- WHEN persistence serialization and loading run
- THEN the loaded snapshot preserves normalized identity, roster, role capabilities, invites, schema revision, and audit facts.

#### Scenario: Corrupt snapshot is rejected

r[hyperion_game_modes.factions_clans.social.persistence.rejects]
- GIVEN a snapshot has duplicate identifiers, invalid roles, missing owner, dangling invites, unsupported schema, malformed text, impossible membership, or truncated data
- WHEN the loader validates it
- THEN the snapshot is rejected deterministically before live ECS mutation
- AND recovery diagnostics identify the invalid field or relationship.

### Requirement: Social tests

r[hyperion_game_modes.factions_clans.social.tests] Factions/clans social-core work MUST include positive tests for valid identity, lifecycle, authorization, shell, chat, presence, and persistence behavior plus negative tests for malformed, duplicate, unauthorized, stale, corrupt, disconnected, and cleanup-leak cases.

#### Scenario: Positive social behavior is covered

r[hyperion_game_modes.factions_clans.social.tests.positive]
- GIVEN representative valid identity, membership, role, shell, chat, presence, persistence, and audit inputs
- WHEN pure cores and focused shell tests run
- THEN supported clan creation, invite acceptance, role authorization, chat routing, presence summary, snapshot round trip, and audit outcomes pass.

#### Scenario: Negative social behavior fails closed

r[hyperion_game_modes.factions_clans.social.tests.negative]
- GIVEN invalid names, duplicate tags, unauthorized actors, stale invites, ownerless clans, privilege escalation, wrong mode, disconnected players, corrupt snapshots, unsupported schemas, malformed chat routes, or cleanup-leak fixtures
- WHEN pure cores and focused shell tests run
- THEN the social core rejects or diagnoses each case deterministically
- AND no roster corruption, unauthorized message, persistent orphan, panic, privilege escalation, or cross-mode mutation occurs.

### Requirement: Social validation

r[hyperion_game_modes.factions_clans.social.validation] Factions/clans social-core work MUST record baseline checks before shared Hyperion edits when applicable, focused social-core tests from `hyperion/`, Bevy shell/plugin checks, command/permission checks, persistence fixture checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Social closeout is reviewable

r[hyperion_game_modes.factions_clans.social.validation.log]
- GIVEN factions/clans social-core work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show relevant baseline checks, positive and negative social-core tests, shell/plugin checks, command/permission checks, persistence fixture checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
