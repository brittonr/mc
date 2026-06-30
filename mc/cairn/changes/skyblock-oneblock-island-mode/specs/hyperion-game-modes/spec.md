# hyperion-game-modes Change Spec: SkyBlock and OneBlock island mode

## Requirements

### Requirement: Island-mode scope

r[hyperion_game_modes.island_mode.scope] SkyBlock/OneBlock work MUST be scoped as a Hyperion-owned optional island-mode plugin with mode-local island, profile, permission, generator, snapshot, recovery, configuration, and diagnostic state.

#### Scenario: Island mode is isolated

r[hyperion_game_modes.island_mode.scope.isolated]
- GIVEN the island-mode plugin is enabled
- WHEN default survival, Bedwars, CTF, arena modes, or compatibility fixtures are also present
- THEN island-mode systems mutate only owned islands, members, visitors, generator state, snapshots, and diagnostics
- AND production economy, marketplace, public anti-grief guarantees, full SkyBlock network behavior, Valence behavior, and broad survival compatibility remain explicit non-claims.

### Requirement: Island lifecycle

r[hyperion_game_modes.island_mode.island_lifecycle] Island mode MUST manage island allocation, spawn/home, ownership, membership, visitor access, reset/delete planning, void recovery, and cleanup through deterministic cores with shell-owned world and storage side effects.

#### Scenario: Island lifecycle creates owned state

r[hyperion_game_modes.island_mode.island_lifecycle.create]
- GIVEN an eligible player requests a new island using a configured island profile
- WHEN the lifecycle core evaluates allocation inputs and available island placement facts
- THEN it returns an island identity, owner assignment, spawn/home facts, starter-state plan, and snapshot seed state
- AND shell code applies world, inventory, teleport, and storage side effects only from that plan.

#### Scenario: Invalid lifecycle action fails closed

r[hyperion_game_modes.island_mode.island_lifecycle.invalid]
- GIVEN a duplicate island request, stale owner, unauthorized reset/delete, invalid profile, exhausted placement pool, or disconnected owner
- WHEN the lifecycle core evaluates the action
- THEN it rejects the request with deterministic diagnostics
- AND no island, snapshot, world chunk, or membership state is partially created or destroyed.

### Requirement: Island permission policy

r[hyperion_game_modes.island_mode.permission_policy] Island mode MUST route build, break, container, invite, visit, reset, admin, and cross-island actions through a central pure permission policy that fails closed for missing or stale authority.

#### Scenario: Authorized island action passes

r[hyperion_game_modes.island_mode.permission_policy.valid]
- GIVEN an actor has a configured role on an island and requests an action inside an owned island volume
- WHEN the permission core evaluates actor, role, island, location, action, and config facts
- THEN it returns an allow decision with any bounded side-effect plan
- AND shell code applies only the allowed mutation.

#### Scenario: Unauthorized island action is rejected

r[hyperion_game_modes.island_mode.permission_policy.rejects]
- GIVEN an actor lacks permission, targets another island, uses stale membership, requests admin-only action, or acts outside owned volumes
- WHEN the permission core evaluates the request
- THEN it returns a deny decision with diagnostics
- AND no block, container, generator, membership, or reset mutation occurs.

### Requirement: Generator progression

r[hyperion_game_modes.island_mode.generator_progression] Island mode MUST implement deterministic SkyBlock starter-state and OneBlock generator progression cores over explicit profile, phase, output-table, mob/chest event, reward, cooldown, and snapshot inputs.

#### Scenario: OneBlock generator advances deterministically

r[hyperion_game_modes.island_mode.generator_progression.valid]
- GIVEN an island has a valid generator state, configured phase, and eligible break interaction
- WHEN the generator core evaluates the action
- THEN it returns the next generator state, block or event output, optional reward, and snapshot update plan
- AND all numeric tuning comes from named configuration or fixtures.

#### Scenario: Invalid generator state is rejected

r[hyperion_game_modes.island_mode.generator_progression.invalid]
- GIVEN a generator state is missing, corrupt, out of phase, unauthorized, outside the island, or inconsistent with the selected profile
- WHEN the generator core evaluates the action
- THEN it rejects the transition or requests recovery according to the snapshot policy
- AND no duplicate reward, impossible phase, or cross-island output is produced.

### Requirement: Island persistence

r[hyperion_game_modes.island_mode.persistence] Island mode MUST define snapshot persistence and recovery contracts for island metadata, generator state, members, roles, bounded inventory facts where scoped, versioning, corruption handling, and audit summaries before making persistence claims.

#### Scenario: Snapshot restore is deterministic

r[hyperion_game_modes.island_mode.persistence.restore]
- GIVEN a valid island snapshot with expected schema version, owner, members, generator state, and bounded world metadata
- WHEN restore validation runs
- THEN it returns a deterministic restore plan for island state and diagnostics
- AND shell code owns actual storage reads, writes, and world mutation.

#### Scenario: Corrupt snapshot fails closed

r[hyperion_game_modes.island_mode.persistence.corrupt]
- GIVEN a snapshot is missing required fields, has inconsistent ownership, invalid generator phase, duplicate members, impossible inventory facts, or unsupported version
- WHEN restore validation runs
- THEN it rejects or quarantines the snapshot according to recovery policy
- AND no partial island restore, cross-island state leak, or panic occurs.

### Requirement: Island-mode tests

r[hyperion_game_modes.island_mode.tests] Island mode work MUST include positive tests for island lifecycle, permissions, generator progression, snapshots, void recovery, and cleanup plus negative tests for unauthorized edits, duplicate ownership, cross-island leaks, invalid generator state, stale members, corrupt snapshots, unauthorized resets, and orphaned world state.

#### Scenario: Positive island behavior is covered

r[hyperion_game_modes.island_mode.tests.positive]
- GIVEN valid island profiles, owners, roles, generator states, snapshots, and action summaries
- WHEN pure-core and focused shell tests run
- THEN create, join, visit, permission, generator, snapshot restore, void recovery, and reset cleanup behavior passes.

#### Scenario: Negative island behavior fails closed

r[hyperion_game_modes.island_mode.tests.negative]
- GIVEN unauthorized actions, duplicate island ownership, cross-island targets, corrupt snapshots, invalid generator states, stale members, unauthorized reset requests, or cleanup edge cases
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, duplicate reward, grief mutation, corrupt restore, or cross-island leak occurs.

### Requirement: Island-mode validation

r[hyperion_game_modes.island_mode.validation] Island mode work MUST record focused Hyperion checks, island/generator tests, shell/plugin tests, persistence recovery fixtures, permission tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Island-mode closeout is reviewable

r[hyperion_game_modes.island_mode.validation.log]
- GIVEN SkyBlock/OneBlock work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant lifecycle, permission, generator, persistence, positive, negative, shell, and plugin checks plus Cairn gates and validation
- AND unsupported production economy, full network, anti-grief, public-server safety, Valence, and broad survival compatibility claims remain non-claims.
