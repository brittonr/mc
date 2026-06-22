# Proposal: Add a scenario manifest fallback budget gate

## Why

The mc-compat scenario manifest now mixes typed-event-ready rows with waiver-backed substring fallback rows. As typed-event migrations land, reviewers need a small fail-closed guard that prevents new fallback debt, unexpected typed-event regressions, or waiver-free fallback additions.

A fallback budget gate keeps the migration visible and measurable without forcing every remaining scenario to migrate in one change.

## What Changes

- Add a deterministic scenario-manifest fallback budget gate over the Nickel-authored manifest and generated surfaces.
- Define a checked-in allowlist of existing fallback rows and their owner/reason/non-claim/next-action waiver text.
- Fail when a new substring fallback row appears without an explicit waiver or when an existing typed-event-ready row regresses to fallback.
- Emit a reviewable report that names added fallback rows, removed fallback rows, unchanged fallback rows, and typed-event regressions.
- Document the gate in README/evidence docs and wire it into the focused mc-compat validation surface.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, `tools/check_scenario_manifest.rs` or a focused Rust checker, generated scenario surfaces, README/evidence docs, flake checks, and Cairn lifecycle files.
- **Testing**: positive fixtures for unchanged baseline and allowed fallback removal; negative fixtures for new fallback debt, missing waiver text, and typed-event regression.
- **Non-claims**: this is a manifest hygiene gate only. It does not migrate rows, change scenario behavior, add live evidence, broaden compatibility claims, or weaken existing fallback waivers.
