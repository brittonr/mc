# Proposal: MCP-controlled compatibility rail

## Why

Stevenarella MCP control and frame capture should become reviewable compatibility evidence only after the parent `mc` runner can launch the client, drive it through MCP, collect artifacts, and record explicit receipts. Without a runner rail, MCP remains a local debugging feature rather than a durable compatibility workflow.

## What Changes

- Add a parent `mc` runner scenario that launches Stevenarella with MCP enabled, drives bounded client actions through MCP, and collects screenshots or frame artifacts when the capture Cairn is available.
- Extend receipt shape with MCP endpoint mode, tool calls, command outcomes, frame artifact metadata, BLAKE3 digests, redaction status, and explicit non-claims.
- Add deterministic dry-run fixtures and fail-closed checkers for missing MCP handshake, stdout contamination, missing command outcome, missing frame digest, stale client revision, and overbroad visual/semantic claims.
- Wire one focused flake check for the dry-run MCP-controlled scenario and keep existing maintained scenarios unchanged.

## Impact

- **Files**: `tools/mc-compat-runner`, `config/mc-compat/scenario-manifest.ncl`, generated scenario tables, `flake.nix`, README command listing, docs/evidence artifacts, and Cairn specs/tasks.
- **Validation**: scenario manifest check, runner receipt tests, positive/negative dry-run fixtures, evidence manifest check, Cairn task-evidence gate, Cairn gates, and parent validation.
- **Non-claims**: screenshots are observability evidence only; they do not prove full Minecraft compatibility, visual regression approval, gameplay semantic equivalence, production readiness, public-server safety, or load/stress behavior.
