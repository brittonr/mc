# Stevenarella frame queue-closed regression task oracle — 2026-05-31

## Question
Does the capture queue still report a closed queue, rather than masking it behind the new pending-request rate limit, when the receiver is dropped with an unserviced request pending?

## Inspected evidence
- `stevenarella` child commit `0583455` (`preserve closed capture queue errors`) adds receiver-open state to `CaptureRequestSender`/`CaptureRequestReceiver` and checks it before reserving a pending capture slot.
- The same child commit adds `capture_queue_reports_closed_after_pending_receiver_drop`, which enqueues one pending capture, drops the receiver, and asserts the next enqueue returns `CaptureQueueError::QueueClosed` instead of `RateLimitExceeded`.
- `docs/evidence/stevenarella-frame-queue-closed-regression-source-2026-05-31.patch` records the exact child diff.
- `docs/evidence/stevenarella-frame-queue-closed-regression-2026-05-31.run.log` records `cargo fmt --check`, the queue-closed regression test, `cargo test focused_validation --lib`, `cargo test capture --lib`, full `cargo test --lib`, and `cargo check --bin stevenarella` with `exit_status=0`.
- `docs/evidence/stevenarella-frame-queue-closed-regression-2026-05-31.b3` hashes the validation log and source patch.
- `docs/evidence/stevenarella-frame-queue-closed-regression-source-2026-05-31.b3` hashes the source patch.

## Finding
The post-review rate-limit masking concern is fixed and covered by a focused regression test. The evidence does not claim a pre-change baseline run; it only records validation at child revision `0583455`.

## Owner
Britton Robitzsch, mc compatibility owner.

## Decision
Keep task 6 complete with supplemental queue-closed regression evidence. Task 7 remains open.

## Follow-up
Proceed to `r[mc_compatibility.stevenarella_frame_capture.artifacts]` final artifact/evidence/archive prep after this supplemental evidence is committed and manifests/gates pass.
