# valence-bevy-ecs Change Spec: Gameplay arena and layer scoping

## Requirements

### Requirement: Gameplay arena scoping inventory

r[valence_bevy_ecs.gameplay_arena_scoping.inventory] Gameplay arena scoping work MUST inventory selected CTF and survival compatibility global resources, event payloads, layer/entity assumptions, cleanup paths, milestone emitters, and cross-mode mutation risks before changing ownership.

#### Scenario: Scope risks are visible

r[valence_bevy_ecs.gameplay_arena_scoping.inventory.visible]
- GIVEN a CTF or survival compatibility system is selected for arena scoping
- WHEN reviewers inspect the inventory
- THEN each global resource, event payload, layer or entity query, cleanup path, milestone emitter, and mutation target records whether it is global, arena-owned, layer-owned, client-owned, or fixture-only
- AND risks for same-app CTF plus survival, multiple CTF arenas, multiple survival fixtures, stale arenas, and wrong-layer entities are identified.

### Requirement: Gameplay arena ownership model

r[valence_bevy_ecs.gameplay_arena_scoping.model] Runtime gameplay mode state SHOULD be represented by explicit arena or layer ownership rather than single global mode resources when multiple instances or modes can coexist.

#### Scenario: Arena owns gameplay instance state

r[valence_bevy_ecs.gameplay_arena_scoping.model.owned]
- GIVEN a gameplay mode has score, flag, fixture, container, rule, or presentation state that can vary by arena
- WHEN the state ownership is reviewed
- THEN it is attached to an arena entity, layer-owned component, or explicitly scoped state handle
- AND any remaining global resource records why it is a default, registry, policy, or compatibility shim rather than per-arena state.

### Requirement: Scoped gameplay wiring

r[valence_bevy_ecs.gameplay_arena_scoping.wiring] Gameplay plugin systems MUST filter by explicit scope and mutate only the arenas, layers, clients, entities, resources, and milestones owned by that scope.

#### Scenario: Systems ignore unrelated scope

r[valence_bevy_ecs.gameplay_arena_scoping.wiring.filtered]
- GIVEN CTF and survival compatibility plugins are installed in the same app with distinct arenas or layers
- WHEN scoped systems process input, rules, world mutation, presentation, or cleanup
- THEN each system only reads and mutates data belonging to its matching gameplay scope
- AND wrong-scope, missing-scope, or stale-scope inputs are ignored, cleaned up, or diagnosed deterministically.

### Requirement: Scoped gameplay events and milestones

r[valence_bevy_ecs.gameplay_arena_scoping.events] Gameplay events, diagnostics, and compatibility milestones SHOULD include arena or scope identity when the same semantic can occur in multiple arenas or modes in one app.

#### Scenario: Receipts distinguish arenas

r[valence_bevy_ecs.gameplay_arena_scoping.events.disambiguated]
- GIVEN multiple gameplay arenas can emit the same event or milestone text
- WHEN downstream systems or compatibility receipts observe those events
- THEN the payload or receipt context identifies the owning arena, layer, or gameplay scope
- AND legacy receipt comparability is preserved through documented adapters or explicit evidence non-claims.

### Requirement: Gameplay arena scoping tests

r[valence_bevy_ecs.gameplay_arena_scoping.tests] Gameplay arena scoping work MUST include positive multi-mode or multi-arena tests and negative wrong-scope, stale-scope, missing-scope, disabled-plugin, and cross-layer mutation tests.

#### Scenario: Multiple scoped arenas coexist

r[valence_bevy_ecs.gameplay_arena_scoping.tests.positive]
- GIVEN CTF and survival compatibility plugins or multiple instances of one mode are installed in one app with distinct arenas
- WHEN valid scoped events and systems run
- THEN each arena updates only its own state, emits scope-identifiable observations, and preserves selected fixture behavior.

#### Scenario: Invalid scope fails closed

r[valence_bevy_ecs.gameplay_arena_scoping.tests.negative]
- GIVEN an event, client, entity, container, flag, score, or cleanup target has a missing, stale, disabled-plugin, or wrong-mode scope
- WHEN gameplay systems process it
- THEN no unrelated arena mutates, no false milestone is emitted, stale ownership is cleaned or ignored deterministically, and no panic occurs.

### Requirement: Gameplay arena scoping validation

r[valence_bevy_ecs.gameplay_arena_scoping.validation] Gameplay arena scoping work MUST record focused CTF/survival checks, selected compatibility rails when touched, Valence schedule hygiene, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests before archive.

#### Scenario: Arena scoping closeout is reviewable

r[valence_bevy_ecs.gameplay_arena_scoping.validation.log]
- GIVEN gameplay arena scoping work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused CTF and survival checks, positive and negative scoping tests, selected compatibility rails if fixture behavior or receipts changed, schedule hygiene, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
