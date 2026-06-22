# Delta: Survival mob AI and loot breadth parity

## Requirements

### Requirement: Mob AI loot breadth contract

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.contract] The `survival-mob-ai-loot-breadth-parity` row MUST define a bounded mob behavior matrix before promotion.

#### Scenario: Contract names finite mob scope

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.contract.scope]
- GIVEN mob breadth work starts
- WHEN the contract is reviewed
- THEN it names exact hostile and passive mob types, spawn state, bounded movement or targeting milestones, loot outputs, pickup metrics, and inventory deltas
- AND all mob classes, broad AI, random loot distribution, spawn-rule breadth, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Mob AI loot checker

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence mob behavior metrics before promotion.

#### Scenario: Weak mob evidence fails closed

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits mob identity, omits AI or loot metrics, mismatches drop item/count, reports stale child revisions, or claims all mob behavior
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid mob metric.

### Requirement: Isolated mob AI loot rail

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.rail] The harness MUST expose an isolated mob AI/loot rail without changing the existing mob-drop row.

#### Scenario: Existing mob-drop row remains unchanged

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.rail.isolated]
- GIVEN the existing Iron Golem mob-drop row is promoted
- WHEN the mob AI/loot rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own hostile/passive mob metrics.

### Requirement: Reviewable mob AI loot receipts

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.receipts] Paired mob AI/loot receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow mob AI loot promotion

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.promotion] Matrix and bundle docs MUST promote only the bounded mob AI/loot row after paired evidence passes.

#### Scenario: Broader mob behavior remains a non-claim

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.promotion.nonclaims]
- GIVEN paired mob AI/loot evidence passes
- WHEN docs are updated
- THEN only the configured mob AI/loot row is marked covered
- AND all mob classes, broad AI, random loot distribution, spawn rules, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Mob AI loot validation evidence

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_mob_ai_loot_breadth_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
