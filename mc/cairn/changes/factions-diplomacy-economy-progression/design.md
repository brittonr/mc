# Design: Add factions diplomacy, economy, and progression

## Context

Diplomacy and economy make factions durable beyond roster and claim ownership. They also create the largest abuse surface: alliance collusion, war-spam, cooldown bypasses, vault theft, currency duplication, tax abuse, reward farming, overflow, stale snapshot spending, and irreversible admin edits. The design needs deterministic cores, explicit audit events, and conservative non-claims.

This layer should consume social-core faction identities and territory/raid facts when available, but it must not redefine rosters or protection policy. It may depend on claim or raid outcomes for rewards and upkeep, yet those outcomes remain inputs to pure economic decisions.

## Decisions

### 1. Model relations as explicit state transitions

**Choice:** Diplomacy uses relation states such as neutral, ally, truce, enemy, declared war, active war, surrendered, and cooldown. Transitions consume actor capabilities, target faction facts, consent policy, cooldown facts, and audit context.

**Rationale:** A relationship edge is a gameplay contract. Pure transition rules make duplicate declarations, one-sided war abuse, stale faction state, and cooldown bypasses testable.

### 2. Separate diplomacy from territory authorization

**Choice:** Diplomacy can influence raid eligibility, friendly fire, shared chat, or visibility only through explicit relation facts consumed by other cores. It does not directly bypass claim protection or mutate territory state.

**Rationale:** Keeping relation state as input prevents alliance or war code from becoming a hidden protection backdoor.

### 3. Use ledger-style economy cores

**Choice:** Vaults/accounts are modeled as deterministic ledger transitions: deposit, withdraw, tax, upkeep, reward, transfer, admin adjustment, and reversal. Each transition records actor, reason, before/after balances, limits, and idempotency keys or event identifiers.

**Rationale:** Currency bugs are hard to repair. Ledger-style pure decisions help reject negative balances, overflow, duplicate events, unauthorized withdrawals, stale snapshots, and replayed rewards.

### 4. Make progression reward sources explicit

**Choice:** Progression grants consume named sources such as participation, defense, raid outcome, upkeep streak, construction, support action, or admin award. Reward caps, cooldowns, decay, and rank unlocks are named config values.

**Rationale:** Progression should motivate play without allowing hidden farms or magic constants. Explicit sources make positive and negative fixtures reviewable.

### 5. Require audit and admin override contracts

**Choice:** Diplomacy, economy, and progression shells emit audit events for accepted and rejected high-impact actions. Admin overrides are separate operations with named capability, reason, target, reversible intent when possible, and non-claim boundaries around public moderation.

**Rationale:** Operators need visibility into disputed actions, but admin power should not silently bypass the same invariants that players rely on.

### 6. Keep persistence and accounting validation fail-closed

**Choice:** Loading relation graphs, ledgers, vaults, tax/upkeep state, and progression snapshots requires pure validation for duplicate edges, impossible balances, invalid schema, unsupported currency, dangling faction ids, stale idempotency keys, and corrupt audit records.

**Rationale:** Economy and diplomacy bugs can affect many players. Bad persisted state should be isolated before live ECS mutation.

## Risks / Trade-offs

- A simple in-game currency may later be replaced by item-backed or external storage. This Cairn should define accounting contracts without real-money or production-banking claims.
- War consent and cooldown policy are balance-sensitive. The contract should name policy inputs and tests, not freeze final tuning.
- Audit records can grow quickly. Start with deterministic fixtures and allow later retention policy Cairns.
- Admin overrides are necessary but risky. Require explicit capabilities, reasons, and audit output instead of hidden command shortcuts.
