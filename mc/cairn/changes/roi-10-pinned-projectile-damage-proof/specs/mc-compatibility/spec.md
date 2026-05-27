# Delta: Pinned projectile damage proof

## ADDED Requirements

### Requirement: Dependency checkpoint

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.dependency_checkpoint] The projectile damage attribution proof MUST record the exact Valence server instrumentation dependency before any live evidence is promoted.

#### Scenario: Dependency is pinned and reviewable

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.dependency_checkpoint.scenario]
- GIVEN projectile damage attribution is being re-scoped after ROI 08 demotion
- WHEN the change records its dependency checkpoint
- THEN the checkpoint identifies the Valence commit, clean worktree status, decision owner, decision, and next action
- AND it forbids using `VALENCE_REV=HEAD` as promoted evidence

### Requirement: Causal runner gate

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.causal_runner_gate] The runner MUST validate projectile damage attribution by ordered client/server evidence, not by unordered milestone presence.

#### Scenario: Ordered proof is required

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.causal_runner_gate.scenario]
- GIVEN a projectile damage attribution run emits client and server logs
- WHEN the runner evaluates the receipt
- THEN it requires client projectile use/swing, server projectile use/hit for the same attacker/victim, and client damage update in causal order
- AND missing, mismatched, or out-of-order milestones fail with explicit diagnostics

### Requirement: Dry-run evidence

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.dry_run_evidence] The change MUST provide a deterministic dry-run gate that exercises the pinned dependency fields and causal ordering contract without requiring a live server.

#### Scenario: Dry-run rejects weak evidence

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.dry_run_evidence.scenario]
- GIVEN the projectile damage attribution dry-run is executed
- WHEN fixture logs contain unordered damage evidence or missing server attribution
- THEN tests or checks reject the receipt
- AND the passing dry-run receipt preserves explicit non-claims for full projectile physics and full combat correctness

### Requirement: Live receipt and promotion

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.live_receipt_and_promotion] The projectile damage attribution row MUST remain absent from maintained matrix/bundle evidence until a pinned live receipt and BLAKE3-backed docs satisfy the causal proof.

#### Scenario: Re-promotion is evidence gated

r[mc_compatibility.roi_10_pinned_projectile_damage_proof.live_receipt_and_promotion.scenario]
- GIVEN dry-run and runner tests pass
- WHEN a live receipt is produced with the pinned Valence commit and ordered projectile proof
- THEN receipt, log, evidence summary, and BLAKE3 sidecars are tracked under `docs/evidence/`
- AND only then may acceptance matrix, current bundle, residual catalog, and checkers re-add projectile damage attribution as a maintained row
