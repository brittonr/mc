# Delta: Survival hunger and health-cycle parity

## Requirements

### Requirement: Hunger health-cycle contract

r[mc_compatibility.survival_hunger_health_cycle_parity.contract] The `survival-hunger-health-cycle-parity` row MUST define a bounded health-cycle evidence contract before promotion.

#### Scenario: Contract names controlled cycle

r[mc_compatibility.survival_hunger_health_cycle_parity.contract.scope]
- GIVEN hunger-health work starts
- WHEN the contract is reviewed
- THEN it names starting health, food, saturation, exhaustion trigger, regeneration checkpoint, starvation or low-food checkpoint, inventory consumption, and normalized comparison metrics
- AND all foods, all exhaustion sources, potion/effect interactions, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Hunger health-cycle checker

r[mc_compatibility.survival_hunger_health_cycle_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence hunger-health metrics before promotion.

#### Scenario: Weak hunger evidence fails closed

r[mc_compatibility.survival_hunger_health_cycle_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits food/health/saturation metrics, mismatches named checkpoints, reports stale child revisions, or claims broad hunger mechanics
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid health-cycle metric.

### Requirement: Isolated hunger health-cycle rail

r[mc_compatibility.survival_hunger_health_cycle_parity.rail] The harness MUST expose an isolated hunger-health rail without changing the existing Bread consumption row.

#### Scenario: Existing hunger row remains unchanged

r[mc_compatibility.survival_hunger_health_cycle_parity.rail.isolated]
- GIVEN the existing Bread consumption row is promoted
- WHEN the hunger-health rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own health-cycle checkpoints.

### Requirement: Reviewable hunger health-cycle receipts

r[mc_compatibility.survival_hunger_health_cycle_parity.receipts] Paired hunger-health receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_hunger_health_cycle_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow hunger health-cycle promotion

r[mc_compatibility.survival_hunger_health_cycle_parity.promotion] Matrix and bundle docs MUST promote only the bounded hunger-health cycle row after paired evidence passes.

#### Scenario: Broader hunger remains a non-claim

r[mc_compatibility.survival_hunger_health_cycle_parity.promotion.nonclaims]
- GIVEN paired hunger-health evidence passes
- WHEN docs are updated
- THEN only the configured hunger-health row is marked covered
- AND all foods, all exhaustion sources, potion/effect interactions, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Hunger health-cycle validation evidence

r[mc_compatibility.survival_hunger_health_cycle_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_hunger_health_cycle_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
