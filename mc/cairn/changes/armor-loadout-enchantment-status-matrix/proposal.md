# Proposal: Armor loadout enchantment status matrix rail

## Why

Current armor evidence covers one diamond chestplate without enchantments or status effects. Armor loadouts, enchantments, status modifiers, and stacking remain non-claims.

## What Changes

- Add a bounded `armor-loadout-enchantment-status-matrix` row for a bounded table of configured armor loadout, enchantment, status-effect, attack type, and expected mitigation rows.
- Define normalized metrics: loadout id, equipment slots, enchantment ids/levels, status effects, attack type, pre/post health, damage delta, mitigation delta, and tolerance fields.
- Require evidence standard: matrix checker with positive and negative rows plus paired reference evidence for any vanilla-parity label.
- Add fixture/runner/checker work: fixtures equip attacker/victim states from a declarative matrix and log normalized damage mitigation metrics for each row.
- Reject overclaims and bad evidence: missing matrix row fields, missing equipment evidence, mismatched damage delta, absent tolerance, unpaired vanilla parity, or all-loadout overclaim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: residual combat breadth.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all armor permutations, all enchantments, all status effects, exact vanilla balancing outside listed rows, production readiness, and full combat correctness.
