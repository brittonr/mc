# Delta: Survival furnace smelting breadth parity

## Requirements

### Requirement: Furnace smelting breadth contract

r[mc_compatibility.survival_furnace_smelting_breadth_parity.contract] The `survival-furnace-smelting-breadth-parity` row MUST define a bounded smelting and fuel matrix before promotion.

#### Scenario: Contract names finite furnace scope

r[mc_compatibility.survival_furnace_smelting_breadth_parity.contract.scope]
- GIVEN furnace breadth work starts
- WHEN the contract is reviewed
- THEN it names configured recipe variants, fuel variants, burn/cook checkpoints, output collection, one rejection case, and normalized comparison metrics
- AND all smelting recipes, all fuels, hopper automation, furnace minecarts, long-running timing parity, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Furnace smelting breadth checker

r[mc_compatibility.survival_furnace_smelting_breadth_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence furnace-breadth metrics before promotion.

#### Scenario: Weak furnace evidence fails closed

r[mc_compatibility.survival_furnace_smelting_breadth_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits burn or cook metrics, mismatches output item/count, reports stale child revisions, omits rejection evidence, or claims all furnace semantics
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid furnace metric.

### Requirement: Isolated furnace breadth rail

r[mc_compatibility.survival_furnace_smelting_breadth_parity.rail] The harness MUST expose an isolated furnace-breadth rail without changing the existing furnace persistence row.

#### Scenario: Existing furnace row remains unchanged

r[mc_compatibility.survival_furnace_smelting_breadth_parity.rail.isolated]
- GIVEN the existing furnace persistence row is promoted
- WHEN the furnace-breadth rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own smelting/fuel matrix metrics.

### Requirement: Reviewable furnace breadth receipts

r[mc_compatibility.survival_furnace_smelting_breadth_parity.receipts] Paired furnace-breadth receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_furnace_smelting_breadth_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow furnace breadth promotion

r[mc_compatibility.survival_furnace_smelting_breadth_parity.promotion] Matrix and bundle docs MUST promote only the bounded furnace-breadth row after paired evidence passes.

#### Scenario: Broader furnace semantics remain non-claims

r[mc_compatibility.survival_furnace_smelting_breadth_parity.promotion.nonclaims]
- GIVEN paired furnace-breadth evidence passes
- WHEN docs are updated
- THEN only the configured furnace-breadth row is marked covered
- AND all recipes, all fuels, automation, timing breadth, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Furnace breadth validation evidence

r[mc_compatibility.survival_furnace_smelting_breadth_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_furnace_smelting_breadth_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
