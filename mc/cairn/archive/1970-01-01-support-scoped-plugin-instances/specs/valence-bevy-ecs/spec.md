# valence-bevy-ecs Change Spec: Scoped plugin instances

## Requirements

### Requirement: Scoped plugin instance inventory

r[valence_bevy_ecs.scoped_plugin_instances.inventory] Scoped plugin instance work MUST inventory current Valence gameplay scope identifiers, plugin contract keys, global resources, event payloads, milestone emitters, cleanup paths, disabled-plugin behavior, and same-app coexistence risks before changing scope ownership.

#### Scenario: Instance-scope risks are reviewable

r[valence_bevy_ecs.scoped_plugin_instances.inventory.reviewable]
- GIVEN scoped plugin instance work is selected
- WHEN reviewers inspect the inventory
- THEN static primary arena IDs, plugin-name-only contract keys, selected CTF/survival/terrain globals, event payloads, milestone emitters, cleanup paths, and wrong-scope risks are recorded
- AND single-primary compatibility assumptions are separated from future multi-instance or multi-mode claims.

### Requirement: Scoped plugin instance model

r[valence_bevy_ecs.scoped_plugin_instances.model] Valence gameplay plugins SHOULD represent runtime plugin instances through explicit mode, arena, layer, plugin-role, or fixture ownership identities rather than plugin name alone when multiple instances can coexist.

#### Scenario: Instance identity distinguishes same plugin twice

r[valence_bevy_ecs.scoped_plugin_instances.model.identity]
- GIVEN two arenas or fixtures use the same gameplay plugin semantics in one app
- WHEN plugin contracts, events, diagnostics, or state ownership are reviewed
- THEN each instance has a deterministic identity that distinguishes its arena, layer, plugin role, or fixture owner
- AND remaining global resources state why they are policy, default, registry, or compatibility shim state.

### Requirement: Scoped plugin instance wiring

r[valence_bevy_ecs.scoped_plugin_instances.wiring] Selected Valence gameplay plugin systems MUST filter by explicit instance or scope identity and mutate only the arenas, layers, clients, entities, resources, diagnostics, and milestones owned by that instance.

#### Scenario: Valid instance mutates only owned state

r[valence_bevy_ecs.scoped_plugin_instances.wiring.valid]
- GIVEN a scoped event or entity belongs to a selected gameplay plugin instance
- WHEN input, rule evaluation, world mutation, presentation, or cleanup systems run
- THEN only state owned by that matching instance is read or mutated
- AND emitted diagnostics or milestones include enough identity to distinguish concurrent instances where required.

#### Scenario: Wrong instance fails closed

r[valence_bevy_ecs.scoped_plugin_instances.wiring.invalid]
- GIVEN an event, entity, layer, client, container, score, flag, or cleanup target has a missing, stale, wrong-mode, wrong-arena, or disabled-plugin scope
- WHEN selected gameplay systems process it
- THEN unrelated instances are not mutated and no false milestone is emitted
- AND the invalid input is ignored, cleaned up, or diagnosed deterministically without panic.

### Requirement: Scoped plugin instance compatibility

r[valence_bevy_ecs.scoped_plugin_instances.compatibility] Scoped plugin instance work MUST preserve selected single-primary fixture behavior, command/env/CLI contracts, and compatibility milestone comparability unless another Cairn changes receipt semantics.

#### Scenario: Legacy fixture remains comparable

r[valence_bevy_ecs.scoped_plugin_instances.compatibility.legacy]
- GIVEN an existing CTF, survival, or terrain compatibility fixture uses its default primary scope
- WHEN scoped instance identity is introduced
- THEN legacy commands, env/CLI inputs, milestone text, and selected receipts remain comparable through default IDs or documented adapters
- AND production multi-tenant gameplay, default Valence gameplay, vanilla parity, and public-server safety remain non-claims.

### Requirement: Scoped plugin instance tests

r[valence_bevy_ecs.scoped_plugin_instances.tests] Scoped plugin instance work MUST include positive multi-mode or multi-instance tests and negative wrong-scope, stale-scope, missing-scope, disabled-plugin, duplicate-identity, and cross-layer mutation tests.

#### Scenario: Multiple instances coexist

r[valence_bevy_ecs.scoped_plugin_instances.tests.positive]
- GIVEN selected gameplay plugins or fixtures are installed with distinct instance identities in one app
- WHEN valid scoped events and systems run
- THEN each instance updates only its own state, emits identity-bearing observations where required, and preserves selected fixture behavior.

#### Scenario: Invalid instance data fails closed

r[valence_bevy_ecs.scoped_plugin_instances.tests.negative]
- GIVEN wrong-scope, stale-scope, missing-scope, duplicate identity, disabled-plugin, or cross-layer fixtures are processed
- WHEN scope tests run
- THEN no unrelated instance mutates, no false milestone is emitted, stale ownership is cleaned or ignored deterministically, and no panic occurs.

### Requirement: Scoped plugin instance validation

r[valence_bevy_ecs.scoped_plugin_instances.validation] Scoped plugin instance work MUST record focused Valence scope tests, selected CTF/survival/terrain checks, selected compatibility rails when behavior or receipts change, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Scoped instance closeout is reviewable

r[valence_bevy_ecs.scoped_plugin_instances.validation.log]
- GIVEN scoped plugin instance work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, model checks, positive and negative scope tests, selected gameplay/example checks, selected compatibility dry-runs if fixture receipts changed, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
