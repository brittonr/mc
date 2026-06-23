# Tasks

- [ ] [serial] Define artifact classes and document the durable evidence, generated output, transient output, and scratch-file boundaries. r[repository_layout.artifact_boundaries.classification]
- [ ] [depends:classification] Move, archive, or retire top-level `evidence/` notes and root live logs according to the durable-evidence contract. r[repository_layout.artifact_boundaries.evidence_location]
- [ ] [depends:classification] Remove or document empty/reserved root directories such as `config/` so their ownership is clear. r[repository_layout.artifact_boundaries.root_dirs]
- [ ] [depends:evidence_location] Update `.gitignore` with targeted transient-output patterns while keeping `docs/evidence/` and `.b3` manifests visible to checks. r[repository_layout.artifact_boundaries.ignore_rules]
- [ ] [depends:ignore_rules] Add or update guards that reject Cairn task citations of target-only, result-only, root-transient, or missing artifacts. r[repository_layout.artifact_boundaries.citation_guard]
- [ ] [depends:citation_guard] Run evidence manifest checks, task evidence gates, artifact-boundary guards, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.artifact_boundaries.validation]
