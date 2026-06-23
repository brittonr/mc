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
