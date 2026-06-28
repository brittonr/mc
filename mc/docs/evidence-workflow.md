# mc-compat evidence workflow

This page owns evidence interpretation, promotion, and Cairn task-citation guidance for the mc compatibility workspace. It preserves the claim boundaries from the root README: evidence rows do not imply broad Minecraft compatibility, semantic equivalence, public-server safety, production readiness, full CTF correctness, or full survival correctness unless a separate accepted aggregate gate explicitly says so.

## Receipt scope and schemas

The current receipt schema is `mc.compat.scenario.receipt.v2`; receipts also retain the legacy marker `mc.compat.smoke.receipt.v1` for older consumers. A receipt records server/client inputs, the headless-isolation contract (`wayland_socket_inherited=false`), typed scenario milestones, server-side correlation when available, and explicit non-claims (`claims_correctness=false`, `claims_semantic_equivalence=false`) for downstream Cairn/Octet review.

Dry-run receipts are deterministic harness-shape evidence only. Live/reference parity claims remain tied to promoted evidence rows and paired comparators. A receipt is evidence that the bounded scenario ran under the specified local fixture, not a claim of full semantic equivalence.

## Failure bundles

Failed runs can write `mc.compat.failure.bundle.v1` diagnostics with `--failure-bundle docs/evidence/<name>.failure-bundle.json` or `MC_COMPAT_FAILURE_BUNDLE`.

Use failure bundles only after copying the receipt/log/typed-event/stderr artifacts you want reviewers to inspect under `docs/evidence/`. The validator rejects path escapes, target-only paths, result-only paths, root-transient paths, malformed BLAKE3 digests, missing artifacts, missing non-claims, and success-labeled bundles.

Failure bundles are diagnostic only: they do not claim scenario success, gameplay parity, full protocol compatibility, public-server safety, production readiness, or semantic equivalence. Record a `.b3` for the bundle and any critical copied artifacts before citing them from Cairn tasks.

## Scenario receipt blocks

`valence-compat-bot-probe` receipts add a `compat_bot_probe` block that records the owned local target, bounded one-client limit, non-public-stress-tool guard, and explicit `external_server_load_authorized=false` non-claim. `reconnect-flag-score` extends gameplay evidence with an explicit reconnect milestone.

Receipts also include bounded blocks for the remaining compatibility seams:

- `status_response_resource`: configured/default status description, version, and player sample expectations used by the status probe.
- `packet_capture_oracle`: headless/redacted packet-summary metadata; raw payloads are not durable evidence by default.
- `typed_event_oracle`: typed event schema/migration metadata. Dry-run and failure receipts mark `migration_status="substring-fallback"`; successful live receipts can write a `.typed-events.log` sidecar derived from client/server milestone evidence and record `event_log_path`, normalized `timeline_blake3`, `event_count`, `contributes_to_pass_fail`, and `raw_payloads_recorded=false`.
- `biome_dimension_join_state`: selected only for `survival-biome-dimension-state`; records scenario identity, protocol context, client-observed join-state markers, server-configured fixture state, correlation diagnostics, and explicit non-claims. It is bounded to one configured join-state row and does not claim dimension travel, portal behavior, all biome semantics, full survival compatibility, broad vanilla parity, public-server safety, or production readiness.
- `negative_live_rail`: dry-run/live envelope metadata for bounded invalid-action scenarios. It records the selected rail, invalid action, expected containment/disconnect outcome vocabulary, observed outcome plus client postcondition milestone when live telemetry exists, owned-local/public authorization fields, client/time bounds, required evidence fields, and explicit non-claims for broad invalid-input, adversarial-security, production, inventory, plugin-message, and CTF semantics.
- `public_server_authorized_safety`: deterministic authorization fixture metadata. It records owner, authorization artifact, non-loopback fixture scope, client/duration/traffic bounds, redaction policy, checkpoint decision, `live_traffic_enabled=false`, and explicit non-claims for live public-server safety, third-party targets without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing.
- `proxy_compat_seam`: direct/proxied route, forwarding mode, owned-local-proxy guard, and non-claims such as `mtls_ported=false` and `credentials_recorded=false`.
- `gameplay_oracles`: Hyperion-derived milestone vocabulary, correlated-evidence requirement, and explicit non-claims for full CTF correctness, broad compatibility, and unbounded soak.

For `flag-score-repeat`, `reconnect-flag-score`, and `multi-client-load-score`, Valence receipts include `server.required_milestones`, `server.observed_milestones`, `server.missing_milestones`, `server.forbidden_matches`, and `server.client_server_correlation`. Multi-client receipts also include `client.usernames` and `client.log_paths` for per-client inspection.

All scenario receipts include a `triage` block with first missing client/server milestones, first forbidden pattern/source, relevant client/server log paths, and a `suggested_boundary` such as `client-probe`, `server-correlation`, `protocol-runtime`, or `preflight-or-server-startup`. The nested `triage.enriched` block adds bounded/redacted context (`last_client_event`, `last_server_event`, `correlation_ids`, `timeline_excerpt`, and `boundary_confidence`) for debugging only; failure triage is not compatibility coverage.

## Typed-event migration caveats

The scenario manifest marks many maintained rows as `typed-event-ready`; those rows have typed-event readiness fixtures and fail closed on missing structured events before substring fallback can satisfy pass/fail. Remaining maintained rows stay waiver-backed substring fallback under the manifest-level owner/reason/non-claim/next-action metadata until typed-event fixtures cover their client, server, and forbidden surfaces.

The CTF wave changes observability/pass-fail only and does not claim full CTF correctness, all races, all invalid actions, adversarial security, public-server safety, production readiness, or vanilla/reference parity. Receipt-schema tests structurally parse evidence-critical JSON fields instead of accepting substring-only matches. The structured checks cover non-claims, child revision cleanliness, typed-event artifact identity, MCP control evidence, frame artifact paths/digests, backend identity, duplicate/wrong-typed fields, and broad overclaim keys; retained substring checks are limited to intentionally free-form text surfaces.

## Scenario-derived surfaces

Scenario-derived harness surfaces are generated from `compat/config/scenario-manifest.ncl` into checked-in static files. `compat/runner/src/scenario_manifest_generated.rs` remains Rust-only at runtime. [evidence/mc-compat-scenario-index.generated.md](evidence/mc-compat-scenario-index.generated.md) is a bounded machine-owned index for reviewer navigation, and [scenario-commands.generated.md](scenario-commands.generated.md) is the machine-owned command table for maintained scenario rows.

Refresh with `tools/check_scenario_manifest.rs --write-generated-surfaces` and verify with the `mc-compat-generated-harness-surfaces` flake check.

The scenario fallback-budget baseline is checked in at `compat/config/scenario-fallback-budget-baseline.ncl`. `tools/check_scenario_manifest.rs` reports `approved`, `removed`, `new`, `typed_event_regressions`, and `missing_waiver_metadata` rows; new fallback debt, missing waiver metadata, and typed-event regressions fail closed. This is migration accounting only and does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, or production readiness.

## Evidence promotion and BLAKE3 manifests

Evidence promotion plans use the typed shape in `compat/config/evidence-promotion-plan.ncl` and the Rust tool `tools/promote_evidence.rs`. Safe workflow: run `nix build .#checks.x86_64-linux.mc-compat-evidence-promotion --no-link -L`, inspect the dry-run plan, apply only to an explicit output directory, then run acceptance matrix, current bundle, evidence manifest, and Cairn validation before claiming a row. The tool never force-adds broad directories; it copies only planned artifacts and writes `promotion-plan.md`.

Evidence BLAKE3 manifests can be checked or refreshed with:

```sh
nix run .#evidence-manifest-refresh -- --check
nix run .#evidence-manifest-refresh -- --refresh
```

The helper scans reviewable `docs/evidence/*.b3` manifests, rewrites stale digest fields only in explicit refresh mode, leaves missing-file rows visible for review, and repeats until a deterministic fixpoint or a non-convergence diagnostic.

Run `nix build .#checks.x86_64-linux.mc-compat-evidence-manifest-refresh --no-link -L` before the broader evidence-manifest/task-evidence gates when a Cairn drain updates logs, accepted specs, archive tasks, or nested manifests.

## Cairn task closeout evidence

Cairn task closeout evidence is checked by `tools/check_cairn_task_evidence.rs` and the flake check `mc-compat-cairn-task-evidence`. Before marking an active Cairn task complete, cite copied `docs/evidence/` command output such as a `.run.log` plus either its `.b3` manifest or an inline BLAKE3 digest.

Missing files, target-only receipts, result-only outputs, root-transient logs, retired root `evidence/` paths, and checked tasks without verification output fail the gate. Task-cited `.run.log` files must contain an explicit `exit_status=0`.
