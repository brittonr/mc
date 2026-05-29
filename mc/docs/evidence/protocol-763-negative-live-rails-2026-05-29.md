# Protocol-763 negative live rails

This evidence covers bounded negative compatibility rails against owned-local Valence CTF. These rows prove only that one invalid action per rail is contained or disconnected without promotion to the target success milestone. They do not claim broad invalid-input coverage, adversarial security, production readiness, full inventory transaction semantics, broad plugin-message semantics, or full CTF correctness.

## Child revisions

- Parent workspace change: active Cairn `add-negative-live-rails`.
- Stevenarella probe commit: `a3c362c` (`add bounded negative compatibility probes`).
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

## Decisions

- Inventory stale state and invalid slot/window rails are containment evidence only. They do not promote all state-id freshness, stack merge/split, drag, malformed click, or all-window behavior.
- Custom payload rail proves one malformed `mc_compat:malformed` payload does not crash the owned-local run. It does not promote plugin-message semantics broadly.
- Wrong score and reconnect race rails require no forbidden score/capture milestones in receipts. They do not broaden the CTF rule ledger beyond the named invalid transitions.
- Negative rails stay out of the acceptance-matrix promoted seam table until a future promotion change adds dedicated checker coverage and updates the matrix/bundle row count.
