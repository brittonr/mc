# Delta: Survival crafting recipe breadth parity

## Requirements

### Requirement: Crafting breadth contract

r[mc_compatibility.survival_crafting_recipe_breadth_parity.contract] The `survival-crafting-recipe-breadth-parity` row MUST define a bounded recipe matrix before evidence is promoted.

#### Scenario: Contract fixes finite recipe scope

r[mc_compatibility.survival_crafting_recipe_breadth_parity.contract.scope]
- GIVEN crafting breadth work starts
- WHEN the evidence contract is reviewed
- THEN it names one shaped recipe, one shapeless recipe, one invalid or insufficient-input rejection, one configured collection mode, and normalized recipe/input/result/inventory metrics
- AND all-recipes, recipe-book UI, arbitrary collection modes, full survival compatibility, and broad vanilla parity remain explicit non-claims.

### Requirement: Crafting breadth checker

r[mc_compatibility.survival_crafting_recipe_breadth_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence crafting-breadth metrics before promotion.

#### Scenario: Weak crafting evidence fails closed

r[mc_compatibility.survival_crafting_recipe_breadth_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits recipe ids, omits slot/result metrics, mismatches item counts, reports stale child revisions, or claims all recipes
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the missing or invalid metric.

### Requirement: Isolated crafting breadth rail

r[mc_compatibility.survival_crafting_recipe_breadth_parity.rail] The harness MUST expose an isolated crafting-breadth rail without changing the existing `survival-crafting-table` row.

#### Scenario: Existing crafting row remains unchanged

r[mc_compatibility.survival_crafting_recipe_breadth_parity.rail.isolated]
- GIVEN the existing stick crafting row is already promoted
- WHEN the crafting-breadth rail is added
- THEN the existing row keeps its milestones, receipts, and non-claims
- AND the new row records its own recipe matrix metrics.

### Requirement: Reviewable crafting breadth receipts

r[mc_compatibility.survival_crafting_recipe_breadth_parity.receipts] Paired crafting-breadth receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are local and paired

r[mc_compatibility.survival_crafting_recipe_breadth_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect the repository
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are available under `docs/evidence/`.

### Requirement: Narrow crafting breadth promotion

r[mc_compatibility.survival_crafting_recipe_breadth_parity.promotion] Matrix and bundle docs MUST promote only the bounded crafting-breadth row after paired evidence passes.

#### Scenario: Broader crafting remains a non-claim

r[mc_compatibility.survival_crafting_recipe_breadth_parity.promotion.nonclaims]
- GIVEN paired crafting-breadth evidence passes
- WHEN docs are updated
- THEN only the configured crafting-breadth row is marked covered
- AND all-recipes, recipe-book UI, arbitrary collection modes, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Crafting breadth validation evidence

r[mc_compatibility.survival_crafting_recipe_breadth_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_crafting_recipe_breadth_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
