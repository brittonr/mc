# Tasks

- [ ] [serial] Define the `mcp-controlled compatibility rail` receipt contract, scenario name, dependencies on Stevenarella MCP/capture readiness, and normalized metric names. r[mc_compatibility.mcp_controlled_compat_rail.contract]
- [ ] [depends:contract] Add scenario manifest and generated runner-table wiring for the MCP-controlled dry-run rail without changing existing scenario semantics. r[mc_compatibility.mcp_controlled_compat_rail.scenario]
- [ ] [depends:scenario] Extend runner receipt DTO/checking with `mcp_control` and optional `frame_artifacts` blocks. r[mc_compatibility.mcp_controlled_compat_rail.receipt]
- [ ] [depends:receipt] Add deterministic checker positive and negative fixtures for valid dry-run, missing handshake, stdout contamination, missing command outcome, missing frame digest, path escape, missing/stale Stevenarella revision, and overclaim wording. r[mc_compatibility.mcp_controlled_compat_rail.checker]
- [ ] [depends:checker] Add a focused flake dry-run check for the MCP-controlled rail and update README command listing. r[mc_compatibility.mcp_controlled_compat_rail.flake]
- [ ] [depends:flake] When Stevenarella MCP and capture prerequisites are available, run one owned local live MCP-controlled smoke and copy receipt/log/frame artifacts under `docs/evidence/`. r[mc_compatibility.mcp_controlled_compat_rail.live_artifacts]
- [ ] [depends:live_artifacts] Promote only the MCP-controlled observability row in matrix/current-bundle docs and keep visual, semantic, production, public-server, and load non-claims explicit. r[mc_compatibility.mcp_controlled_compat_rail.matrix]
- [ ] [depends:matrix] Run scenario manifest check, runner receipt checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.mcp_controlled_compat_rail.validation]
