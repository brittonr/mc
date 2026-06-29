# Valence Hyperion Integration Specification

## Purpose

Defines the `valence-hyperion-integration` capability.

## Requirements

### Requirement: Hyperion integration inventory

r[valence_hyperion_integration.boundaries.inventory] Future Hyperion-to-Valence integration work MUST classify inspected Hyperion sources as adopt, port, reference, or reject before implementation.

#### Scenario: Source classification precedes code changes

r[valence_hyperion_integration.boundaries.inventory.precedes]
- GIVEN a future integration Cairn proposes using Hyperion code or concepts
- WHEN reviewers inspect its design
- THEN each relevant source is classified as adopt, port, reference, or reject
- AND the classification explains ownership, safety, and API impact.

### Requirement: Forbidden Valence core merges

r[valence_hyperion_integration.boundaries.forbidden_core] Integration work MUST NOT merge Bedwars-specific game logic, replace Valence's runtime with Hyperion's runtime wholesale, add custom combat as Valence core behavior, or import unaudited nightly/unsafe-heavy code directly into Valence core.

#### Scenario: Forbidden source is rejected

r[valence_hyperion_integration.boundaries.forbidden_core.rejected]
- GIVEN an inspected Hyperion source is Bedwars-specific, runtime-replacement scope, custom combat core behavior, or unaudited nightly/unsafe-heavy implementation
- WHEN the integration inventory is evaluated
- THEN the source is classified as reject or reference-only
- AND no Valence core task depends on copying it directly.

### Requirement: Optional gameplay plugin boundary

r[valence_hyperion_integration.boundaries.optional_plugins] Gameplay semantics inspired by Hyperion MAY be implemented only as optional plugins or examples unless separate accepted Valence scope and reference evidence justify core behavior.

#### Scenario: Combat remains optional without reference evidence

r[valence_hyperion_integration.boundaries.optional_plugins.combat]
- GIVEN Hyperion combat behavior is considered for Valence
- WHEN no separate vanilla/reference evidence proves the intended core behavior
- THEN the work is scoped as an optional plugin, example, or reference-only note
- AND Valence core behavior remains unchanged.

### Requirement: Integration review gate

r[valence_hyperion_integration.boundaries.review_gate] Future Hyperion integration Cairns SHOULD cite the boundary inventory and non-claim checklist before archive.

#### Scenario: Non-claim checklist is present

r[valence_hyperion_integration.boundaries.review_gate.non_claims]
- GIVEN an integration Cairn is ready to archive
- WHEN reviewers inspect its proposal and evidence
- THEN production-scale, vanilla-parity, Hyperion-compatibility, default-behavior, and safety claims are each either supported by evidence or explicitly left as non-claims.

### Requirement: Boundary fixtures

r[valence_hyperion_integration.boundaries.fixtures] Boundary work SHOULD include positive and negative checklist examples or fixtures for allowed reference use and forbidden direct imports.

#### Scenario: Reference-only use passes

r[valence_hyperion_integration.boundaries.fixtures.reference_only]
- GIVEN a future Cairn uses Hyperion code only as design reference
- WHEN the boundary checklist is evaluated
- THEN it passes if no copied code or unsupported behavior claim is present
- AND it records the referenced source and resulting Valence-owned design.

### Requirement: Boundary validation

r[valence_hyperion_integration.boundaries.validation] Boundary work MUST record inventory/checklist validation, negative forbidden-import examples, and Cairn gates before archive.

#### Scenario: Boundary closeout is reviewable

r[valence_hyperion_integration.boundaries.validation.log]
- GIVEN the boundary change is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show inventory/checklist validation, positive reference-only examples, negative forbidden-import examples, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Hyperion bot packet scope

r[valence_hyperion_integration.hyperion_bot_packet.scope] Hyperion bot packet utility work MUST be scoped as Hyperion tool-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion bot ownership is explicit

r[valence_hyperion_integration.hyperion_bot_packet.scope.owned]
- GIVEN Hyperion bot packet work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local tool work
- AND it does not claim Valence adoption, public-server safety, or compatibility evidence.

### Requirement: Hyperion bot packet core

r[valence_hyperion_integration.hyperion_bot_packet.core] Hyperion bot packet utilities SHOULD expose pure cores for packet construction, packet classification, byte-shape validation, and protocol-assumption checks.

#### Scenario: Bot packet decision is testable without network

r[valence_hyperion_integration.hyperion_bot_packet.core.testable]
- GIVEN bot packet input summaries
- WHEN the Hyperion bot packet core processes them
- THEN the result can be tested without socket IO, connection state, async runtime, timing, or logging.

### Requirement: Hyperion bot packet shell boundary

r[valence_hyperion_integration.hyperion_bot_packet.shell_boundary] Hyperion bot packet extraction MUST keep socket IO, connection state, async tasks, sleeps/timing, logging, and bot orchestration outside pure packet cores.

#### Scenario: Bot packet side effects remain in shell

r[valence_hyperion_integration.hyperion_bot_packet.shell_boundary.effects]
- GIVEN the bot packet core returns a packet or classification decision
- WHEN the Hyperion bot shell applies that decision
- THEN only the shell performs IO, mutates connection state, schedules async work, sleeps, or logs diagnostics.

### Requirement: Hyperion bot packet parity

r[valence_hyperion_integration.hyperion_bot_packet.parity] Hyperion bot packet extraction MUST preserve bot tool CLI/API behavior, packet bytes, protocol assumptions, and non-claims.

#### Scenario: Hyperion bot packet behavior remains stable

r[valence_hyperion_integration.hyperion_bot_packet.parity.stable]
- GIVEN a supported pre-refactor bot packet input
- WHEN the extracted packet core and shell process the same input
- THEN packet bytes, classification, public tool behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion bot packet tests

r[valence_hyperion_integration.hyperion_bot_packet.tests] The change MUST include positive and negative tests for valid packet construction, packet classification, malformed packet bytes, unsupported protocol assumptions, closed connections, and missing bot state.

#### Scenario: Bot packet fixtures cover success and failure

r[valence_hyperion_integration.hyperion_bot_packet.tests.coverage]
- GIVEN representative valid and invalid bot packet inputs
- WHEN bot packet tests run
- THEN they prove supported packet paths pass and malformed paths fail closed.

### Requirement: Hyperion bot packet validation

r[valence_hyperion_integration.hyperion_bot_packet.validation] The change MUST record focused Hyperion bot tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_bot_packet.validation.logs]
- GIVEN Hyperion bot packet extraction is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local bot tests plus Cairn gates passing.

### Requirement: Hyperion player join scope

r[valence_hyperion_integration.hyperion_player_join.scope] Hyperion player-join modularity work MUST be scoped as Hyperion-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion player-join ownership is explicit

r[valence_hyperion_integration.hyperion_player_join.scope.owned]
- GIVEN Hyperion player-join work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local
- AND it does not claim Valence adoption or mc-compat evidence.

### Requirement: Hyperion player join core

r[valence_hyperion_integration.hyperion_player_join.core] Hyperion player-join egress SHOULD expose pure cores for initial packet selection, packet ordering, state summaries, chunk/view facts, and diagnostics.

#### Scenario: Join plan is testable without runtime

r[valence_hyperion_integration.hyperion_player_join.core.testable]
- GIVEN player, world, chunk, and connection summaries
- WHEN the Hyperion player-join core processes them
- THEN the join plan can be tested without ECS app, network/proxy state, tracing, scheduling, or packet sends.

### Requirement: Hyperion player join shell boundary

r[valence_hyperion_integration.hyperion_player_join.shell_boundary] Hyperion player-join extraction MUST keep ECS reads, packet sends, network/proxy state, tracing, scheduling, and runtime side effects outside pure join cores.

#### Scenario: Join side effects remain in shell

r[valence_hyperion_integration.hyperion_player_join.shell_boundary.effects]
- GIVEN the player-join core returns a join plan
- WHEN the Hyperion egress shell applies that plan
- THEN only the shell reads ECS state, sends packets, touches network/proxy state, records traces, or wires schedules.

### Requirement: Hyperion player join parity

r[valence_hyperion_integration.hyperion_player_join.parity] Hyperion player-join extraction MUST preserve Hyperion join behavior, packet order, public APIs, performance-sensitive boundaries, and non-claims.

#### Scenario: Hyperion join behavior remains stable

r[valence_hyperion_integration.hyperion_player_join.parity.stable]
- GIVEN a supported pre-refactor Hyperion player-join input
- WHEN the extracted join core and shell process the same input
- THEN packet order, join output, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion player join tests

r[valence_hyperion_integration.hyperion_player_join.tests] The change MUST include positive and negative tests for valid join plans, missing player state, invalid chunk/view facts, packet-order regressions, and rejected join inputs.

#### Scenario: Player-join fixtures cover success and failure

r[valence_hyperion_integration.hyperion_player_join.tests.coverage]
- GIVEN representative valid and invalid Hyperion player-join inputs
- WHEN join tests run
- THEN supported join plans pass and malformed join inputs fail closed.

### Requirement: Hyperion player join validation

r[valence_hyperion_integration.hyperion_player_join.validation] The change MUST record focused Hyperion tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_player_join.validation.logs]
- GIVEN Hyperion player-join extraction is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tests plus Cairn gates passing.

### Requirement: Hyperion inventory scope

r[valence_hyperion_integration.hyperion_inventory.scope] Hyperion inventory modularity work MUST be scoped as Hyperion-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion ownership is explicit

r[valence_hyperion_integration.hyperion_inventory.scope.owned]
- GIVEN Hyperion inventory modularity work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local
- AND it does not claim Valence adoption or compatibility evidence.

### Requirement: Hyperion inventory core

r[valence_hyperion_integration.hyperion_inventory.core] Hyperion inventory simulation SHOULD expose pure cores for inventory transitions, slot validation, transaction outcomes, and packet-facing summaries.

#### Scenario: Inventory transition is testable without runtime

r[valence_hyperion_integration.hyperion_inventory.core.testable]
- GIVEN inventory state and transaction summaries
- WHEN the Hyperion inventory core processes them
- THEN the result can be tested without Bevy runtime, network IO, proxy state, or tracing side effects.

### Requirement: Hyperion inventory shell boundary

r[valence_hyperion_integration.hyperion_inventory.shell_boundary] Hyperion inventory extraction MUST keep ECS mutation, packet emission, scheduling, tracing, and network/proxy side effects outside pure inventory cores.

#### Scenario: Inventory side effects remain in shell

r[valence_hyperion_integration.hyperion_inventory.shell_boundary.effects]
- GIVEN the inventory core returns a transition decision
- WHEN the Hyperion shell applies that decision
- THEN only the shell mutates ECS state, emits packets, wires schedules, records traces, or touches network/proxy state.

### Requirement: Hyperion inventory parity

r[valence_hyperion_integration.hyperion_inventory.parity] Hyperion inventory extraction MUST preserve Hyperion public APIs, simulation behavior, packet-facing behavior, performance-sensitive boundaries, and non-claims.

#### Scenario: Hyperion inventory behavior remains stable

r[valence_hyperion_integration.hyperion_inventory.parity.stable]
- GIVEN a supported pre-refactor Hyperion inventory input
- WHEN the extracted core and shell process the same input
- THEN inventory state, packet-facing output, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion inventory tests

r[valence_hyperion_integration.hyperion_inventory.tests] The change MUST include positive and negative tests for valid transactions, invalid slots, malformed packets, empty inventories, boundary stack sizes, and rejected transitions.

#### Scenario: Inventory fixtures cover success and failure

r[valence_hyperion_integration.hyperion_inventory.tests.coverage]
- GIVEN representative valid and invalid Hyperion inventory inputs
- WHEN inventory tests run
- THEN they prove supported transitions pass and malformed transitions fail closed.

### Requirement: Hyperion inventory validation

r[valence_hyperion_integration.hyperion_inventory.validation] The change MUST record focused Hyperion tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_inventory.validation.logs]
- GIVEN Hyperion inventory extraction is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tests plus Cairn gates passing.

### Requirement: Hyperion block loader scope

r[valence_hyperion_integration.hyperion_block_loader.scope] Hyperion block-loader modularity work MUST be scoped as Hyperion-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion block-loader ownership is explicit

r[valence_hyperion_integration.hyperion_block_loader.scope.owned]
- GIVEN Hyperion block-loader modularity work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local
- AND it does not claim Valence adoption or compatibility evidence.

### Requirement: Hyperion block loader core

r[valence_hyperion_integration.hyperion_block_loader.core] Hyperion block loading SHOULD expose pure cores for parsing summaries, validation decisions, palette or section plans, and storage update plans.

#### Scenario: Block-loader decision is testable without runtime IO

r[valence_hyperion_integration.hyperion_block_loader.core.testable]
- GIVEN block-loader input summaries
- WHEN the Hyperion block-loader core processes them
- THEN the result can be tested without file IO, decompression, ECS mutation, tracing, or runtime scheduling.

### Requirement: Hyperion block loader shell boundary

r[valence_hyperion_integration.hyperion_block_loader.shell_boundary] Hyperion block-loader extraction MUST keep file/resource reads, decompression, storage mutation, ECS mutation, tracing, and runtime scheduling outside pure block-loader cores.

#### Scenario: Block-loader side effects remain in shell

r[valence_hyperion_integration.hyperion_block_loader.shell_boundary.effects]
- GIVEN the block-loader core returns a parse or update plan
- WHEN the Hyperion shell applies that plan
- THEN only the shell reads resources, decompresses data, mutates storage, mutates ECS, records traces, or wires schedules.

### Requirement: Hyperion block loader parity

r[valence_hyperion_integration.hyperion_block_loader.parity] Hyperion block-loader modularization MUST preserve Hyperion block-loader APIs, world/block behavior, performance-sensitive boundaries, and non-claims.

#### Scenario: Hyperion block-loader behavior remains stable

r[valence_hyperion_integration.hyperion_block_loader.parity.stable]
- GIVEN a supported pre-refactor Hyperion block-loader input
- WHEN the modularized loader processes the same input
- THEN block state, storage-facing output, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion block loader tests

r[valence_hyperion_integration.hyperion_block_loader.tests] The change MUST include positive and negative tests for valid sections, malformed sections, palette edge cases, missing resources, invalid block ids, and rejected update plans.

#### Scenario: Block-loader fixtures cover success and failure

r[valence_hyperion_integration.hyperion_block_loader.tests.coverage]
- GIVEN representative valid and invalid Hyperion block-loader inputs
- WHEN block-loader tests run
- THEN they prove supported inputs pass and malformed inputs fail closed.

### Requirement: Hyperion block loader validation

r[valence_hyperion_integration.hyperion_block_loader.validation] The change MUST record focused Hyperion tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_block_loader.validation.logs]
- GIVEN Hyperion block-loader modularization is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tests plus Cairn gates passing.

### Requirement: Spatial/raycast inventory

r[valence_hyperion_integration.spatial_raycast.inventory] The integration MUST inventory Valence spatial APIs and Hyperion geometry/raycast/collision helpers before changing public spatial behavior.

#### Scenario: Source classification is recorded

r[valence_hyperion_integration.spatial_raycast.inventory.classified]
- GIVEN spatial/raycast work is selected
- WHEN reviewers inspect the inventory
- THEN each relevant Hyperion and Valence source is classified as adopt, port, reference, or reject
- AND any nightly or unsafe-heavy source is rejected or marked for separate audit.

### Requirement: Raycast collision contract

r[valence_hyperion_integration.spatial_raycast.contract] Valence SHOULD define deterministic semantics for entity hitboxes, block hits, owner exclusion, invalid rays, boundary hits, starts inside hitboxes, and block/entity tie ordering.

#### Scenario: Entity and block tie is deterministic

r[valence_hyperion_integration.spatial_raycast.contract.tie]
- GIVEN an entity hit and block hit occur at the same documented collision distance
- WHEN the collision comparator evaluates both hits
- THEN it returns the documented winner or tie result deterministically
- AND gameplay callers do not observe iteration-order-dependent behavior.

### Requirement: Pure spatial/raycast core

r[valence_hyperion_integration.spatial_raycast.core] Ray traversal, hitbox intersection, and collision comparison MUST be pure deterministic functions over explicit inputs, with ECS/world access limited to thin shells.

#### Scenario: Invalid ray produces deterministic result

r[valence_hyperion_integration.spatial_raycast.core.invalid_ray]
- GIVEN a ray has a zero-length direction, NaN coordinate, or invalid bound
- WHEN the pure core evaluates it
- THEN it returns the documented error or no-hit result
- AND it does not panic or inspect global state.

### Requirement: Spatial/raycast fixture coverage

r[valence_hyperion_integration.spatial_raycast.fixtures] Spatial/raycast work MUST include positive and negative fixtures for common rays and edge cases.

#### Scenario: Owner exclusion prevents self hit

r[valence_hyperion_integration.spatial_raycast.fixtures.owner_exclusion]
- GIVEN a ray starts at a player's eye position and the player's own hitbox intersects the ray
- WHEN owner exclusion is enabled
- THEN the player's own entity is not returned as the first entity hit
- AND other valid hits remain eligible.

### Requirement: Optional gameplay wiring

r[valence_hyperion_integration.spatial_raycast.wiring] Valence MAY expose gameplay/plugin helpers for raycast and collision queries, but those helpers MUST NOT claim vanilla combat parity without separate reference evidence.

#### Scenario: Helper docs avoid combat overclaiming

r[valence_hyperion_integration.spatial_raycast.wiring.non_overclaiming]
- GIVEN the raycast helper docs are published
- WHEN reviewers inspect them
- THEN they describe query mechanics and edge cases
- AND they do not claim vanilla combat, projectile damage, or reach parity without separate evidence.

### Requirement: Spatial/raycast validation

r[valence_hyperion_integration.spatial_raycast.validation] Spatial/raycast work MUST record geometry fixtures, gameplay smoke tests, selected projectile/raycast mc-compat scenarios, and Cairn gates before archive.

#### Scenario: Spatial closeout is reviewable

r[valence_hyperion_integration.spatial_raycast.validation.log]
- GIVEN spatial/raycast work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive geometry fixtures, negative invalid-ray fixtures, owner-exclusion tests, gameplay smoke output, selected mc-compat scenarios, and Cairn validation.

### Requirement: Command ergonomics scope

r[valence_hyperion_integration.command_ergonomics.scope] The integration MUST review Hyperion command ergonomics and Valence command graph APIs before adding command helper macros or builders.

#### Scenario: Valence command ownership is preserved

r[valence_hyperion_integration.command_ergonomics.scope.ownership]
- GIVEN command ergonomics work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes state that Hyperion's command framework is reference-only
- AND Valence command graph, parser, scope, and handler internals remain authoritative.

### Requirement: Command helper contract

r[valence_hyperion_integration.command_ergonomics.contract] Optional command helpers SHOULD define semantics for literals, arguments, parsers, suggestions, scopes, handlers, diagnostics, and manual fallback.

#### Scenario: Duplicate literal is rejected

r[valence_hyperion_integration.command_ergonomics.contract.duplicate]
- GIVEN helper input defines duplicate command literals at the same graph level
- WHEN the helper validates or expands the command
- THEN it reports a deterministic diagnostic
- AND no ambiguous command graph is registered.

### Requirement: Inspectable command helper output

r[valence_hyperion_integration.command_ergonomics.prototype] Command helper output MUST be inspectable as Valence command graph data or equivalent testable structures.

#### Scenario: Helper graph matches manual graph

r[valence_hyperion_integration.command_ergonomics.prototype.parity]
- GIVEN a helper-defined command and an equivalent manually built command graph
- WHEN graph parity is checked
- THEN literals, arguments, parsers, executable nodes, scopes, and suggestions match the documented expected graph.

### Requirement: Command helper tests

r[valence_hyperion_integration.command_ergonomics.tests] Command ergonomics work MUST include positive and negative tests for generated graphs, manual parity, parser errors, duplicate literals, missing handlers, invalid scopes, suggestions, and disabled behavior.

#### Scenario: Missing handler fails clearly

r[valence_hyperion_integration.command_ergonomics.tests.missing_handler]
- GIVEN helper input defines an executable command without a valid handler
- WHEN the helper validates or expands it
- THEN it reports the missing handler with a deterministic diagnostic
- AND the incomplete command is not registered.

### Requirement: Command helper docs

r[valence_hyperion_integration.command_ergonomics.docs] Command helper documentation SHOULD explain usage, diagnostics, limitations, and when manual graph construction remains preferred.

#### Scenario: Docs avoid framework replacement claim

r[valence_hyperion_integration.command_ergonomics.docs.non_claim]
- GIVEN command helper docs are published
- WHEN reviewers inspect them
- THEN they describe helpers as optional ergonomics over Valence command APIs
- AND they do not claim replacement of the command graph internals.

### Requirement: Command ergonomics validation

r[valence_hyperion_integration.command_ergonomics.validation] Command ergonomics work MUST record macro or builder tests, graph parity tests, parser/suggestion fixtures, and Cairn gates before archive.

#### Scenario: Command ergonomics closeout is reviewable

r[valence_hyperion_integration.command_ergonomics.validation.log]
- GIVEN command ergonomics work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive helper tests, negative diagnostic tests, graph parity fixtures, parser and suggestion fixtures, plugin-disabled checks, and Cairn validation.

### Requirement: Admin permission scope

r[valence_hyperion_integration.admin_permissions.scope] The integration MUST compare Hyperion permission/admin behavior with Valence command scopes before adding permission ergonomics.

#### Scenario: Command-system ownership is clear

r[valence_hyperion_integration.admin_permissions.scope.ownership]
- GIVEN admin permission work is selected
- WHEN reviewers inspect the scope notes
- THEN they identify the Hyperion concepts referenced, the Valence command/scopes surfaces affected, and the decision not to introduce a parallel command framework.

### Requirement: Pure permission evaluator

r[valence_hyperion_integration.admin_permissions.evaluator] Permission decisions MUST be implemented as pure deterministic evaluation over command metadata, player roles/scopes, and explicit context.

#### Scenario: Denied command is deterministic

r[valence_hyperion_integration.admin_permissions.evaluator.denied]
- GIVEN a player lacks the required role or scope for a command
- WHEN the evaluator checks that command
- THEN it returns the documented denial result
- AND it does not inspect ECS world state, storage, clocks, or network state.

### Requirement: Command integration

r[valence_hyperion_integration.admin_permissions.command_integration] Optional permission ergonomics SHOULD integrate with Valence's existing command system for command visibility, execution denial, and command-tree refresh.

#### Scenario: Role change refreshes command visibility

r[valence_hyperion_integration.admin_permissions.command_integration.refresh]
- GIVEN a player's command permission context changes
- WHEN the command integration observes the change
- THEN the player's command tree visibility is refreshed according to the evaluator result
- AND commands outside the player's scope are hidden or denied according to the documented policy.

### Requirement: Permission storage boundary

r[valence_hyperion_integration.admin_permissions.storage] Permission persistence MAY be provided, but storage MUST be optional and separated from pure permission evaluation.

#### Scenario: Missing storage row uses documented default

r[valence_hyperion_integration.admin_permissions.storage.missing]
- GIVEN persistence is enabled and a player has no permission row
- WHEN the storage adapter loads permission context
- THEN it returns the documented default or diagnostic
- AND the evaluator receives an explicit context value.

### Requirement: Admin permission tests

r[valence_hyperion_integration.admin_permissions.tests] Admin permission work MUST include positive and negative tests for allowed commands, denied commands, missing permission data, stale command trees, invalid storage rows, and plugin-disabled behavior.

#### Scenario: Plugin disabled preserves commands

r[valence_hyperion_integration.admin_permissions.tests.disabled]
- GIVEN the optional admin permission plugin is disabled
- WHEN existing Valence command tests run
- THEN command registration, parsing, execution, and suggestions preserve their previous behavior.

### Requirement: Admin permission validation

r[valence_hyperion_integration.admin_permissions.validation] Admin permission work MUST record permission tests, command integration tests, plugin-off regressions, and Cairn gates before archive.

#### Scenario: Admin permission closeout is reviewable

r[valence_hyperion_integration.admin_permissions.validation.log]
- GIVEN admin permission work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show evaluator tests, negative denial/storage fixtures, command integration tests, plugin-off regressions, docs checks if present, and Cairn validation.

### Requirement: Tool inventory

r[valence_hyperion_integration.tools.inventory] The integration MUST inventory Hyperion tools and Valence tools before adapting load or packet diagnostics.

#### Scenario: Tool concepts are classified

r[valence_hyperion_integration.tools.inventory.classified]
- GIVEN tool integration is selected
- WHEN reviewers inspect the inventory
- THEN each load-bot, packet-inspector, capture, and wrapper concept is classified as adopt, port, reference, or reject
- AND public API impact is marked as none unless separately justified.

### Requirement: Tool contract

r[valence_hyperion_integration.tools.contract] Load and packet tools MUST define typed configuration, safe target rules, output contracts, redaction policy, and non-claim boundaries.

#### Scenario: Unsafe target is rejected

r[valence_hyperion_integration.tools.contract.unsafe_target]
- GIVEN a tool config requests a network target outside the documented safe target policy
- WHEN the config validator runs
- THEN it rejects the config with a deterministic diagnostic
- AND no network connection is attempted.

### Requirement: Load bot tooling

r[valence_hyperion_integration.tools.load_bot] Valence MAY include load-bot tooling for loopback smoke and stress evidence, but the tool MUST report failures structurally and avoid compatibility overclaims.

#### Scenario: Load run failure is structured

r[valence_hyperion_integration.tools.load_bot.failure]
- GIVEN a load run cannot connect, times out, or receives an unexpected disconnect
- WHEN the load tool exits
- THEN it records the failing phase, target, configured scenario, and exit status
- AND it does not mark compatibility evidence as passing.

### Requirement: Packet inspector tooling

r[valence_hyperion_integration.tools.packet_inspector] Packet inspection tooling MUST bound capture output, handle malformed captures, and apply the documented redaction policy.

#### Scenario: Malformed capture fails closed

r[valence_hyperion_integration.tools.packet_inspector.malformed]
- GIVEN a capture contains malformed packet bytes
- WHEN the packet inspector decodes it
- THEN it reports the malformed boundary deterministically
- AND it does not emit unbounded or unredacted raw output.

### Requirement: Tool documentation

r[valence_hyperion_integration.tools.docs] Tool documentation SHOULD describe commands, configs, outputs, evidence usage, and non-claims.

#### Scenario: Tool docs separate evidence classes

r[valence_hyperion_integration.tools.docs.non_claims]
- GIVEN tool documentation is published
- WHEN reviewers inspect it
- THEN load, packet-diagnostic, compatibility, and vanilla-reference evidence are described as separate evidence classes.

### Requirement: Tool validation

r[valence_hyperion_integration.tools.validation] Tool work MUST record config tests, malformed capture fixtures, loopback smoke tests, selected dry runs, and Cairn gates before archive.

#### Scenario: Tool closeout is reviewable

r[valence_hyperion_integration.tools.validation.log]
- GIVEN tool integration is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show config positive/negative tests, malformed capture handling, loopback smoke tests, selected load dry runs, docs checks if present, and Cairn validation.

### Requirement: Anti-cheat statistics scope

r[valence_hyperion_integration.anticheat_stats.scope] The integration MUST audit Hyperion statistics behavior and Valence event sources before adding an anti-cheat statistics plugin.

#### Scenario: Metric scope is bounded

r[valence_hyperion_integration.anticheat_stats.scope.bounded]
- GIVEN anti-cheat statistics work is selected
- WHEN reviewers inspect the scope notes
- THEN the selected metrics, event sources, sampling windows, non-goals, and no-default-enforcement boundary are recorded.

### Requirement: Stable statistics core

r[valence_hyperion_integration.anticheat_stats.core] Statistics calculations MUST be implemented first as a pure stable Rust core over explicit samples and sample-window settings.

#### Scenario: Empty sample window is handled

r[valence_hyperion_integration.anticheat_stats.core.empty]
- GIVEN the statistics core receives an empty sample window
- WHEN it computes selected metrics
- THEN it returns the documented empty-window result
- AND it does not panic, divide by zero, read clocks, or mutate global state.

### Requirement: Statistics fixture coverage

r[valence_hyperion_integration.anticheat_stats.fixtures] Anti-cheat statistics work MUST include positive and negative fixtures for normal samples and invalid/boundary inputs.

#### Scenario: Invalid sample window fails closed

r[valence_hyperion_integration.anticheat_stats.fixtures.invalid_window]
- GIVEN a metric config has an invalid sample window
- WHEN the fixture validator runs
- THEN it returns a deterministic diagnostic
- AND the plugin does not emit a misleading score for that metric.

### Requirement: Optional statistics plugin

r[valence_hyperion_integration.anticheat_stats.plugin] Valence MAY expose an optional statistics plugin that samples explicit event streams and emits observations, but it MUST NOT enforce kicks, bans, or gameplay mutations by default.

#### Scenario: Plugin disabled has no effect

r[valence_hyperion_integration.anticheat_stats.plugin.disabled]
- GIVEN the statistics plugin is not enabled
- WHEN existing Valence gameplay and networking tests run
- THEN no anti-cheat components, metrics, or enforcement behavior are added.

### Requirement: Statistics documentation

r[valence_hyperion_integration.anticheat_stats.docs] Statistics plugin documentation SHOULD describe metric meanings, false-positive risks, data retention, and non-claims.

#### Scenario: Docs warn about enforcement limits

r[valence_hyperion_integration.anticheat_stats.docs.limits]
- GIVEN statistics docs are published
- WHEN reviewers inspect them
- THEN they state that metrics are advisory signals unless a separate policy plugin consumes them.

### Requirement: Statistics validation

r[valence_hyperion_integration.anticheat_stats.validation] Anti-cheat statistics work MUST record statistics tests, negative fixtures, plugin-off regressions, sampling smoke tests, and Cairn gates before archive.

#### Scenario: Statistics closeout is reviewable

r[valence_hyperion_integration.anticheat_stats.validation.log]
- GIVEN statistics plugin work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show pure statistics tests, negative invalid-input tests, plugin-off regressions, sampling smoke tests, docs checks if present, and Cairn validation.

### Requirement: GUI helper scope

r[valence_hyperion_integration.gui_helper.scope] The integration MUST review Hyperion GUI helper concepts and Valence inventory/window behavior before adding a GUI helper plugin.

#### Scenario: Inventory ownership remains clear

r[valence_hyperion_integration.gui_helper.scope.inventory_ownership]
- GIVEN GUI helper work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify which behavior remains owned by `valence_inventory` and which behavior belongs to the optional GUI helper.

### Requirement: GUI model contract

r[valence_hyperion_integration.gui_helper.model] The GUI helper SHOULD define an explicit model for windows, slots, readonly behavior, click outcomes, close events, and lifecycle cleanup.

#### Scenario: Readonly slot rejects mutation

r[valence_hyperion_integration.gui_helper.model.readonly]
- GIVEN a GUI slot is marked readonly
- WHEN a client click attempts to mutate that slot
- THEN the model returns the documented rejection or action result
- AND no inventory mutation is planned for that slot.

### Requirement: Pure GUI transitions

r[valence_hyperion_integration.gui_helper.core] GUI state transitions SHOULD be pure deterministic helpers over explicit model inputs, with ECS/event shells applying packets, commands, or inventory mutations.

#### Scenario: Stale window click is rejected

r[valence_hyperion_integration.gui_helper.core.stale_window]
- GIVEN a click references a stale or closed GUI window
- WHEN the GUI transition helper evaluates it
- THEN it returns a deterministic stale-window result
- AND no click action or inventory mutation is emitted.

### Requirement: GUI helper tests

r[valence_hyperion_integration.gui_helper.tests] GUI helper work MUST include positive and negative tests for open, click, readonly slots, stale window IDs, invalid slots, close events, disconnect cleanup, and plugin-disabled behavior.

#### Scenario: Disconnect cleans up viewer state

r[valence_hyperion_integration.gui_helper.tests.disconnect]
- GIVEN a client is viewing a GUI
- WHEN the client disconnects
- THEN viewer state is removed or marked closed
- AND later clicks from that window are rejected.

### Requirement: GUI helper docs

r[valence_hyperion_integration.gui_helper.docs] GUI helper documentation SHOULD show common menu examples and avoid claiming full vanilla container parity.

#### Scenario: Docs preserve inventory non-claims

r[valence_hyperion_integration.gui_helper.docs.non_claims]
- GIVEN GUI helper docs are published
- WHEN reviewers inspect them
- THEN they state that the helper builds on existing inventory semantics
- AND they do not claim untested vanilla container parity.

### Requirement: GUI helper validation

r[valence_hyperion_integration.gui_helper.validation] GUI helper work MUST record GUI tests, inventory integration tests, example smoke tests, selected inventory mc-compat dry runs, and Cairn gates before archive.

#### Scenario: GUI helper closeout is reviewable

r[valence_hyperion_integration.gui_helper.validation.log]
- GIVEN GUI helper work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show GUI model tests, negative stale/invalid-click tests, inventory integration tests, example smoke output, selected inventory mc-compat dry runs, and Cairn validation.

### Requirement: Observability scope

r[valence_hyperion_integration.observability.scope] The integration MUST review Hyperion observability/profiling patterns and Valence tracing/logging surfaces before adding instrumentation hooks.

#### Scenario: Hook scope is bounded

r[valence_hyperion_integration.observability.scope.bounded]
- GIVEN observability work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify selected subsystems, disabled-mode behavior, optional adapters, and non-goals such as mandatory profiler dependency or production-capacity proof.

### Requirement: Observability contract

r[valence_hyperion_integration.observability.contract] Observability hooks MUST define span and metric names, bounded labels, redaction policy, overhead expectations, adapter boundaries, and disabled-mode behavior.

#### Scenario: High-cardinality label is rejected

r[valence_hyperion_integration.observability.contract.cardinality]
- GIVEN a proposed metric label contains unbounded player text, addresses, packet payloads, or arbitrary identifiers
- WHEN the observability contract validator or review checklist evaluates it
- THEN the label is rejected or transformed according to redaction policy.

### Requirement: Pure observability classification

r[valence_hyperion_integration.observability.core] Mapping subsystem events to observability records SHOULD be pure deterministic classification over explicit inputs, with clocks, exporters, and profilers in shells.

#### Scenario: Redaction is deterministic

r[valence_hyperion_integration.observability.core.redaction]
- GIVEN an event includes sensitive fields and public fields
- WHEN the classification core creates an observability record
- THEN sensitive fields are omitted, hashed, or redacted according to policy
- AND public labels remain bounded and deterministic.

### Requirement: Optional observability wiring

r[valence_hyperion_integration.observability.wiring] Valence MAY wire optional observability hooks into selected subsystems, but disabled hooks MUST preserve existing behavior and avoid mandatory exporter dependencies.

#### Scenario: Disabled hooks are no-op

r[valence_hyperion_integration.observability.wiring.disabled]
- GIVEN observability hooks are disabled
- WHEN existing Valence tests run
- THEN no profiler/exporter dependency is required
- AND subsystem behavior and public packet output remain unchanged.

### Requirement: Observability tests

r[valence_hyperion_integration.observability.tests] Observability work MUST include positive and negative tests for disabled hooks, enabled labels, redaction, unknown metrics, exporter failure, and overhead checks when overhead is claimed.

#### Scenario: Exporter failure does not stop server logic

r[valence_hyperion_integration.observability.tests.exporter_failure]
- GIVEN an optional exporter returns an error
- WHEN a hook emits an observability record
- THEN the error is reported according to policy
- AND core server behavior continues unless policy explicitly requires fail-closed behavior.

### Requirement: Observability validation

r[valence_hyperion_integration.observability.validation] Observability work MUST record tests, plugin-disabled regressions, smoke trace/export checks, overhead checks if claimed, and Cairn gates before archive.

#### Scenario: Observability closeout is reviewable

r[valence_hyperion_integration.observability.validation.log]
- GIVEN observability work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show classification tests, redaction tests, disabled-mode regressions, exporter failure fixtures, smoke trace output, overhead evidence if claimed, and Cairn validation.

### Requirement: World snapshot loader scope

r[valence_hyperion_integration.world_snapshot_loader.scope] The integration MUST review Hyperion block/region loading and Valence Anvil/layer loading before adding a static world snapshot loader.

#### Scenario: Static-world non-goals are recorded

r[valence_hyperion_integration.world_snapshot_loader.scope.non_goals]
- GIVEN snapshot loader work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes distinguish static snapshots, controlled reloads, terrain generation, save editing, and Hyperion loader parity claims.

### Requirement: Loader contract

r[valence_hyperion_integration.world_snapshot_loader.contract] The loader MUST define typed plan inputs, region selection, resource limits, dimension and biome validation, async boundaries, and partial-load policy.

#### Scenario: Dimension mismatch rejects snapshot

r[valence_hyperion_integration.world_snapshot_loader.contract.dimension_mismatch]
- GIVEN a snapshot declares or implies dimension bounds incompatible with the target Valence layer
- WHEN the loader validates the plan or chunk data
- THEN it rejects the snapshot with a deterministic diagnostic
- AND no client-visible layer mutation is applied.

### Requirement: Pure loader core

r[valence_hyperion_integration.world_snapshot_loader.core] Loader plan validation and chunk snapshot normalization SHOULD be pure deterministic logic over explicit in-memory inputs.

#### Scenario: Corrupt chunk input reports error

r[valence_hyperion_integration.world_snapshot_loader.core.corrupt_chunk]
- GIVEN a chunk input has malformed or missing required NBT fields
- WHEN the normalization core evaluates it
- THEN it returns the documented parse diagnostic
- AND it does not read files, spawn tasks, or mutate layers.

### Requirement: Loader adapters

r[valence_hyperion_integration.world_snapshot_loader.adapters] Filesystem discovery, memory mapping, async reads, decompression, and layer mutation MUST remain shell adapters around the loader core.

#### Scenario: Missing region file fails by policy

r[valence_hyperion_integration.world_snapshot_loader.adapters.missing_region]
- GIVEN the plan requires a region file that is absent
- WHEN the filesystem adapter runs
- THEN it follows the documented missing-file policy
- AND records whether the load failed, skipped, or produced a partial result.

### Requirement: Loader tests

r[valence_hyperion_integration.world_snapshot_loader.tests] Snapshot loader work MUST include positive and negative tests for valid regions, missing files, corrupt NBT, out-of-range sections, dimension mismatch, biome mismatch, partial loads, and cancellation.

#### Scenario: Partial load policy is deterministic

r[valence_hyperion_integration.world_snapshot_loader.tests.partial]
- GIVEN one selected region loads and another selected region fails
- WHEN partial loads are configured
- THEN the loader produces the documented partial result or rollback result deterministically.

### Requirement: Loader validation

r[valence_hyperion_integration.world_snapshot_loader.validation] Snapshot loader work MUST record loader tests, corrupt-region fixtures, smoke tests, selected chunk/dimension compatibility dry runs, and Cairn gates before archive.

#### Scenario: Loader closeout is reviewable

r[valence_hyperion_integration.world_snapshot_loader.validation.log]
- GIVEN snapshot loader work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show plan validation tests, corrupt-input fixtures, adapter failure fixtures, loader smoke output, selected chunk/dimension dry runs if behavior changes, and Cairn validation.

### Requirement: Packet compose contract

r[valence_hyperion_integration.packet_compose.contract] Valence SHOULD define a packet compose API contract for bundle construction, route intents, ordering guarantees, errors, and direct-write migration guidance.

#### Scenario: Compose scope is documented

r[valence_hyperion_integration.packet_compose.contract.documented]
- GIVEN the compose API is introduced
- WHEN reviewers inspect the API docs
- THEN docs distinguish packet bundling, route planning, direct flush, future proxy routing, and cases where direct client writes remain appropriate.

### Requirement: Pure packet delivery planner

r[valence_hyperion_integration.packet_compose.planner] Packet delivery planning MUST be a pure deterministic operation over route intents, client visibility inputs, channel/group inputs, exclusions, and bundle metadata.

#### Scenario: Exclusion applies to global route

r[valence_hyperion_integration.packet_compose.planner.global_exclusion]
- GIVEN a global route intent with an excluded client
- WHEN the planner evaluates active clients
- THEN the delivery plan contains every active non-excluded client
- AND the excluded client receives no planned packet.

### Requirement: Direct-mode flush wiring

r[valence_hyperion_integration.packet_compose.direct_flush] The compose API MAY add direct-mode flush wiring, but it MUST NOT change existing direct packet-write behavior for code that does not opt into compose.

#### Scenario: Non-compose packet writes remain stable

r[valence_hyperion_integration.packet_compose.direct_flush.stable]
- GIVEN an existing Valence system writes packets directly to clients
- WHEN compose support is enabled in the workspace
- THEN direct packet writes preserve their previous ordering, encoding, and flush behavior.

### Requirement: Packet compose tests

r[valence_hyperion_integration.packet_compose.tests] Packet compose work MUST include positive and negative tests for ordering, route resolution, exclusions, closed clients, encode failures, invalid route targets, and partial flush errors.

#### Scenario: Closed client reports partial failure

r[valence_hyperion_integration.packet_compose.tests.closed_client]
- GIVEN a delivery plan includes a client that closes before flush
- WHEN the direct flush shell processes the plan
- THEN it reports a structured partial failure for that client
- AND it does not reorder packets for remaining clients.

### Requirement: Packet compose documentation

r[valence_hyperion_integration.packet_compose.docs] Compose API documentation SHOULD include examples for unicast, broadcast, local visibility, grouped delivery, exclusions, and direct-write alternatives.

#### Scenario: Examples avoid proxy overclaiming

r[valence_hyperion_integration.packet_compose.docs.non_overclaiming]
- GIVEN compose examples are published
- WHEN reviewers inspect them
- THEN they do not claim proxy mode or large-scale performance unless those backends have separate evidence.

### Requirement: Packet compose validation

r[valence_hyperion_integration.packet_compose.validation] Packet compose work MUST record planner tests, direct flush regressions, examples or playground smoke tests, selected mc-compat dry runs, and Cairn gates before archive.

#### Scenario: Compose closeout is reviewable

r[valence_hyperion_integration.packet_compose.validation.log]
- GIVEN compose API work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive planner tests, negative route/flush tests, direct flush regressions, example smoke output, selected mc-compat dry runs, and Cairn validation.

### Requirement: Tick scheduler scope

r[valence_hyperion_integration.tick_scheduler.scope] The integration MUST review Hyperion scheduler behavior and Valence tick/timer patterns before adding a scheduler utility.

#### Scenario: Scheduler scope is bounded

r[valence_hyperion_integration.tick_scheduler.scope.bounded]
- GIVEN tick scheduler work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify adopted concepts, affected Valence surfaces, gameplay examples, and non-goals such as async task scheduling or wall-clock timers.

### Requirement: Tick scheduler contract

r[valence_hyperion_integration.tick_scheduler.contract] The scheduler MUST define scheduling, peeking, draining, equal-key ordering, clearing, optional cancellation, and error behavior.

#### Scenario: Not-due work remains queued

r[valence_hyperion_integration.tick_scheduler.contract.not_due]
- GIVEN scheduled work has a key greater than the drain limit
- WHEN due work is drained
- THEN the not-due work remains queued
- AND subsequent peeking reports the earliest remaining work.

### Requirement: Pure scheduler core

r[valence_hyperion_integration.tick_scheduler.core] Scheduler queue operations MUST be pure deterministic operations over explicit keys and values, with ECS systems and tick resources kept in thin shells.

#### Scenario: Core has no implicit time

r[valence_hyperion_integration.tick_scheduler.core.no_time]
- GIVEN the scheduler core drains work
- WHEN it decides whether work is due
- THEN it uses only the explicit drain limit supplied by the caller
- AND it does not read wall-clock time, runtime state, or global tick resources.

### Requirement: Scheduler fixture coverage

r[valence_hyperion_integration.tick_scheduler.fixtures] Scheduler work MUST include positive and negative fixtures for queue boundaries and ordering behavior.

#### Scenario: Empty queue drains cleanly

r[valence_hyperion_integration.tick_scheduler.fixtures.empty]
- GIVEN the scheduler queue is empty
- WHEN due work is drained
- THEN the result is empty
- AND no panic, underflow, or stale item is reported.

### Requirement: Optional scheduler wiring

r[valence_hyperion_integration.tick_scheduler.wiring] Valence MAY expose the scheduler through an optional plugin or utility shell, but gameplay policy MUST remain outside the core scheduler.

#### Scenario: Plugin disabled has no timer effect

r[valence_hyperion_integration.tick_scheduler.wiring.disabled]
- GIVEN the scheduler plugin is not enabled
- WHEN existing Valence gameplay tests run
- THEN no scheduled gameplay behavior is inserted or drained by default.

### Requirement: Scheduler validation

r[valence_hyperion_integration.tick_scheduler.validation] Scheduler work MUST record pure scheduler tests, plugin smoke tests, example checks, and Cairn gates before archive.

#### Scenario: Scheduler closeout is reviewable

r[valence_hyperion_integration.tick_scheduler.validation.log]
- GIVEN scheduler work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive scheduler tests, negative boundary tests, plugin-disabled checks, example timer output if examples changed, and Cairn validation.

### Requirement: Metadata diff audit

r[valence_hyperion_integration.metadata_diff.audit] The integration MUST audit Hyperion metadata diff tracking and Valence entity metadata update flow before changing metadata update behavior.

#### Scenario: Metadata gaps are recorded

r[valence_hyperion_integration.metadata_diff.audit.gaps]
- GIVEN metadata diff work is selected
- WHEN reviewers inspect the audit
- THEN the audit identifies Valence behavior that is already sufficient, behavior that differs intentionally, and any gaps selected for implementation.

### Requirement: Metadata diff invariants

r[valence_hyperion_integration.metadata_diff.invariants] Metadata diff work MUST define invariants for default metadata, changed metadata, same-tick changes, spawn/update ordering, despawn cleanup, and invalid metadata indices.

#### Scenario: Unchanged metadata is not resent

r[valence_hyperion_integration.metadata_diff.invariants.unchanged]
- GIVEN an entity metadata value is unchanged between snapshots
- WHEN the diff helper computes updates
- THEN no incremental metadata update is produced for that value
- AND required spawn metadata remains governed by the documented spawn path.

### Requirement: Pure metadata diff core

r[valence_hyperion_integration.metadata_diff.core] Metadata diffing SHOULD be implemented as pure deterministic helpers over previous and current metadata snapshots, with ECS queries and packet flushing in shells.

#### Scenario: Same-tick final-state policy is deterministic

r[valence_hyperion_integration.metadata_diff.core.same_tick]
- GIVEN metadata changes multiple times within one tick
- WHEN the diff core evaluates the previous and final snapshots
- THEN it emits the documented final-state update or documented transition sequence deterministically.

### Requirement: Metadata diff tests

r[valence_hyperion_integration.metadata_diff.tests] Metadata diff work MUST include positive and negative tests for unchanged values, changed values, same-tick changes, default suppression, invalid metadata indices, despawn cleanup, and packet ordering.

#### Scenario: Invalid metadata index fails closed

r[valence_hyperion_integration.metadata_diff.tests.invalid_index]
- GIVEN metadata data references an invalid index for the entity metadata contract
- WHEN validation or encoding evaluates it
- THEN it reports a deterministic diagnostic
- AND no malformed metadata packet is emitted.

### Requirement: Metadata update wiring

r[valence_hyperion_integration.metadata_diff.wiring] Valence MAY wire metadata diff improvements into entity update systems only when audit evidence shows the change preserves client-visible semantics.

#### Scenario: Spawn metadata ordering is preserved

r[valence_hyperion_integration.metadata_diff.wiring.spawn_order]
- GIVEN an entity spawns with non-default metadata
- WHEN metadata diff improvements are enabled
- THEN spawn and metadata packets preserve the documented ordering expected by clients.

### Requirement: Metadata diff validation

r[valence_hyperion_integration.metadata_diff.validation] Metadata diff work MUST record metadata tests, packet fixtures, selected entity compatibility scenarios, and Cairn gates before archive.

#### Scenario: Metadata closeout is reviewable

r[valence_hyperion_integration.metadata_diff.validation.log]
- GIVEN metadata diff work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show audit notes, positive metadata fixtures, negative invalid-index fixtures, packet ordering checks, selected entity dry runs if behavior changes, and Cairn validation.

### Requirement: Paletted container inventory

r[valence_hyperion_integration.palette_optimization.inventory] The integration MUST compare Hyperion and Valence paletted container behavior before changing Valence chunk storage internals.

#### Scenario: Representation differences are recorded

r[valence_hyperion_integration.palette_optimization.inventory.recorded]
- GIVEN paletted container optimization is selected
- WHEN reviewers inspect the inventory
- THEN the inventory records representation states, encode paths, query helpers, mutation behavior, and unsafe or nightly dependencies.

### Requirement: Paletted container invariants

r[valence_hyperion_integration.palette_optimization.invariants] Paletted container work MUST define correctness invariants for indexing, representation transitions, unique queries, iteration, encoding, invalid indices, and mutation behavior.

#### Scenario: Representation transition preserves values

r[valence_hyperion_integration.palette_optimization.invariants.transition]
- GIVEN a section transitions from compact palette storage to direct storage
- WHEN all indices are read after the transition
- THEN every block state matches the pre-transition logical section state.

### Requirement: Benchmark and fixture baseline

r[valence_hyperion_integration.palette_optimization.baseline] Paletted container work MUST capture baseline correctness fixtures and benchmark results before modifying Valence internals.

#### Scenario: Baseline evidence names workloads

r[valence_hyperion_integration.palette_optimization.baseline.workloads]
- GIVEN baseline benchmarks are recorded
- WHEN reviewers inspect the evidence
- THEN each benchmark names the section distributions, mutation pattern, encode path, and command used to run it.

### Requirement: Stable-safe optimization port

r[valence_hyperion_integration.palette_optimization.port] Valence SHOULD port only measured stable-safe optimization concepts unless separate audit evidence approves unsafe or nightly-specific code.

#### Scenario: Unaudited unsafe implementation is rejected

r[valence_hyperion_integration.palette_optimization.port.reject_unsafe]
- GIVEN a proposed optimization depends on unaudited unsafe or nightly-only behavior
- WHEN the implementation plan is reviewed
- THEN the code is rejected, rewritten in stable safe Rust, or moved to a separate audit Cairn.

### Requirement: Paletted container tests

r[valence_hyperion_integration.palette_optimization.tests] Paletted container work MUST include positive and negative tests for storage states, transitions, invalid inputs, and encode parity.

#### Scenario: Invalid index fails correctly

r[valence_hyperion_integration.palette_optimization.tests.invalid_index]
- GIVEN a read or write uses an out-of-range section index
- WHEN the container API handles the request
- THEN it returns the documented error or panic boundary
- AND it does not corrupt any in-range block state.

### Requirement: Paletted container validation

r[valence_hyperion_integration.palette_optimization.validation] Paletted container work MUST record correctness tests, benchmark evidence, selected chunk compatibility checks when behavior changes, and Cairn gates before archive.

#### Scenario: Optimization closeout is reviewable

r[valence_hyperion_integration.palette_optimization.validation.log]
- GIVEN paletted container work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show baseline results, final benchmarks, positive correctness tests, negative invalid-input tests, encode parity checks, selected chunk dry runs if needed, and Cairn validation.

### Requirement: Chunk cache scope

r[valence_hyperion_integration.chunk_cache.scope] The integration MUST review Hyperion chunk egress/cache behavior and Valence layer/chunk serialization before adding cached chunk egress.

#### Scenario: Cache scope records non-goals

r[valence_hyperion_integration.chunk_cache.scope.non_goals]
- GIVEN cached chunk egress is selected
- WHEN reviewers inspect the scope notes
- THEN they identify cache-eligible chunk states, affected Valence APIs, and non-goals such as world-generation parity or Hyperion map-loader parity.

### Requirement: Chunk cache key contract

r[valence_hyperion_integration.chunk_cache.key] Cached chunk bytes MUST be keyed by every setting and input that can affect client-visible chunk packets, including chunk position, dimension/registry inputs, block/biome/light data, protocol version, and compression behavior.

#### Scenario: Compression change invalidates cache

r[valence_hyperion_integration.chunk_cache.key.compression]
- GIVEN a cached chunk entry was created with one compression setting
- WHEN the server sends the same chunk with a different compression setting
- THEN the cache key does not match the stale entry
- AND bytes are regenerated or a matching entry is selected.

### Requirement: Pure chunk cache core

r[valence_hyperion_integration.chunk_cache.core] Chunk packet rendering for cacheable inputs SHOULD be a deterministic core over chunk snapshots and render settings, with storage, eviction, metrics, and network writes in shells.

#### Scenario: Same snapshot renders same bytes

r[valence_hyperion_integration.chunk_cache.core.deterministic]
- GIVEN identical chunk snapshots and render settings
- WHEN the renderer runs multiple times
- THEN it returns byte-identical packet payloads and identical cache metadata.

### Requirement: Chunk cache fixture coverage

r[valence_hyperion_integration.chunk_cache.fixtures] Cached chunk egress MUST include positive and negative fixtures for cache hits, invalidation, missing inputs, and stale cached bytes.

#### Scenario: Block mutation invalidates entry

r[valence_hyperion_integration.chunk_cache.fixtures.block_mutation]
- GIVEN a chunk cache entry exists for a snapshot
- WHEN a block mutation changes client-visible chunk data
- THEN the old entry is not reused for the mutated snapshot
- AND the fixture fails if stale bytes are emitted.

### Requirement: Optional cached egress wiring

r[valence_hyperion_integration.chunk_cache.wiring] Valence MAY expose cached chunk egress as an optional path, but default uncached semantics MUST remain available and compatible.

#### Scenario: Uncached send remains valid

r[valence_hyperion_integration.chunk_cache.wiring.uncached]
- GIVEN cached egress is disabled
- WHEN existing chunk-send tests or selected mc-compat chunk scenarios run
- THEN Valence sends chunks through the existing uncached path with unchanged semantics.

### Requirement: Chunk cache validation

r[valence_hyperion_integration.chunk_cache.validation] Cached chunk egress work MUST record chunk renderer tests, stale-cache rejection, direct chunk-send regressions, selected mc-compat chunk scenarios, and Cairn gates before archive.

#### Scenario: Chunk cache closeout is reviewable

r[valence_hyperion_integration.chunk_cache.validation.log]
- GIVEN cached chunk egress is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show deterministic render fixtures, invalidation fixtures, stale-cache rejection, direct chunk-send regressions, selected mc-compat chunk scenarios, optional benchmark output if performance is claimed, and Cairn validation.

### Requirement: Byte-backed protocol audit

r[valence_hyperion_integration.byte_protocol.audit] The integration MUST audit Hyperion's byte-backed protocol usage and Valence's current protocol/event surfaces before adding public byte-backed APIs.

#### Scenario: Required byte behavior is identified

r[valence_hyperion_integration.byte_protocol.audit.recorded]
- GIVEN byte-backed protocol work is selected
- WHEN reviewers inspect the audit notes
- THEN the notes identify required Hyperion fork behavior, affected Valence packet/event APIs, migration risks, and out-of-scope packet-channel/runtime behavior.

### Requirement: Stable byte-backed API

r[valence_hyperion_integration.byte_protocol.api] Valence MUST define stable byte-backed raw-payload and validated text/byte field APIs with explicit ownership, bounds, and validation invariants.

#### Scenario: Invalid text is rejected

r[valence_hyperion_integration.byte_protocol.api.invalid_text]
- GIVEN client packet bytes contain invalid text for a validated string field
- WHEN the byte-backed constructor validates the field
- THEN it returns a deterministic error
- AND no public packet event exposes the invalid field.

### Requirement: Pure packet framing core

r[valence_hyperion_integration.byte_protocol.core] Packet framing, compression decisions, and body validation SHOULD be implemented as pure deterministic cores over in-memory byte buffers, with socket I/O and channel orchestration kept in thin shells.

#### Scenario: Split frame is decoded deterministically

r[valence_hyperion_integration.byte_protocol.core.split_frame]
- GIVEN a valid packet frame split across multiple input buffers
- WHEN the framing core receives the buffers in order
- THEN it returns the same completed packet body as a single-buffer decode
- AND it preserves incomplete-frame state without reading from sockets or global state.

### Requirement: Byte protocol fixture coverage

r[valence_hyperion_integration.byte_protocol.fixtures] Byte-backed protocol work MUST include positive and negative fixtures for valid frames and malformed input boundaries.

#### Scenario: Oversized packet fails closed

r[valence_hyperion_integration.byte_protocol.fixtures.oversized]
- GIVEN packet bytes declare a length beyond the configured packet limit
- WHEN the framing fixture runs
- THEN decoding fails with a deterministic oversized-packet diagnostic
- AND no partial public event is emitted.

### Requirement: Incremental byte protocol migration

r[valence_hyperion_integration.byte_protocol.migration] Selected Valence event-loop packet paths SHOULD migrate behind compatibility shims or feature flags before existing owned packet paths are removed.

#### Scenario: Existing direct mode still decodes packets

r[valence_hyperion_integration.byte_protocol.migration.direct_stable]
- GIVEN byte-backed protocol support is present but default direct networking remains configured
- WHEN existing Valence packet/event tests execute
- THEN packet decoding and event delivery remain compatible with the previous owned path.

### Requirement: Byte protocol validation

r[valence_hyperion_integration.byte_protocol.validation] Byte-backed protocol work MUST record protocol tests, negative malformed-input tests, event-loop regressions, selected mc-compat dry runs, and Cairn gates before archive.

#### Scenario: Byte protocol closeout is reviewable

r[valence_hyperion_integration.byte_protocol.validation.log]
- GIVEN byte-backed protocol work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive protocol fixtures, negative malformed-input fixtures, event-loop regressions, selected mc-compat dry runs, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Profile cache scope

r[valence_hyperion_integration.profile_cache.scope] The integration MUST review Hyperion profile/cache code and Valence login/profile surfaces before adding profile cache behavior.

#### Scenario: Authentication boundary is recorded

r[valence_hyperion_integration.profile_cache.scope.auth_boundary]
- GIVEN profile cache work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify which data is cached, which login/authentication behavior remains unchanged, and which provider/cache policies are out of scope.

### Requirement: Typed profile cache configuration

r[valence_hyperion_integration.profile_cache.config] Profile caching MUST define typed configuration for providers, request budgets, cache backends, TTLs, offline fallback, and privacy retention.

#### Scenario: Missing provider config fails before requests

r[valence_hyperion_integration.profile_cache.config.missing_provider]
- GIVEN profile lookup is enabled without a valid provider configuration
- WHEN the configuration validator runs
- THEN it returns a deterministic diagnostic
- AND no HTTP request is attempted.

### Requirement: Pure profile cache core

r[valence_hyperion_integration.profile_cache.core] Profile response parsing, cache decisions, staleness decisions, and rate-limit admission MUST be pure deterministic logic over explicit inputs.

#### Scenario: Missing profile id is rejected

r[valence_hyperion_integration.profile_cache.core.missing_id]
- GIVEN a provider response omits the profile identifier field
- WHEN the parser evaluates the response
- THEN it returns the documented parse error
- AND no cache entry is created.

### Requirement: Optional HTTP and storage adapters

r[valence_hyperion_integration.profile_cache.adapters] HTTP clients and cache storage MUST be optional adapters with explicit configuration and no hard-coded provider or storage path assumptions.

#### Scenario: Corrupted cache entry fails safely

r[valence_hyperion_integration.profile_cache.adapters.corrupt]
- GIVEN the configured cache backend returns a corrupted profile entry
- WHEN the adapter decodes it
- THEN it reports a deterministic corruption diagnostic
- AND the configured fallback policy determines whether lookup continues or fails.

### Requirement: Profile cache tests

r[valence_hyperion_integration.profile_cache.tests] Profile cache work MUST include positive and negative tests for provider parsing, rate limiting, cache state, storage errors, and disabled behavior.

#### Scenario: Rate limit exhaustion blocks request

r[valence_hyperion_integration.profile_cache.tests.rate_limit]
- GIVEN the configured request budget is exhausted
- WHEN a new profile lookup is requested
- THEN the rate-limit core rejects or delays the request according to policy
- AND the HTTP shell does not issue an immediate request.

### Requirement: Profile cache validation

r[valence_hyperion_integration.profile_cache.validation] Profile cache work MUST record parser/cache tests, fake-provider tests, storage corruption tests, login/profile smoke tests, and Cairn gates before archive.

#### Scenario: Profile cache closeout is reviewable

r[valence_hyperion_integration.profile_cache.validation.log]
- GIVEN profile cache work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive parser tests, negative malformed-provider tests, rate-limit tests, cache corruption tests, plugin-disabled checks, login/profile smoke output, and Cairn validation.

### Requirement: Proxy broadcast scope

r[valence_hyperion_integration.proxy_broadcast.scope] The integration MUST record the Hyperion proxy surfaces, Valence direct-networking surfaces, and proxy-mode non-goals before implementation.

#### Scenario: Proxy scope is reviewable

r[valence_hyperion_integration.proxy_broadcast.scope.reviewed]
- GIVEN proxy backend work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify the Hyperion source files/docs inspected, the Valence crates affected, the preserved direct-mode behavior, and the out-of-scope Hyperion runtime pieces.

### Requirement: Proxy message contract

r[valence_hyperion_integration.proxy_broadcast.contract] Proxy mode MUST define a stable server-to-proxy and proxy-to-server message contract for unicast, global broadcast, local broadcast, channel broadcast, player position updates, subscriptions, stream lifecycle, backpressure, and shutdown.

#### Scenario: Contract rejects invalid visibility state

r[valence_hyperion_integration.proxy_broadcast.contract.invalid_state]
- GIVEN a proxy message references an unknown stream, unknown channel, stale subscription, malformed payload, or invalid player position
- WHEN the proxy contract validator evaluates it
- THEN the message is rejected with a deterministic diagnostic
- AND no delivery plan is produced for unintended clients.

### Requirement: Pure proxy routing core

r[valence_hyperion_integration.proxy_broadcast.routing_core] Proxy route selection MUST be implemented as a pure deterministic core over player positions, subscriptions, exclusions, and broadcast intents.

#### Scenario: Local broadcast excludes sender

r[valence_hyperion_integration.proxy_broadcast.routing_core.local_exclude]
- GIVEN a local broadcast intent with a center chunk, visibility radius, and excluded stream
- WHEN the routing core evaluates active player positions
- THEN only matching in-range streams are returned
- AND the excluded stream is absent from the delivery plan.

### Requirement: Optional Valence backend

r[valence_hyperion_integration.proxy_broadcast.valence_backend] Valence SHOULD expose proxy mode as an optional backend or plugin without changing the default direct-networking mode.

#### Scenario: Direct mode remains stable

r[valence_hyperion_integration.proxy_broadcast.valence_backend.direct_stable]
- GIVEN proxy mode is not enabled
- WHEN existing Valence networking tests and selected mc-compat dry runs execute
- THEN direct client connection, login, packet flush, and disconnect behavior remain unchanged.

### Requirement: Proxy backend evidence

r[valence_hyperion_integration.proxy_broadcast.evidence] Proxy backend work MUST record direct-mode regression evidence, proxy-mode smoke evidence, malformed-message rejection, and non-overclaiming compatibility notes before archive.

#### Scenario: Proxy evidence is non-overclaiming

r[valence_hyperion_integration.proxy_broadcast.evidence.non_overclaiming]
- GIVEN proxy mode has smoke evidence
- WHEN the evidence is promoted
- THEN it claims only the exercised proxy routing/backend behavior
- AND it does not claim full Hyperion compatibility, full production-scale readiness, or default Valence behavior changes.

### Requirement: Proxy backend validation

r[valence_hyperion_integration.proxy_broadcast.validation] The change MUST pass Cairn proposal, design, tasks, and repository validation gates before archive.

#### Scenario: Proxy Cairn closeout is reviewable

r[valence_hyperion_integration.proxy_broadcast.validation.log]
- GIVEN the proxy backend change is ready to archive
- WHEN reviewers inspect the evidence logs
- THEN logs show routing fixtures, negative proxy-state fixtures, direct-mode regressions, proxy-mode smoke tests, selected mc-compat dry runs, and Cairn validation.

### Requirement: Packet buffer reuse audit

r[valence_hyperion_integration.packet_buffer_reuse.audit] The integration MUST audit Hyperion encoder/buffer reuse patterns and Valence packet encode/flush paths before adding reusable buffers.

#### Scenario: Optimization scope is recorded

r[valence_hyperion_integration.packet_buffer_reuse.audit.scope]
- GIVEN packet buffer reuse work is selected
- WHEN reviewers inspect the audit
- THEN the audit identifies target workloads, affected encode/flush paths, compression boundaries, and protocol semantics that must remain unchanged.

### Requirement: Packet buffer lifecycle contract

r[valence_hyperion_integration.packet_buffer_reuse.contract] Buffer reuse work MUST define buffer lifecycle, compression settings, capacity policy, reset/discard behavior, packet limit behavior, and safety invariants.

#### Scenario: Error resets reusable buffer

r[valence_hyperion_integration.packet_buffer_reuse.contract.error_reset]
- GIVEN packet encoding or compression fails while using a reusable buffer
- WHEN the encoder returns the error
- THEN the buffer is reset or discarded according to the documented policy
- AND stale partial bytes are not reused for later packets.

### Requirement: Packet buffer baseline

r[valence_hyperion_integration.packet_buffer_reuse.baseline] Packet buffer reuse work MUST record baseline allocation or benchmark evidence for selected workloads before implementation.

#### Scenario: Baseline names encode workload

r[valence_hyperion_integration.packet_buffer_reuse.baseline.named]
- GIVEN baseline evidence is recorded
- WHEN reviewers inspect it
- THEN the evidence names packet mix, compression settings, client count or batch size, command, and environment.

### Requirement: Packet buffer reuse implementation

r[valence_hyperion_integration.packet_buffer_reuse.implementation] Valence MAY implement reusable encoder buffers or pools only when invariants and baseline evidence justify the change.

#### Scenario: Default semantics are preserved

r[valence_hyperion_integration.packet_buffer_reuse.implementation.default_semantics]
- GIVEN reusable buffers are enabled internally
- WHEN existing direct-mode packet tests run
- THEN packet bytes, ordering, compression behavior, and error behavior match the previous public semantics.

### Requirement: Packet buffer reuse tests

r[valence_hyperion_integration.packet_buffer_reuse.tests] Buffer reuse work MUST include positive and negative tests for compression, packet limits, error resets, stale bytes, closed clients, and default behavior.

#### Scenario: Oversized packet does not poison pool

r[valence_hyperion_integration.packet_buffer_reuse.tests.oversized]
- GIVEN an oversized packet fails to encode
- WHEN a subsequent valid packet is encoded using the same pool or encoder path
- THEN the valid packet bytes contain no data from the oversized failure
- AND the oversized diagnostic remains deterministic.

### Requirement: Packet buffer reuse validation

r[valence_hyperion_integration.packet_buffer_reuse.validation] Buffer reuse work MUST record encode tests, compression edge tests, direct-mode regressions, benchmark evidence, selected compatibility dry runs, and Cairn gates before archive.

#### Scenario: Buffer reuse closeout is reviewable

r[valence_hyperion_integration.packet_buffer_reuse.validation.log]
- GIVEN packet buffer reuse work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show baseline and final benchmarks, positive encode fixtures, negative error-reset fixtures, compression edge tests, direct-mode regressions, selected dry runs if behavior changed, and Cairn validation.
