# Protocol-763 negative live rails

This evidence covers bounded negative compatibility rails against owned-local Valence CTF. These rows prove only that one invalid action per rail is contained or disconnected without promotion to the target success milestone. They do not claim broad invalid-input coverage, adversarial security, production readiness, full inventory transaction semantics, broad plugin-message semantics, or full CTF correctness.

## Child revisions

- Parent workspace change: archived Cairn `add-negative-live-rails`.
- Stevenarella probe commits: `a3c362c` (`add bounded negative compatibility probes`) and `199c817` (`record negative probe containment milestones`).
- Valence worktree: `main` resolved in each live receipt under `child_revisions.valence`.

## Evidence

| Rail | Receipt | Run/client/server evidence | Manifest |
| --- | --- | --- | --- |
| Dry-run envelope | `docs/evidence/negative-live-rails-dry-run-2026-05-29.json` | dry-run receipt only | `docs/evidence/negative-live-rails-dry-run-2026-05-29.b3` |
| Public target rejection | `docs/evidence/negative-live-rails-public-target-reject-2026-05-29.json` | `docs/evidence/negative-live-rails-public-target-reject-2026-05-29.log` | `docs/evidence/negative-live-rails-public-target-reject-2026-05-29.b3` |
| Stale inventory state id | `docs/evidence/negative-live-rails-inventory-stale-state-2026-05-29.json` | `.log`, `.client.log`, `.server.log`, `.typed-events.log` sidecars with same stem | `docs/evidence/negative-live-rails-inventory-stale-state-2026-05-29.b3` |
| Invalid slot/window click | `docs/evidence/negative-live-rails-inventory-invalid-click-2026-05-29.json` | `.log`, `.client.log`, `.server.log`, `.typed-events.log` sidecars with same stem | `docs/evidence/negative-live-rails-inventory-invalid-click-2026-05-29.b3` |
| Malformed custom payload | `docs/evidence/negative-live-rails-custom-payload-2026-05-29.json` | `.log`, `.client.log`, `.server.log`, `.typed-events.log` sidecars with same stem | `docs/evidence/negative-live-rails-custom-payload-2026-05-29.b3` |
| Wrong CTF score path | `docs/evidence/negative-live-rails-ctf-wrong-score-2026-05-29.json` | `.log`, `.client.log`, `.server.log`, `.typed-events.log` sidecars with same stem | `docs/evidence/negative-live-rails-ctf-wrong-score-2026-05-29.b3` |
| Reconnect race | `docs/evidence/negative-live-rails-reconnect-race-2026-05-29.json` | `.log`, `.client-session-1.log`, `.client-session-2.log`, `.server.log`, `.typed-events.log` sidecars with same stem | `docs/evidence/negative-live-rails-reconnect-race-2026-05-29.b3` |

## Observed outcomes

Each live receipt now carries `negative_live_rail.observed_outcome = "containment_observed"`, `negative_live_rail.observed_outcome_source = "client_milestone:<postcondition>"`, `negative_live_rail.postcondition_milestone`, and `negative_live_rail.telemetry_present = true`. The scenario `required_milestones`/`observed_milestones` also include the postcondition milestone:

- `negative_inventory_stale_state_contained`
- `negative_inventory_invalid_click_restored`
- `negative_custom_payload_contained`
- `negative_wrong_score_contained`
- `negative_reconnect_race_contained`

## Fail-closed fixture evidence

The focused unit fixture command `nix develop --no-update-lock-file --option substitute false -c cargo test --manifest-path tools/mc-compat-runner/Cargo.toml negative_live -- --nocapture` now runs six tests. They cover: passing envelope/receipt shape, public+unauthenticated rejection, unbounded client-count rejection, missing telemetry rejection, missing expected-outcome rejection, and live observed-containment receipt fields.

## Validation

- `docs/evidence/negative-live-rails-validation-2026-05-29.run.log`
- `docs/evidence/negative-live-rails-validation-2026-05-29.b3`

The validation log records `cargo fmt --check`, 73 runner tests, runner/scenario-manifest Nix checks, evidence manifest checks, acceptance/current-bundle checks, and Cairn validation after archive.

## Decisions

- Inventory stale state and invalid slot/window rails are containment evidence only. They do not promote all state-id freshness, stack merge/split, drag, malformed click, or all-window behavior.
- Custom payload rail proves one malformed `mc_compat:malformed` payload is followed by an explicit client containment milestone. It does not promote plugin-message semantics broadly.
- Wrong score and reconnect race rails require explicit containment milestones plus no forbidden score/capture milestones in receipts. They do not broaden the CTF rule ledger beyond the named invalid transitions.
- Negative rails stay out of the acceptance-matrix promoted seam table until a future promotion change adds dedicated checker coverage and updates the matrix/bundle row count.
