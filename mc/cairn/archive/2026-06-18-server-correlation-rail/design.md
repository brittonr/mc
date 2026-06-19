## Context

The current compatibility evidence model has targeted-packet KV receipts and live-capability registry entries. Those entries can keep rows fixture-bounded, but they do not provide a reusable fail-closed proof that a server-side event correlates with a specific Stevenarella driver milestone.

## Design

Introduce a narrow `mc.compat.server_correlation_receipt.v1` JSON receipt validated by a single-file Rust checker. The checker is the functional core: it parses in-memory JSON into deterministic receipt structs, validates each row-specific contract, and returns diagnostics without touching the filesystem. The imperative shell reads file paths from CLI arguments, runs the core, and prints diagnostics.

The first supported rows are:

- `resource-pack-status`: owned-local offer `mc-compat-local-resource-pack`, client milestone `resource_pack_status_sent`, server event `resource_pack_status_declined_observed`, no external fetch, and status `declined`.
- `sign-editor-open-update`: sign position `28,64,0`, payload `MC|Compat|Sign|Edit`, client milestones `sign_editor_open_observed` and `sign_update_sent`, and server event `sign_update_accepted_observed`.

Receipts must include row, scenario, actor, backend path, client path, client/backend revisions, owned-local scope, redaction policy, packet rows, client milestones, server events, correlation status, and non-claims. A promotion-capable receipt must set `correlation.status=observed`; placeholder/blocker receipts must fail if passed as promotion evidence.

## Non-goals

This change does not run a live Valence/Paper server, update targeted packet rows to `passed`, claim public-server safety, or prove broad sign/resource-pack semantics. It adds the maintained rail and fail-closed receipt contract that future live runs must satisfy.
