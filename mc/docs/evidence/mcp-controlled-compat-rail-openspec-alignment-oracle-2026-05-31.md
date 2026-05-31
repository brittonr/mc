# MCP-controlled compatibility rail OpenSpec alignment oracle

## Question

Does the active `mcp-controlled-compat-rail` change cover the current MCP-controlled dry-run work, and are the completed task claims aligned with the repo-local proposal, design, delta spec, implementation, and evidence?

## Inspected evidence

- Active change artifacts:
  - `cairn/changes/mcp-controlled-compat-rail/proposal.md`
  - `cairn/changes/mcp-controlled-compat-rail/design.md`
  - `cairn/changes/mcp-controlled-compat-rail/specs/mc-compatibility/spec.md`
  - `cairn/changes/mcp-controlled-compat-rail/tasks.md`
- The proposal scopes this change to a parent runner rail for Stevenarella MCP control, deterministic dry-run fixtures, fail-closed receipt checks, and explicit non-claims before any live visual or compatibility promotion.
- The design records the controlling choices: runner-owned orchestration, dry-run first, visual artifacts as observability evidence only, child-revision fail-closed behavior, and a new isolated scenario instead of mutating existing rails.
- The delta spec defines requirements for:
  - `r[mc_compatibility.mcp_controlled_compat_rail.contract]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.scenario]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.receipt]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.frame_artifacts]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.checker]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.flake]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.live_artifacts]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.matrix]`
  - `r[mc_compatibility.mcp_controlled_compat_rail.validation]`
- Current implementation evidence inspected for the completed dry-run scope:
  - `tools/mc-compat-runner/src/main.rs`
  - `tools/mc-compat-runner/src/scenario_manifest_generated.rs`
  - `tools/check_mcp_controlled_compat_rail.rs`
  - `config/mc-compat/scenario-manifest.ncl`
  - `flake.nix`
  - `README.md`
- Validation evidence inspected:
  - `docs/evidence/mcp-controlled-compat-rail-validation-2026-05-31.run.log`
  - `docs/evidence/mcp-controlled-compat-rail-followup-gates-2026-05-31.run.log`
- The implemented dry-run contract names the scenario `mcp-controlled-smoke`, records MCP control under `mcp_control`, records optional visual output under `frame_artifacts`, and keeps the live run fail-closed until owned-local MCP/frame evidence exists.
- The completed task evidence intentionally does not complete `live_artifacts`, `matrix`, or final archive validation tasks.

## Decision

The active `mcp-controlled-compat-rail` change is the correct repo-local OpenSpec/Cairn change for the current work. The completed dry-run tasks are aligned with the change artifacts:

1. Contract/task alignment is supported by the proposal, design, delta spec, runner receipt field names, and validation logs.
2. Scenario isolation is supported by the new `mcp-controlled-smoke` row and unchanged existing scenario names/semantics.
3. Receipt shape is supported by `mcp_control` and `frame_artifacts` blocks in runner output.
4. Fail-closed checking is supported by the Rust checker fixtures and focused dry-run flake check.
5. Live evidence and matrix promotion remain explicitly incomplete and must not be claimed or archived from dry-run evidence alone.

This checkpoint resolves the review ambiguity where an external review did not identify the active change from supplied evidence. Future completion claims for this change should cite both this oracle and the specific run logs or manifests for the task being marked complete.

## Inspected evidence gaps

- No live MCP-controlled smoke run has been promoted.
- No frame artifacts have been copied under `docs/evidence/` for this rail.
- No acceptance-matrix row has been promoted for MCP-controlled observability.
- The aggregate `mc-compat-maintained-dry-runs` check remains blocked by an unrelated adversarial-network oracle current-bundle issue and is not evidence against the focused MCP dry-run rail.

## Owner

- Decision owner: current user request selecting the OpenSpec alignment checkpoint.
- Implementation owner: agent for adding the checkpoint, manifest, task citation, and rerunning focused gates.

## Next action

Use this checkpoint as the alignment reference, then continue with the next open task only when live Stevenarella MCP/frame prerequisites are available: run one owned-local MCP-controlled smoke, copy review-critical artifacts under `docs/evidence/`, and keep non-claims explicit.
