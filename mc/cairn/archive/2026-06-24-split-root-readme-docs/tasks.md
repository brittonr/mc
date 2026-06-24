# Tasks

- [x] [serial] Define the README scope and target docs map for scenarios, commands, config, evidence, and verification content. r[repository_layout.readme_split.scope]
  - Evidence: `README.md`; `docs/scenario-commands.md`; `docs/evidence-workflow.md`; `docs/configuration.md`; `docs/verification.md`; `docs/evidence/split-root-readme-docs-local-checks.run.log`; `docs/evidence/split-root-readme-docs-validation.b3`.
- [x] [depends:scope] Move detailed scenario command listings into focused docs with generated or checked machine-owned blocks. r[repository_layout.readme_split.scenario_docs]
  - Evidence: `docs/scenario-commands.md`; `docs/scenario-commands.generated.md`; `docs/evidence/mc-compat-scenario-index.generated.md`; `docs/evidence/split-root-readme-docs-generated-checks.run.log`; `docs/evidence/split-root-readme-docs-validation.b3`.
- [x] [depends:scenario_docs] Move evidence workflow and non-claim guidance into focused docs while keeping README links and summary caveats. r[repository_layout.readme_split.evidence_docs]
  - Evidence: `README.md`; `docs/evidence-workflow.md`; `docs/evidence/split-root-readme-docs-local-checks.run.log`; `docs/evidence/split-root-readme-docs-validation.b3`.
- [x] [depends:evidence_docs] Move config and verification details into focused docs or generated indexes as appropriate. r[repository_layout.readme_split.config_verification_docs]
  - Evidence: `docs/configuration.md`; `docs/verification.md`; `docs/evidence/split-root-readme-docs-local-checks.run.log`; `docs/evidence/split-root-readme-docs-validation.b3`.
- [x] [depends:config_verification_docs] Add or update freshness/link checks for moved generated docs and command parity. r[repository_layout.readme_split.checks]
  - Evidence: `tools/check_scenario_manifest.rs`; `docs/scenario-derived-surfaces.md`; `docs/evidence/split-root-readme-docs-local-checks.run.log`; `docs/evidence/split-root-readme-docs-generated-checks.run.log`; `docs/evidence/split-root-readme-docs-command-dry-runs.run.log`; `docs/evidence/split-root-readme-docs-validation.b3`.
- [x] [depends:checks] Run docs checks, generated freshness checks, selected command dry-runs, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.readme_split.validation]
  - Evidence: `docs/evidence/split-root-readme-docs-pre-gates.run.log`; `docs/evidence/split-root-readme-docs-local-checks.run.log`; `docs/evidence/split-root-readme-docs-generated-checks.run.log`; `docs/evidence/split-root-readme-docs-command-dry-runs.run.log`; `docs/evidence/split-root-readme-docs-cairn-gates.run.log`; `docs/evidence/split-root-readme-docs-validation.b3`.
