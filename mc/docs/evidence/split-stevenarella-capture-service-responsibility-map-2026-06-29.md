# Stevenarella capture service responsibility map

## Question

Which responsibilities previously lived together in `clients/stevenarella/src/capture.rs`, and where are they owned after Cairn change `split-stevenarella-capture-service`?

## Inspected evidence

- Change scope: `cairn/changes/split-stevenarella-capture-service/{proposal.md,design.md,tasks.md}` and `cairn/changes/split-stevenarella-capture-service/specs/mc-compatibility/spec.md`.
- Affected subtree workflow: `clients/stevenarella/AGENTS.md` and `clients/stevenarella/README.md`.
- Baseline logs: `docs/evidence/run-logs/2026-06-29/split-stevenarella-capture-service.baseline-capture-tests.run.log` and `docs/evidence/run-logs/2026-06-29/split-stevenarella-capture-service.baseline-mcp-capture-tests.run.log`.
- Post-change logs: `docs/evidence/run-logs/2026-06-29/split-stevenarella-capture-service.post-format-capture-tests.run.log`, `docs/evidence/run-logs/2026-06-29/split-stevenarella-capture-service.post-format-mcp-capture-tests.run.log`, `docs/evidence/run-logs/2026-06-29/split-stevenarella-capture-service.post-stevenarella-dry-run.run.log`, and `docs/evidence/run-logs/2026-06-29/split-stevenarella-capture-service.post-mc-compat-smoke-valence-dry-run.run.log`.

## Responsibility owners

| Responsibility | Owner after refactor | Side-effect boundary |
| --- | --- | --- |
| Public capture request, plan, metadata, digest, error, policy, and frame model shapes | `clients/stevenarella/src/capture/model.rs` | Plain data and validation-adjacent constructors only; no filesystem, renderer, channel, or clock side effects. |
| Request validation, artifact path containment, dimensions, recording bounds, and metadata checks | `clients/stevenarella/src/capture/validation.rs` | Pure over explicit request/policy/metadata inputs; returns plans or diagnostics. |
| Queue send/receive, pending-slot accounting, and receiver-open tracking | `clients/stevenarella/src/capture/queue.rs` | Channel and atomic state stay in the queue adapter; request shape validation runs before enqueue. |
| Framebuffer readback normalization and buffer length decisions | `clients/stevenarella/src/capture/readback.rs` | Buffer sizing and row normalization are pure; `gl::read_pixels_rgba` remains in the readback shell. |
| PNG encoding, BLAKE3 metadata, artifact size guard, directory creation, and file writes | `clients/stevenarella/src/capture/persistence.rs` | PNG bytes and digest are computed at the persistence boundary; filesystem writes remain there. |
| Recording startup, cadence/state decisions, frame path planning, and recording servicing | `clients/stevenarella/src/capture/recording.rs` | `recording_cadence_decision` is pure over `RecordingCadenceSnapshot`; readback and persistence are called only from service shells. |
| One-shot capture orchestration | `clients/stevenarella/src/capture/service.rs` | Validation, readback callback, and persistence are wired in a thin shell. |
| Stable public capture API facade and focused positive/negative tests | `clients/stevenarella/src/capture.rs` and `clients/stevenarella/src/capture/tests.rs` | Existing callers keep `crate::capture::*`; tests exercise pure cores and side-effect shells with in-memory or temp-dir fixtures. |

## Decision

The capture refactor keeps request types, artifact path semantics, BLAKE3 metadata, redaction state, recording bounds, MCP-facing outputs, and non-claim boundaries stable while giving each capture responsibility a focused owner. Side effects are isolated to queue, readback, persistence, and service shells; dimension/path/digest/metadata/recording cadence decisions are testable with explicit in-memory inputs.

## Non-claims

This evidence is architecture and focused-test evidence only. It does not claim broad Minecraft compatibility, rendering correctness, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness.
