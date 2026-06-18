# mc-compatibility Change Spec: Recipe-book client settings live rail

## Requirements

### Requirement: Recipe-book settings live contract

r[mc_compatibility.recipe_book_settings_live_rail.contract] The `recipe-book-client-settings` live rail MUST define a bounded owned-local settings transition contract before live promotion is attempted.

#### Scenario: Contract names one settings transition

r[mc_compatibility.recipe_book_settings_live_rail.contract.scope]
- GIVEN the recipe-book settings row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, packet row `RecipeBookDataC2SPacket`, configured recipe-book state fields, client action metric, backend/client path, expected Valence server correlation, and non-claims
- AND recipe-book UI behavior, recipe discovery, all recipe categories, all recipes, crafting breadth, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Recipe-book settings baseline

r[mc_compatibility.recipe_book_settings_live_rail.baseline] The change MUST run existing targeted packet, matrix, current-bundle, and packet-inventory checks before modifying recipe-book rail behavior.

#### Scenario: Fixture status is recorded first

r[mc_compatibility.recipe_book_settings_live_rail.baseline.recorded]
- GIVEN `recipe-book-client-settings` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record its existing evidence classification and non-claims before live evidence is introduced.

### Requirement: Recipe-book settings live rail

r[mc_compatibility.recipe_book_settings_live_rail.rail] The harness MUST expose an isolated recipe-book settings rail or deterministic missing-driver blocker for the configured transition.

#### Scenario: Settings row stays separate from crafting parity

r[mc_compatibility.recipe_book_settings_live_rail.rail.isolated]
- GIVEN existing crafting evidence covers one crafting-table recipe path
- WHEN the recipe-book settings rail is added
- THEN existing crafting claims and survival scenario semantics remain unchanged
- AND the new row records only settings packet evidence and server correlation.

### Requirement: Recipe-book settings live evidence

r[mc_compatibility.recipe_book_settings_live_rail.evidence] Recipe-book settings live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes settings fields

r[mc_compatibility.recipe_book_settings_live_rail.evidence.reviewable]
- GIVEN the configured recipe-book settings transition is observed or blocked by a missing driver
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `recipe-book-client-settings`, packet row, configured settings fields, client action metric, server correlation or blocker, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Recipe-book settings live checker

r[mc_compatibility.recipe_book_settings_live_rail.checker] The targeted packet live-evidence checker MUST pass before `recipe-book-client-settings` moves beyond fixture-bounded status.

#### Scenario: Weak recipe-book evidence fails closed

r[mc_compatibility.recipe_book_settings_live_rail.checker.rejects]
- GIVEN recipe-book evidence is missing, names the wrong packet row or settings fields, omits client action or server correlation, reports a stale receipt digest, or claims broad recipe-book/crafting coverage
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Recipe-book settings narrow promotion

r[mc_compatibility.recipe_book_settings_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `recipe-book-client-settings` after row-specific live evidence passes.

#### Scenario: Broader recipe behavior remains non-claim

r[mc_compatibility.recipe_book_settings_live_rail.promotion.nonclaims]
- GIVEN recipe-book settings live evidence passes
- WHEN docs are updated
- THEN only the configured settings row moves beyond fixture-bounded status
- AND recipe-book UI, discovery, all recipes, crafting breadth, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Recipe-book settings validation

r[mc_compatibility.recipe_book_settings_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.recipe_book_settings_live_rail.validation.logs]
- GIVEN recipe-book settings live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.
