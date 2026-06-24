let Contracts = import "../component-registry-contracts.ncl" in
{
  schema = "mc.compat.component-registry.fixture.undocumented-nested-git",
  schema_version = 1,
  components = [{
    path = "vendor/",
    role = "validation-tools",
    owner = "parent-repository",
    vcs_boundary = "parent-git",
    commands = ["tools/check_component_registry.rs --root ."],
    default_gate_participation = "included",
    evidence_policy = "copy-review-artifacts",
    local_notes = ["README.md"],
    nested_git_exception = false,
  }],
} | Contracts.Registry
