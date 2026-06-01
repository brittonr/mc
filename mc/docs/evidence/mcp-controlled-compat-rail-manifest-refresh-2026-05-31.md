# MCP-controlled compatibility rail manifest refresh checkpoint (2026-05-31)

## Question
Which stale historical BLAKE3 manifests must be refreshed before final validation of `mcp-controlled-compat-rail`?

## Inspected evidence
- Baseline manifest checker output copied to `docs/evidence/mcp-controlled-compat-rail-manifest-refresh-before-2026-05-31.run.log`.
- Baseline detected 29 directly stale manifests after the accepted MCP-controlled matrix/current-bundle/checker updates.
- Refreshing direct stales exposed three cascade manifests that cite refreshed `.b3` files; the final fixed-point refresh touched the 32 manifests listed below.
- The checksum mismatches were for tracked shared files changed by accepted evidence/checker updates, including the acceptance matrix, current evidence bundle, MCP-controlled tasks, scenario manifest/generated parser, runner source, MCP checker, matrix/current-bundle checkers, flake wrapper, and `.b3` manifests that cite those refreshed manifests.

## Decision
Refresh only digest lines in existing `.b3` manifests so each manifest continues to cite the same repo-relative files at current accepted content. Do not change historical evidence prose, receipts, logs, or scoped claims as part of this refresh.

## Manifests refreshed
- `docs/evidence/mcp-controlled-compat-rail-live-preflight-blocker-2026-05-31.b3`
- `docs/evidence/mcp-controlled-compat-rail-live-preflight-blocker-validation-2026-05-31.b3`
- `docs/evidence/mcp-controlled-compat-rail-live-validation-2026-05-31.b3`
- `docs/evidence/mcp-controlled-compat-rail-openspec-alignment-oracle-2026-05-31.b3`
- `docs/evidence/mcp-controlled-compat-rail-overclaim-hardening-2026-05-31.b3`
- `docs/evidence/nix-run-valence-stevenarella-2026-05-30.b3`
- `docs/evidence/protocol-763-adversarial-network-oracle-checker-2026-05-29.b3`
- `docs/evidence/protocol-763-adversarial-network-oracle-matrix-2026-05-29.b3`
- `docs/evidence/protocol-763-adversarial-network-oracle-validation-2026-05-29.b3`
- `docs/evidence/protocol-763-armor-loadout-enchantment-status-checker-2026-05-29.b3`
- `docs/evidence/protocol-763-armor-loadout-enchantment-status-rail-2026-05-29.b3`
- `docs/evidence/protocol-763-armor-loadout-enchantment-status-row-2026-05-29.b3`
- `docs/evidence/protocol-763-armor-loadout-enchantment-status-validation-2026-05-29.b3`
- `docs/evidence/protocol-763-broad-coverage-ledger-gate-2026-05-28.b3`
- `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.b3`
- `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.b3`
- `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.b3`
- `docs/evidence/protocol-763-equipment-slot-item-expansion-checker-2026-05-29.b3`
- `docs/evidence/protocol-763-equipment-slot-item-expansion-rail-2026-05-29.b3`
- `docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.b3`
- `docs/evidence/protocol-763-equipment-slot-item-expansion-validation-2026-05-29.b3`
- `docs/evidence/protocol-763-production-network-safety-gate-2026-05-28.b3`
- `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`
- `docs/evidence/protocol-763-python-gate-rust-migration-2026-05-31.b3`
- `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.b3`
- `docs/evidence/protocol-763-survival-chest-persistence-runner-2026-05-29.b3`
- `docs/evidence/protocol-763-survival-coverage-reference-parity-sync-2026-05-28.b3`
- `docs/evidence/protocol-763-survival-crafting-table-postreview-2026-05-31.b3`
- `docs/evidence/protocol-763-survival-crafting-table-progress-2026-05-30.b3`
- `docs/evidence/protocol-763-survival-reference-paper-fixture-gate-2026-05-28.b3`
- `docs/evidence/protocol-763-survival-reference-parity-gate-2026-05-28.b3`
- `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.b3`

## Owner
Agent performing `mcp-controlled-compat-rail` final validation.

## Next action
Run the Rust evidence manifest checker and Cairn validation with logs under `docs/evidence/`.
