# Server-correlation rail contract (2026-06-18)

## Purpose

The server-correlation rail defines the receipt contract future owned-local live promotions must satisfy before resource-pack status or sign-editor driver output can change targeted-packet row status.

## Receipt contract

Schema: `mc.compat.server_correlation_receipt.v1`.

Required common fields:
- `receipt_kind`: `owned-local-live` for promotable receipts, `checker-fixture` only for checker fixtures
- `row`, `scenario`, `actor=compatbot`, `scope=owned-local`
- `packet_rows`, `client_milestones`, `server_events`
- `backend_path`, `client_path`, `backend_revision`, `client_revision`
- `correlation_status=observed`
- `redaction_policy=no-secrets-no-public-addresses`
- nonclaims for public-server safety, production readiness, full protocol 763 compatibility, and broad Minecraft compatibility

Supported initial rows:
- `resource-pack-status`: requires offer `mc-compat-local-resource-pack`, status `declined`, no external fetch, milestone `resource_pack_status_sent`, and event `resource_pack_status_declined_observed`.
- `sign-editor-open-update`: requires sign position `28,64,0`, payload `MC|Compat|Sign|Edit`, milestones `sign_editor_open_observed` and `sign_update_sent`, and event `sign_update_accepted_observed`.

## Fail-closed behavior

`tools/check_server_correlation_receipts.rs` validates the schema with a pure in-memory core and a thin CLI file-reading shell. `--promotable` rejects `checker-fixture` receipts so fixture receipts cannot be mistaken for live promotion evidence.

## Non-promotion status

This change adds the maintained rail and fixture validation only. It does not run a live server receipt and does not promote `resource-pack-status` or `sign-editor-open-update` targeted-packet rows.

## Non-claims

No public-server safety, production readiness, full protocol 763 compatibility, broad Minecraft compatibility, arbitrary sign semantics, all block entities, or resource-pack asset trust is claimed.
