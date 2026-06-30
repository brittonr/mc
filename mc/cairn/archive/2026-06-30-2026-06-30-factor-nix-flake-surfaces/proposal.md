# Proposal: Factor root Nix flake surfaces

## Why

The root Nix surfaces have grown into long, repetitive files. `nix/checks.nix` carries many unrelated check families in one file, while `nix/packages.nix` and `nix/apps.nix` repeat many scenario wrapper patterns that are already represented by generated scenario metadata. This makes wrapper/check changes hard to review and increases drift risk.

## What Changes

- Inventory package, app, check, devshell, generated wrapper metadata, and flake-output inventory ownership before splitting Nix files.
- Split checks into focused modules by family, such as layout/docs, generated surfaces, evidence, runner/scenario, component/Valence, and Octet/checker framework.
- Introduce small Nix helper functions for repeated `runCommand` and wrapper patterns while preserving existing output names.
- Use generated scenario wrapper metadata where practical to reduce hand-maintained app/check/package duplication.
- Preserve `nix flake check`, flake output names, app/package main programs, dry-run behavior, and baseline output inventory compatibility.

## Impact

- **Files**: `flake.nix`, `nix/checks.nix`, `nix/packages.nix`, `nix/apps.nix`, new `nix/checks/*.nix` or helper modules, generated metadata checks, docs if ownership changes, and Cairn artifacts.
- **Testing**: baseline flake output inventory, focused Nix eval/build checks, generated-surface freshness when touched, `nix flake check` or smallest equivalent, Cairn gates, and Cairn validation.
- **Non-claims**: Nix maintainability only; this does not change compatibility evidence, component behavior, or live gameplay claims.
