# hyperion-game-modes Change Spec: LifeSteal SMP mode

## Requirements

### Requirement: LifeSteal scope

r[hyperion_game_modes.lifesteal.scope] LifeSteal SMP work MUST be scoped as a Hyperion-owned optional mode with mode-local heart capacity, death attribution, transfer, final-death, recovery, persistence, admin repair, configuration, and diagnostic state.

#### Scenario: LifeSteal does not change default health

r[hyperion_game_modes.lifesteal.scope.isolated]
- GIVEN the LifeSteal plugin is enabled in a Hyperion app
- WHEN default survival, Bedwars, CTF, arena modes, or compatibility fixtures are also present
- THEN LifeSteal heart rules apply only to players scoped to the LifeSteal mode
- AND vanilla health behavior, combat parity, public moderation, Valence behavior, Bedwars behavior, and broad survival compatibility remain explicit non-claims.

### Requirement: LifeSteal heart state

r[hyperion_game_modes.lifesteal.heart_state] LifeSteal MUST model heart capacity as mode-local player state and project it into runtime health behavior only through explicit shell boundaries for LifeSteal-scoped players.

#### Scenario: Heart state is projected for scoped players

r[hyperion_game_modes.lifesteal.heart_state.scoped]
- GIVEN a LifeSteal player has valid persisted or initialized heart-capacity state
- WHEN the shell prepares health-related gameplay for that player
- THEN it applies the configured LifeSteal heart capacity and related feedback only for that mode-scoped player
- AND players outside LifeSteal keep their existing health behavior.

#### Scenario: Invalid heart state is rejected

r[hyperion_game_modes.lifesteal.heart_state.invalid]
- GIVEN heart state is missing, corrupt, below configured minimum, above configured maximum, stale for the player identity, or inconsistent with snapshot version
- WHEN heart-state validation runs
- THEN it rejects or repairs the state according to named recovery configuration
- AND no default health, unrelated player, or other mode state is mutated by the core.

### Requirement: Death transfer core

r[hyperion_game_modes.lifesteal.death_transfer] LifeSteal MUST decide PvP death attribution, transfer eligibility, heart gain/loss, duplicate-event rejection, and overflow prevention through pure cores over explicit death, combat-tag, player-state, and configuration inputs.

#### Scenario: Valid PvP death transfers hearts

r[hyperion_game_modes.lifesteal.death_transfer.valid]
- GIVEN a LifeSteal victim and killer are valid mode-scoped players and the death event has valid PvP attribution
- WHEN the transfer core evaluates death facts, combat tags, current heart state, and transfer configuration
- THEN it returns bounded heart-capacity changes, audit facts, and feedback plans for victim and killer
- AND shell code owns health projection, inventory, chat, storage, and packet side effects.

#### Scenario: Invalid death does not transfer hearts

r[hyperion_game_modes.lifesteal.death_transfer.invalid]
- GIVEN a death event is environmental, self-inflicted, duplicate, stale, unauthenticated, cross-mode, disconnected, or would exceed configured heart bounds
- WHEN the transfer core evaluates the event
- THEN it returns a deterministic no-transfer or rejection outcome
- AND no heart capacity, score, inventory, or persistence state is double-applied.

### Requirement: Final-death and recovery policy

r[hyperion_game_modes.lifesteal.final_death] LifeSteal MUST implement final-death, exclusion, recovery, grace, and optional heart item craft/trade hooks as named configurable policies rather than hard-coded public-server ban behavior.

#### Scenario: Final-death policy is applied from config

r[hyperion_game_modes.lifesteal.final_death.configured]
- GIVEN a victim reaches the configured final-death threshold after a valid transfer
- WHEN the final-death core evaluates policy inputs
- THEN it returns the configured spectator, exclusion, recovery, penalty, or other bounded final-death action
- AND the outcome is auditable without claiming production moderation or public-server safety.

#### Scenario: Recovery action is bounded

r[hyperion_game_modes.lifesteal.final_death.recovery]
- GIVEN a recovery, grace, craft-heart, trade-heart, or admin-granted recovery action is requested
- WHEN the policy core evaluates actor authority, target state, cooldown facts, item facts, and configuration
- THEN it returns an allow or deny decision with bounded heart-state changes
- AND invalid or unauthorized recovery requests do not mutate heart state.

### Requirement: LifeSteal persistence and admin repair

r[hyperion_game_modes.lifesteal.persistence_admin] LifeSteal MUST define snapshot persistence, restore validation, audit summaries, and bounded admin repair actions for heart state before claiming state durability.

#### Scenario: Heart snapshot restores deterministically

r[hyperion_game_modes.lifesteal.persistence_admin.restore]
- GIVEN a valid LifeSteal heart snapshot for a player and supported schema version
- WHEN restore validation runs
- THEN it returns a deterministic heart-state restore plan and audit summary
- AND shell code owns storage reads, writes, and runtime health projection.

#### Scenario: Unauthorized admin repair is rejected

r[hyperion_game_modes.lifesteal.persistence_admin.unauthorized]
- GIVEN an admin repair request lacks authority, targets a stale player, exceeds configured bounds, conflicts with audit state, or uses malformed input
- WHEN the admin repair core evaluates the request
- THEN it rejects the request with deterministic diagnostics
- AND no heart, snapshot, or audit state is changed.

### Requirement: LifeSteal tests

r[hyperion_game_modes.lifesteal.tests] LifeSteal work MUST include positive tests for valid PvP transfers, final-death policy, recovery, snapshot restore, bounded admin repair, and mode isolation plus negative tests for self-kills, environmental deaths, stale tags, duplicate events, overflow, unauthorized admin actions, corrupt snapshots, and cross-mode mutation.

#### Scenario: Positive LifeSteal behavior is covered

r[hyperion_game_modes.lifesteal.tests.positive]
- GIVEN valid LifeSteal players, death events, combat tags, heart states, final-death policies, recovery inputs, and snapshots
- WHEN pure-core and focused shell tests run
- THEN transfer, heart projection, final-death, recovery, snapshot restore, admin repair, and mode isolation behavior passes.

#### Scenario: Negative LifeSteal behavior fails closed

r[hyperion_game_modes.lifesteal.tests.negative]
- GIVEN invalid death events, stale combat tags, self-kills, environmental deaths, duplicate events, overflow attempts, unauthorized admin repair, corrupt snapshots, or cross-mode players
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, duplicate transfer, unauthorized heart change, corrupt restore, or cross-mode health mutation occurs.

### Requirement: LifeSteal validation

r[hyperion_game_modes.lifesteal.validation] LifeSteal work MUST record focused Hyperion checks, transfer core tests, shell/plugin tests, persistence fixtures, admin command tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: LifeSteal closeout is reviewable

r[hyperion_game_modes.lifesteal.validation.log]
- GIVEN LifeSteal work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant heart-state, transfer, final-death, recovery, persistence, admin, positive, negative, and plugin-isolation checks plus Cairn gates and validation
- AND unsupported vanilla-health, combat-parity, production-moderation, Valence, Bedwars, and broad survival claims remain non-claims.
