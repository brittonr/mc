# hyperion-game-modes Change Spec: Murder Mystery social-deduction mode

## Requirements

### Requirement: Murder Mystery scope

r[hyperion_game_modes.murder_mystery.scope] Murder Mystery work MUST be scoped as a Hyperion-owned optional social-deduction plugin with mode-local role, visibility, item, kill, timer, victory, spectator, cleanup, configuration, and diagnostic state using original presentation.

#### Scenario: Social-deduction mode is isolated

r[hyperion_game_modes.murder_mystery.scope.isolated]
- GIVEN the Murder Mystery plugin is enabled in a Hyperion app
- WHEN other modes or default server behavior are active
- THEN Murder Mystery systems mutate only mode-owned roles, players, items, visibility state, timers, spectators, scores, and diagnostics
- AND Hypixel presentation, protected maps, production moderation, anti-stream-sniping, Valence behavior, Bedwars behavior, and public-server safety remain explicit non-claims.

### Requirement: Role assignment and visibility

r[hyperion_game_modes.murder_mystery.role_visibility] Murder Mystery MUST decide role assignment, role reveal policy, visibility filtering, party/team avoidance where scoped, and player-facing output authorization through pure cores before shell systems send UI, chat, scoreboard, item, or diagnostic output.

#### Scenario: Roles are assigned without leaks

r[hyperion_game_modes.murder_mystery.role_visibility.assign]
- GIVEN a valid player set and configured role distribution policy
- WHEN the role core assigns roles and visibility facts
- THEN it returns role assignments, per-recipient visibility summaries, and reveal constraints
- AND shell code sends only information allowed by the visibility core.

#### Scenario: Hidden role leak is rejected

r[hyperion_game_modes.murder_mystery.role_visibility.leak]
- GIVEN a scoreboard, chat, action bar, diagnostic, item name, spectator, or command output would reveal a hidden role to an unauthorized recipient
- WHEN the visibility core evaluates the output request
- THEN it rejects or redacts the output deterministically
- AND no hidden role, target identity, or private team fact is leaked.

### Requirement: Item and kill policy

r[hyperion_game_modes.murder_mystery.item_kill_policy] Murder Mystery MUST validate asymmetric item grants, item uses, kill attempts, interaction attempts, cooldowns, stale state, and spectator restrictions through pure mode-local policies.

#### Scenario: Valid role action is allowed

r[hyperion_game_modes.murder_mystery.item_kill_policy.valid]
- GIVEN a mode-scoped player has a role, item eligibility, active phase, and valid target or interaction facts
- WHEN the item or kill policy core evaluates the action
- THEN it returns an allow decision with bounded inventory, cooldown, kill, reveal, or feedback plans
- AND shell code applies only those plans.

#### Scenario: Invalid role action fails closed

r[hyperion_game_modes.murder_mystery.item_kill_policy.invalid]
- GIVEN a player uses a wrong-role item, targets an invalid player, acts during wrong phase, has stale state, is disconnected, is a spectator, or repeats a cooldown-gated action
- WHEN the item or kill policy core evaluates the action
- THEN it rejects the action with deterministic diagnostics
- AND no kill, item grant, cooldown bypass, role reveal, or score mutation occurs.

### Requirement: Victory and cleanup policy

r[hyperion_game_modes.murder_mystery.victory_cleanup] Murder Mystery MUST determine timer state, innocent win, murderer win, detective-like role outcomes, disconnect handling, spectator transitions, and cleanup through deterministic mode-local policies.

#### Scenario: Victory is deterministic

r[hyperion_game_modes.murder_mystery.victory_cleanup.win]
- GIVEN an active round has current alive roles, timer state, disconnect facts, spectator state, and configured win policy
- WHEN the victory core evaluates round state
- THEN it returns no-win, innocent-side win, murderer-side win, or configured special outcome with feedback plans
- AND stale or duplicate events do not produce conflicting winners.

#### Scenario: Cleanup removes hidden state

r[hyperion_game_modes.murder_mystery.victory_cleanup.cleanup]
- GIVEN a round ends, cancels, or fails during setup
- WHEN cleanup planning and shell cleanup run
- THEN role assignments, hidden visibility state, role items, cooldowns, spectators, scoreboard rows, timers, and diagnostics are cleared according to the cleanup plan
- AND no hidden role data remains visible to later rounds or unrelated modes.

### Requirement: Murder Mystery tests

r[hyperion_game_modes.murder_mystery.tests] Murder Mystery work MUST include positive tests for valid role assignment, hidden information, item grants, valid kills, innocent-side win, murderer-side win, spectator transitions, and cleanup plus negative tests for role leaks, invalid kills, duplicate grants, stale players, disconnects, spectator actions, wrong-mode interactions, and cleanup leaks.

#### Scenario: Positive social-deduction behavior is covered

r[hyperion_game_modes.murder_mystery.tests.positive]
- GIVEN valid players, role configuration, visibility requests, item grants, kill actions, timer facts, victory facts, and cleanup inputs
- WHEN pure-core and focused shell tests run
- THEN role assignment, authorized visibility, role item grants, valid kills, victory outcomes, spectator transitions, and cleanup pass.

#### Scenario: Negative social-deduction behavior fails closed

r[hyperion_game_modes.murder_mystery.tests.negative]
- GIVEN hidden-role leak attempts, invalid kills, duplicate grants, stale players, disconnected players, spectator actions, wrong-mode interactions, malformed visibility requests, or cleanup edge cases
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or redacted
- AND no panic, unauthorized kill, role leak, item duplication, conflicting winner, or hidden-state leak occurs.

### Requirement: Murder Mystery validation

r[hyperion_game_modes.murder_mystery.validation] Murder Mystery work MUST record focused Hyperion checks, role and victory tests, shell/plugin tests, hidden-information leak tests, cleanup fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Murder Mystery closeout is reviewable

r[hyperion_game_modes.murder_mystery.validation.log]
- GIVEN Murder Mystery work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant role, visibility, item, kill, victory, cleanup, positive, negative, and plugin-isolation checks plus Cairn gates and validation
- AND unsupported Hypixel, protected-presentation, production-moderation, anti-stream-sniping, Valence, Bedwars, and public-server safety claims remain non-claims.
