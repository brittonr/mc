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
