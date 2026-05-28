# Protocol-763 survival Paper reference fixture checkpoint — 2026-05-28

## Review-critical question

Can the current local Paper fixture produce a reviewable reference receipt for the same `survival-break-place-pickup` probe?

## Candidate fixture

- Backend: `paper`
- Image: `itzg/minecraft-server:java17`
- Server version: `1.20.1`
- Protocol: `763`
- Container: `mc-compat-paper-survival-reference-20260528`
- Command:

```sh
SERVER_PROTOCOL=763 SERVER_VERSION=1.20.1 SERVER_NAME=mc-compat-paper-survival-reference-20260528 CLIENT_TIMEOUT=180 SMOKE_RECEIPT=target/mc-compat-survival-reference-parity/paper-survival-reference.json nix run --no-update-lock-file .#mc-compat-smoke -- --run --server-backend paper --scenario survival-break-place-pickup --receipt target/mc-compat-survival-reference-parity/paper-survival-reference.json
```

## Inspected evidence

| Evidence | Observation |
| --- | --- |
| `docs/evidence/protocol-763-survival-reference-paper-dry-run-matrix-2026-05-28.run.log` | Dry-run matrix for Paper+Valence now passes at protocol 763 after the runner compares against configured `SERVER_PROTOCOL` instead of hard-coded protocol 758. |
| `docs/evidence/protocol-763-survival-reference-paper-dry-run-matrix-2026-05-28.paper.receipt.json` | Dry-run Paper receipt records scenario `survival-break-place-pickup`, backend `paper`, protocol `763`, and deterministic dry-run shape. |
| `docs/evidence/protocol-763-survival-reference-paper-probe-2026-05-28.run.log` | Live Paper probe starts Paper 1.20.1/protocol 763 and reaches login/join/render/break-send, then fails with missing `survival_break_update`, `survival_pickup_seen`, `survival_place_sent`, `survival_place_update`, and forbidden `panic`. |
| `docs/evidence/protocol-763-survival-reference-paper-probe-2026-05-28.client-compatbot.log` | Stevenarella panics on vanilla/Paper play traffic with `bad packet id 0x6b in Clientbound Play` before observing the required survival update/pickup/place milestones. |
| `docs/evidence/protocol-763-survival-reference-paper-probe-2026-05-28.paper.log` | Paper accepts the offline local client, then disconnects it with `Bad packet id 105`. No plugin/server-side `MC-COMPAT-MILESTONE survival_*` events exist for the comparator's server metrics. |
| `docs/evidence/protocol-763-survival-reference-paper-probe-2026-05-28.receipt.json` | Receipt status is `fail`; `server.backend=paper`, `server.protocol=763`, and the error preserves the missing milestone list. |

## Decision

The current local Paper Docker fixture is a valid candidate reference backend for protocol 763 status/login, but it is not yet a usable reference receipt for the survival parity rail. Two blockers remain:

1. Stevenarella's vanilla/Paper protocol path panics on clientbound play packet `0x6b` before the survival update/pickup/place milestones complete.
2. Plain Paper lacks a fixture plugin or other server-side instrumentation that emits normalized `server_survival_*` metrics required by `tools/check_survival_reference_parity.py`.

Do not check off the reference receipt or comparator tasks yet. Do not promote survival parity.

## Owner

Compatibility evidence owner: `mc` maintainers.

## Next action

Add a Paper reference fixture plugin or equivalent harness that sets the same target block/player state and emits normalized server-side survival milestones, and patch/extend Stevenarella's Paper 1.20.1 parser path for packet `0x6b`. Then rerun the live Paper receipt and the paired parity comparator.
