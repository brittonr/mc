# Tasks

- [ ] [serial] Define the checker crate layout, binary naming contract, and wrapper compatibility strategy for existing flake checks. r[repository_layout.checker_crate_consolidation.contract]
- [ ] [depends:contract] Move shared key-value parsing, diagnostic, clean-revision, overclaim, and fixture helpers into a pure checker library with positive and negative unit tests. r[repository_layout.checker_crate_consolidation.shared_core]
- [ ] [depends:shared_core] Migrate an initial family of Rust checkers into crate binaries while preserving flake check names and command shapes. r[repository_layout.checker_crate_consolidation.rust_migration]
- [ ] [depends:rust_migration] Inventory legacy Python gates and mark each as migrated, untouched debt, or explicitly waived with owner and next action. r[repository_layout.checker_crate_consolidation.python_policy]
- [ ] [depends:python_policy] Add per-checker positive and negative fixtures for every migrated checker and fail closed on invalid evidence. r[repository_layout.checker_crate_consolidation.fixtures]
- [ ] [depends:fixtures] Run checker tests, selected flake checks, evidence/task gates if touched, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.checker_crate_consolidation.validation]
