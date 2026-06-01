# Delta: Protocol command recipe advancement family coverage rail

## Requirements

### Requirement: Contract

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.contract] The `command/recipe/advancement packet family` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.contract.scope]
- GIVEN `protocol-command-recipe-advancement-family` work starts
- WHEN the evidence contract is reviewed
- THEN it names selected command, recipe, or advancement packet rows with reviewed mapping/parser fixtures and bounded live evidence for the selected feature
- AND it states that all commands, all recipes, all advancements, recipe-book semantics, command execution semantics, full protocol-763 compatibility, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.checker] A deterministic checker MUST compare normalized metrics before the `command/recipe/advancement packet family` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.checker.rejects]
- GIVEN evidence is missing or mismatches packet family, wire id, semantic fixture id, parser fixture result, malformed fixture status, live scenario feature, receipt path, and digest
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.checker.standard]
- GIVEN the row requires protocol ledger rows require no fallback alias, parser fixture, live feature receipt, owner, next action, and explicit semantic non-claims for raw paths
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.rail] The harness MUST expose a `protocol-command-recipe-advancement-family` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `protocol-command-recipe-advancement-family` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.evidence] `command/recipe/advancement packet family` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.evidence.reviewable]
- GIVEN the `command/recipe/advancement packet family` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.matrix] Acceptance matrix and current-bundle docs MUST promote only the `command/recipe/advancement packet family` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.matrix.nonclaims]
- GIVEN `command/recipe/advancement packet family` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `command/recipe/advancement packet family` row is marked covered
- AND all commands, all recipes, all advancements, recipe-book semantics, command execution semantics, full protocol-763 compatibility, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.validation.log]
- GIVEN the `command/recipe/advancement packet family` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
