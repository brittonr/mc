# Tasks

- [ ] [serial] Define the negative live-rail safety envelope, target ownership fields, bounds, expected outcome vocabulary, and receipt non-claims. r[mc_compatibility.negative_live_rails.envelope]
- [ ] [depends:envelope] Add dry-run receipt support and checker fixtures that reject unbounded, public, unauthenticated, missing-telemetry, or missing-expected-outcome negative rails. r[mc_compatibility.negative_live_rails.dry_run]
- [ ] [depends:dry_run] Add a stale inventory state-id rail with live Valence containment/rejection evidence and explicit non-claims for other inventory transactions. r[mc_compatibility.negative_live_rails.inventory_stale_state]
- [ ] [depends:dry_run] Add an invalid slot/window click rail with live containment/rejection evidence and slot restoration diagnostics. r[mc_compatibility.negative_live_rails.inventory_invalid_click]
- [ ] [depends:dry_run] Add a malformed custom-payload rail with live rejection/disconnect classification and no broad plugin-message semantics claim. r[mc_compatibility.negative_live_rails.custom_payload]
- [ ] [depends:dry_run] Add a reconnect flag-state race rail that proves one bounded duplicate/reconnect invalid transition is contained without score corruption. r[mc_compatibility.negative_live_rails.reconnect_race]
- [ ] [depends:dry_run] Add a wrong team/score path rail that proves one invalid scoring attempt does not produce a promoted score milestone. r[mc_compatibility.negative_live_rails.ctf_wrong_score]
- [ ] [depends:ctf_wrong_score] Copy live receipts/logs under `docs/evidence/`, update matrix/current-bundle non-claims, run evidence manifests, maintained dry-runs, and Cairn validation. r[mc_compatibility.negative_live_rails.validation]
