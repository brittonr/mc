# Protocol-763 public-server authorized safety contract — 2026-05-30

## Scope

`public-server-authorized-safety` covers one deterministic, authorized, non-loopback fixture envelope. It proves the evidence contract, redaction policy, authorization checkpoint, and fail-closed promotion gate before any live public target can be claimed.

This row promotes only `covered_authorized_fixture_only`. third-party target safety without authorization remains a non-claim, live public-server safety remains a non-claim, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing remain non-claims.

## Normalized metric contract

| Metric | Required value |
| --- | --- |
| row | `public-server-authorized-safety` |
| status | `covered_authorized_fixture_only` |
| target_owner | `review-fixture-owner` |
| authorization_artifact | `docs/evidence/protocol-763-public-server-authorized-safety-checkpoint-2026-05-30.md` |
| target_scope | `authorized-non-loopback-fixture` |
| client_count | `1` |
| duration_secs | `30` |
| traffic_limits | `client_count<=1,duration_secs<=30,status_probe_only,live_traffic_enabled=false` |
| telemetry | `target_owner`, `authorization_artifact`, `target_scope`, `client_count`, `duration_secs`, `traffic_limits`, `abort_criteria`, `redaction_policy`, `checkpoint_decision` |
| abort_criteria | `missing_authorization_or_bound_violation` |
| redaction_policy | `no_secrets_no_raw_public_address` |
| checkpoint_decision | `approved_for_deterministic_fixture_only` |

Inline key/value aliases required by the checker: `client_count=1`, `duration_secs=30`, `status_probe_only`, `live_traffic_enabled=false`, `redaction_policy=no_secrets_no_raw_public_address`.

## Fail-closed rejection vocabulary

The row checker rejects these promotion gaps before any matrix/current-bundle claim:

- `missing_owner`
- `missing_written_authorization`
- `missing_bounds`
- `missing_telemetry`
- `missing_checkpoint`
- `secret_leak`
- `production_readiness_overclaim`
- `public_server_overclaim`

## Required evidence standard

- Checkpoint: repo-local authorization/oracle checkpoint with `## Question`, `## Inspected evidence`, `## Decision`, `## Owner`, and `## Next action`.
- Receipt: repo-local dry-run fixture receipt under `docs/evidence/` with `public_server_authorized_safety.selected=true` and `live_traffic_enabled=false`.
- Record: key/value normalized metrics under `docs/evidence/` for deterministic positive and negative fixture validation.
- Checker: `tools/check_public_server_authorized_safety.rs --self-test` and repo check output copied under `docs/evidence/`.
- Matrix/current bundle: promote only the authorized deterministic fixture envelope; keep live public-server safety and production readiness false.
