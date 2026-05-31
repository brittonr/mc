# Stevenarella frame screenshot hook task oracle — 2026-05-31

## Question
Can task 3 in `cairn/changes/stevenarella-frame-capture-artifacts/tasks.md` be marked complete after adding the post-render/pre-swap hook and wiring one-shot capture requests through the MCP/control queue?

## Inspected evidence
- `cairn/changes/stevenarella-frame-capture-artifacts/tasks.md`: task 3 is marked `[x]` and cites screenshot-hook evidence artifacts.
- `docs/evidence/stevenarella-frame-screenshot-hook-source-2026-05-31.patch`: child commit `33e5ec0` adds `CaptureRequestSender`, `CaptureRequestReceiver`, `service_one_shot_capture_request_with_readback`, screenshot/latest-frame queue tests, `ControlCommand::CaptureScreenshot`, `ControlCommand::CaptureLatestFrame`, MCP `enqueue_control` JSON-RPC coverage for `capture_screenshot`, and `Game::service_pending_mcp_capture_requests()` called immediately after `game.renderer.tick(...)` and before the native buffer swap in the event loop.
- `docs/evidence/stevenarella-frame-screenshot-hook-validation-2026-05-31.run.log`: records `cargo fmt --check`, `cargo test capture --lib` passing with 18 capture/MCP-control tests, and `cargo test parses_valid_initial_command_set --lib` passing.
- `docs/evidence/stevenarella-frame-screenshot-hook-validation-2026-05-31.b3`: hashes the validation log and source patch.
- `docs/evidence/stevenarella-frame-screenshot-hook-source-2026-05-31.b3`: hashes the source patch.

## Finding
The completed task text requires a post-render/pre-swap hook that services one-shot screenshot and latest-frame requests from the MCP/control queue. The source evidence now wires the sender into `Game`, adds control actions accepted through the existing MCP `stevenarella.enqueue_control` path, queues one-shot screenshot/latest-frame capture requests from the control handler, services at most one capture after `Renderer::tick(...)`, and rejects recording requests from the one-shot path. This evidence does not claim MCP capture resources, file-backed artifacts, PNG encoding, or bounded recording.

## Owner
Britton Robitzsch, mc compatibility owner.

## Decision
Keep task 3 marked complete. Do not mark the recording, MCP resources, validation, or artifacts tasks complete from this evidence.

## Follow-up
Next task remains `r[mc_compatibility.stevenarella_frame_capture.recording]`: implement bounded recording to `--capture-dir` with fps, duration/frame-count, byte, and path-containment guards.
