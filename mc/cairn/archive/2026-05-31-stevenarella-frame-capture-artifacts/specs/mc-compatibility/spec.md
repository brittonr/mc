# Delta: Stevenarella frame capture artifacts

## Requirements

### Requirement: Capture artifact contract

r[mc_compatibility.stevenarella_frame_capture.contract] Stevenarella frame capture MUST define bounded capture request, policy, and artifact metadata contracts before MCP capture tools are promoted.

#### Scenario: Capture scope is explicit

r[mc_compatibility.stevenarella_frame_capture.contract.scope]
- GIVEN frame capture work starts
- WHEN the capture contract is reviewed
- THEN it names single screenshot, latest frame, and bounded recording as the supported initial capture modes
- AND it states that visual-regression approval, semantic gameplay correctness, web capture, and headless EGL/OSMesa support remain non-claims.

#### Scenario: Invalid capture requests fail closed

r[mc_compatibility.stevenarella_frame_capture.contract.invalid]
- GIVEN a capture request uses an unsupported format, out-of-range fps, missing capture directory, path traversal, unbounded duration, or oversized output policy
- WHEN capture validation runs
- THEN it returns a structured error before GL readback or file write.

### Requirement: GL framebuffer readback

r[mc_compatibility.stevenarella_frame_capture.readback] Stevenarella frame capture MUST read pixels from Stevenarella's GL framebuffer rather than external host screenshot tools.

#### Scenario: Readback occurs after render

r[mc_compatibility.stevenarella_frame_capture.readback.after_render]
- GIVEN a native frame has completed `Renderer::tick(...)`
- WHEN a capture request is pending
- THEN Stevenarella reads RGBA pixels from the rendered framebuffer before native buffer swap
- AND the artifact metadata records the rendered dimensions and frame id.

#### Scenario: Origin is normalized

r[mc_compatibility.stevenarella_frame_capture.readback.origin]
- GIVEN GL returns pixels with bottom-left origin
- WHEN the screenshot buffer is encoded or digested
- THEN the buffer is normalized to top-left origin.

### Requirement: One-shot screenshot capture

r[mc_compatibility.stevenarella_frame_capture.screenshot] Stevenarella MUST support bounded one-shot screenshot capture through the control plane.

#### Scenario: Screenshot returns artifact metadata

r[mc_compatibility.stevenarella_frame_capture.screenshot.metadata]
- GIVEN an MCP client requests one screenshot
- WHEN the frame capture hook services the request
- THEN the response includes width, height, frame id, sequence id, format, UI inclusion status, and BLAKE3 digest
- AND it either includes bounded MCP image content or a contained artifact path.

### Requirement: Bounded frame recording

r[mc_compatibility.stevenarella_frame_capture.recording] Stevenarella frame recording MUST be bounded by policy.

#### Scenario: Recording cannot run unbounded

r[mc_compatibility.stevenarella_frame_capture.recording.bounded]
- GIVEN an MCP client requests frame recording
- WHEN the request lacks an explicit fps and duration or frame-count bound
- THEN recording is rejected before artifacts are written.

#### Scenario: Recording writes contained artifacts

r[mc_compatibility.stevenarella_frame_capture.recording.contained]
- GIVEN a valid bounded recording request and configured capture directory
- WHEN frames are captured
- THEN every written artifact stays under the configured capture directory
- AND every artifact has recorded BLAKE3 digest metadata.

### Requirement: MCP capture resources

r[mc_compatibility.stevenarella_frame_capture.mcp_resources] MCP capture tools/resources MUST expose screenshots and frames without forcing large unbounded JSON payloads.

#### Scenario: Large outputs are file-backed

r[mc_compatibility.stevenarella_frame_capture.mcp_resources.file_backed]
- GIVEN a capture output exceeds the configured inline response limit or belongs to a recording
- WHEN MCP returns the result
- THEN it returns artifact metadata and contained file paths rather than unbounded base64 content.

### Requirement: Capture validation evidence

r[mc_compatibility.stevenarella_frame_capture.validation] Frame capture MUST have positive and negative tests before promotion.

#### Scenario: Validation covers happy and sad paths

r[mc_compatibility.stevenarella_frame_capture.validation.tests]
- GIVEN frame capture implementation is complete
- WHEN focused tests run
- THEN they cover valid screenshot metadata and origin normalization
- AND they cover invalid format, path traversal, rate-limit rejection, and unbounded recording rejection.

### Requirement: Capture evidence artifacts

r[mc_compatibility.stevenarella_frame_capture.artifacts] Review-critical frame capture evidence MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.stevenarella_frame_capture.artifacts.reviewable]
- GIVEN the frame capture Cairn is ready to archive
- WHEN reviewers inspect the parent repo
- THEN focused test output, sample capture metadata, Cairn gate output, validation output, and BLAKE3 manifests are present under `docs/evidence/`.
