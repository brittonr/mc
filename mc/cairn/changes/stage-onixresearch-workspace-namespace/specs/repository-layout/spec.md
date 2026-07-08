## ADDED Requirements

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

r[repository_layout.onixresearch_workspace_namespace.compatibility_links] Migrated repositories MUST retain temporary compatibility links from legacy `~/git/<repo>` paths to canonical `~/git/OnixResearch/<repo>` paths until active consumers are updated or explicitly waived.

#### Scenario: Legacy commands keep working during migration

r[repository_layout.onixresearch_workspace_namespace.compatibility_links.legacy]
- GIVEN a repository has moved to the canonical namespace
- WHEN a still-active legacy command resolves `~/git/<repo>` or `path:/home/brittonr/git/<repo>#...`
- THEN the compatibility link resolves to the canonical repository path
- AND validation records which consumers still depend on the compatibility path.

#### Scenario: Compatibility removal fails closed

r[repository_layout.onixresearch_workspace_namespace.compatibility_links.retirement]
- GIVEN a compatibility link is proposed for removal
- WHEN active path inventory still contains unwaived legacy consumers
- THEN removal is blocked with diagnostics naming those consumers and the required migration action.

### Requirement: OnixResearch reference updates

r[repository_layout.onixresearch_workspace_namespace.reference_updates] Active workspace docs, scripts, skills, and Nix path references SHOULD migrate toward a shared canonical root setting instead of adding new hard-coded legacy sibling paths.

#### Scenario: Updated automation uses a shared root

r[repository_layout.onixresearch_workspace_namespace.reference_updates.shared_root]
- GIVEN an active script, doc, skill, or path-input command is updated for the namespace migration
- WHEN reviewers inspect the updated reference
- THEN it uses `ONIX_RESEARCH_ROOT`, an equivalent documented root configuration, or an explicit compatibility-path waiver
- AND it does not introduce a new unclassified hard-coded legacy sibling path.

### Requirement: OnixResearch namespace validation

r[repository_layout.onixresearch_workspace_namespace.validation] OnixResearch workspace namespace migration work MUST record focused validation through both canonical and compatibility paths before archive.

#### Scenario: Migration closeout is reviewable

r[repository_layout.onixresearch_workspace_namespace.validation.log]
- GIVEN namespace migration work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show path-reference inventory freshness, canonical-path command smoke checks, compatibility-path command smoke checks, selected Nix path-input validation, Cairn proposal/design/tasks gates, Cairn validation, and any waivers or rollback notes
- AND the evidence preserves non-claims for remote changes, history rewriting, release eligibility, behavioral correctness, whole-stack safety, and compatibility-link retirement.
