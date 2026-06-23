# Proposal: Extend manifest-derived harness surfaces

## Why

The Nickel scenario manifest is already treated as the source of truth for scenario-derived harness surfaces, but app wrappers, check wiring, README command listings, and evidence index rows still contain repeated hand-maintained structure. Drift checks catch mismatches after edits; deriving more surfaces from the manifest would prevent many mismatches from being authored in the first place.

## What Changes

- Inventory every scenario-derived surface and classify it as generated, human-authored, or intentionally duplicated.
- Extend generation only to stable bounded outputs with clear machine-owned markers.
- Prefer manifest-derived metadata for flake app/check wrapper generation rather than repeated manual blocks.
- Keep runtime Rust-only: the runner consumes checked-in generated artifacts and does not evaluate Nickel at startup.
- Add freshness checks and positive/negative generator fixtures for every newly generated surface class.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generator/checker code, generated Rust/static surfaces, `flake.nix` or imported Nix modules, README generated blocks, docs/evidence indexes, and Cairn artifacts.
- **Testing**: manifest validation, generator positive/negative fixtures, generated-output freshness check, selected app/check dry-runs, maintained dry-run aggregate, and Cairn gates/validation.
- **Non-claims**: this reduces drift and boilerplate only; it does not change scenario semantics, receipt success criteria, live evidence, or compatibility claims.
