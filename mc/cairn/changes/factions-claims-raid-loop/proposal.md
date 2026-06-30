# Proposal: Add factions claims and raid loop

## Why

Traditional factions gameplay is defined by owned territory, protected building, contested raid windows, and recoverable world damage. Those rules should not be bundled into the initial clan roster work because claims require different invariants: chunk or region ownership, overlap policy, protection checks on block and inventory actions, siege timing, raid scoring, rollback or repair, and admin observability.

A dedicated Cairn keeps territory and raiding reviewable while depending on the social core for faction identity and permissions. It also prevents accidental claims that Hyperion or Valence defaults have become production-safe faction servers.

## What Changes

- Add a Hyperion-owned factions territory layer for claim geometry, ownership, power/upkeep hooks, protection policy, build/break/container/interaction gates, raid eligibility, siege windows, raid resolution, and cleanup.
- Use pure cores for claim validation, overlap checks, protection decisions, raid window eligibility, siege scoring, and repair plans.
- Wire Bevy shell systems around block, container, entity, and command events without changing Bedwars, Valence, or default non-factions behavior.
- Add deterministic map/visibility summaries for claims and contested areas without making full UI or scoreboard parity claims.
- Require positive and negative tests for valid claims, overlaps, unauthorized mutation, stale faction state, wrong-mode actions, raid timing, exploit-shaped block updates, disconnects, rollback, and cleanup.

## Impact

- **Files**: Hyperion factions/clans event or plugin modules under `hyperion/events/`, possible generic world-action guard seams under Hyperion-owned crates if justified, claim/raid pure-core tests, shell/plugin tests, map/visibility fixtures, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared world-action edits when applicable, focused claim/protection/raid tests, Bevy schedule/plugin checks, map/visibility checks, rollback/repair fixtures, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement clan social lifecycle, diplomacy, economy, public-server anti-grief guarantees, adversarial anti-cheat, production rollback storage, vanilla terrain parity, Valence behavior, Bedwars behavior, or broad Minecraft compatibility.
