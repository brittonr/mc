# Typed contracts for the workspace component registry.
# The registry is check-time metadata only; runtime code consumes checked-in static data.

let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in

let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in

let SupportedSchemaVersion = std.contract.from_predicate (fun value =>
  std.is_number value && value == 1
) in

let ComponentPath = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.is_match "^[A-Za-z0-9][A-Za-z0-9._/-]*/$" value
  && !std.string.is_match "(^|/)\\.\\.(/|$)" value
  && !std.string.is_match "(^|/)\\.git(/|$)" value
) in

let ComponentRole = std.contract.from_predicate (fun value =>
  std.is_string value && (
    value == "core-client"
    || value == "core-server"
    || value == "compatibility-root"
    || value == "compatibility-runner"
    || value == "compatibility-config"
    || value == "paper-reference-fixture"
    || value == "cairn-lifecycle"
    || value == "cairn-policy"
    || value == "repository-docs"
    || value == "durable-evidence"
    || value == "compatibility-scripts"
    || value == "validation-tools"
    || value == "independent-engine"
    || value == "reference-client"
  )
) in

let ComponentOwner = std.contract.from_predicate (fun value =>
  std.is_string value && (
    value == "parent-repository"
    || value == "independent-nested-repository"
    || value == "reference-only-checkout"
  )
) in

let VcsBoundary = std.contract.from_predicate (fun value =>
  std.is_string value && (
    value == "parent-git"
    || value == "nested-git-and-jj"
    || value == "nested-git"
  )
) in

let GateParticipation = std.contract.from_predicate (fun value =>
  std.is_string value && (
    value == "included"
    || value == "selected"
    || value == "excluded"
  )
) in

let EvidencePolicy = std.contract.from_predicate (fun value =>
  std.is_string value && (
    value == "copy-review-artifacts"
    || value == "generated-checked-in"
    || value == "durable-evidence-root"
    || value == "reference-excluded"
  )
) in

{
  Component = {
    path | ComponentPath,
    role | ComponentRole,
    owner | ComponentOwner,
    vcs_boundary | VcsBoundary,
    commands | NonEmptyArray | Array NonEmptyString,
    default_gate_participation | GateParticipation,
    evidence_policy | EvidencePolicy,
    local_notes | NonEmptyArray | Array NonEmptyString,
    nested_git_exception | Bool,
  },

  Registry = {
    schema | NonEmptyString,
    schema_version | SupportedSchemaVersion,
    components | NonEmptyArray | Array Component,
  },
}
