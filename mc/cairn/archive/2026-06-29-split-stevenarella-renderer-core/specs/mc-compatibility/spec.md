# mc-compatibility Change Spec: Stevenarella renderer core

## Requirements

### Requirement: Stevenarella renderer boundaries

r[mc_compatibility.stevenarella_render.renderer_boundaries] Stevenarella renderer code SHOULD expose cohesive boundaries for camera/view state, chunk buffers, texture management, skin or remote texture cache, pending uploads, frame orchestration, and capture/readback integration.

#### Scenario: Renderer responsibility has one owner

r[mc_compatibility.stevenarella_render.renderer_boundaries.ownership]
- GIVEN a renderer responsibility is reviewed
- WHEN maintainers inspect renderer modules
- THEN the responsibility is owned by a focused module
- AND unrelated GL, cache, chunk, texture, frame, and capture concerns are not reintroduced into one module.

### Requirement: Stevenarella render core

r[mc_compatibility.stevenarella_render.render_core] Renderer URL, cache-path, upload-plan, chunk visibility/order, and frame/capture planning decisions SHOULD be pure over explicit inputs.

#### Scenario: Render plan is testable without OpenGL

r[mc_compatibility.stevenarella_render.render_core.testable]
- GIVEN renderer state summaries and resource identifiers
- WHEN the render core computes a cache, upload, chunk, or frame plan
- THEN the plan can be tested without OpenGL, resource-manager locks, filesystem, or network side effects.

### Requirement: Stevenarella renderer parity

r[mc_compatibility.stevenarella_render.parity] Renderer splitting MUST preserve visible rendering behavior, capture interactions, texture cache semantics, GL side-effect boundaries, and evidence non-claims.

#### Scenario: Renderer behavior remains stable

r[mc_compatibility.stevenarella_render.parity.stable]
- GIVEN a supported pre-refactor render or capture input
- WHEN the split renderer processes the same input
- THEN the selected render plan, cache behavior, capture interaction, and non-claim boundary remain equivalent.

### Requirement: Stevenarella renderer positive tests

r[mc_compatibility.stevenarella_render.positive_tests] The change MUST include positive tests for texture URL normalization, skin cache paths, upload plans, chunk render plans, camera/view facts, and capture frame plans.

#### Scenario: Supported renderer paths pass

r[mc_compatibility.stevenarella_render.positive_tests.coverage]
- GIVEN representative supported renderer inputs
- WHEN extracted renderer cores process them
- THEN tests prove the expected cache, upload, chunk, frame, or capture plans are produced.

### Requirement: Stevenarella renderer negative tests

r[mc_compatibility.stevenarella_render.negative_tests] The change MUST include negative tests for invalid texture URLs, unsafe cache paths, missing resources, invalid frame dimensions, empty chunk buffers, and unavailable capture contexts.

#### Scenario: Invalid renderer paths fail closed

r[mc_compatibility.stevenarella_render.negative_tests.fail_closed]
- GIVEN invalid renderer inputs
- WHEN extracted renderer cores process them
- THEN tests prove the inputs are rejected, ignored, or contained before unsafe side effects occur.

### Requirement: Stevenarella renderer validation

r[mc_compatibility.stevenarella_render.validation] The change MUST record focused Stevenarella render/capture tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_render.validation.logs]
- GIVEN renderer splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative renderer tests plus affected dry-runs and Cairn gates passing.
