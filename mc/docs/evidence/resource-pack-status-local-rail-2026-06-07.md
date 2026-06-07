# Resource-pack status local rail — 2026-06-07

## Scope

This evidence supports Cairn change `add-resource-pack-status-local-rail`. The change defines and validates a bounded local resource-pack offer/status contract for row `resource-pack-status` but does not promote the row beyond its existing fixture-bounded status.

## Contract

The pure runner contract names exactly one local exchange candidate:

- scenario: `mcp-controlled-smoke`;
- actor: `compatbot`;
- fixture identity: `owned-local-resource-pack-offer-fixture`;
- offer id: `mc-compat-local-resource-pack`;
- expected status: `declined`;
- packet rows: `ResourcePackSendS2CPacket` and `ResourcePackStatusC2SPacket`;
- no-external-fetch guarantee: `true`;
- redaction policy: `no-secrets-no-public-addresses`;
- expected server correlation: `resource_pack_status_declined_observed`.

`tools/mc-compat-runner/src/scenario_core.rs` keeps this contract in pure data and validates it with positive and negative tests. The scenario capability registry now points the resource-pack row at `deterministic-resource-pack-offer-contract` and keeps the client path as `stevenarella-resource-pack-status-driver-missing`.

## Blocker decision

No maintained Stevenarella live driver currently handles the configured local resource-pack offer/status response, so the row remains fixture-bounded. The blocker is recorded in:

- `docs/evidence/resource-pack-status-local-rail-2026-06-07.kv`;
- `docs/evidence/resource-pack-status-local-rail-2026-06-07.receipt.json`.

The targeted packet live-evidence checker rejects the blocker KV as expected because `live.promotion.status=blocked`, `live.evidence.mode=fixture-bounded-blocker`, and live metrics are not `ok`. This prevents matrix, bundle, or packet-inventory promotion.

## Validation evidence

- Baseline targeted packet/matrix/bundle checks: `docs/evidence/resource-pack-status-local-rail-baseline-2026-06-07.run.log` (`exit_status=0`).
- Implementation checks: `docs/evidence/resource-pack-status-local-rail-checks-2026-06-07.run.log` (`exit_status=0`).
- Blocker live-checker assertion: `docs/evidence/resource-pack-status-local-rail-blocker-checker-2026-06-07.run.log` (`exit_status=0`).
- Non-promotion matrix/bundle/targeted checks: `docs/evidence/resource-pack-status-local-rail-nonpromotion-checks-2026-06-07.run.log` (`exit_status=0`).
- Cairn gates: `docs/evidence/resource-pack-status-local-rail-cairn-gates-2026-06-07.run.log` (`exit_status=0`).
- Evidence manifest refresh/checks and task-evidence gate: `docs/evidence/resource-pack-status-local-rail-evidence-manifest-refresh-2026-06-07.run.log` and `docs/evidence/resource-pack-status-local-rail-evidence-checks-2026-06-07.run.log` (`exit_status=0`).
- Sync/archive/post-archive validation: `docs/evidence/resource-pack-status-local-rail-sync-2026-06-07.run.log`, `docs/evidence/resource-pack-status-local-rail-post-sync-validate-2026-06-07.run.log`, `docs/evidence/resource-pack-status-local-rail-archive-2026-06-07.run.log`, `docs/evidence/resource-pack-status-local-rail-final-manifest-refresh-2026-06-07.run.log`, and `docs/evidence/resource-pack-status-local-rail-post-archive-checks-2026-06-07.run.log` (`exit_status=0`).

## Non-claims

This change does not claim resource-pack download/application behavior, trust/security validation, all status variants, public-server safety, production readiness, broad Minecraft compatibility, or full protocol 763 compatibility.
