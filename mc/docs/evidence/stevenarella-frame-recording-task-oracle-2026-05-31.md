# Stevenarella frame recording task oracle — 2026-05-31

## Question
Can task 4 in `cairn/changes/stevenarella-frame-capture-artifacts/tasks.md` be marked complete after adding bounded `--capture-dir` artifact persistence and recording support?

## Inspected evidence
- `stevenarella` child commit `303fff2` (`make frame captures durable`) adds BLAKE3-backed PNG artifact persistence, `CapturePolicy::local`, sequence-aware artifact metadata, bounded `RecordingSession`, startup `--capture-dir` / `--capture-record-*` CLI wiring, and post-render servicing for active recordings.
- `docs/evidence/stevenarella-frame-recording-source-2026-05-31.patch` records the exact child diff for `Cargo.toml`, `Cargo.lock`, `src/capture.rs`, and `src/main.rs`.
- `docs/evidence/stevenarella-frame-recording-validation-2026-05-31.run.log` records `cargo fmt --check`, `cargo test capture --lib`, full `cargo test --lib`, and `cargo check --bin stevenarella` with `exit_status=0`.
- `docs/evidence/stevenarella-frame-recording-validation-2026-05-31.b3` hashes the validation log and source patch.
- `docs/evidence/stevenarella-frame-recording-source-2026-05-31.b3` hashes the source patch.

## Finding
The completed task text requires bounded recording to `--capture-dir` with fps, duration/frame-count, byte, and path-containment guards. The source evidence validates capture directory containment before writes, rejects missing capture directories/path traversal/inline recording/unbounded recordings, checks fps and duration/frame-count bounds, guards PNG artifact byte size before writes, records BLAKE3 digest metadata, and tests that recording respects frame-rate delay and max-frame completion. This evidence does not claim MCP capture resources, large-output resource negotiation, visual regression approval, semantic gameplay correctness, or live MCP-controlled compatibility evidence.

## Owner
Britton Robitzsch, mc compatibility owner.

## Decision
Mark task 4 complete. Do not mark MCP resources, validation, final artifacts, or MCP-controlled live rail tasks complete from this evidence.

## Follow-up
Next task remains `r[mc_compatibility.stevenarella_frame_capture.mcp_resources]`: expose MCP capture tools/resources returning image content for bounded single frames and artifact path plus BLAKE3 digest for durable files.
