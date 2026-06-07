## Why

`creative-inventory-action` is one of the targeted packet rows that remained fixture-bounded because the runner lacks a deterministic owned-local creative-mode path. A narrow live rail can prove exactly one creative slot mutation without broad creative-inventory or public-server claims.

## What Changes

- Add an isolated owned-local creative inventory live rail that configures one actor in creative mode, performs one slot/item/count mutation, and records backend correlation.
- Emit reviewable evidence under `docs/evidence/` with the targeted packet row id, packet identifier, scenario name, backend/client path, revision metadata when available, and explicit non-claims.
- Use the targeted packet live-evidence checker before promoting `creative-inventory-action` beyond fixture-bounded status.
- Keep every other targeted packet row fixture-bounded unless separate live evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/**`, targeted packet checker inputs, `docs/evidence/**`, `docs/evidence/protocol-763-acceptance-matrix.md`, `docs/evidence/protocol-763-current-evidence-bundle.md`, `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv`
- **Testing**: Baseline targeted packet checks, runner dry-run/unit checks, creative live rail or deterministic fixture check, live-evidence checker positive/negative tests, matrix/bundle/inventory checks, evidence-manifest/task-evidence checks, Cairn gates and validation.
