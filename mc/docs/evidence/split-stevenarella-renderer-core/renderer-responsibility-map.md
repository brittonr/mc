# split-stevenarella-renderer-core responsibility map

## Question

Drain `split-stevenarella-renderer-core` without promoting broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claims.

## Requirement IDs and acceptance criteria

- `r[mc_compatibility.stevenarella_render.renderer_boundaries]`: renderer responsibilities are owned by focused modules for camera/view state, chunk buffers, texture management, skin or remote texture cache, pending uploads, frame orchestration, and capture/readback integration.
- `r[mc_compatibility.stevenarella_render.render_core]`: URL normalization, cache paths, upload plans, chunk visibility/order, and frame/capture planning are pure over explicit inputs.
- `r[mc_compatibility.stevenarella_render.parity]`: shell wiring preserves existing visible rendering behavior, capture interactions, texture cache semantics, GL side-effect boundaries, and non-claims for supported inputs.
- `r[mc_compatibility.stevenarella_render.positive_tests]`: positive tests cover texture URL normalization, skin cache paths, upload plans, chunk render plans, camera/view facts, and capture frame plans.
- `r[mc_compatibility.stevenarella_render.negative_tests]`: negative tests cover invalid texture URLs, unsafe cache paths, missing resources, invalid frame dimensions, empty chunk buffers, and unavailable capture contexts.
- `r[mc_compatibility.stevenarella_render.validation]`: focused render/capture tests, affected dry-runs, Cairn gates, and Cairn validation are captured as logs with `exit_status=0`.

## Owner subtree and dependencies

- Owner subtree: `clients/stevenarella/`.
- Cairn artifacts/evidence live in the parent mc repo under `cairn/` and `docs/evidence/`.
- No Hyperion or Leafish code was used for this change.

## Responsibility split

- `clients/stevenarella/src/render/camera.rs`: camera/view facts, perspective planning, camera matrices.
- `clients/stevenarella/src/render/chunk_buffers.rs`: chunk buffer ownership, layer counts, chunk draw order/offset planning.
- `clients/stevenarella/src/render/texture_manager.rs`: texture manager, texture model, dynamic texture fallback decisions, skin update shell hooks.
- `clients/stevenarella/src/render/skin_cache.rs`: Minecraft texture URL normalization and skin cache path planning.
- `clients/stevenarella/src/render/upload_queue.rs`: pending texture upload and texture-array resize planning.
- `clients/stevenarella/src/render/frame.rs`: frame dimension validation and frame-id advancement planning.
- `clients/stevenarella/src/render/capture_readback.rs`: renderer capture context/readback planning.
- `clients/stevenarella/src/render/mod.rs`: imperative shell for GL calls, resource-manager locks, texture uploads, frame orchestration, chunk drawing, and public renderer surface re-exports.

## Baseline evidence

- `docs/evidence/split-stevenarella-renderer-core/baseline-stevenarella-render-tests.run.log`: pre-refactor `cargo test render::tests` baseline, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/baseline-stevenarella-capture-tests.run.log`: pre-refactor `cargo test capture::tests` baseline, `exit_status=0`.

## Post-change focused evidence

- `docs/evidence/split-stevenarella-renderer-core/post-stevenarella-render-tests.run.log`: focused render core tests, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/post-stevenarella-capture-tests.run.log`: focused capture/capture-startup tests, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/post-stevenarella-wrapper-dry-run.run.log`: Stevenarella wrapper dry-run, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/post-mc-compat-smoke-dry-run.run.log`: mc-compat smoke dry-run, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/post-gate-proposal.run.log`: post-implementation Cairn proposal gate, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/post-gate-design.run.log`: post-implementation Cairn design gate, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/post-gate-tasks-open.run.log`: post-implementation Cairn tasks gate before final validation task closeout, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/post-validate-open.run.log`: post-implementation Cairn validation before final validation task closeout, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/mid-task-evidence.run.log`: task-evidence check for completed implementation/test tasks, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/final-gate-proposal.run.log`: final Cairn proposal gate after task closeout, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/final-gate-design.run.log`: final Cairn design gate after task closeout, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/final-gate-tasks.run.log`: final Cairn tasks gate after task closeout, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/final-validate.run.log`: final Cairn validation after task closeout, `exit_status=0`.
- `docs/evidence/split-stevenarella-renderer-core/final-task-evidence.run.log`: final task-evidence check after task closeout, `exit_status=0`.

## Non-claims preserved

This evidence supports the renderer architecture split and focused local tests only. It does not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness.
