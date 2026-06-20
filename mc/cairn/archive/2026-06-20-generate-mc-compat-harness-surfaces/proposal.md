# Proposal: Generate mc-compat harness surfaces from the scenario manifest

## Why

The Nickel scenario manifest is documented as the source of truth, but the runner tables, flake wrappers, README command examples, and current-bundle references are still kept aligned by drift checks and manual edits. Drift checks catch mistakes after the fact; generation would make the expected surfaces deterministic and reduce repeated boilerplate.

## What Changes

- Add a deterministic generator that reads the typed scenario manifest and emits checked-in harness surfaces.
- Generate Rust scenario tables and wrapper metadata where the output can be stable and reviewable.
- Keep human-authored prose separate from generated command/index blocks.
- Add a Nix check that regenerates outputs and fails when checked-in generated files are stale.

## Impact

- **Files**: `config/mc-compat/scenario-manifest.ncl`, generated Rust tables, generated wrapper/index artifacts, `flake.nix`, README generated sections, scenario-manifest checker, evidence docs/manifests, and Cairn artifacts.
- **Testing**: generator positive/negative fixtures, generated-output freshness check, runner tests, scenario manifest check, maintained dry-run aggregate, and Cairn gates/validation.
- **Non-claims**: this change improves harness maintainability and drift resistance only; it does not change scenario semantics, receipt evidence, live coverage, or compatibility claims.
