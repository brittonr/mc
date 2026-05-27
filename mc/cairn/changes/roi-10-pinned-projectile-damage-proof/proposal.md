# Proposal: Pinned projectile damage proof

## Summary

Re-scope projectile damage attribution as a fresh proof that pins the Valence server instrumentation and requires causally ordered client/server evidence before any maintained acceptance claim is restored.

## Motivation

ROI 08 was demoted because it used `VALENCE_REV=HEAD` and accepted milestone presence rather than proving the client projectile action, server attribution, and client health update belonged to the same ordered event. The residual combat catalog now ranks projectile damage attribution as the next drainable seam, but only after dependency and causality gaps are closed.

## Scope

- Record a reviewable dependency checkpoint for the exact Valence instrumentation commit used by the proof.
- Make the runner reject projectile damage receipts unless client projectile use/swing, server projectile use/hit, and client damage update are causally ordered.
- Add deterministic negative tests for out-of-order or missing projectile milestones.
- Produce dry-run and live evidence only after the pinned dependency and ordering gate pass.
- Re-promote the matrix/bundle row only after the proof is reviewable and manifest-checked.

## Out of scope

- Full projectile physics, travel, collision simulation, all projectile weapons, enchantments/status effects, production PvP readiness, or vanilla-exact balancing.
- Treating ROI 08 artifacts as accepted evidence without a fresh proof.
