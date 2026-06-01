# Tasks

- [x] [serial] Define the bounded `command/recipe/advancement packet family` evidence contract and normalized metric names. r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `command/recipe/advancement packet family`. r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.checker]
- [ ] [depends:checker] Add the `protocol-command-recipe-advancement-family` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.rail]
  - Detail: Select family subset.
  - Detail: Add semantic/parser fixtures.
  - Detail: Add live feature receipt.
  - Detail: Update ledger/current bundle.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.evidence]
- [ ] [depends:evidence] Promote only the `command/recipe/advancement packet family` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.protocol_command_recipe_advancement_family_coverage.validation]
