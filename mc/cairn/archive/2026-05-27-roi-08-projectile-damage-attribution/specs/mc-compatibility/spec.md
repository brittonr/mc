# Delta: Projectile damage attribution rail

## Requirements

### Requirement: Feasibility checkpoint

r[mc_compatibility.roi_08_projectile_damage_attribution.feasibility_checkpoint] The change MUST record a checkpoint that identifies whether existing Stevenarella and Valence CTF instrumentation can support a bounded projectile collision/damage attribution claim.

#### Scenario: Feasibility is decided before implementation

r[mc_compatibility.roi_08_projectile_damage_attribution.feasibility_checkpoint.scenario]
- GIVEN the prior projectile rail only proves use/swing and server loadout
- WHEN this change begins implementation
- THEN the runner and Valence CTF evidence surfaces are inspected first
- AND the checkpoint records the question, inspected evidence, decision owner, decision, and next action

### Requirement: Dry-run gate

r[mc_compatibility.roi_08_projectile_damage_attribution.dry_run_gate] If the rail is feasible, a deterministic dry-run gate MUST exist before any live run is claimed.

#### Scenario: Dry-run exists before live evidence

r[mc_compatibility.roi_08_projectile_damage_attribution.dry_run_gate.scenario]
- GIVEN a projectile damage attribution scenario is added
- WHEN local validation runs
- THEN a Nix dry-run check exercises the scenario receipt shape without requiring live Minecraft clients
- AND the dry-run receipt preserves explicit non-claims

### Requirement: Live receipt or explicit non-claim

r[mc_compatibility.roi_08_projectile_damage_attribution.live_receipt_or_nonclaim] The change MUST either produce a tracked live receipt with correlated client/server damage attribution milestones or preserve an explicit non-claim explaining why the rail is not yet supportable.

#### Scenario: Claim requires correlated evidence

r[mc_compatibility.roi_08_projectile_damage_attribution.live_receipt_or_nonclaim.scenario]
- GIVEN a live projectile damage attribution receipt is claimed
- WHEN reviewers inspect the receipt
- THEN client-side and server-side projectile damage/collision milestones are both present
- AND missing milestones, forbidden matches, BLAKE3 evidence, and scoped non-claims are reviewable under `docs/evidence`
