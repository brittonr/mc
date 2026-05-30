# Protocol-763 WAN tolerance bounded telemetry contract — 2026-05-29

## Scope

`wan-tolerance-bounded-telemetry` covers one owned-local, authorized, bounded telemetry row over the maintained `inventory-interaction` rail. It reuses the bounded Valence CTF inventory semantic oracle and records WAN-like delay/jitter metadata without privileged host network mutation.

This row promotes only the bounded owned-local telemetry envelope. Public-server safety, internet-path safety remains a non-claim, adversarial-network safety, production readiness, unbounded soak/reconnect safety, third-party target safety, and packet-loss tolerance beyond `loss_percent=0` remain non-claims.

## Normalized metric contract

| Metric | Required value |
| --- | --- |
| row | `wan-tolerance-bounded-telemetry` |
| scenario | `inventory-interaction` |
| mechanism | `bounded-client-cadence` |
| target_ownership | `owned-local-loopback` |
| authorization | `owned-local-fixture-approved` |
| delay_ms | `80` |
| jitter_ms | `30` |
| loss_percent | `0` |
| timeout_secs | `180` |
| duration_secs | `180` |
| client_count | `1` |
| reconnect_count | `0` |
| pass_fail_criteria | `inventory_interaction_client_server_milestones` |
| telemetry | `scenario_required_milestones`, `scenario_observed_milestones`, `server_required_milestones`, `server_observed_milestones`, `client_classification`, `triage_boundary` |

Inline key/value aliases required by the checker: `delay_ms=80`, `jitter_ms=30`, `loss_percent=0`, `timeout_secs=180`, `duration_secs=180`, `client_count=1`, `reconnect_count=0`.

## Fail-closed rejection vocabulary

The row checker and record fixture reject these promotion gaps before any matrix/current-bundle claim:

- `missing_authorization`
- `missing_target_ownership`
- `missing_delay_ms`
- `missing_jitter_ms`
- `missing_loss_percent`
- `missing_telemetry`
- `missing_fail_closed_preflight`
- `wan_overclaim`
- `public_target_overclaim`
- `production_readiness_overclaim`

## Required evidence standard

- Receipt: repo-local copy under `docs/evidence/` with `latency_jitter_tolerance.selected=true` and the normalized fields above.
- Record: key/value normalized metrics under `docs/evidence/` for deterministic positive and negative fixture validation.
- Checker: `tools/check_wan_tolerance_bounded_telemetry.rs --self-test` and repo check output copied under `docs/evidence/`.
- Matrix/current bundle: promote only the bounded owned-local WAN telemetry row; keep public/internet WAN safety, packet-loss tolerance beyond `loss_percent=0`, adversarial-network safety, and production readiness false.
