# Stevenarella frame readback task oracle — 2026-05-31

## Question
Can task 2 in `cairn/changes/stevenarella-frame-capture-artifacts/tasks.md` be marked complete after adding the GL RGBA readback helper and top-left-origin normalization core?

## Inspected evidence
- `cairn/changes/stevenarella-frame-capture-artifacts/tasks.md`: task 2 is marked `[x]` and cites readback evidence artifacts.
- `docs/evidence/stevenarella-frame-readback-source-2026-05-31.patch`: child commit `c7d6105` adds `gl::read_pixels_rgba`, `read_current_framebuffer_rgba_top_left`, `captured_rgba_from_bottom_left`, `normalize_rgba_bottom_left_to_top_left`, explicit RGBA buffer sizing, and readback error types.
- `docs/evidence/stevenarella-frame-readback-validation-2026-05-31.run.log`: records `cargo fmt --check` and `cargo test capture --lib` passing with 11 capture tests.
- `docs/evidence/stevenarella-frame-readback-validation-2026-05-31.b3`: hashes the validation log and source patch.
- `docs/evidence/stevenarella-frame-readback-source-2026-05-31.b3`: hashes the source patch.

## Finding
The completed task text requires a GL RGBA readback helper and pure origin normalization without ordinary rendering changes. The source evidence adds the GL read-pixels wrapper and pure top-left normalization helpers, while the tests cover valid GL bottom-left normalization and invalid buffer shape. No render loop or MCP hook was claimed by this task.

## Owner
Britton Robitzsch, mc compatibility owner.

## Decision
Keep task 2 marked complete. Do not mark the pending post-render capture hook, screenshot, recording, MCP resource, or artifact tasks complete from this evidence.

## Follow-up
Next task remains `r[mc_compatibility.stevenarella_frame_capture.screenshot]`: add the post-render/pre-swap hook that services pending capture requests from the MCP queue.
