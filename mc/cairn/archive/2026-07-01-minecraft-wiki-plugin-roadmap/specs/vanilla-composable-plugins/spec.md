# vanilla-composable-plugins Change Spec: Minecraft Wiki-guided plugin roadmap

## Requirements

### Requirement: Minecraft Wiki source inventory

r[vanilla_composable_plugins.wiki_inventory] Wiki-guided plugin work MUST inventory selected Minecraft Wiki entry points, target edition/version/protocol assumptions, related extracted-data sources, existing Valence plugin surfaces, and existing compatibility rails before deriving implementation slices.

#### Scenario: Wiki sources are version scoped

r[vanilla_composable_plugins.wiki_inventory.version_scoped]
- GIVEN a wiki page, category, or protocol document is selected as a guide for plugin work
- WHEN reviewers inspect the inventory
- THEN the source URL, page title, target edition, target game version, target protocol when relevant, retrieval date or evidence snapshot, and known version-drift risk are recorded
- AND newer-release behavior is not treated as target-version behavior without a separate version decision.

### Requirement: Domain-to-plugin taxonomy

r[vanilla_composable_plugins.taxonomy] The roadmap SHOULD map wiki domains to candidate composable Valence plugin groups, individual feature plugins, dependency edges, schedule impact, evidence requirements, and explicit non-claims.

#### Scenario: Domain maps to bounded plugins

r[vanilla_composable_plugins.taxonomy.bounded]
- GIVEN a domain such as crafting, smelting, effects, equipment, block interactions, block entities, redstone, mobs, biomes, commands, or protocol behavior is selected
- WHEN the taxonomy is reviewed
- THEN candidate plugin group names, feature plugin names, required dependencies, optional dependencies, default-membership decisions, and unsupported broad claims are distinguishable.

### Requirement: Behavior card contract

r[vanilla_composable_plugins.behavior_cards] Each follow-on wiki-guided plugin implementation SHOULD start from a behavior card that records source pages, version scope, pure rule core, Bevy/ECS shell, data dependencies, positive tests, negative tests, schedule impact, parity evidence, and non-claims.

#### Scenario: Behavior card is implementation-ready

r[vanilla_composable_plugins.behavior_cards.ready]
- GIVEN a bounded plugin feature is selected for implementation
- WHEN reviewers inspect its behavior card
- THEN they can identify the deterministic inputs and outputs for the pure core, the ECS resources/events/components owned by the shell, required game-data tables, required schedule evidence, and the exact compatibility claim that evidence may promote.

### Requirement: Functional core and Bevy shell boundary

r[vanilla_composable_plugins.core_shell] Wiki-derived plugin implementations MUST keep vanilla rule decisions in pure deterministic cores and keep ECS mutation, packet emission, logging, filesystem access, network access, and schedule registration in thin Bevy shell systems.

#### Scenario: Rule core is testable without server I/O

r[vanilla_composable_plugins.core_shell.testable]
- GIVEN a wiki-derived gameplay rule such as recipe matching, burn progress, hunger recovery, armor mitigation, effect ticking, block drop selection, or projectile damage attribution
- WHEN its core is tested
- THEN tests can exercise valid and invalid inputs without starting Valence, connecting a client, reading files, writing packets, or depending on wall-clock time.

### Requirement: Evidence and test policy

r[vanilla_composable_plugins.evidence_policy] Wiki-guided plugin work MUST include positive and negative tests for promoted rule cores and MUST require extracted-data checks or Paper/vanilla parity receipts before claiming target-version vanilla behavior.

#### Scenario: Wiki-derived behavior does not overclaim

r[vanilla_composable_plugins.evidence_policy.non_overclaiming]
- GIVEN a plugin feature is implemented from wiki-guided behavior
- WHEN its evidence is promoted
- THEN the claim names the bounded feature, target version, tested inputs, rejected invalid inputs, and non-claims such as broad vanilla parity, all recipes, all block entities, all mobs, public-server safety, or production readiness unless separately proven.

### Requirement: Plugin sequence and stop conditions

r[vanilla_composable_plugins.sequence] The roadmap SHOULD sequence follow-on plugin Cairns by dependency order, existing evidence-rail reuse, implementation risk, and explicit stop conditions.

#### Scenario: High-risk domains are deferred until prerequisites exist

r[vanilla_composable_plugins.sequence.deferred]
- GIVEN a high-complexity domain such as broad redstone, broad mob AI, world generation, or all-block update semantics is proposed
- WHEN the roadmap sequence is reviewed
- THEN prerequisite data, schedule, parity, and architecture gaps are named before implementation starts
- AND smaller bounded survival or combat seams may proceed independently when their evidence rails are sufficient.

### Requirement: Roadmap validation

r[vanilla_composable_plugins.validation] Roadmap work MUST record Cairn proposal, design, tasks, repository validation, and evidence-manifest checks for promoted roadmap artifacts before archive.

#### Scenario: Roadmap closeout is reviewable

r[vanilla_composable_plugins.validation.log]
- GIVEN the wiki-guided plugin roadmap is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show Cairn proposal/design/tasks gates, Cairn validation, promoted inventory or roadmap artifacts, evidence manifests when cited, and explicit non-claims for implementation, default Valence behavior, broad Minecraft compatibility, vanilla parity, public-server safety, and production readiness.
