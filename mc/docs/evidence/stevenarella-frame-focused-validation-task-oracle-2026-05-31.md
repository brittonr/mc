# Stevenarella frame focused validation task oracle — 2026-05-31

## Question
Can task 6 in `cairn/changes/stevenarella-frame-capture-artifacts/tasks.md` be marked complete after adding focused positive and negative validation tests?

## Inspected evidence
- `stevenarella` child commit `02aca92` (`harden capture validation coverage`) adds focused tests named `focused_validation_covers_valid_screenshot_metadata`, `focused_validation_covers_vertical_flip_normalization`, `focused_validation_rejects_invalid_format`, `focused_validation_rejects_path_traversal`, `focused_validation_rejects_capture_rate_limit`, and `focused_validation_rejects_unbounded_recording`.
- The same child commit adds an explicit pending-capture rate limit in `src/capture.rs` (`MAX_PENDING_CAPTURE_REQUESTS`) so the rate-limit rejection test proves behavior rather than only documenting a wait path.
- `docs/evidence/stevenarella-frame-focused-validation-source-2026-05-31.patch` records the exact child diff for `src/capture.rs`, `src/main.rs`, and `src/mcp.rs`.
- `docs/evidence/stevenarella-frame-focused-validation-2026-05-31.run.log` records `cargo fmt --check`, `cargo test focused_validation --lib`, `cargo test capture --lib`, full `cargo test --lib`, and `cargo check --bin stevenarella` with `exit_status=0`.
- `docs/evidence/stevenarella-frame-focused-validation-2026-05-31.b3` hashes the validation log and source patch.
- `docs/evidence/stevenarella-frame-focused-validation-source-2026-05-31.b3` hashes the source patch.

## Finding
The completed task text requires focused tests for valid screenshot metadata, vertical flip normalization, invalid format, path traversal, rate-limit rejection, and unbounded-recording rejection. The new focused tests cover exactly those six validation families, while the full library test confirms the added rate-limit behavior does not regress existing capture, MCP, control, server, or world tests. This evidence does not claim final archive readiness or live MCP-controlled compatibility evidence.

## Owner
Britton Robitzsch, mc compatibility owner.

## Decision
Mark task 6 complete. Do not mark final artifacts/archive task complete from this evidence.

## Follow-up
Next task remains `r[mc_compatibility.stevenarella_frame_capture.artifacts]`: record final reviewable capture/test/Cairn validation output under `docs/evidence/` with BLAKE3 manifests before archiving.
