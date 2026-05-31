# Delta: MCP-controlled compatibility rail

## Requirements

### Requirement: MCP-controlled rail contract

r[mc_compatibility.mcp_controlled_compat_rail.contract] The parent `mc` harness MUST define a bounded receipt contract before promoting any MCP-controlled Stevenarella compatibility rail.

#### Scenario: Contract names dependencies and scope

r[mc_compatibility.mcp_controlled_compat_rail.contract.scope]
- GIVEN MCP-controlled compatibility work starts
- WHEN the rail contract is reviewed
- THEN it names the scenario, Stevenarella MCP control prerequisite, optional frame capture prerequisite, receipt fields, and normalized metrics
- AND it states that screenshots alone, visual regression approval, semantic equivalence, production readiness, public-server safety, and load testing remain non-claims.

### Requirement: Scenario wiring is isolated

r[mc_compatibility.mcp_controlled_compat_rail.scenario] The MCP-controlled rail MUST be added as an isolated named scenario without changing existing maintained scenario semantics.

#### Scenario: Existing rails stay unchanged

r[mc_compatibility.mcp_controlled_compat_rail.scenario.isolated]
- GIVEN existing maintained scenario manifest entries
- WHEN the MCP-controlled rail is added
- THEN existing scenario required milestones, forbidden patterns, and claims remain unchanged
- AND the new rail declares its own MCP-specific evidence fields.

### Requirement: MCP receipt block

r[mc_compatibility.mcp_controlled_compat_rail.receipt] MCP-controlled rail receipts MUST record the MCP handshake, tool calls, command outcomes, and client revision.

#### Scenario: Receipt records control outcomes

r[mc_compatibility.mcp_controlled_compat_rail.receipt.control_outcomes]
- GIVEN the runner drives Stevenarella through MCP
- WHEN it writes the receipt
- THEN the receipt records endpoint mode, handshake status, tool-list digest, attempted calls, successful calls, first failure if any, stdout-clean status, command outcome ids, and Stevenarella child revision.

#### Scenario: Missing child revision fails closed

r[mc_compatibility.mcp_controlled_compat_rail.receipt.child_revision]
- GIVEN a receipt is promoted as live MCP evidence
- WHEN the receipt cannot machine-record the Stevenarella child revision
- THEN a repo-local oracle checkpoint with question, inspected evidence, decision, owner, and next action is required before promotion.

### Requirement: Frame artifact receipt block

r[mc_compatibility.mcp_controlled_compat_rail.frame_artifacts] MCP-controlled rail receipts that capture frames MUST record durable artifact metadata.

#### Scenario: Frame artifacts are digest-addressed

r[mc_compatibility.mcp_controlled_compat_rail.frame_artifacts.digest]
- GIVEN the rail captures a screenshot or frame sequence
- WHEN the receipt is written
- THEN it records artifact paths, width, height, frame id or sequence id, format, BLAKE3 digest, redaction status, and whether UI was included.

#### Scenario: Target-only artifacts are not enough

r[mc_compatibility.mcp_controlled_compat_rail.frame_artifacts.reviewable]
- GIVEN frame artifacts are used as promoted evidence
- WHEN the change is prepared for review
- THEN review-critical artifacts are copied under `docs/evidence/`
- AND transient `target/` paths alone are rejected.

### Requirement: Fail-closed checker fixtures

r[mc_compatibility.mcp_controlled_compat_rail.checker] The MCP-controlled rail MUST include deterministic positive and negative checker fixtures.

#### Scenario: Bad receipts are rejected

r[mc_compatibility.mcp_controlled_compat_rail.checker.rejects]
- GIVEN receipts are missing handshake success, command outcomes, stdout-clean proof, frame digest when frame capture is claimed, contained artifact paths, Stevenarella revision, or contain overclaim wording
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Dry-run flake check

r[mc_compatibility.mcp_controlled_compat_rail.flake] The MCP-controlled rail MUST have a deterministic dry-run flake check before live evidence is promoted.

#### Scenario: Dry-run validates schema without live client

r[mc_compatibility.mcp_controlled_compat_rail.flake.dry_run]
- GIVEN live Stevenarella MCP prerequisites are unavailable or not yet implemented
- WHEN the dry-run flake check runs
- THEN it validates scenario manifest wiring, receipt shape, non-claims, and fail-closed fixtures without launching a live client.

### Requirement: Live MCP artifacts

r[mc_compatibility.mcp_controlled_compat_rail.live_artifacts] Live MCP-controlled rail evidence MUST be durable and bounded.

#### Scenario: Live run captures bounded owned-local evidence

r[mc_compatibility.mcp_controlled_compat_rail.live_artifacts.owned_local]
- GIVEN Stevenarella MCP control and capture prerequisites are complete
- WHEN a live MCP-controlled rail is run
- THEN it targets only an owned local fixture, uses bounded duration/client/action limits, records receipt/log/frame artifacts, and copies review-critical outputs under `docs/evidence/`.

### Requirement: Matrix and bundle promotion

r[mc_compatibility.mcp_controlled_compat_rail.matrix] Acceptance matrix and current bundle updates MUST promote only the MCP-controlled observability row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.mcp_controlled_compat_rail.matrix.nonclaims]
- GIVEN MCP-controlled rail evidence passes
- WHEN docs are updated
- THEN only the MCP-controlled observability row is marked covered
- AND visual regression approval, semantic equivalence, full Minecraft compatibility, production readiness, public-server safety, and load testing remain explicit non-claims.

### Requirement: Rail validation artifacts

r[mc_compatibility.mcp_controlled_compat_rail.validation] The MCP-controlled rail MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.mcp_controlled_compat_rail.validation.log]
- GIVEN the rail is archived
- WHEN validation is reviewed
- THEN repo-local logs show scenario manifest check, runner receipt checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
