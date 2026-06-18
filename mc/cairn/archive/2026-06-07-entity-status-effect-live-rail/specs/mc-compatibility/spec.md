# mc-compatibility Change Spec: Entity status-effect live rail

## Requirements

### Requirement: Entity status-effect live contract

r[mc_compatibility.entity_status_effect_live_rail.contract] The `entity-status-effect-packets` live rail MUST define a bounded owned-local status-effect contract before live promotion is attempted.

#### Scenario: Contract names one effect transition

r[mc_compatibility.entity_status_effect_live_rail.contract.scope]
- GIVEN the status-effect packet row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor or target entity, effect id or name, amplifier, duration, packet row or rows, backend/client path, expected server correlation, and non-claims
- AND all effects, stacking, particles/UI, gameplay modifiers, combat balancing, survival parity, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Entity status-effect baseline

r[mc_compatibility.entity_status_effect_live_rail.baseline] The change MUST run existing targeted packet, matrix, current-bundle, and packet-inventory checks before modifying status-effect rail behavior.

#### Scenario: Fixture status is recorded first

r[mc_compatibility.entity_status_effect_live_rail.baseline.recorded]
- GIVEN `entity-status-effect-packets` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record its existing evidence classification and non-claims before live evidence is introduced.

### Requirement: Entity status-effect live rail

r[mc_compatibility.entity_status_effect_live_rail.rail] The harness MUST expose an isolated status-effect rail or deterministic missing-signal blocker for the configured effect transition.

#### Scenario: Effect packet row stays separate from mechanics claims

r[mc_compatibility.entity_status_effect_live_rail.rail.isolated]
- GIVEN existing combat and survival rows have separate scoped claims
- WHEN the status-effect rail is added
- THEN existing combat, survival, CTF, inventory, network, and negative-live semantics remain unchanged
- AND the status-effect row records only packet observation and server-correlation metrics.

### Requirement: Entity status-effect live evidence

r[mc_compatibility.entity_status_effect_live_rail.evidence] Status-effect live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes effect metrics

r[mc_compatibility.entity_status_effect_live_rail.evidence.reviewable]
- GIVEN the configured effect transition is observed or blocked by a missing signal
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `entity-status-effect-packets`, packet rows, entity identity, effect id or name, amplifier, duration, client apply and optional remove observations, server correlation or blocker, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Entity status-effect live checker

r[mc_compatibility.entity_status_effect_live_rail.checker] The targeted packet live-evidence checker MUST pass before `entity-status-effect-packets` moves beyond fixture-bounded status.

#### Scenario: Weak status-effect evidence fails closed

r[mc_compatibility.entity_status_effect_live_rail.checker.rejects]
- GIVEN status-effect evidence is missing, uses stale revisions or receipt digest, names the wrong entity, effect, amplifier, duration, or packet row, omits required apply or remove correlation, or claims broad effect/modifier semantics
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Entity status-effect narrow promotion

r[mc_compatibility.entity_status_effect_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `entity-status-effect-packets` after row-specific live evidence passes.

#### Scenario: Broader effect behavior remains non-claim

r[mc_compatibility.entity_status_effect_live_rail.promotion.nonclaims]
- GIVEN status-effect live evidence passes
- WHEN docs are updated
- THEN only the configured status-effect packet row moves beyond fixture-bounded status
- AND all effects, stacking, particles/UI, modifiers, combat balancing, survival parity, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Entity status-effect validation

r[mc_compatibility.entity_status_effect_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.entity_status_effect_live_rail.validation.logs]
- GIVEN status-effect live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.
