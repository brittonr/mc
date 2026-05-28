# Proposal: Vanilla combat parity proof

## Summary

Create a proof package for exact vanilla damage, knockback, and mitigation parity between Valence behavior observed by Stevenarella and a named vanilla/reference oracle.

## Motivation

Current combat receipts prove bounded Valence/Stevenarella correlations for damage, knockback, armor mitigation, and projectile damage attribution. They explicitly do not prove exact vanilla balancing or parity. A parity claim needs a reference oracle, tolerance policy, repeated measurements, and negative checks that prevent Valence-only evidence from being promoted as vanilla equivalence.

## Scope

- Select and document a reference oracle for vanilla combat behavior.
- Define parity metrics and tolerances for melee damage, knockback, armor mitigation, and projectile damage where claimed.
- Add deterministic positive/negative fixtures for parity comparison logic.
- Produce paired Valence/reference receipts before promoting any parity row.

## Out of scope

- Full combat breadth, all equipment/enchantment/status combinations, or production PvP readiness unless separately covered.
- Claiming parity from existing Valence-only receipts.
