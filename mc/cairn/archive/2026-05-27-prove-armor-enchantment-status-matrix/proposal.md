# Proposal: Armor, enchantment, and status modifier proof

## Summary

Create a proof package for armor loadout breadth, enchantment effects, and potion/status-effect combat modifiers beyond the current bounded armor mitigation rail.

## Motivation

The maintained armor equipment mitigation receipt proves one bounded mitigation path. It does not prove all armor loadouts, armor materials, partial/full armor combinations, enchantment modifiers, potion/status effects, or exact parity of modifier stacking.

## Scope

- Define a loadout/modifier matrix for armor materials, armor slots, enchantment representatives, and status-effect representatives.
- Add positive scenarios for selected mitigation/modifier cases with Valence damage calculations and Stevenarella health updates.
- Add negative scenarios that reject missing modifier attribution, wrong loadout, or stale equipment state.
- Keep exact vanilla parity as a separate claim unless this package includes a reference oracle and tolerances.

## Out of scope

- All equipment update observation breadth.
- Full vanilla combat parity unless explicitly added through the vanilla parity Cairn.
- Production PvP readiness.
