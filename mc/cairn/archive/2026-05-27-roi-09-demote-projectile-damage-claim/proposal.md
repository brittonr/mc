# Proposal: Demote projectile damage claim

## Summary

Demote the ROI 08 projectile damage attribution row from maintained acceptance evidence until its Valence dependency and client/server causality proof are reviewable.

## Motivation

Same-family review found the ROI 08 claim depended on Valence server instrumentation at `HEAD`/`e5d18ad` without a repo-local dependency checkpoint and that the live receipt only proved milestone presence, not causal ordering between projectile use/hit and client health update. Keeping the row in the acceptance matrix overstates evidence quality.

## Scope

- Remove projectile damage attribution from accepted matrix/bundle rows.
- Move projectile damage attribution back into residual/next work as blocked on pinned Valence and causality proof.
- Record a repo-local oracle checkpoint for the demotion decision.
- Preserve tracked ROI 08 receipt/log artifacts as experimental evidence, clearly superseded and non-claiming.
- Update local checkers to expect the demoted row count.

## Non-goals

- No live rerun.
- No Valence instrumentation change.
- No claim that existing ROI 08 artifacts prove projectile damage attribution.
