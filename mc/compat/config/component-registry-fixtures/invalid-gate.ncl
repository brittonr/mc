let Contracts = import "../component-registry-contracts.ncl" in
{
  schema = "mc.compat.component-registry.fixture.invalid-gate",
  schema_version = 1,
  components = [{
    path = "clients/stevenarella/",
    role = "core-client",
    owner = "parent-repository",
    vcs_boundary = "parent-git",
    commands = ["nix run .#stevenarella -- --dry-run"],
    default_gate_participation = "always",
    evidence_policy = "copy-review-artifacts",
    local_notes = ["clients/stevenarella/AGENTS.md"],
    nested_git_exception = false,
  }],
} | Contracts.Registry
