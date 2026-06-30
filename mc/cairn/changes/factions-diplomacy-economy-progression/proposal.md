# Proposal: Add factions diplomacy, economy, and progression

## Why

Once clans and claims exist, factions need longer-term reasons to cooperate, compete, trade, declare wars, maintain upkeep, and progress. Diplomacy and economy touch many abuse-prone surfaces: alliances, truces, war declarations, shared vaults, taxes, upkeep, rewards, cooldowns, leader powers, admin intervention, and audit logs.

This Cairn keeps those social/economic systems separate from the roster and territory foundations so each layer can be tested with explicit inputs and fail-closed behavior. It also keeps balance and production moderation claims out of scope until evidence exists.

## What Changes

- Add a Hyperion-owned diplomacy layer for allies, truces, enemies, war declarations, war consent policy, cooldowns, surrender, neutral resets, and relationship audit records.
- Add a faction economy/progression layer for shared vault/account abstractions, deposits, withdrawals, taxes, upkeep, rewards, contribution scoring, rank/progression grants, and bounded admin adjustments.
- Keep relation, wallet, upkeep, reward, and progression decisions in pure cores with Bevy/command/storage shells.
- Add observability, audit, and admin override contracts that are explicit and reversible where possible.
- Require positive and negative tests for valid relations/economy/progression plus malformed, unauthorized, stale, overflow, duplication, cooldown, abuse, and corruption cases.

## Impact

- **Files**: Hyperion factions/clans event or plugin modules under `hyperion/events/`, possible pure-core modules under Hyperion-owned crates only when reused, command/economy/storage fixtures, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared command/storage edits when applicable, focused diplomacy/economy/progression tests, Bevy shell/plugin checks, persistence/accounting fixtures, admin/audit fixture checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement clan social lifecycle, territory claims, raid protection, real-money economy, public-server moderation, anti-fraud guarantees, production balance, Valence behavior, Bedwars behavior, or broad Minecraft compatibility.
