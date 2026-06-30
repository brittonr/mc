# hyperion-game-modes Change Spec: Factions diplomacy, economy, and progression

## Requirements

### Requirement: Diplomacy/economy inventory

r[hyperion_game_modes.factions_clans.diplomacy_economy.inventory] Factions diplomacy/economy work MUST inventory Hyperion command, permission, stats, inventory/item, persistence, chat/visibility, audit/logging, event, and schedule seams before implementing relations, vaults, or progression.

#### Scenario: Diplomacy/economy baseline is reviewable

r[hyperion_game_modes.factions_clans.diplomacy_economy.inventory.reviewable]
- GIVEN diplomacy, economy, and progression work is selected
- WHEN reviewers inspect the inventory
- THEN existing command paths, permission surfaces, stats/accounting choices, item or inventory hooks, persistence options, chat/visibility consumers, audit/logging surfaces, and schedule seams are recorded
- AND clan social lifecycle, territory protection, raid correctness, real-money economy, public-server moderation, anti-fraud guarantees, production balance, Bedwars behavior, Valence behavior, and broad compatibility remain explicit non-claims.

### Requirement: Diplomacy/economy scope

r[hyperion_game_modes.factions_clans.diplomacy_economy.scope] Factions diplomacy/economy work MUST be scoped as a Hyperion-owned plugin layer that consumes social identity and territory facts without redefining roster, claim, raid, or protection state.

#### Scenario: Relations and economy consume external facts

r[hyperion_game_modes.factions_clans.diplomacy_economy.scope.consumes_facts]
- GIVEN relation, vault, upkeep, reward, or progression logic needs faction, claim, or raid facts
- WHEN the core evaluates the request
- THEN it consumes explicit social-core and territory-core snapshots as inputs
- AND it does not create a parallel roster, hidden claim table, protection bypass, or raid-resolution path.

### Requirement: Diplomacy relations

r[hyperion_game_modes.factions_clans.diplomacy_economy.relations] Factions diplomacy MUST model neutral, ally, truce, enemy, declared war, active war, surrender, neutral reset, cooldown, consent, and stale-state transitions as deterministic relation decisions.

#### Scenario: Valid relation transition is accepted

r[hyperion_game_modes.factions_clans.diplomacy_economy.relations.valid]
- GIVEN an actor has required capability, source and target factions are valid, consent or declaration policy is satisfied, cooldowns allow the transition, and relation state is current
- WHEN the diplomacy core evaluates the transition
- THEN it returns the new relation state, audit facts, feedback, cooldown updates, and exported relation facts for authorized consumers.

#### Scenario: Invalid relation transition is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.relations.rejects]
- GIVEN an actor lacks capability, targets a missing faction, duplicates a declaration, bypasses cooldown, violates consent policy, uses stale state, declares self-war, or tries to smuggle claim protection changes
- WHEN the diplomacy core evaluates the transition
- THEN it rejects the request deterministically
- AND no relation edge, protection state, raid state, chat route, economy balance, or progression record is mutated.

### Requirement: Diplomacy shell

r[hyperion_game_modes.factions_clans.diplomacy_economy.diplomacy_shell] Factions diplomacy shell systems MUST gather command/event inputs, call pure relation cores, update Bevy state, emit feedback and audit records, and expose relation facts to other systems only through explicit inputs.

#### Scenario: Shell publishes approved relation facts

r[hyperion_game_modes.factions_clans.diplomacy_economy.diplomacy_shell.publishes]
- GIVEN the relation core accepts a diplomacy transition
- WHEN the Bevy/Hyperion shell applies the result
- THEN it updates only relation state, feedback, audit records, persistence queue entries, and exported relation facts returned by the core
- AND claim, raid, chat, visibility, or economy consumers must read those relation facts explicitly.

#### Scenario: Shell contains malformed diplomacy input

r[hyperion_game_modes.factions_clans.diplomacy_economy.diplomacy_shell.rejects]
- GIVEN command input references a disconnected actor, stale faction, malformed target, disabled plugin, stale relation revision, or unauthorized admin path
- WHEN diplomacy shell systems run
- THEN no unapproved relation mutation is applied
- AND diagnostics are emitted according to configured feedback policy.

### Requirement: Ledger accounting

r[hyperion_game_modes.factions_clans.diplomacy_economy.ledger] Factions economy MUST model vault deposits, withdrawals, transfers, taxes, upkeep, rewards, admin adjustments, reversals, idempotency, bounds, and balances through deterministic ledger decisions.

#### Scenario: Valid ledger transition is accepted

r[hyperion_game_modes.factions_clans.diplomacy_economy.ledger.valid]
- GIVEN an actor or system source has required capability, account snapshots are current, amount and currency are valid, limits allow the operation, and the event identifier is not already applied
- WHEN the ledger core evaluates the operation
- THEN it returns before/after balances, ledger entries, audit facts, feedback, and persistence intents.

#### Scenario: Invalid ledger transition is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.ledger.rejects]
- GIVEN an operation has a negative amount, overflowing balance, unknown currency, unauthorized actor, stale snapshot, insufficient funds, duplicated event identifier, malformed account, replayed reward, or invalid reversal
- WHEN the ledger core evaluates the operation
- THEN it rejects the operation deterministically
- AND no balance, vault, reward, progression, persistence, or audit-success mutation is applied.

### Requirement: Economy shell

r[hyperion_game_modes.factions_clans.diplomacy_economy.economy_shell] Factions economy shell systems MUST gather command, reward, upkeep, tax, storage, and admin inputs, call pure ledger cores, apply returned state, queue persistence, and emit accepted/rejected audit records.

#### Scenario: Shell applies ledger-approved state

r[hyperion_game_modes.factions_clans.diplomacy_economy.economy_shell.applies]
- GIVEN the ledger core returns an approved account transition
- WHEN the economy shell system runs
- THEN it mutates only the account resources, vault components, feedback, audit records, metrics, and persistence queue entries returned by the core
- AND clocks, command parsing, storage writes, logs, and external IO remain shell responsibilities.

#### Scenario: Shell rejects stale account state

r[hyperion_game_modes.factions_clans.diplomacy_economy.economy_shell.rejects]
- GIVEN economy input references a stale revision, missing account, disconnected actor, disabled plugin, malformed reward source, or unsupported currency
- WHEN the economy shell system runs
- THEN no unapproved balance or progression mutation is applied
- AND the rejection is visible through diagnostics or audit records.

### Requirement: Progression rewards

r[hyperion_game_modes.factions_clans.diplomacy_economy.progression] Factions progression MUST grant contribution, rank, reward, upkeep streak, defense, raid-outcome, construction, support-action, decay, and cooldown effects only from explicit sources and named configuration values.

#### Scenario: Valid progression grant is accepted

r[hyperion_game_modes.factions_clans.diplomacy_economy.progression.valid]
- GIVEN a progression source is supported, faction and actor snapshots are current, caps and cooldowns allow the grant, and the event identifier has not already been applied
- WHEN the progression core evaluates the grant
- THEN it returns bounded contribution, rank, reward, cooldown, audit, and feedback changes.

#### Scenario: Progression abuse is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.progression.rejects]
- GIVEN a grant uses an unsupported source, duplicates an event identifier, exceeds caps, bypasses cooldowns, references stale faction state, fabricates raid or defense facts, or overflows progression totals
- WHEN the progression core evaluates the grant
- THEN it rejects the grant deterministically
- AND no reward, rank, contribution, economy, or audit-success mutation is applied.

### Requirement: Admin audit and observability

r[hyperion_game_modes.factions_clans.diplomacy_economy.admin_audit] Factions diplomacy/economy work MUST define audit and observability contracts for high-impact accepted and rejected actions, manual adjustments, reversible intents, permission checks, and operator summaries.

#### Scenario: Admin adjustment is explicit

r[hyperion_game_modes.factions_clans.diplomacy_economy.admin_audit.adjustment]
- GIVEN an authorized admin performs a relation, balance, progression, or cooldown adjustment with a configured reason
- WHEN the admin core and shell process the adjustment
- THEN the operation records actor, target, reason, before/after facts, reversible intent where supported, and audit visibility
- AND normal player authorization paths remain unchanged.

#### Scenario: Invalid admin path is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.admin_audit.rejects]
- GIVEN an actor lacks admin capability, omits a required reason, targets a missing faction, requests an irreversible operation without policy, or attempts to hide audit output
- WHEN admin adjustment validation runs
- THEN the operation is rejected deterministically
- AND no hidden relation, balance, progression, or cooldown mutation occurs.

### Requirement: Diplomacy/economy persistence

r[hyperion_game_modes.factions_clans.diplomacy_economy.persistence] Factions diplomacy/economy persistence MUST validate relation graphs, ledgers, vaults, tax/upkeep state, progression records, idempotency keys, schema revisions, and corrupt audit records before live mutation.

#### Scenario: Valid economic snapshot round trips

r[hyperion_game_modes.factions_clans.diplomacy_economy.persistence.round_trip]
- GIVEN a valid snapshot with relations, account balances, ledger entries, tax/upkeep state, progression records, idempotency keys, schema revision, and audit metadata
- WHEN serialization and loading run
- THEN the loaded state preserves relation edges, balances, applied events, progression facts, schema, and audit records.

#### Scenario: Corrupt economic snapshot is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.persistence.rejects]
- GIVEN a snapshot has duplicate relation edges, dangling faction ids, impossible balances, invalid currencies, missing ledger entries, stale idempotency keys, unsupported schema, invalid progression totals, or corrupt audit records
- WHEN the loader validates it
- THEN the snapshot is rejected before live ECS mutation
- AND recovery diagnostics identify the invalid field or relationship.

### Requirement: Diplomacy/economy tests

r[hyperion_game_modes.factions_clans.diplomacy_economy.tests] Factions diplomacy/economy work MUST include positive tests for valid relations, ledger operations, progression, admin/audit, and persistence plus negative tests for unauthorized, duplicate, stale, overflow, replay, corruption, cooldown, bypass, and leak cases.

#### Scenario: Positive diplomacy/economy behavior is covered

r[hyperion_game_modes.factions_clans.diplomacy_economy.tests.positive]
- GIVEN representative valid relation, wallet, vault, upkeep, tax, reward, progression, admin, audit, and persistence inputs
- WHEN pure cores and focused shell tests run
- THEN supported alliance, truce, war declaration, surrender, cooldown, deposit, withdrawal, tax, upkeep, reward, progression, admin adjustment, audit, and snapshot round-trip outcomes pass.

#### Scenario: Negative diplomacy/economy behavior fails closed

r[hyperion_game_modes.factions_clans.diplomacy_economy.tests.negative]
- GIVEN unauthorized relation changes, duplicate declarations, cooldown bypasses, stale snapshots, alliance protection bypass attempts, negative balances, overflow balances, duplicated rewards, replayed event identifiers, invalid admin overrides, corrupt ledgers, dangling factions, malformed progression, hidden audit attempts, or leak fixtures
- WHEN pure cores and focused shell tests run
- THEN diplomacy/economy logic rejects or diagnoses each case deterministically
- AND no unauthorized relation, protection bypass, currency duplication, hidden admin mutation, progression exploit, audit leak, panic, or cross-mode mutation occurs.

### Requirement: Diplomacy/economy validation

r[hyperion_game_modes.factions_clans.diplomacy_economy.validation] Factions diplomacy/economy work MUST record baseline checks before shared Hyperion command/storage edits when applicable, focused diplomacy/economy/progression tests from `hyperion/`, Bevy shell/plugin checks, persistence/accounting fixtures, admin/audit fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Diplomacy/economy closeout is reviewable

r[hyperion_game_modes.factions_clans.diplomacy_economy.validation.log]
- GIVEN factions diplomacy/economy work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show relevant baseline checks, positive and negative diplomacy/economy/progression tests, shell/plugin checks, persistence/accounting fixture checks, admin/audit fixture checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
