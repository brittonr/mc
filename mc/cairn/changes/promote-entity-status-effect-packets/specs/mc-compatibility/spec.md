# mc-compatibility Change Spec: Entity status-effect packet promotion

## Requirements

### Requirement: Entity status-effect packet contract

r[mc_compatibility.entity_status_effect_packets_promotion.contract] The `entity-status-effect-packets` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one effect apply/remove scope

r[mc_compatibility.entity_status_effect_packets_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor or target entity, effect id or name, amplifier, duration, packet row or rows, server correlation, child revisions, and checker metrics
- AND all effects, stacking, particles/UI, gameplay modifiers, combat balancing, survival parity, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Entity status-effect checker

r[mc_compatibility.entity_status_effect_packets_promotion.checker] A deterministic Rust checker MUST validate normalized entity status-effect evidence before promotion.

#### Scenario: Valid status-effect evidence passes

r[mc_compatibility.entity_status_effect_packets_promotion.checker.valid]
- GIVEN normalized evidence names `entity-status-effect-packets`, clean child revisions, the configured effect metrics, client apply and optional remove observations, and Valence server correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak status-effect evidence fails closed

r[mc_compatibility.entity_status_effect_packets_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale revisions, names the wrong entity/effect/amplifier/duration, omits required apply or remove correlation, or claims broad effect or modifier semantics
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Entity status-effect rail

r[mc_compatibility.entity_status_effect_packets_promotion.rail] The harness MUST expose an isolated status-effect rail without changing existing combat, survival, CTF, inventory, network, or negative-live semantics.

#### Scenario: Effect packet row stays separate from modifier claims

r[mc_compatibility.entity_status_effect_packets_promotion.rail.isolated]
- GIVEN existing combat and survival rows have their own scoped claims
- WHEN the status-effect rail is added
- THEN existing rows remain unchanged
- AND the status-effect row records only packet observation/correlation metrics.

### Requirement: Entity status-effect artifacts

r[mc_compatibility.entity_status_effect_packets_promotion.artifacts] Review-critical status-effect artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.entity_status_effect_packets_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow entity status-effect matrix promotion

r[mc_compatibility.entity_status_effect_packets_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured status-effect row after checker and evidence gates pass.

#### Scenario: Broader effect mechanics remain a non-claim

r[mc_compatibility.entity_status_effect_packets_promotion.matrix.nonclaims]
- GIVEN status-effect packet evidence passes
- WHEN docs are updated
- THEN only the configured effect packet row is marked covered
- AND all broader effect, modifier, combat, survival, full protocol, and production claims remain explicit non-claims.

### Requirement: Entity status-effect validation evidence

r[mc_compatibility.entity_status_effect_packets_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.entity_status_effect_packets_promotion.validation.log]
- GIVEN the status-effect packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
