# MCP-controlled compatibility rail live evidence (2026-05-31)

## Question
Can the parent `mc` runner pass the live `mcp-controlled-smoke --run` path beyond `validate_mcp_controlled_live_preflight`, drive Stevenarella over MCP stdio, and record durable frame evidence under `docs/evidence/`?

## Inspected evidence
- Live command: `MC_COMPAT_MCP_CONTROLLED_SMOKE_RECEIPT=docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.json nix run --no-update-lock-file .#mc-compat-mcp-controlled-smoke -- --run`
- Command log: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.run.log` (`exit_status=0`).
- Receipt: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.json`.
- MCP transcript: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.mcp-transcript.log`.
- Client stderr: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.stderr.log` (empty on the promoted run).
- Typed event log: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.typed-events.log`.
- Captured frame: `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt-frames/mcp-controlled-smoke/latest-frame.png`.
- Receipt checker: `tools/check_mcp_controlled_compat_rail.rs` accepted the promoted receipt.

## Decision
Promote as live MCP-controlled observability evidence for the active `mcp-controlled-compat-rail` change. The receipt records MCP initialize/tools/list/status/look/key/chat/capture calls, clean stdio, Stevenarella child revision `4d1b1554650bd91924f7ce99c9dab69a91142edc`, and a BLAKE3-addressed PNG frame artifact under `docs/evidence/`.

## Non-claims
This evidence does not claim visual regression approval, semantic gameplay equivalence, full Minecraft compatibility, production readiness, public-server safety, load testing, WAN tolerance, or adversarial safety.

## Next action
Use this receipt to update only the MCP-controlled observability row, then run matrix/current-bundle gates, evidence manifest validation, `cairn gate tasks mcp-controlled-compat-rail --root .`, and `cairn validate --root .` before archiving.
