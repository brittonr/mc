# repository-layout Change Spec: Subtree agent documentation

## Requirements

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
