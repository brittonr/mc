# Delta: Demote projectile damage claim

## Requirements

### Requirement: Claim demoted

r[mc_compatibility.roi_09_demote_projectile_damage_claim.claim_demoted] Projectile damage attribution MUST NOT appear as a maintained accepted matrix/bundle row until pinned Valence dependency and causal client/server correlation evidence exist.

#### Scenario: Matrix no longer overclaims damage attribution

r[mc_compatibility.roi_09_demote_projectile_damage_claim.claim_demoted.scenario]
- GIVEN ROI 08 evidence is not review-sufficient
- WHEN an operator reads the acceptance matrix and current evidence bundle
- THEN projectile damage attribution is absent from maintained evidence rows
- AND residual docs describe it as blocked follow-up work

### Requirement: Blocker checkpoint

r[mc_compatibility.roi_09_demote_projectile_damage_claim.blocker_checkpoint] The repository MUST record a checkpoint explaining why ROI 08 artifacts are demoted and what evidence is required before re-promotion.

#### Scenario: Blocker checkpoint is reviewable

r[mc_compatibility.roi_09_demote_projectile_damage_claim.blocker_checkpoint.scenario]
- GIVEN a reviewer asks why projectile damage attribution is no longer accepted
- WHEN they inspect `docs/evidence`
- THEN a checkpoint records the question, inspected evidence, decision owner, decision, and next action
- AND it identifies the missing pinned Valence dependency and causal ordering proof

### Requirement: Checker alignment

r[mc_compatibility.roi_09_demote_projectile_damage_claim.checker_alignment] Local matrix and bundle checkers MUST match the demoted maintained evidence row count.

#### Scenario: Checkers match demotion

r[mc_compatibility.roi_09_demote_projectile_damage_claim.checker_alignment.scenario]
- GIVEN projectile damage attribution is demoted
- WHEN matrix and bundle checkers run
- THEN they expect the maintained row count without the demoted row
- AND they continue to validate receipt paths, BLAKE3 values, and non-claims for remaining rows
