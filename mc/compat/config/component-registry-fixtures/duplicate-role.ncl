let Contracts = import "../component-registry-contracts.ncl" in
{
  schema = "mc.compat.component-registry.fixture.duplicate-role",
  schema_version = 1,
  components = [
    {
      path = "clients/stevenarella/",
      role = "core-client",
      owner = "parent-repository",
      vcs_boundary = "parent-git",
      commands = ["nix run .#stevenarella -- --dry-run"],
      default_gate_participation = "selected",
      evidence_policy = "copy-review-artifacts",
      local_notes = ["clients/stevenarella/AGENTS.md"],
      nested_git_exception = false,
    },
    {
      path = "servers/valence/",
      role = "core-client",
      owner = "parent-repository",
      vcs_boundary = "parent-git",
      commands = ["nix run .#valence -- --dry-run"],
      default_gate_participation = "selected",
      evidence_policy = "copy-review-artifacts",
      local_notes = ["servers/valence/AGENTS.md"],
      nested_git_exception = false,
    },
  ],
} | Contracts.Registry
