# Delta: Bounded armor combat reference parity

## Requirements

### Requirement: Armor combat reference contract

r[mc_compatibility.vanilla_combat_armor_reference_parity.contract] The `vanilla-combat-armor-reference-parity` row MUST define a bounded paired-reference evidence contract before any armor combat reference-parity evidence is promoted.

#### Scenario: Contract names one diamond-chestplate melee interaction

r[mc_compatibility.vanilla_combat_armor_reference_parity.contract.scope]
- GIVEN the armor combat reference row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic attacker, victim, iron-sword weapon, diamond-chestplate armor state, no enchantment, no status effect, health delta, knockback metric, tolerance bound, reference oracle, reference version, Valence revision, and client revision
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor loadouts, enchantments/status effects, modifier stacking, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Row-specific Rust parity checker

r[mc_compatibility.vanilla_combat_armor_reference_parity.checker] A Rust checker MUST compare normalized paired Paper-reference and Valence combat metrics through pure deterministic row-specific logic before the armor row is promoted.

#### Scenario: Valid armor paired evidence passes

r[mc_compatibility.vanilla_combat_armor_reference_parity.checker.valid]
- GIVEN a Paper-reference record and a Valence record name `vanilla-combat-armor-reference-parity`, `compatbota`, `compatbotb`, `iron_sword`, `diamond_chestplate`, `20.0`, `15.3`, `4.7`, the reference version, knockback metric, and tolerance bounds
- WHEN the checker compares the records
- THEN it passes only if damage, health, armor state, and knockback are within the configured row contract and bounds.

#### Scenario: Weak or mismatched armor evidence fails closed

r[mc_compatibility.vanilla_combat_armor_reference_parity.checker.rejects]
- GIVEN evidence is missing the reference record, contains only Valence evidence, uses an unknown row, uses the wrong reference version, omits tolerance bounds, reports a stale required revision, reports no armor for the armor row, mismatches weapon or armor state, or reports no-armor damage for the armor row
- WHEN the checker compares the records
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated armor runner rail

r[mc_compatibility.vanilla_combat_armor_reference_parity.rail] The harness MUST expose an isolated `vanilla-combat-armor-reference-parity` rail for the bounded paired armor interaction without broadening existing scenario semantics.

#### Scenario: Existing combat rows remain unchanged

r[mc_compatibility.vanilla_combat_armor_reference_parity.rail.isolated]
- GIVEN existing no-armor reference, CTF combat, knockback, armor mitigation, projectile, and survival rows are maintained separately
- WHEN the armor reference rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the new rail records its own explicit client and server milestones.

### Requirement: Armor reference and Valence fixtures

r[mc_compatibility.vanilla_combat_armor_reference_parity.fixtures] Paper-reference and Valence fixtures MUST record the same normalized server-side combat metrics for the configured diamond-chestplate interaction.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.vanilla_combat_armor_reference_parity.fixtures.comparable]
- GIVEN the configured armor melee interaction runs against Paper-reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for attacker, victim, weapon, armor state, pre-health, post-health, damage delta, knockback metric, tolerance bounds, backend identity, and row id
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable armor paired receipts

r[mc_compatibility.vanilla_combat_armor_reference_parity.receipts] Paired armor combat reference evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.vanilla_combat_armor_reference_parity.receipts.reviewable]
- GIVEN the armor row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper-reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow armor promotion

r[mc_compatibility.vanilla_combat_armor_reference_parity.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `vanilla-combat-armor-reference-parity` row after the paired comparator passes.

#### Scenario: Broad armor parity remains a non-claim

r[mc_compatibility.vanilla_combat_armor_reference_parity.promotion.nonclaims]
- GIVEN the paired Paper-reference and Valence armor evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured diamond-chestplate combat reference row is marked covered
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor loadouts, enchantments/status effects, modifier stacking, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Armor validation and archive evidence

r[mc_compatibility.vanilla_combat_armor_reference_parity.validation] The change MUST record checker, paired comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.vanilla_combat_armor_reference_parity.validation.log]
- GIVEN the armor combat reference row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record the checker self-tests, paired comparator, fixture and runner tests, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
