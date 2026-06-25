# isolate-runner-json-evidence inventory and boundary

## Question
Which runner JSON/evidence schemas are compatibility-sensitive, and where does the extraction boundary live?

## Inventory

| Surface | Consumer | Compatibility-sensitive fields | Owner after extraction |
| --- | --- | --- | --- |
| Smoke/scenario receipt `mc.compat.scenario.receipt.v2` plus legacy marker `mc.compat.smoke.receipt.v1` | Cairn task evidence, compare-receipts, evidence bundle promotion, README-documented receipt workflow | `schema`, `legacy_schema`, `contract.claims_*`, scenario/server/client summaries, child revision fields, non-claims | `compat/runner/src/receipts.rs`, `compat/runner/src/receipt_validation.rs`, `compat/runner/src/evidence_types.rs` |
| Failure bundle `mc.compat.failure.bundle.v1` | Fail-only diagnostic evidence under `docs/evidence/` | `diagnostic_only`, artifact path containment, BLAKE3, required non-claims, false success/parity/public-server/production claims | `compat/runner/src/evidence_bundle.rs` |
| Typed-event artifact and receipt block | Typed-event oracle checks and migrated scenario graph tests | `schema_version`, event log path, timeline BLAKE3, event count, pass/fail contribution, raw-payload non-claim | `compat/runner/src/evidence_core.rs`, `compat/runner/src/evidence_types.rs` |
| MCP control receipt block | MCP controlled smoke evidence and frame artifact checks | tool digest/names, call outcomes, stdout cleanliness, child revision status, non-claims | `compat/runner/src/evidence_receipts.rs`, `compat/runner/src/evidence_types.rs` |
| Frame artifacts receipt block | MCP frame capture evidence | artifact path/relative path, dimensions, frame/sequence IDs, byte length, BLAKE3, redaction, UI flag, non-claims | `compat/runner/src/evidence_receipts.rs`, `compat/runner/src/evidence_types.rs` |
| Latency/jitter and load/network safety receipts | Bounded WAN/load envelope checks | target ownership/authorization, bounded metrics, client/duration/reconnect limits, telemetry readiness, false WAN/public/production claims | env collection in `main.rs`; pure typed validation/rendering in `compat/runner/src/evidence_receipts.rs` |
| Public-server authorized safety receipt | Dry-run authorization-envelope evidence | owner, authorization artifact, target scope, traffic limits, checkpoint decision, fixture-only and false live-public claims | env collection in `main.rs`; pure rendering in `compat/runner/src/evidence_receipts.rs` |
| Projectile causality and scenario evidence | Scenario receipt pass/fail and typed-event derivation | observed/missing/forbidden milestones, ordered projectile steps, actor names, pass/fail | `compat/runner/src/evidence_core.rs`, `compat/runner/src/evidence_types.rs` |
| Compare-receipt inputs | Paper/Valence receipt comparison | backend/protocol/port, pass status, headless isolation, false correctness/equivalence claims | `compat/runner/src/receipt_validation.rs` |

## Boundary decision

`main.rs` remains the imperative shell for CLI parsing, environment reads, process execution, file IO, directory creation, BLAKE3 file hashing, stdout/stderr, and exit-code handling. The extracted modules own in-memory schema data, JSON escaping/parsing helpers, failure-bundle validation/rendering, receipt evidence validation/rendering, and typed-event graph logic.

Dependency policy: keep the runner dependency-light for this change. No `serde` dependency was added; hand-rolled JSON helpers are centralized in `json_support.rs` with positive escaping tests and negative malformed/duplicate-key tests.

## Decision

The extraction preserves schema names, legacy markers, non-claim fields, overclaim rejection, reviewable evidence-path rules, and existing compare/validation behavior. It does not promote broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, full survival correctness, or vanilla parity.

## Owner

`compat/runner`.

## Next action

Use the focused runner tests and dry-run/evidence gates recorded for `isolate-runner-json-evidence` before syncing and archiving the Cairn change.
