# Proposal: Protocol chunk and biome family coverage rail

## Why

Chunk and biome features remain broad protocol non-claims even though a few raw chunk-related packet rows have parser fixtures. This rail adds a named family slice without claiming worldgen parity.

## What Changes

- Add a bounded `protocol-chunk-biome-family` row for selected chunk/biome packet rows with reviewed parser fixtures and one live fixture proving client receipt of configured environment data.
- Define normalized metrics: packet name, wire id, chunk position, biome id or environment id, parser fixture id, live receipt path, and malformed fixture status where supported.
- Require evidence standard: protocol ledger entries plus live fixture evidence; worldgen and terrain parity remain separate.
- Add fixture/runner/checker work: protocol tests cover selected chunk/biome payloads and live fixture logs configured chunk/biome observations.
- Reject overclaims and bad evidence: missing parser fixture, missing live observation, terrain/worldgen overclaim, fallback alias, or full chunk/biome claim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: broad protocol coverage.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: world generation parity, all chunks, all biomes, lighting/weather, structures, full protocol-763 compatibility, full survival compatibility, and production readiness.
