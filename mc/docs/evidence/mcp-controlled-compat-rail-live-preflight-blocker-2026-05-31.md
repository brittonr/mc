# MCP-controlled compatibility rail live preflight blocker — 2026-05-31

## Question

Can the active `mcp-controlled-compat-rail` change advance task `live_artifacts` by running one owned-local live MCP-controlled smoke after the Stevenarella MCP/control and frame-capture prerequisites landed?

## Inspected evidence

- Dry-run flake check command:
  - `nix build --no-update-lock-file .#checks.x86_64-linux.mc-compat-mcp-controlled-smoke-dry-run --no-link -L`
  - Result in `docs/evidence/mcp-controlled-compat-rail-live-preflight-blocker-2026-05-31.run.log`: `exit_status=0`.
- Live command attempted:
  - `nix run --no-update-lock-file .#mc-compat-mcp-controlled-smoke -- --run`
  - Result in `docs/evidence/mcp-controlled-compat-rail-live-preflight-blocker-2026-05-31.run.log`: `exit_status=1`.
  - Diagnostic: `mcp-controlled-smoke live rail is not implemented yet; use --dry-run until frame/capture prerequisites land`.
- Reviewable failed live preflight receipt copy:
  - `docs/evidence/mcp-controlled-compat-rail-live-preflight-blocker-2026-05-31.receipt.json`
  - `status=fail`, `mode=run`, `dry_run=false`.
  - `mcp_control.first_failure=live-mcp-controlled-rail-not-implemented`.
  - `mcp_control.stevenarella_child_revision=4d1b1554650bd91924f7ce99c9dab69a91142edc` and `revision_status=clean`.
  - `frame_artifacts.selected=false`, `artifact_count=0`, `promotion_ready=false`.

## Decision

Do not mark `mcp_controlled_compat_rail.live_artifacts` complete. The dry-run rail remains healthy, but the live rail still fails closed before launching the client because `tools/mc-compat-runner/src/main.rs` rejects `mcp-controlled-smoke --run` in `validate_mcp_controlled_live_preflight`.

## Owner

`mcp-controlled-compat-rail` implementer.

## Next action

Implement the live `mcp-controlled-smoke` runner path so the parent harness starts Stevenarella with MCP enabled, performs initialize/tools-list/tool-call/capture commands, copies frame artifacts under `docs/evidence/`, and records a passing live receipt before promoting any MCP-controlled observability row.

## Non-claims

This blocker does not promote live MCP-controlled compatibility, visual regression approval, gameplay semantic equivalence, production readiness, public-server safety, or load/stress behavior.
