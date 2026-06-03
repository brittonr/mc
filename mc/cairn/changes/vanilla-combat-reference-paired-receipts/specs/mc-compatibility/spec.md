# Delta: Paired vanilla combat reference receipts

## Requirements

### Requirement: Paired combat reference contract

r[mc_compatibility.vanilla_combat_reference_paired_receipts.contract] The `vanilla-combat-reference-parity` row MUST define a bounded paired-reference evidence contract before any combat reference-parity evidence is promoted.

#### Scenario: Contract names one bounded melee interaction

r[mc_compatibility.vanilla_combat_reference_paired_receipts.contract.scope]
- GIVEN the combat reference row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic attacker, victim, weapon, armor state, health delta, knockback metric, tolerance bound, reference oracle, reference version, Valence revision, and client revision
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Rust parity checker

r[mc_compatibility.vanilla_combat_reference_paired_receipts.checker] A Rust checker MUST compare normalized paired reference and Valence combat metrics through pure deterministic logic before the row is promoted.

#### Scenario: Valid paired evidence passes

r[mc_compatibility.vanilla_combat_reference_paired_receipts.checker.valid]
- GIVEN a Paper-reference record and a Valence record name the same row, attacker, victim, weapon, armor state, reference version, damage metric, knockback metric, and tolerance bounds
- WHEN the checker compares the records
- THEN it passes only if damage and knockback are within the configured bounds.

#### Scenario: Weak or mismatched evidence fails closed

r[mc_compatibility.vanilla_combat_reference_paired_receipts.checker.rejects]
- GIVEN evidence is missing the reference record, contains only Valence evidence, uses the wrong reference version, omits tolerance bounds, exceeds damage tolerance, exceeds knockback tolerance, reports a stale required revision, or mismatches weapon or armor state
- WHEN the checker compares the records
- THEN it fails and names the first missing or mismatched metric.

### Requirement: Isolated runner rail

r[mc_compatibility.vanilla_combat_reference_paired_receipts.rail] The harness MUST expose an isolated `vanilla-combat-reference-parity` rail for the bounded paired combat interaction without broadening existing scenario semantics.

#### Scenario: Existing combat rows remain unchanged

r[mc_compatibility.vanilla_combat_reference_paired_receipts.rail.isolated]
- GIVEN existing CTF combat, knockback, armor, projectile, and survival rows are maintained separately
- WHEN the paired combat reference rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the new rail records its own explicit client and server milestones.

### Requirement: Reference and Valence fixtures

r[mc_compatibility.vanilla_combat_reference_paired_receipts.fixtures] Paper-reference and Valence fixtures MUST record the same normalized server-side combat metrics for the configured interaction.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.vanilla_combat_reference_paired_receipts.fixtures.comparable]
- GIVEN the configured melee interaction runs against Paper-reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for attacker, victim, weapon, armor state, pre-health, post-health, damage delta, knockback metric, tolerance bounds, and backend identity
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable paired receipts

r[mc_compatibility.vanilla_combat_reference_paired_receipts.receipts] Paired combat reference evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.vanilla_combat_reference_paired_receipts.receipts.reviewable]
- GIVEN the paired row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper-reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and any oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow promotion

r[mc_compatibility.vanilla_combat_reference_paired_receipts.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `vanilla-combat-reference-parity` row after the paired comparator passes.

#### Scenario: Broad parity remains a non-claim

r[mc_compatibility.vanilla_combat_reference_paired_receipts.promotion.nonclaims]
- GIVEN the paired Paper-reference and Valence evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured combat reference-parity row is marked covered
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Validation and archive evidence

r[mc_compatibility.vanilla_combat_reference_paired_receipts.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.vanilla_combat_reference_paired_receipts.validation.log]
- GIVEN the paired combat reference row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record the checker self-tests, paired comparator, maintained dry-runs where applicable, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
