# Delta: Residual combat non-claim catalog

## Requirements

### Requirement: Residual Scope

r[mc_compatibility.roi_03_residual_combat_catalog.residual_scope] The evidence set MUST explicitly catalog combat semantics still outside the maintained knockback/damage rails.

#### Scenario: Residual Scope evidence is required

r[mc_compatibility.roi_03_residual_combat_catalog.residual_scope.scenario]
- GIVEN `Residual combat non-claim catalog` is drained
- WHEN the evidence and checks are reviewed
- THEN `residual_scope` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Residual Nonclaims

r[mc_compatibility.roi_03_residual_combat_catalog.residual_nonclaims] Projectile and armor/enchantment semantics MUST remain explicit non-claims until backed by receipts.

#### Scenario: Residual Nonclaims evidence is required

r[mc_compatibility.roi_03_residual_combat_catalog.residual_nonclaims.scenario]
- GIVEN `Residual combat non-claim catalog` is drained
- WHEN the evidence and checks are reviewed
- THEN `residual_nonclaims` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant
