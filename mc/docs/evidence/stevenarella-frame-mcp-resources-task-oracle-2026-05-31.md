# Stevenarella frame MCP resources task oracle — 2026-05-31

## Question
Can task 5 in `cairn/changes/stevenarella-frame-capture-artifacts/tasks.md` be marked complete after exposing MCP capture tools/resources for inline images and durable artifacts?

## Inspected evidence
- `stevenarella` child commit `ae529bb` (`expose frame capture through MCP`) wires `McpCaptureTools` into the stdio/TCP JSON-RPC runtime, shares the capture request queue with the post-render hook, and shares the capture sequence counter with control-queued captures.
- `docs/evidence/stevenarella-frame-mcp-resources-source-2026-05-31.patch` records the exact child diff for `src/mcp.rs` and `src/main.rs`.
- `docs/evidence/stevenarella-frame-mcp-resources-validation-2026-05-31.run.log` records `cargo fmt --check`, `cargo test mcp::tests --lib`, full `cargo test --lib`, and `cargo check --bin stevenarella` with `exit_status=0`.
- `docs/evidence/stevenarella-frame-mcp-resources-validation-2026-05-31.b3` hashes the validation log and source patch.
- `docs/evidence/stevenarella-frame-mcp-resources-source-2026-05-31.b3` hashes the source patch.

## Finding
The completed task text requires MCP capture tools/resources returning image content for bounded single frames and artifact path plus BLAKE3 digest for durable files. The source evidence adds `stevenarella.capture_screenshot` and `stevenarella.capture_latest_frame` tools, `stevenarella://capture/screenshot` and `stevenarella://capture/latest-frame` resources, inline PNG image/blob responses bounded by the existing inline response limit, artifact output with contained relative paths, BLAKE3 digest metadata, and tests for inline tool output, artifact tool output, resource read output, tool/resource listing, and invalid-format fail-closed behavior. This evidence does not claim visual regression approval, semantic gameplay correctness, live MCP-controlled compatibility evidence, or final archive readiness.

## Owner
Britton Robitzsch, mc compatibility owner.

## Decision
Mark task 5 complete. Do not mark focused validation, final artifacts, or MCP-controlled live rail tasks complete from this evidence.

## Follow-up
Next task remains `r[mc_compatibility.stevenarella_frame_capture.validation]`: add focused tests/evidence for valid screenshot metadata, vertical flip normalization, invalid format, path traversal, rate-limit rejection, and unbounded-recording rejection before archive.
