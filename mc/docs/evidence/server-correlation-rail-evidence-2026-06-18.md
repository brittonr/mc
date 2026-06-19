# Server-correlation rail evidence (2026-06-18)

## Implementation

`tools/check_server_correlation_receipts.rs` adds a deterministic owned-local server-correlation receipt checker. The core validates receipt text in memory and returns diagnostics; the CLI shell only reads file paths and prints results.

The checker supports the first two driver-backed rows:

- `resource-pack-status`: validates owned-local offer `mc-compat-local-resource-pack`, status `declined`, no external fetch, client milestone `resource_pack_status_sent`, and server event `resource_pack_status_declined_observed`.
- `sign-editor-open-update`: validates sign position `28,64,0`, payload `MC|Compat|Sign|Edit`, client milestones `sign_editor_open_observed` and `sign_update_sent`, and server event `sign_update_accepted_observed`.

`--promotable` rejects `checker-fixture` receipts and requires `owned-local-live`, preventing fixture receipts from being reused as live promotion evidence.

## Validation

- `docs/evidence/server-correlation-rail-baseline-2026-06-18.run.log` records pre-change Cairn gates plus scenario-manifest and targeted-packet nonpromotion checks.
- `docs/evidence/server-correlation-rail-checks-2026-06-18.run.log` records checker compile, self-test, positive fixture validation, expected promotable-mode rejection for checker fixtures, the flake check, targeted-packet nonpromotion, and scenario-manifest validation.

## Non-promotion status

This change creates the maintained rail only. It does not provide live Valence/Paper server receipts and does not promote `resource-pack-status` or `sign-editor-open-update` targeted-packet rows.

## Non-claims

No public-server safety, production readiness, full protocol 763 compatibility, broad Minecraft compatibility, arbitrary sign semantics, all block entities, or resource-pack asset trust is claimed.
