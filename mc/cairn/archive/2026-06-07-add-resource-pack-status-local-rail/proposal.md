## Why

`resource-pack-status` remains fixture-bounded because the runner lacks a deterministic owned-local resource-pack offer/status exchange. A local-only rail can exercise the status packet path without requiring external downloads or claiming resource-pack trust/application behavior.

## What Changes

- Add an isolated local resource-pack status rail that offers one local fixture pack or synthetic offer and records one configured client status response.
- Prove the exchange stays owned-local and does not require external asset fetching.
- Emit reviewable KV/receipt/log evidence and validate it through the targeted packet live-evidence checker before promotion.
- Promote only `resource-pack-status` if live evidence passes; leave all unrelated targeted rows fixture-bounded.

## Impact

- **Files**: `tools/mc-compat-runner/src/**`, local resource-pack fixture metadata if needed, targeted packet checker inputs, `docs/evidence/**`, acceptance matrix/current bundle/packet inventory docs.
- **Testing**: Baseline targeted packet and doc checks, local rail tests/dry-runs, no-external-fetch validation, live-evidence checker positive/negative coverage, evidence-manifest/task-evidence checks, Cairn gates and validation.
