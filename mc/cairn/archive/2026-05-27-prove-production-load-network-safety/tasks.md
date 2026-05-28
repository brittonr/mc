# Tasks

- [x] [serial] Define authorization and safety preflight policy. r[mc_compatibility.prove_production_load_network_safety.authorization_preflight]
- [x] [serial] Define bounded load/network envelopes. r[mc_compatibility.prove_production_load_network_safety.bounded_envelopes]
- [x] [serial] Add positive and negative envelope fixtures. r[mc_compatibility.prove_production_load_network_safety.envelope_fixtures]
- [x] [serial] Promote only authorized bounded live receipts. r[mc_compatibility.prove_production_load_network_safety.safety_promotion_gate]

## Progress

- Runner preflight now rejects load/network runs when the target is marked public without `MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED=1` or when client/duration bounds are exceeded.
- Receipts now include `load_network_safety` with target scope, authorization, client/duration/reconnect/network bounds, telemetry readiness, preflight status, promotion readiness, and explicit false production/public/WAN/unbounded claims.
- Positive and negative unit fixtures cover owned-local bounded evidence and public/unbounded/missing-telemetry failure diagnostics.
- Evidence and policy are documented in `docs/evidence/protocol-763-load-network-safety-2026-05-27.md`.
- Live bounded receipt is tracked at `docs/evidence/protocol-763-load-network-safety-live-2026-05-27.receipt.json`; it records `server.protocol=763`, `mode=run`, `dry_run=false`, `status=pass`, `load_network_safety.live_receipt=true`, `telemetry_present=true`, and `promotion_ready=true`.
- Live receipt/log BLAKE3 sidecar is `docs/evidence/protocol-763-load-network-safety-live-2026-05-27.b3`.
- Gate evidence passed in `docs/evidence/protocol-763-load-network-safety-gate-2026-05-27.run.log` with sidecar `docs/evidence/protocol-763-load-network-safety-gate-2026-05-27.b3`.
