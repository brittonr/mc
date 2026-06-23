# repository-layout Change Spec: Check tier taxonomy

## Requirements

### Requirement: Check tier taxonomy

r[repository_layout.check_tiers.taxonomy] The repository SHOULD define named check tiers with scope, required evidence, expected runtime cost, and explicit non-claims.

#### Scenario: Tier purpose is clear

r[repository_layout.check_tiers.taxonomy.clear]
- GIVEN a developer needs to validate a change
- WHEN they inspect the check-tier taxonomy
- THEN each tier states what change scope it covers, what commands or gates belong to it, what evidence it produces, and what it does not claim.

### Requirement: Check inventory by tier

r[repository_layout.check_tiers.inventory] Existing flake checks, app dry-runs, component tests, evidence gates, manual/live rails, and Cairn gates MUST be classified into the check-tier inventory before new tier wrappers are authoritative.

#### Scenario: Existing gate is classified

r[repository_layout.check_tiers.inventory.classified]
- GIVEN an existing flake check, manual command, or Cairn gate is used for validation
- WHEN the inventory is reviewed
- THEN the check is assigned to a tier or explicitly marked legacy/manual with owner and next action.

### Requirement: Check tier documentation

r[repository_layout.check_tiers.docs] Documentation SHOULD tell developers which tier to run for docs-only, generated-surface, runner-core, component-code, evidence, live/manual, and archive-closeout changes.

#### Scenario: Developer selects smallest sufficient tier

r[repository_layout.check_tiers.docs.selection]
- GIVEN a change touches only generated docs or only runner core or only evidence manifests
- WHEN the developer reads tier docs
- THEN the docs identify the smallest relevant tier and any additional affected-component checks
- AND they do not imply that a fast tier proves live compatibility.

### Requirement: Tier entrypoints

r[repository_layout.check_tiers.entrypoints] Common tier entrypoints MAY be exposed as flake apps/checks or generated indexes only if existing public check names remain stable.

#### Scenario: Tier wrapper preserves existing checks

r[repository_layout.check_tiers.entrypoints.compat]
- GIVEN a tier wrapper is added
- WHEN existing check names are evaluated
- THEN existing checks remain available
- AND the tier wrapper delegates to documented commands without changing their evidence semantics.

### Requirement: Tier freshness

r[repository_layout.check_tiers.freshness] Tier docs and wrapper inventories SHOULD fail freshness checks when check names or tier assignments drift.

#### Scenario: Stale tier inventory fails

r[repository_layout.check_tiers.freshness.drift]
- GIVEN a flake check is added, removed, or renamed without updating the tier inventory
- WHEN tier freshness validation runs
- THEN the stale inventory is reported
- AND archive is blocked until the docs or inventory are updated.

### Requirement: Check tier validation

r[repository_layout.check_tiers.validation] The check-tier taxonomy MUST record tier inventory validation, wrapper dry-runs or evaluation, docs checks, and Cairn gates before archive.

#### Scenario: Tier closeout is reviewable

r[repository_layout.check_tiers.validation.log]
- GIVEN check tiers have been documented or wired
- WHEN the change is archived
- THEN reviewable logs show inventory validation, tier wrapper dry-runs or evaluation, docs checks, Cairn proposal/design/tasks gates, and Cairn validation.
