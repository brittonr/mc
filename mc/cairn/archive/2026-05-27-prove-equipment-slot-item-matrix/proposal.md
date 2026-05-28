# Proposal: Equipment slot and item matrix proof

## Summary

Create a proof package for equipment-update breadth: all relevant equipment slots, item types, and packet permutations beyond the current single equipment observation rail.

## Motivation

The maintained equipment update observation receipt proves one bounded remote-player equipment update path. It does not prove all equipment slots, item categories, update orderings, packet permutations, or client/server consistency across repeated equipment changes.

## Scope

- Define an equipment matrix covering armor slots, main hand, off hand, empty slots, item category representatives, and repeated update permutations.
- Add positive scenarios for valid equipment changes observed by Stevenarella and correlated with Valence state.
- Add negative scenarios for stale, missing, duplicate, or contradictory equipment update evidence.
- Promote only matrix rows with tracked receipts and BLAKE3 manifests.

## Out of scope

- Armor mitigation, enchantment/status effects, exact vanilla combat balancing, and full inventory semantics.
