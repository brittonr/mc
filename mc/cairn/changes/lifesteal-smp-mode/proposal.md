# Proposal: Add LifeSteal SMP mode

## Why

LifeSteal SMP is a creator-driven survival variant with strong public-server presence. It is also a good fit for this codebase because it builds on existing survival/combat rails while adding one distinctive rule: player-vs-player deaths transfer heart capacity according to a configured policy.

A separate Cairn keeps the rule bounded and testable. Heart transfers, minimum/maximum heart limits, final-death handling, persistence, and exploit prevention should not be mixed into vanilla survival compatibility or default Hyperion combat behavior.

## What Changes

- Add a Hyperion-owned LifeSteal SMP plugin with mode-local heart capacity, PvP death attribution, heart transfer, final-death/exclusion policy, recovery policy, player join/leave, and persistence boundaries.
- Keep death attribution, transfer eligibility, heart-cap changes, final-death decisions, recovery grants, and snapshot validation in pure deterministic cores with thin Bevy/combat/storage shells.
- Define named configuration for minimum hearts, maximum hearts, transfer amount, final-death behavior, crafting/trading heart items when scoped, grace periods, and admin repair actions.
- Add positive and negative tests for valid transfers, final-death policy, recovery, persistence restore, malformed attribution, self-kill, environmental death, stale combat tags, duplicate death events, overflow, unauthorized admin actions, and corrupt snapshots.

## Impact

- **Files**: new or extended Hyperion LifeSteal modules under `hyperion/events/`, death/heart snapshot fixtures, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared combat/player-state/storage edits when applicable, pure transfer tests, Bevy shell/plugin tests, persistence fixtures, admin command tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not change vanilla health globally, implement public-server moderation, guarantee anticheat, prove combat parity, change Bedwars, change Valence, or provide broad survival compatibility.
