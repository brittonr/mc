# MCP-controlled compatibility rail matrix promotion (2026-05-31)

## Question
Can the live MCP-controlled observability evidence be promoted into the protocol-763 acceptance matrix and current evidence bundle without broadening visual, semantic, production, public-server, or load claims?

## Inspected evidence
- Live evidence checkpoint: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.md`.
- Live receipt: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.json`.
- MCP transcript: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.mcp-transcript.log`.
- Captured frame artifact: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt-frames/mcp-controlled-smoke/latest-frame.png`.
- Live evidence manifest: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.b3`.
- Matrix row: `docs/evidence/protocol-763-acceptance-matrix.md` (`MCP-controlled observability`).
- Current bundle row/checkpoint: `docs/evidence/protocol-763-current-evidence-bundle.md` (`MCP-controlled observability`).
- Checkers: `tools/check_acceptance_matrix.rs`, `tools/check_current_evidence_bundle.rs`, and `tools/check_mcp_controlled_compat_rail.rs`.

## Decision
Promote exactly one `MCP-controlled observability` row. The row uses maintained command `nix run .#mc-compat-mcp-controlled-smoke -- --run`, receipt digest `5eaa78082bfca069219fed40939e7003c9fce4e5f1d527a68ecdbbae9d610acf`, parent commit `6e1cde0`, Valence `3359f85`, and Stevenarella `4d1b155`.

## Scoped claim
The promoted row claims one bounded owned-local Stevenarella MCP stdio rail that initializes, lists tools, applies status/look/key/chat commands, records clean stdout and child revisions, and captures one BLAKE3-addressed latest-frame PNG under `docs/evidence/`.

## Non-claims
This promotion does not claim visual regression approval, semantic gameplay equivalence, full Minecraft compatibility, production readiness, public-server safety, load testing, broad MCP API coverage, or screenshot-only correctness.

## Next action
Run final validation only after refreshing stale historical `.b3` manifests caused by shared matrix/bundle/checker edits.
