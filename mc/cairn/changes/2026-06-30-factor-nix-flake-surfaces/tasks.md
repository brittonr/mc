# Tasks

- [ ] [serial] Inventory current package/app/check/devshell outputs, generated wrapper metadata, repeated Nix patterns, and baseline flake output inventory. r[repository_layout.nix_surface_modularization.inventory]
- [ ] [serial] Split Nix check/package/app definitions into focused modules or helper functions while preserving aggregator entrypoints. r[repository_layout.nix_surface_modularization.module_boundaries]
- [ ] [serial] Preserve flake output names, app/package main programs, dry-run behavior, baseline output inventory, and generated scenario metadata freshness. r[repository_layout.nix_surface_modularization.output_parity]
- [ ] [serial] Keep repeated `runCommand`, log-copying, checker, and scenario wrapper helpers explicit enough that each check's evidence output remains reviewable. r[repository_layout.nix_surface_modularization.reviewability]
- [ ] [serial] Add positive checks for valid split outputs and negative checks for missing outputs, unexpected outputs, stale generated metadata, missing helper evidence copies, and broken dry-run wrappers. r[repository_layout.nix_surface_modularization.tests]
- [ ] [serial] Run focused Nix eval/build checks, flake output inventory, generated-surface checks when touched, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks with reviewable logs before archive. r[repository_layout.nix_surface_modularization.validation]
