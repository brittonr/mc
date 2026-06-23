# Tasks

- [ ] [serial] Record the current public flake output inventory for packages, apps, checks, and dev shells that must remain stable. r[repository_layout.flake_module_split.public_interface]
- [ ] [depends:public_interface] Factor package definitions and shared native/GUI tool lists into explicit local Nix modules. r[repository_layout.flake_module_split.package_modules]
- [ ] [depends:package_modules] Factor app wrappers and check definitions into focused local Nix modules while preserving output names and command shapes. r[repository_layout.flake_module_split.app_check_modules]
- [ ] [depends:app_check_modules] Factor dev shell definitions into a focused local Nix module without changing documented environment behavior. r[repository_layout.flake_module_split.devshell_module]
- [ ] [depends:devshell_module] Add or update parity checks for public output names, selected dry-run wrappers, and stale generated wrapper data if touched. r[repository_layout.flake_module_split.parity_checks]
- [ ] [depends:parity_checks] Run focused Nix evaluation/dry-runs, selected check builds, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.flake_module_split.validation]
