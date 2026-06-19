# Stevenarella sign-editor driver evidence (2026-06-07)

## Implementation

Stevenarella child repo revision `c27f0db` implements a bounded `sign_editor_update` / `sign-editor-update` MCP control command. The pure validation path in `src/control.rs` validates explicit inputs before any protocol output:

- matching `position` and `open_position`
- `open_observed=true`
- exactly four sign lines
- per-line maximum of 64 characters
- malformed/non-string payload rejection
- explicit overclaim rejection via `claim_broad_sign_editing=true`

The main-thread shell in `src/main.rs` rejects disconnected clients through the existing connected-command gate, confirms the tracked `SignEditorOpen` position from `src/server/mod.rs`, writes a serverbound `SetSign` packet for the configured payload, and logs `sign_update_sent`. It does not synthesize host OS input, mutate sign state from a worker thread, or claim sign persistence.

## Tests and integration

- `docs/evidence/stevenarella-sign-editor-driver-baseline-2026-06-07.run.log` records the prior live blocker, runs pre-driver Stevenarella control tests from child revision `01fb507`, records a sign-editor dry-run, and runs the targeted-packet fixture check.
- `docs/evidence/stevenarella-sign-editor-driver-tests-2026-06-07.run.log` records positive valid open/update coverage plus negative missing-open-state, wrong-position, malformed-payload, line-count, line-length, disconnected-state, and overclaim coverage.
- `docs/evidence/stevenarella-sign-editor-driver-integration-2026-06-07.run.log` records runner scenario-core tests, scenario-manifest validation, a sign-editor dry-run, and targeted-packet nonpromotion checks after the isolated capability-registry update.

## Live promotion status

The `sign-editor-open-update` capability registry row now names `stevenarella-sign-editor-driver` as the client path, but remains fixture-bounded/blocked because no maintained live server-correlation receipt was produced in this change.

## Non-claims

No sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, public-server safety, production readiness, full protocol 763 compatibility, or broad Minecraft compatibility is claimed.
