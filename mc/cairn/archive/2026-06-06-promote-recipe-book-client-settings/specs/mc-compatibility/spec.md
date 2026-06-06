# mc-compatibility Change Spec: Recipe-book client settings promotion

## Requirements

### Requirement: Recipe-book client settings contract

r[mc_compatibility.recipe_book_client_settings_promotion.contract] The `recipe-book-client-settings` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one settings transition

r[mc_compatibility.recipe_book_client_settings_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, packet row `RecipeBookDataC2SPacket`, configured recipe-book state fields, client action metric, Valence server correlation, child revisions, and checker metrics
- AND recipe-book UI behavior, all recipe categories, recipe discovery, all recipes, full crafting coverage, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Recipe-book client settings checker

r[mc_compatibility.recipe_book_client_settings_promotion.checker] A deterministic Rust checker MUST validate normalized recipe-book settings evidence before promotion.

#### Scenario: Valid recipe-book settings evidence passes

r[mc_compatibility.recipe_book_client_settings_promotion.checker.valid]
- GIVEN normalized evidence names `recipe-book-client-settings`, clean child revisions, configured settings fields, client action metric, and Valence server correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak recipe-book settings evidence fails closed

r[mc_compatibility.recipe_book_client_settings_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale revisions, names the wrong settings fields, omits client or server correlation, or claims broad recipe-book/crafting coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Recipe-book client settings rail

r[mc_compatibility.recipe_book_client_settings_promotion.rail] The harness MUST expose an isolated recipe-book settings rail without changing existing crafting, survival, inventory, CTF, combat, or network semantics.

#### Scenario: Crafting rows stay separate

r[mc_compatibility.recipe_book_client_settings_promotion.rail.isolated]
- GIVEN existing crafting evidence covers one crafting-table recipe path
- WHEN the recipe-book settings rail is added
- THEN existing crafting claims remain unchanged
- AND the new row records only settings packet evidence.

### Requirement: Recipe-book settings artifacts

r[mc_compatibility.recipe_book_client_settings_promotion.artifacts] Review-critical recipe-book settings artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.recipe_book_client_settings_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow recipe-book settings matrix promotion

r[mc_compatibility.recipe_book_client_settings_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured recipe-book settings row after checker and evidence gates pass.

#### Scenario: Broader recipe-book semantics remain a non-claim

r[mc_compatibility.recipe_book_client_settings_promotion.matrix.nonclaims]
- GIVEN recipe-book settings evidence passes
- WHEN docs are updated
- THEN only the configured settings row is marked covered
- AND recipe-book UI, discovery, all recipes, crafting breadth, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Recipe-book settings validation evidence

r[mc_compatibility.recipe_book_client_settings_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.recipe_book_client_settings_promotion.validation.log]
- GIVEN the recipe-book settings row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
