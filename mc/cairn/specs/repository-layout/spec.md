# Repository Layout Specification

## Purpose

Defines the `repository-layout` capability.

## Requirements

### Requirement: Cairn policy ownership contract

r[repository_layout.cairn_policy_ownership.contract] The repository MUST document ownership, generation command, schema constraints, and path rationale for generated Cairn policy artifacts.

#### Scenario: Policy owner is clear

r[repository_layout.cairn_policy_ownership.contract.owner]
- GIVEN a generated Cairn policy file exists in the repository
- WHEN a developer reads layout or lifecycle docs
- THEN the docs name the owning lifecycle system, generation command, expected schema compatibility, and path rationale.

### Requirement: Cairn policy reference inventory

r[repository_layout.cairn_policy_ownership.references] Policy path changes MUST be preceded by an inventory of code, flake, docs, checks, and Cairn validation references to the current policy path.

#### Scenario: Embedded path is found

r[repository_layout.cairn_policy_ownership.references.embedded]
- GIVEN a policy path is embedded in a command, flake check, docs page, or validation script
- WHEN the reference inventory runs
- THEN the reference is recorded with owner and migration action before any path move.

### Requirement: Cairn policy path decision

r[repository_layout.cairn_policy_ownership.path_decision] The repository MAY keep `cairn-policy/` top-level or migrate it under `cairn/` only after recording compatibility with the pinned Cairn binary and validation commands.

#### Scenario: Path move is compatibility-proven

r[repository_layout.cairn_policy_ownership.path_decision.compat]
- GIVEN maintainers decide to move generated policy artifacts
- WHEN path migration is implemented
- THEN the repo-pinned Cairn validation and policy checks accept the new or configured path
- AND old path references are updated or intentionally retained with migration notes.

### Requirement: Cairn policy freshness

r[repository_layout.cairn_policy_ownership.freshness] Generated Cairn policy artifacts SHOULD have freshness or schema-compatibility checks that fail when checked-in policy output is stale or incompatible.

#### Scenario: Stale policy fails

r[repository_layout.cairn_policy_ownership.freshness.stale]
- GIVEN policy source inputs change without refreshing generated policy artifacts
- WHEN policy freshness validation runs
- THEN the stale policy artifact is reported
- AND archive is blocked until regeneration or an explicit compatibility decision occurs.

### Requirement: Cairn policy docs

r[repository_layout.cairn_policy_ownership.docs] README, architecture, or agent notes SHOULD explain how generated Cairn policy artifacts are maintained and validated.

#### Scenario: Regeneration command is discoverable

r[repository_layout.cairn_policy_ownership.docs.command]
- GIVEN a developer needs to refresh Cairn policy output
- WHEN they inspect lifecycle docs
- THEN they can find the repo-pinned command, expected output path, and validation check.

### Requirement: Cairn policy validation

r[repository_layout.cairn_policy_ownership.validation] Cairn policy ownership changes MUST record Cairn validation, policy freshness/schema checks, and Cairn gates before archive.

#### Scenario: Policy closeout is reviewable

r[repository_layout.cairn_policy_ownership.validation.log]
- GIVEN Cairn policy ownership or path docs have changed
- WHEN the change is archived
- THEN reviewable logs show Cairn validation, policy freshness or schema checks, any path compatibility checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Leafish ownership classification

r[repository_layout.leafish_classification.ownership] The repository MUST classify `Leafish/` as an owned client component, a reference/vendor input, or an external checkout before moving, deleting, or wiring it into compatibility gates.

#### Scenario: Ownership decision is recorded

r[repository_layout.leafish_classification.ownership.recorded]
- GIVEN `Leafish/` exists as a root-level nested Git checkout
- WHEN the layout decision is reviewed
- THEN the decision records whether Leafish is owned, reference-only, or external
- AND records the owner, command boundary, default gate participation, and next action.

### Requirement: Owned Leafish role path

r[repository_layout.leafish_classification.owned_role_path] If Leafish is an owned client component, it SHOULD live under a client role path and have documented local workflow boundaries.

#### Scenario: Owned client role is explicit

r[repository_layout.leafish_classification.owned_role_path.client]
- GIVEN Leafish is classified as an owned client component
- WHEN developers inspect the workspace layout
- THEN Leafish appears under an owned client role path or a documented transition path
- AND docs name its build/test commands, VCS boundary, and compatibility-gate participation.

### Requirement: Reference or external Leafish boundary

r[repository_layout.leafish_classification.reference_boundary] If Leafish is reference-only or external, it MUST be documented as non-default compatibility input and excluded from default gates unless explicitly selected.

#### Scenario: Reference input is opt-in

r[repository_layout.leafish_classification.reference_boundary.opt_in]
- GIVEN Leafish is classified as reference-only or external
- WHEN default repo checks or compatibility gates run
- THEN Leafish is not required for default success
- AND any Leafish-based comparison is invoked by an explicit opt-in command or documented external path.

### Requirement: Layout documentation update

r[repository_layout.leafish_classification.docs] Workspace layout documentation MUST name every intentional nested Git exception and describe ownership, command boundary, and parent-repo interaction.

#### Scenario: Nested Git exception is reviewable

r[repository_layout.leafish_classification.docs.nested_git]
- GIVEN a nested Git directory remains under `mc/`
- WHEN a developer reads `AGENTS.md`, README, or architecture docs
- THEN the nested repo is named with ownership, command scope, and whether parent repo status/gates include it.

### Requirement: Undocumented nested Git guard

r[repository_layout.leafish_classification.layout_guard] The repository SHOULD include a guard or review checklist that catches undocumented root-level nested Git checkouts.

#### Scenario: Surprise nested checkout is rejected

r[repository_layout.leafish_classification.layout_guard.surprise]
- GIVEN a new root-level nested Git checkout appears without documentation
- WHEN the layout guard or review checklist runs
- THEN it reports the undocumented path
- AND default validation does not treat the path as an owned component until it is classified.

### Requirement: Leafish classification validation

r[repository_layout.leafish_classification.validation] The classification change MUST record layout docs/checks, any affected component checks, and Cairn gates before archive.

#### Scenario: Classification closeout is reviewable

r[repository_layout.leafish_classification.validation.log]
- GIVEN the Leafish layout boundary has been classified
- WHEN the change is archived
- THEN reviewable logs show layout guard/check output, affected component checks when files moved, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Layout guard contract

r[repository_layout.layout_guard.contract] The repository SHOULD define a layout guard that reports undocumented root directories, surprise nested Git checkouts, root transient artifacts, missing subtree documentation, generated marker drift, and component-registry mismatches.

#### Scenario: Guard scope is explicit

r[repository_layout.layout_guard.contract.scope]
- GIVEN the layout guard is introduced
- WHEN reviewers inspect its contract
- THEN the guard lists each diagnostic class, waiver mechanism, source-of-truth input, and non-claim
- AND it does not claim live compatibility, semantic parity, or evidence correctness outside layout policy.

### Requirement: Pure layout guard core

r[repository_layout.layout_guard.core] The layout guard core MUST be a pure deterministic function over an in-memory repository tree, registry/config inputs, and rule settings.

#### Scenario: Guard core has no side effects

r[repository_layout.layout_guard.core.pure]
- GIVEN the shell passes a modeled file tree and registry to the guard core
- WHEN guard validation runs
- THEN diagnostics are returned deterministically
- AND the core does not read files, inspect environment, spawn processes, use clocks, or mutate repository state.

### Requirement: Layout guard fixtures

r[repository_layout.layout_guard.fixtures] The guard MUST include positive and negative fixtures for valid layout and each enforced diagnostic class.

#### Scenario: Surprise nested Git fixture fails

r[repository_layout.layout_guard.fixtures.nested_git]
- GIVEN a fixture contains a nested Git checkout that is absent from documented exceptions or the component registry
- WHEN the guard evaluates the fixture
- THEN it reports the path as an undocumented nested Git boundary
- AND the fixture fails until the path is classified or removed.

### Requirement: Layout guard wiring

r[repository_layout.layout_guard.wiring] The guard MAY start as a focused or advisory check, but required diagnostics MUST fail once known transition-state findings are resolved or waived.

#### Scenario: Focused guard reports actionable diagnostics

r[repository_layout.layout_guard.wiring.focused]
- GIVEN the guard runs in focused mode
- WHEN layout findings exist
- THEN diagnostics include path, rule, owner or waiver hint, and suggested next action
- AND required findings fail the check.

### Requirement: Registry and artifact-rule integration

r[repository_layout.layout_guard.registry_integration] The guard SHOULD consume component-registry and artifact-boundary rules as inputs when those sources exist, instead of maintaining independent allowlists.

#### Scenario: Registry-owned root passes

r[repository_layout.layout_guard.registry_integration.registry]
- GIVEN a component root is documented in the component registry with expected VCS and evidence policy
- WHEN the layout guard evaluates the root
- THEN the root passes component-root classification checks
- AND any mismatch between registry data and observed layout is reported.

### Requirement: Layout guard validation

r[repository_layout.layout_guard.validation] Layout guard work MUST record guard fixture tests, focused check output, and Cairn gates before archive.

#### Scenario: Guard closeout is reviewable

r[repository_layout.layout_guard.validation.log]
- GIVEN the layout guard is implemented or wired
- WHEN the change is archived
- THEN reviewable logs show positive fixtures, negative fixtures for each diagnostic class, focused flake check output, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Transition path inventory

r[repository_layout.transition_path_retirement.inventory] The repository MUST inventory legacy transition paths and active references before removing transition-path support from layout resolution.

#### Scenario: Legacy reference is classified

r[repository_layout.transition_path_retirement.inventory.classified]
- GIVEN a code, docs, flake, test, or evidence reference names a legacy transition path
- WHEN the inventory is reviewed
- THEN the reference is classified as active, historical, generated, or removable
- AND active references have a migration action.

### Requirement: Canonical role paths

r[repository_layout.transition_path_retirement.canonical_paths] Active layout docs and commands MUST use canonical role-based component paths after transition-path retirement.

#### Scenario: Active docs use canonical paths

r[repository_layout.transition_path_retirement.canonical_paths.docs]
- GIVEN a developer reads current README, architecture, or agent guidance
- WHEN component roots are named
- THEN the docs use canonical role paths for active client, server, compat, config, and fixture roots
- AND legacy paths appear only as historical or migration context.

### Requirement: Resolver transition support retirement

r[repository_layout.transition_path_retirement.resolver] The layout resolver SHOULD stop accepting legacy transition roots as active defaults once canonical role paths are established.

#### Scenario: Legacy root receives actionable diagnostic

r[repository_layout.transition_path_retirement.resolver.diagnostic]
- GIVEN only a legacy transition root exists for a component
- WHEN required layout resolution runs
- THEN the resolver reports the missing canonical role path and names the migration action
- AND it does not silently select the legacy root as the active component.

### Requirement: Layout resolver tests

r[repository_layout.transition_path_retirement.tests] Transition-path retirement MUST include positive tests for canonical roots and negative tests for ambiguous or invalid roots.

#### Scenario: Duplicate roots fail closed

r[repository_layout.transition_path_retirement.tests.duplicate]
- GIVEN both a canonical role root and legacy transition root exist for the same component
- WHEN layout resolution runs
- THEN deterministic diagnostics report ambiguity
- AND the runner does not guess which root to use.

### Requirement: Historical path documentation

r[repository_layout.transition_path_retirement.docs] Historical evidence MAY keep legacy path references only when docs make clear that they are historical and not active defaults.

#### Scenario: Historical evidence remains understandable

r[repository_layout.transition_path_retirement.docs.history]
- GIVEN archived evidence mentions a legacy transition path
- WHEN a reviewer reads active layout docs
- THEN the active docs explain the canonical path and, when necessary, the historical path context
- AND current tasks do not cite legacy paths as active roots.

### Requirement: Transition retirement validation

r[repository_layout.transition_path_retirement.validation] Transition-path retirement MUST record layout tests, missing-checkout diagnostics, runner dry-runs, and Cairn gates before archive.

#### Scenario: Retirement closeout is reviewable

r[repository_layout.transition_path_retirement.validation.log]
- GIVEN transition-path support has been retired or deprecated
- WHEN the change is archived
- THEN reviewable logs show canonical-root positive tests, invalid-root negative tests, selected runner dry-runs, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Component registry contract

r[repository_layout.component_registry.contract] The workspace SHOULD define a typed component registry that records component path, role, owner, VCS boundary, command boundary, default gate participation, and evidence policy.

#### Scenario: Registry row is complete

r[repository_layout.component_registry.contract.complete]
- GIVEN a component is represented in the registry
- WHEN registry validation runs
- THEN the row includes a repository-relative path, role, owner, VCS boundary, build/test command notes, default gate participation, and evidence policy
- AND invalid enum values or missing required fields are rejected.

### Requirement: Current component inventory

r[repository_layout.component_registry.current_inventory] The initial registry MUST describe the current workspace components and documented nested-repo exceptions before it is used to drive path moves.

#### Scenario: Current layout is captured

r[repository_layout.component_registry.current_inventory.captured]
- GIVEN Stevenarella, Valence, compat runner/config/fixtures, Cairn, docs/evidence, and any classified reference clients exist
- WHEN the registry is reviewed
- THEN each current role or exception is represented with its current path
- AND no component is silently reclassified by registry introduction alone.

### Requirement: Registry fixtures

r[repository_layout.component_registry.fixtures] Registry validation MUST include positive and negative fixtures for component rows and layout edge cases.

#### Scenario: Invalid registry fails closed

r[repository_layout.component_registry.fixtures.negative]
- GIVEN a registry fixture has a missing owner, duplicate role key, unsafe path escape, undocumented nested Git boundary, or invalid gate-participation value
- WHEN validation evaluates the fixture
- THEN deterministic diagnostics identify the invalid row
- AND no generated layout artifact is accepted.

### Requirement: Registry-derived surfaces

r[repository_layout.component_registry.generated_surfaces] Registry-derived docs or checks MAY be generated only as checked-in static artifacts or check-time outputs; runtime code MUST NOT evaluate Nickel to discover component layout.

#### Scenario: Runtime remains static

r[repository_layout.component_registry.generated_surfaces.runtime]
- GIVEN registry-derived artifacts exist
- WHEN the compatibility runner starts
- THEN it consumes checked-in Rust/static data or existing CLI arguments
- AND it does not evaluate Nickel at runtime.

### Requirement: Registry layout guard

r[repository_layout.component_registry.guard] The repository SHOULD use the registry to guard against undocumented component roots, nested Git directories, and gate participation drift.

#### Scenario: Undocumented component is reported

r[repository_layout.component_registry.guard.undocumented]
- GIVEN a new component-like directory, nested Git checkout, or gate-participating path appears outside the registry
- WHEN the registry guard runs
- THEN the path is reported with a classification diagnostic
- AND default validation does not treat it as an owned component until the registry is updated.

### Requirement: Registry validation evidence

r[repository_layout.component_registry.validation] The registry change MUST record registry validation, fixture tests, generated freshness checks if added, and Cairn gates before archive.

#### Scenario: Registry closeout is reviewable

r[repository_layout.component_registry.validation.log]
- GIVEN the component registry is introduced
- WHEN the change is archived
- THEN reviewable logs show positive fixtures, negative fixtures, registry validation, any generated-surface freshness checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Subtree agent documentation inventory

r[repository_layout.subtree_agent_docs.inventory] The workspace SHOULD inventory major owned and reference subtrees and record whether each has local agent/workflow notes or an explicit waiver.

#### Scenario: Missing local notes are visible

r[repository_layout.subtree_agent_docs.inventory.visible]
- GIVEN a major owned component root exists
- WHEN the subtree documentation inventory is reviewed
- THEN the component is marked as having local notes or an explicit waiver
- AND missing notes are not silently accepted for long-lived owned components.

### Requirement: Stevenarella local agent docs

r[repository_layout.subtree_agent_docs.stevenarella] The Stevenarella client subtree SHOULD have local agent guidance for devshell commands, protocol tests, compatibility instrumentation, VCS boundary, and evidence rules.

#### Scenario: Stevenarella edit workflow is local

r[repository_layout.subtree_agent_docs.stevenarella.workflow]
- GIVEN an agent or developer edits `clients/stevenarella/`
- WHEN they read the subtree-local guidance
- THEN they can find the expected Cargo invocation through the mc devshell, relevant protocol/client test notes, compat probe boundaries, VCS ownership, and evidence expectations.

### Requirement: Reference subtree notes

r[repository_layout.subtree_agent_docs.references] Reference or classified non-default subtrees SHOULD have local notes or waivers that explain ownership and default-gate participation.

#### Scenario: Reference subtree is not mistaken for default component

r[repository_layout.subtree_agent_docs.references.boundary]
- GIVEN a reference-only or external subtree remains under the workspace
- WHEN a developer reads its local notes or waiver
- THEN the notes state whether edits are expected, which commands apply, and whether default compatibility gates include it.

### Requirement: Root links to subtree docs

r[repository_layout.subtree_agent_docs.root_links] Root guidance SHOULD link to subtree-local notes instead of duplicating detailed commands for every major component.

#### Scenario: Root guidance routes to local docs

r[repository_layout.subtree_agent_docs.root_links.navigation]
- GIVEN a developer reads root `AGENTS.md` or architecture docs
- WHEN they need component-specific commands
- THEN the root guidance points to the relevant subtree-local notes
- AND root docs retain only workspace-wide constraints.

### Requirement: Subtree docs guard

r[repository_layout.subtree_agent_docs.guard] The repository SHOULD report major owned component roots without local agent docs or explicit waivers.

#### Scenario: New major component lacks local notes

r[repository_layout.subtree_agent_docs.guard.missing]
- GIVEN a new major owned component root is added
- WHEN the layout guard runs
- THEN the missing local agent docs or waiver is reported
- AND the component is not treated as fully documented until the gap is closed.

### Requirement: Subtree docs validation

r[repository_layout.subtree_agent_docs.validation] Subtree documentation changes MUST record docs/layout checks, relevant command dry-runs if added, and Cairn gates before archive.

#### Scenario: Subtree docs closeout is reviewable

r[repository_layout.subtree_agent_docs.validation.log]
- GIVEN subtree-local agent docs have been added or waived
- WHEN the change is archived
- THEN reviewable logs show docs/layout checks, any documented command dry-runs, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Check tier taxonomy

r[repository_layout.check_tiers.taxonomy] The repository SHOULD define named check tiers with scope, required evidence, expected runtime cost, and explicit non-claims.

#### Scenario: Tier purpose is clear

r[repository_layout.check_tiers.taxonomy.clear]
- GIVEN a developer needs to validate a change
- WHEN they inspect the check-tier taxonomy
- THEN each tier states what change scope it covers, what commands or gates belong to it, what evidence it produces, and what it does not claim.

### Requirement: Check inventory by tier

r[repository_layout.check_tiers.inventory] Existing flake checks, app dry-runs, component tests, evidence gates, manual/live rails, and Cairn gates MUST be classified into the check-tier inventory before new tier wrappers are authoritative.

#### Scenario: Existing gate is classified

r[repository_layout.check_tiers.inventory.classified]
- GIVEN an existing flake check, manual command, or Cairn gate is used for validation
- WHEN the inventory is reviewed
- THEN the check is assigned to a tier or explicitly marked legacy/manual with owner and next action.

### Requirement: Check tier documentation

r[repository_layout.check_tiers.docs] Documentation SHOULD tell developers which tier to run for docs-only, generated-surface, runner-core, component-code, evidence, live/manual, and archive-closeout changes.

#### Scenario: Developer selects smallest sufficient tier

r[repository_layout.check_tiers.docs.selection]
- GIVEN a change touches only generated docs or only runner core or only evidence manifests
- WHEN the developer reads tier docs
- THEN the docs identify the smallest relevant tier and any additional affected-component checks
- AND they do not imply that a fast tier proves live compatibility.

### Requirement: Tier entrypoints

r[repository_layout.check_tiers.entrypoints] Common tier entrypoints MAY be exposed as flake apps/checks or generated indexes only if existing public check names remain stable.

#### Scenario: Tier wrapper preserves existing checks

r[repository_layout.check_tiers.entrypoints.compat]
- GIVEN a tier wrapper is added
- WHEN existing check names are evaluated
- THEN existing checks remain available
- AND the tier wrapper delegates to documented commands without changing their evidence semantics.

### Requirement: Tier freshness

r[repository_layout.check_tiers.freshness] Tier docs and wrapper inventories SHOULD fail freshness checks when check names or tier assignments drift.

#### Scenario: Stale tier inventory fails

r[repository_layout.check_tiers.freshness.drift]
- GIVEN a flake check is added, removed, or renamed without updating the tier inventory
- WHEN tier freshness validation runs
- THEN the stale inventory is reported
- AND archive is blocked until the docs or inventory are updated.

### Requirement: Check tier validation

r[repository_layout.check_tiers.validation] The check-tier taxonomy MUST record tier inventory validation, wrapper dry-runs or evaluation, docs checks, and Cairn gates before archive.

#### Scenario: Tier closeout is reviewable

r[repository_layout.check_tiers.validation.log]
- GIVEN check tiers have been documented or wired
- WHEN the change is archived
- THEN reviewable logs show inventory validation, tier wrapper dry-runs or evaluation, docs checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Runner functional-core boundary

r[repository_layout.compat_runner_modularization.boundary] The compatibility runner MUST document and enforce a boundary between pure deterministic core logic and imperative shell orchestration.

#### Scenario: Boundary is reviewable

r[repository_layout.compat_runner_modularization.boundary.review]
- GIVEN the runner modularization begins
- WHEN reviewers inspect the design and code layout
- THEN scenario parsing, scenario metadata, receipt models, receipt validation, and config normalization are assigned to pure core modules
- AND CLI parsing, filesystem access, process execution, Docker/Paper handling, sockets, clocks, environment access, stdout/stderr, and exit-code handling are assigned to the shell.

### Requirement: Scenario core extraction

r[repository_layout.compat_runner_modularization.scenario_core] Scenario definitions, milestone specs, forbidden-pattern specs, aliases, behavior metadata, and dry-run metadata MUST live outside the runner shell while preserving existing scenario semantics.

#### Scenario: Scenario behavior remains stable

r[repository_layout.compat_runner_modularization.scenario_core.parity]
- GIVEN scenario metadata has moved out of the shell
- WHEN the runner enumerates, parses, and dry-runs every maintained scenario
- THEN scenario names, aliases, required client milestones, required server milestones, forbidden patterns, behavior kinds, and migration states match the pre-move behavior.

### Requirement: Pure validation modules

r[repository_layout.compat_runner_modularization.pure_validation] Receipt, config, and evidence validation SHOULD be expressed as pure functions over in-memory inputs before any shell writes receipts or exits.

#### Scenario: Invalid validation input fails closed

r[repository_layout.compat_runner_modularization.pure_validation.negative]
- GIVEN an in-memory receipt/config fixture is missing required fields, has malformed values, has wrong typed fields, or contains broad compatibility overclaims
- WHEN the pure validation module evaluates it
- THEN deterministic diagnostics are returned
- AND no filesystem mutation, process execution, network access, or runtime state mutation occurs.

### Requirement: Dependency direction is shell-to-core

r[repository_layout.compat_runner_modularization.dependency_direction] Core runner modules MUST NOT import constants, helpers, or side-effecting functions from `main.rs` or another shell-only module.

#### Scenario: Core dependency audit passes

r[repository_layout.compat_runner_modularization.dependency_direction.audit]
- GIVEN the runner core modules are extracted
- WHEN dependency direction is inspected by tests, static checks, or review
- THEN shell modules depend on core modules
- AND core modules do not depend on shell-owned constants, process orchestration helpers, filesystem helpers, or CLI exit behavior.

### Requirement: Modularization validation

r[repository_layout.compat_runner_modularization.validation] The modularization MUST be validated with focused positive and negative tests plus existing dry-run/evidence gates before archive.

#### Scenario: Refactor closeout is reviewable

r[repository_layout.compat_runner_modularization.validation.log]
- GIVEN the runner internals are modularized
- WHEN the change is archived
- THEN reviewable logs show focused positive tests, focused negative tests, maintained dry-run receipt checks, any touched generated-surface freshness checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Checker crate contract

r[repository_layout.checker_crate_consolidation.contract] Evidence checkers SHOULD live in a repo-owned Rust checker crate with stable binary or wrapper names for existing flake checks.

#### Scenario: Existing command surface remains available

r[repository_layout.checker_crate_consolidation.contract.compat]
- GIVEN a checker moves from a standalone `tools/*.rs` file into a checker crate
- WHEN the corresponding flake check or documented command executes
- THEN the command name and high-level usage remain available
- AND any intentional rename is tied to a separate reviewed change.

### Requirement: Shared checker core

r[repository_layout.checker_crate_consolidation.shared_core] Shared checker parsing and diagnostic helpers MUST be pure functions over in-memory evidence text and typed records.

#### Scenario: Shared parser rejects malformed evidence

r[repository_layout.checker_crate_consolidation.shared_core.negative]
- GIVEN evidence text has malformed key-value rows, duplicate keys, missing required values, or broad truthy overclaims
- WHEN the shared checker core parses and validates it
- THEN deterministic diagnostics identify the invalid condition
- AND no filesystem, process, network, clock, or environment access occurs in the pure core.

### Requirement: Rust checker migration

r[repository_layout.checker_crate_consolidation.rust_migration] Migrated Rust checkers MUST preserve their evidence contract while moving domain-specific validation into crate binaries.

#### Scenario: Migrated checker remains equivalent

r[repository_layout.checker_crate_consolidation.rust_migration.parity]
- GIVEN a Rust checker has been migrated into the checker crate
- WHEN its valid and invalid fixtures run through the new binary
- THEN valid evidence still passes
- AND invalid evidence still fails with diagnostics that cover the same claim boundary as before.

### Requirement: Python checker migration policy

r[repository_layout.checker_crate_consolidation.python_policy] Legacy Python evidence gates MAY remain only as inventoried migration debt; touched or extended gates SHOULD migrate to Rust unless an explicit waiver records owner, reason, and next action.

#### Scenario: Touched Python gate is not silently extended

r[repository_layout.checker_crate_consolidation.python_policy.touched]
- GIVEN a legacy Python checker needs new validation behavior
- WHEN implementation work starts
- THEN the gate is migrated to Rust or a waiver records why migration is blocked
- AND the waiver includes owner, reason, non-claim impact, and next action.

### Requirement: Checker fixture coverage

r[repository_layout.checker_crate_consolidation.fixtures] Every migrated checker MUST include positive and negative fixtures for evidence it accepts and evidence it rejects.

#### Scenario: Negative fixture proves fail-closed behavior

r[repository_layout.checker_crate_consolidation.fixtures.negative]
- GIVEN a migrated checker has invalid evidence with missing fields, wrong values, malformed rows, or overclaiming claim keys
- WHEN fixture tests run
- THEN the checker fails closed with a specific diagnostic
- AND no invalid evidence is reported as acceptable.

### Requirement: Checker consolidation validation

r[repository_layout.checker_crate_consolidation.validation] Checker consolidation MUST record checker tests, selected flake checks, touched evidence gates, and Cairn gates before archive.

#### Scenario: Checker closeout is reviewable

r[repository_layout.checker_crate_consolidation.validation.log]
- GIVEN checker binaries have been consolidated
- WHEN the change is archived
- THEN reviewable logs show positive fixtures, negative fixtures, selected flake checks, any touched evidence/task gates, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Evidence partition scheme

r[repository_layout.evidence_partition.scheme] Durable evidence under `docs/evidence/` SHOULD follow a documented partition scheme for receipts, run logs, manifests, generated indexes, oracle notes, and archived/historical artifacts.

#### Scenario: Evidence path class is clear

r[repository_layout.evidence_partition.scheme.clear]
- GIVEN a durable evidence artifact is created or promoted
- WHEN its path is reviewed
- THEN the path category identifies whether it is a receipt, run log, BLAKE3 manifest, generated index, oracle note, or archived artifact
- AND transient outputs are not mixed into durable evidence partitions.

### Requirement: Existing evidence inventory

r[repository_layout.evidence_partition.inventory] Existing `docs/evidence/` artifacts MUST be inventoried before bulk movement or partition migration.

#### Scenario: Existing citation is protected

r[repository_layout.evidence_partition.inventory.citation]
- GIVEN an existing Cairn task, spec, or evidence note cites an artifact
- WHEN evidence partition migration is planned
- THEN the artifact is marked stay-flat, migrate-now, migrate-later, or historical
- AND migration includes any required citation and manifest updates.

### Requirement: Manifest rules for partitioned evidence

r[repository_layout.evidence_partition.manifest_rules] Evidence manifest tooling MUST resolve partitioned durable evidence paths and reject stale, missing, target-only, result-only, or path-escaping entries.

#### Scenario: Partitioned manifest validates

r[repository_layout.evidence_partition.manifest_rules.valid]
- GIVEN a `.b3` manifest references artifacts in approved evidence partitions
- WHEN evidence manifest validation runs
- THEN digest rows resolve within durable evidence paths
- AND stale digests, missing files, path escapes, and transient-only paths fail.

### Requirement: Evidence index

r[repository_layout.evidence_partition.index] The repository SHOULD provide a reviewable evidence index mapping changes, scenarios, dates, and artifact classes to durable evidence paths.

#### Scenario: Reviewer finds scenario evidence

r[repository_layout.evidence_partition.index.findable]
- GIVEN a reviewer wants evidence for a maintained scenario or Cairn change
- WHEN they inspect the evidence index
- THEN they can find the relevant receipt, run log, manifest, and oracle-note paths when those artifacts exist.

### Requirement: Evidence partition guards

r[repository_layout.evidence_partition.guards] Partition migration MUST include guards or fixtures for stale manifests, broken citations, missing index rows, and unsafe paths.

#### Scenario: Broken citation fails

r[repository_layout.evidence_partition.guards.broken]
- GIVEN a task or index row points to a moved or missing evidence artifact
- WHEN validation runs
- THEN the broken path is reported
- AND archive is blocked until the path or manifest is corrected.

### Requirement: Evidence partition validation

r[repository_layout.evidence_partition.validation] Evidence partitioning MUST record evidence manifest checks, task evidence validation, index freshness checks, and Cairn gates before archive.

#### Scenario: Partition closeout is reviewable

r[repository_layout.evidence_partition.validation.log]
- GIVEN evidence directories have been partitioned or partition rules added
- WHEN the change is archived
- THEN reviewable logs show manifest validation, task evidence validation, index freshness checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Public flake interface remains stable

r[repository_layout.flake_module_split.public_interface] Splitting the root flake MUST preserve existing public package, app, check, and dev-shell output names unless a separate accepted change explicitly renames them.

#### Scenario: Output inventory matches

r[repository_layout.flake_module_split.public_interface.inventory]
- GIVEN the root flake is split into local modules
- WHEN the public output inventory is compared against the pre-split inventory
- THEN existing package, app, check, and dev-shell names remain present
- AND any intentionally new or removed output is tied to a separate reviewed change.

### Requirement: Package module boundaries

r[repository_layout.flake_module_split.package_modules] Package definitions and shared package helper data SHOULD live in focused local Nix modules with explicit inputs.

#### Scenario: Package module is explicit

r[repository_layout.flake_module_split.package_modules.explicit]
- GIVEN package definitions move out of `flake.nix`
- WHEN reviewers inspect the imported package module
- THEN required inputs such as `pkgs`, `lib`, source paths, tool lists, and shared constants are passed explicitly
- AND package behavior remains equivalent to the root-flake definition it replaced.

### Requirement: App and check module boundaries

r[repository_layout.flake_module_split.app_check_modules] App wrappers and check definitions SHOULD be factored into focused local modules without changing command shapes or evidence semantics.

#### Scenario: Wrapper command shape is stable

r[repository_layout.flake_module_split.app_check_modules.dry_run]
- GIVEN app wrappers or checks move into imported modules
- WHEN selected dry-run app wrappers and focused checks execute
- THEN command names, default arguments, receipt paths, backend defaults, and non-claim text match the pre-split behavior.

### Requirement: Dev shell module boundary

r[repository_layout.flake_module_split.devshell_module] Dev shell definitions MAY move into a focused local module only if documented environment variables, native dependencies, and GUI/client dependencies remain stable.

#### Scenario: Dev shell contract remains documented

r[repository_layout.flake_module_split.devshell_module.contract]
- GIVEN dev shell definitions are imported from a local module
- WHEN developers enter the shell or inspect dry-run documentation
- THEN expected cargo, Nickel, Cairn, Octet, GUI, Xvfb, OpenSSL, and Docker-client support remains available or explicitly documented as unchanged.

### Requirement: Flake split parity checks

r[repository_layout.flake_module_split.parity_checks] The repository MUST include focused checks that prove the flake split preserved output inventory and selected wrapper behavior.

#### Scenario: Missing output fails fast

r[repository_layout.flake_module_split.parity_checks.missing]
- GIVEN a local module accidentally omits a previously available public output
- WHEN the parity check runs
- THEN the missing output name is reported
- AND the check fails before archive.

### Requirement: Flake split validation

r[repository_layout.flake_module_split.validation] The flake split MUST record focused Nix evaluation, selected dry-run/check output, Cairn gates, and Cairn validation before archive.

#### Scenario: Refactor closeout is reviewable

r[repository_layout.flake_module_split.validation.log]
- GIVEN the flake has been split into local modules
- WHEN the change is archived
- THEN reviewable logs show public output inventory parity, selected wrapper dry-runs, selected check builds, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: README scope

r[repository_layout.readme_split.scope] The root README SHOULD serve as a concise quickstart and navigation index instead of the canonical home for every scenario command and evidence workflow detail.

#### Scenario: README remains navigable

r[repository_layout.readme_split.scope.quickstart]
- GIVEN a new developer opens the root README
- WHEN they read the first sections
- THEN they can identify the repository purpose, main component layout, fastest dry-run/live commands, key evidence caveats, and links to deeper docs
- AND they are not required to scan every maintained scenario row to find the basic workflow.

### Requirement: Scenario command docs

r[repository_layout.readme_split.scenario_docs] Detailed scenario command listings SHOULD live in focused docs or generated indexes that are checked against scenario metadata.

#### Scenario: Scenario command listing stays fresh

r[repository_layout.readme_split.scenario_docs.fresh]
- GIVEN a scenario wrapper, manifest row, or generated command changes
- WHEN documentation freshness checks run
- THEN stale scenario command docs are reported
- AND human-authored prose outside generated blocks is preserved.

### Requirement: Evidence workflow docs

r[repository_layout.readme_split.evidence_docs] Evidence workflow details and non-claim boundaries MUST remain visible from README links after content is moved into focused docs.

#### Scenario: Evidence caveats remain discoverable

r[repository_layout.readme_split.evidence_docs.caveats]
- GIVEN a reader follows README links for evidence workflow
- WHEN they inspect the focused evidence docs
- THEN durable evidence location, BLAKE3 manifest expectations, non-claims, and Cairn task citation rules remain documented
- AND generated command blocks do not overwrite human evidence interpretation.

### Requirement: Config and verification docs

r[repository_layout.readme_split.config_verification_docs] Config and verification details MAY move from README into focused docs only if README keeps stable entry links and commands remain checkable.

#### Scenario: Verification command remains findable

r[repository_layout.readme_split.config_verification_docs.findable]
- GIVEN a developer needs to run repository verification
- WHEN they read the README and linked docs
- THEN focused verification docs list the relevant fast, generated, evidence, and full gates
- AND README links point to those docs.

### Requirement: Documentation checks

r[repository_layout.readme_split.checks] The README split SHOULD include checks for generated doc freshness, command parity, and broken local documentation links when practical.

#### Scenario: Broken moved link fails

r[repository_layout.readme_split.checks.links]
- GIVEN README links to a moved docs page or generated command index
- WHEN the docs check runs
- THEN missing local docs or stale generated command blocks are reported
- AND archive is blocked until docs are refreshed.

### Requirement: README split validation

r[repository_layout.readme_split.validation] The README split MUST record docs checks, generated freshness checks, selected command parity, and Cairn gates before archive.

#### Scenario: Documentation closeout is reviewable

r[repository_layout.readme_split.validation.log]
- GIVEN root README content has been split into focused docs
- WHEN the change is archived
- THEN reviewable logs show docs checks, generated freshness checks, selected command dry-runs, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Nix surface modularization inventory

r[repository_layout.nix_surface_modularization.inventory] Root Nix surface modularization work MUST inventory current package, app, check, devshell, generated wrapper metadata, repeated helper patterns, and flake output inventory before splitting files.

#### Scenario: Nix output ownership is reviewable

r[repository_layout.nix_surface_modularization.inventory.reviewable]
- GIVEN root Nix surface modularization is selected
- WHEN reviewers inspect the inventory
- THEN package, app, check, devshell, generated metadata, baseline inventory, and repeated wrapper/check helper responsibilities are named
- AND baseline flake output inventory validation is recorded before core changes.

### Requirement: Nix surface module boundaries

r[repository_layout.nix_surface_modularization.module_boundaries] Root Nix surfaces SHOULD be split into focused modules or helper functions for layout/docs, generated surfaces, evidence, runner/scenario, component behavior, Octet, checker framework, packages, and apps while preserving aggregator entrypoints.

#### Scenario: Nix checks have focused owners

r[repository_layout.nix_surface_modularization.module_boundaries.focused]
- GIVEN a Nix check, package, app, or helper is reviewed
- WHEN maintainers inspect the Nix module tree
- THEN the definition belongs to the focused module for its validation or output family
- AND unrelated flake surfaces are not reintroduced into one long catch-all file.

### Requirement: Nix output parity

r[repository_layout.nix_surface_modularization.output_parity] Nix surface modularization MUST preserve flake output names, app and package main programs, dry-run behavior, baseline output inventory compatibility, and generated scenario wrapper metadata freshness.

#### Scenario: Flake output inventory remains stable

r[repository_layout.nix_surface_modularization.output_parity.stable]
- GIVEN existing package, app, check, and devshell outputs
- WHEN the modularized Nix surfaces evaluate
- THEN the output inventory matches the accepted baseline except for explicitly allowed additions
- AND existing app/package programs and dry-run wrapper behavior remain compatible.

### Requirement: Nix check reviewability

r[repository_layout.nix_surface_modularization.reviewability] Nix helper abstractions SHOULD keep each check's command intent, required inputs, failure diagnostics, and copied `$out` evidence artifacts reviewable.

#### Scenario: Helper output remains inspectable

r[repository_layout.nix_surface_modularization.reviewability.evidence]
- GIVEN a flake check uses a shared helper
- WHEN reviewers inspect the check definition or output
- THEN the command intent, input paths, copied logs, and diagnostics remain visible
- AND helper abstraction does not hide evidence artifacts required by docs or Cairn tasks.

### Requirement: Nix surface modularization tests

r[repository_layout.nix_surface_modularization.tests] The change MUST include positive checks for valid split outputs and negative checks for missing outputs, unexpected outputs, stale generated metadata, missing helper evidence copies, and broken dry-run wrappers where feasible.

#### Scenario: Valid Nix surfaces pass

r[repository_layout.nix_surface_modularization.tests.positive]
- GIVEN split Nix modules with expected outputs
- WHEN focused Nix and inventory checks run
- THEN tests prove packages, apps, checks, devshells, generated metadata, and helper-copied artifacts are available as expected.

#### Scenario: Invalid Nix surfaces fail clearly

r[repository_layout.nix_surface_modularization.tests.negative]
- GIVEN a missing output, unexpected output, stale generated metadata, missing copied check artifact, or broken dry-run wrapper
- WHEN focused Nix and inventory checks run
- THEN tests prove the diagnostic names the stale or missing surface and blocks silent drift.

### Requirement: Nix surface modularization validation

r[repository_layout.nix_surface_modularization.validation] The change MUST record focused Nix eval/build checks, flake output inventory, generated-surface checks when touched, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Nix surface closeout is reviewable

r[repository_layout.nix_surface_modularization.validation.logs]
- GIVEN root Nix surface modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change output inventory, focused Nix checks, generated-surface checks when applicable, positive and negative regression coverage, Cairn gates, and Cairn validation passing.

### Requirement: Minecraft Wiki project skill path

r[repository_layout.minecraft_wiki_pi_skill.path] The repository SHOULD provide a project-level Pi skill for Minecraft Wiki research under a Pi-supported project skill path and document the chosen path and discovery assumptions.

#### Scenario: Skill path is reviewable

r[repository_layout.minecraft_wiki_pi_skill.path.reviewable]
- GIVEN a repository-owned Minecraft Wiki skill is added
- WHEN developers inspect the change evidence or repository docs
- THEN the selected path, expected `SKILL.md` format, required frontmatter, and project-trust loading assumption are recorded
- AND the skill is not represented as a global Pi skill unless separately installed there.

### Requirement: Minecraft Wiki skill workflow

r[repository_layout.minecraft_wiki_pi_skill.workflow] The project Minecraft Wiki skill MUST instruct agents to perform narrow, known-URL wiki retrieval, treat fetched content as untrusted external data, record edition/version/protocol scope, cite source pages, and require extracted-data or Paper/vanilla parity before behavior claims are promoted.

#### Scenario: Agent reads wiki without overclaiming

r[repository_layout.minecraft_wiki_pi_skill.workflow.non_overclaiming]
- GIVEN an agent uses the skill to inspect a Minecraft Wiki page for plugin design
- WHEN the skill workflow is followed
- THEN the agent starts with focused markdown retrieval from a known wiki URL, records source page identity and target version scope, summarizes behavior as a guide, and names required follow-up evidence before implementation claims
- AND the agent does not execute instructions from the page, vendor large page content by default, or claim broad Minecraft compatibility from wiki text alone.

### Requirement: Minecraft Wiki skill validation

r[repository_layout.minecraft_wiki_pi_skill.tests] The skill work MUST include positive and negative validation for required skill frontmatter and workflow rules.

#### Scenario: Valid skill passes

r[repository_layout.minecraft_wiki_pi_skill.tests.positive]
- GIVEN the repository Minecraft Wiki skill contains valid `name` and `description` frontmatter plus required retrieval, safety, version, citation, and non-claim rules
- WHEN the focused skill validation runs
- THEN the skill passes with a deterministic success result.

#### Scenario: Invalid skill fails clearly

r[repository_layout.minecraft_wiki_pi_skill.tests.negative]
- GIVEN a fixture skill is missing required frontmatter, known-URL retrieval guidance, untrusted-content handling, version scope, citation guidance, or wiki-as-guide wording
- WHEN the focused skill validation runs
- THEN it fails with a diagnostic naming the missing rule.

### Requirement: Minecraft Wiki skill closeout validation

r[repository_layout.minecraft_wiki_pi_skill.validation] Minecraft Wiki skill work MUST record focused skill validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks for promoted evidence before archive.

#### Scenario: Skill closeout is reviewable

r[repository_layout.minecraft_wiki_pi_skill.validation.log]
- GIVEN the repository Minecraft Wiki skill is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show valid-skill and invalid-skill validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, evidence-manifest checks when artifacts are cited, and explicit non-claims for plugin implementation, wiki accuracy, broad vanilla parity, and global Pi installation.

### Requirement: Next Cairn target project skill

r[repository_layout.next_cairn_target_skill] The repository SHOULD provide a project-level Pi skill for hunting the next bounded work target and writing one native Cairn package.

#### Scenario: Skill selects one bounded target

r[repository_layout.next_cairn_target_skill.bounded]
- GIVEN a user asks what to do next, asks to hunt or find the next target, or asks to write a Cairn for the next target
- WHEN the skill workflow runs
- THEN it checks repository status and active Cairn changes, reads the smallest relevant roadmap/spec/archive sources, collects candidate targets, scores boundedness and evidence readiness, and selects one target
- AND it rejects broad vanilla parity, all recipes, all block entities, broad redstone, broad mob AI, world generation, default plugin membership changes, public-server safety, or production-readiness scope unless explicitly requested.

#### Scenario: Skill writes a native Cairn package

r[repository_layout.next_cairn_target_skill.package]
- GIVEN a bounded next target is selected
- WHEN the skill writes the Cairn
- THEN it creates proposal, design, tasks, and spec delta files under `cairn/changes/<slug>/`
- AND it runs Cairn validation plus proposal/design/tasks gates
- AND it does not implement, sync, archive, commit, push, or claim broad compatibility unless the user explicitly asks for those next steps.

### Requirement: Next Cairn target skill documentation

r[repository_layout.next_cairn_target_skill.docs] The next-target skill work MUST document the selected project skill path, Pi discovery assumptions, workflow, scoring rubric, Cairn output shape, and non-claims.

#### Scenario: Skill docs are reviewable

r[repository_layout.next_cairn_target_skill.docs.reviewable]
- GIVEN reviewers inspect the repository documentation
- WHEN they compare it with the project skill
- THEN they can identify the `.pi/skills/next-cairn-target/SKILL.md` path, frontmatter expectations, project-trust loading assumption, candidate sources, scoring criteria, output shape, validation steps, and explicit non-claims.

### Requirement: Next Cairn target skill validation

r[repository_layout.next_cairn_target_skill.validation] The next-target skill work MUST include focused validation with positive and negative self-tests for required skill and documentation rules.

#### Scenario: Complete target-hunt skill passes

r[repository_layout.next_cairn_target_skill.validation.positive]
- GIVEN the skill and docs contain valid frontmatter, preflight steps, candidate sources, scoring rubric, Cairn package output shape, validation gates, and non-claims
- WHEN focused validation runs
- THEN it passes with a deterministic success result.

#### Scenario: Incomplete target-hunt skill fails clearly

r[repository_layout.next_cairn_target_skill.validation.negative]
- GIVEN a fixture skill or documentation set is missing frontmatter, status/change-list preflight, candidate sources, scoring rubric, Cairn package output shape, validation gates, or non-claims
- WHEN focused validation runs
- THEN it fails with a diagnostic naming the missing rule.

### Requirement: Next Cairn target skill closeout

r[repository_layout.next_cairn_target_skill.closeout] The next-target skill work MUST record focused validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Target skill closeout is reviewable

r[repository_layout.next_cairn_target_skill.closeout.log]
- GIVEN the next-target project skill change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused positive/negative validation, Cairn gates, Cairn validation, task-evidence validation, accepted-spec requirement IDs, evidence-manifest freshness, flake checks, and archive receipts
- AND the evidence preserves non-claims for target implementation, global Pi installation, broad project prioritization, broad Minecraft compatibility, vanilla parity, public-server safety, and production readiness.

### Requirement: OnixResearch workspace namespace inventory

r[repository_layout.onixresearch_workspace_namespace.inventory] OnixResearch workspace namespace migration work MUST inventory active and historical references to legacy sibling repository paths before moving repositories or changing default workspace path guidance.

#### Scenario: Legacy path references are classified

r[repository_layout.onixresearch_workspace_namespace.inventory.classified]
- GIVEN repositories are proposed to move from `~/git/<repo>` to `~/git/OnixResearch/<repo>`
- WHEN the migration inventory is reviewed
- THEN scripts, docs, Pi skills, Nix `path:` inputs, flake locks, evidence notes, and validation commands that reference legacy sibling paths are recorded
- AND each reference is classified as active, historical, generated, blocked, removable, or intentionally retained with a migration action or waiver.

### Requirement: OnixResearch canonical workspace paths

r[repository_layout.onixresearch_workspace_namespace.canonical_paths] Workspace layout documentation SHOULD define `~/git/OnixResearch/<repo>` as the canonical home for in-scope OnixResearch repositories only after the repository set, migration order, cleanliness requirements, and non-claims are recorded.

#### Scenario: Canonical namespace contract is reviewable

r[repository_layout.onixresearch_workspace_namespace.canonical_paths.contract]
- GIVEN the namespace migration contract is proposed
- WHEN maintainers inspect the contract
- THEN it names the initial in-scope repositories, migration order, expected clean-worktree or explicit-dirty-state handling, and the shared root variable or equivalent configuration surface
- AND it states that local path migration does not change remotes, branch names, commit identity, package ownership, release evidence, behavioral correctness, or provenance claims.

### Requirement: OnixResearch compatibility links

r[repository_layout.onixresearch_workspace_namespace.compatibility_links] Migrated repositories MUST retain temporary compatibility links from legacy `~/git/<repo>` paths to canonical `~/git/OnixResearch/<repo>` paths for shell and Git path consumers until active consumers are updated or explicitly waived.

#### Scenario: Legacy shell and Git commands keep working during migration

r[repository_layout.onixresearch_workspace_namespace.compatibility_links.legacy]
- GIVEN a repository has moved to the canonical namespace
- WHEN a still-active shell or Git command resolves `~/git/<repo>`
- THEN the compatibility link resolves to the canonical repository path
- AND validation records which consumers still depend on the compatibility path
- AND literal Nix `path:` inputs are migrated to the canonical path or recorded as expected symlink-root rejections until they are updated.

#### Scenario: Compatibility removal fails closed

r[repository_layout.onixresearch_workspace_namespace.compatibility_links.retirement]
- GIVEN a compatibility link is proposed for removal
- WHEN active path inventory still contains unwaived legacy consumers
- THEN removal is blocked with diagnostics naming those consumers and the required migration action.

### Requirement: OnixResearch reference updates

r[repository_layout.onixresearch_workspace_namespace.reference_updates] Active workspace docs, scripts, skills, and literal Nix path references SHOULD migrate toward a shared canonical root setting instead of adding new hard-coded legacy sibling paths.

#### Scenario: Updated automation uses a shared root

r[repository_layout.onixresearch_workspace_namespace.reference_updates.shared_root]
- GIVEN an active script, doc, skill, or path-input command is updated for the namespace migration
- WHEN reviewers inspect the updated reference
- THEN it uses `ONIX_RESEARCH_ROOT`, an equivalent documented root configuration, or an explicit compatibility-path waiver
- AND it does not introduce a new unclassified hard-coded legacy sibling path.

### Requirement: OnixResearch namespace validation

r[repository_layout.onixresearch_workspace_namespace.validation] OnixResearch workspace namespace migration work MUST record focused validation through canonical paths, compatibility shell/Git paths, and selected canonical Nix path inputs before archive.

#### Scenario: Migration closeout is reviewable

r[repository_layout.onixresearch_workspace_namespace.validation.log]
- GIVEN namespace migration work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show path-reference inventory freshness, canonical-path command smoke checks, compatibility-path shell/Git command smoke checks, selected canonical Nix path-input validation, expected symlink-root rejection checks for literal compatibility Nix `path:` inputs, Cairn proposal/design/tasks gates, Cairn validation, and any waivers or rollback notes
- AND the evidence preserves non-claims for remote changes, history rewriting, release eligibility, behavioral correctness, whole-stack safety, and compatibility-link retirement.
