# Delta: Projectile hit rail

## Requirements

### Requirement: Projectile Scenario

r[mc_compatibility.roi_03_projectile_hit_rail.projectile_scenario] The repo MUST define a bounded two-client projectile scenario with deterministic actor roles, owned local Valence CTF target, and explicit projectile-use milestones.

#### Scenario: Projectile Scenario evidence is required

r[mc_compatibility.roi_03_projectile_hit_rail.projectile_scenario.scenario]
- GIVEN `Projectile hit rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `projectile_scenario` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Hit Attribution

r[mc_compatibility.roi_03_projectile_hit_rail.hit_attribution] The rail MUST correlate projectile spawn/travel or action evidence with server-observed hit attribution and client-observed health/damage impact.

#### Scenario: Hit Attribution evidence is required

r[mc_compatibility.roi_03_projectile_hit_rail.hit_attribution.scenario]
- GIVEN `Projectile hit rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `hit_attribution` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Receipt Nonclaims

r[mc_compatibility.roi_03_projectile_hit_rail.receipt_nonclaims] The rail MUST preserve non-claims for full projectile physics, all weapons, enchantments/status effects, production load, and broad protocol compatibility.

#### Scenario: Receipt Nonclaims evidence is required

r[mc_compatibility.roi_03_projectile_hit_rail.receipt_nonclaims.scenario]
- GIVEN `Projectile hit rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `receipt_nonclaims` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Dry Run Check

r[mc_compatibility.roi_03_projectile_hit_rail.dry_run_check] A deterministic dry-run check MUST validate projectile scenario shape, expected packet summary, required milestones, and non-claims before live runs are required.

#### Scenario: Dry Run Check evidence is required

r[mc_compatibility.roi_03_projectile_hit_rail.dry_run_check.scenario]
- GIVEN `Projectile hit rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `dry_run_check` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant
