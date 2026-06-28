# mc-compatibility Change Spec: Projectile travel/collision live rail

## Requirements

### Requirement: Projectile live rail inventory

r[mc_compatibility.projectile_travel_collision_live_rail.inventory] The change MUST inventory existing projectile use/loadout evidence, projectile damage-attribution evidence, residual projectile non-claims, and candidate live signals before selecting a travel/collision row.

#### Scenario: Existing projectile boundaries are visible

r[mc_compatibility.projectile_travel_collision_live_rail.inventory.reviewable]
- GIVEN projectile travel/collision work begins
- WHEN reviewers inspect the inventory
- THEN it names covered projectile rows, residual non-claims, candidate travel/collision signals, and the selected bounded row.

### Requirement: Focused projectile matrix row

r[mc_compatibility.projectile_travel_collision_live_rail.matrix] The selected live rail MUST define one focused projectile matrix row with weapon representative, projectile representative, attacker identity, target or collision identity, ordered observations, comparison rule, and explicit non-claims.

#### Scenario: Row scopes the projectile claim

r[mc_compatibility.projectile_travel_collision_live_rail.matrix.scoped]
- GIVEN the projectile matrix row is reviewed
- WHEN the selected row is inspected
- THEN it names the projectile representative, weapon representative, attacker, target or collision identity, required server events, required client observations, and non-claim labels
- AND unselected projectile weapons and collision surfaces remain non-claims.

### Requirement: Pure projectile comparator

r[mc_compatibility.projectile_travel_collision_live_rail.comparator] Projectile travel/collision comparison MUST be a pure deterministic core over normalized records and MUST fail closed for missing travel, missing collision, wrong target, wrong weapon, unordered sequence, ambiguous projectile identity, or overbroad parity claims.

#### Scenario: Complete projectile row passes

r[mc_compatibility.projectile_travel_collision_live_rail.comparator.positive]
- GIVEN normalized server and client records contain the selected projectile launch, ordered travel observation, collision or hit result, attacker, target, and weapon representative
- WHEN the comparator evaluates the row
- THEN comparison passes with stable diagnostics.

#### Scenario: Weak projectile row fails

r[mc_compatibility.projectile_travel_collision_live_rail.comparator.negative]
- GIVEN projectile records are missing travel observation, missing collision result, unordered, wrong-target, wrong-weapon, ambiguous, or claiming broad vanilla physics
- WHEN the comparator evaluates the row
- THEN comparison fails and names the missing or invalid evidence.

### Requirement: Projectile live rail wiring

r[mc_compatibility.projectile_travel_collision_live_rail.wiring] Runner, Stevenarella probe, and Valence fixture shells MUST emit typed projectile metrics for the selected owned-local live row while preserving existing combat scenario names, receipt behavior, and non-claims.

#### Scenario: Existing combat rails remain stable

r[mc_compatibility.projectile_travel_collision_live_rail.wiring.compatible]
- GIVEN the new projectile rail is wired
- WHEN existing combat damage, knockback, projectile use/loadout, and damage-attribution scenarios run or dry-run
- THEN their scenario names, wrapper behavior, and bounded non-claim fields remain compatible.

### Requirement: Projectile live evidence

r[mc_compatibility.projectile_travel_collision_live_rail.evidence] The selected live rail MUST produce reviewable receipts, client logs, server logs, comparator output, evidence docs, and BLAKE3 manifests under `docs/evidence/` before promotion.

#### Scenario: Evidence bundle is reviewable

r[mc_compatibility.projectile_travel_collision_live_rail.evidence.reviewable]
- GIVEN the live projectile rail passes
- WHEN evidence is promoted
- THEN the receipt, logs, comparator output, evidence note, and BLAKE3 manifest are copied under `docs/evidence/`
- AND the evidence note states the bounded row and adjacent non-claims.

### Requirement: Projectile row promotion boundary

r[mc_compatibility.projectile_travel_collision_live_rail.promotion] Acceptance/current-bundle docs MUST promote only the configured projectile travel/collision row after comparator and manifest validation pass.

#### Scenario: Adjacent projectile claims remain blocked

r[mc_compatibility.projectile_travel_collision_live_rail.promotion.bounded]
- GIVEN the selected row is promoted
- WHEN reviewers inspect acceptance and current-bundle docs
- THEN only the configured row is marked covered
- AND exact vanilla projectile physics, all projectile weapons, all collision surfaces, full combat correctness, public-server safety, and production readiness remain non-claims.

### Requirement: Projectile rail closeout

r[mc_compatibility.projectile_travel_collision_live_rail.closeout] The change MUST record focused live validation, comparator fixtures, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.projectile_travel_collision_live_rail.closeout.log]
- GIVEN the projectile travel/collision rail is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show positive and negative comparator fixtures, focused live validation, evidence manifest validation, task-evidence validation, Cairn proposal/design/tasks gates, and Cairn validation.
