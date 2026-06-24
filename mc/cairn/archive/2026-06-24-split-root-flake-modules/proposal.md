# Proposal: Split the root flake into focused local modules

## Why

The root `flake.nix` is large and combines package definitions, app wrappers, checks, dev shells, generated/maintained scenario wrappers, and helper constants in one file. That makes reviews noisy and encourages copy-paste changes. Focused local Nix modules can preserve the public flake interface while making ownership and validation clearer.

## What Changes

- Keep `flake.nix` as the public façade for inputs and output assembly.
- Move focused implementation into local files such as `nix/packages.nix`, `nix/apps.nix`, `nix/checks.nix`, `nix/devshells.nix`, and scenario wrapper data as appropriate.
- Preserve existing flake output names, app behavior, check behavior, package names, and dev shell environment variables unless another Cairn explicitly changes them.
- Add focused evaluation/dry-run checks that prove split modules match pre-split output names and command shapes.

## Impact

- **Files**: `flake.nix`, new `nix/*.nix` local modules, README command references if paths are documented, generated surface checks if wrapper generation is touched, and Cairn artifacts.
- **Testing**: Nix evaluation, focused app dry-runs, selected check builds, maintained dry-run aggregate if wrapper wiring changes, and Cairn validation/gates.
- **Non-claims**: this is a flake maintainability refactor only; it does not change compatibility semantics, live evidence, or production readiness.
