# Repository layout checklist

Use this checklist for layout-only reviews. Run `tools/check_layout_boundaries.rs --self-test` and `tools/check_layout_boundaries.rs --root .` through the script's pinned Nix shell to verify the current component registry, documented nested Git boundaries, root artifact boundaries, subtree docs, and generated-surface markers.

## Layout guard contract

### Diagnostic classes

- `undocumented_root_directory` (required): a non-hidden root directory is present but absent from the component registry tables below.
- `undocumented_nested_git` (required): a nested `.git` boundary is present but the owning root is not listed as an intentional nested Git exception.
- `root_transient_artifact` (advisory): root scratch artifacts such as `result`, `result-*`, `target/`, `target-*.log`, or root `*.run.log` are present. These are not review evidence unless copied under `docs/evidence/` with a BLAKE3 manifest.
- `missing_subtree_documentation` (required): a registry row names local notes that are absent, or root docs stop linking to required subtree guidance.
- `generated_marker_drift` (required): a machine-owned generated surface is missing its ownership marker.
- `component_registry_mismatch` (required): observed layout conflicts with registry ownership, such as a parent-owned component root missing or containing an unexpected nested Git boundary.

### Waiver model

Waivers live in the component registry row that needs them, not in hidden tool state. A waiver MUST state owner, command boundary, parent-repo interaction, default-gate participation, and next action. Reference-only or external checkouts use this registry row as their waiver; parent-owned component rows SHOULD instead point at local `AGENTS.md` guidance.

### Source-of-truth inputs

The component registry is the set of Markdown tables in this checklist with `Path`, `Role`, `Ownership`, `Local notes`, and `Default gates` columns. Artifact-boundary rules are the transient root patterns named in `root_transient_artifact` plus the evidence boundary in `docs/architecture.md`. Generated-surface ownership markers are read from the checked-in generated files and must match their generator comments.

### Non-claims

The layout guard does not claim live compatibility, semantic parity, production readiness, public-server safety, or evidence correctness outside layout policy. It only reports repository structure and documentation hygiene.

## Major component roots

| Path | Role | Ownership | Local notes | Default gates |
| --- | --- | --- | --- | --- |
| `clients/stevenarella/` | Core client | Parent repository owned | `clients/stevenarella/AGENTS.md` | Included through mc-compat checks when selected by scenario |
| `servers/valence/` | Core server | Parent repository owned | `servers/valence/AGENTS.md` | Included through mc-compat and Valence checks when selected |
| `compat/` | Compatibility harness/config/fixtures | Parent repository owned | Root `AGENTS.md` plus `docs/architecture.md` | Included by runner, generated-surface, and evidence checks |
| `hyperion/` | Independent engine/proxy repository | Nested repo, separate jj/git workflow | `hyperion/AGENTS.md` and `hyperion/.agent/napkin.md` | Excluded from parent default gates unless explicitly selected |
| `Leafish/` | Reference client checkout | Reference-only nested Git checkout | Waived here because the nested checkout is not parent-owned | Excluded from default gates unless explicitly selected |

## Intentional root directories

| Path | Role | Ownership | Local notes | Default gates |
| --- | --- | --- | --- | --- |
| `cairn/` | Cairn lifecycle specs, changes, and archives | Parent repository owned | Root `AGENTS.md` and `README.md` | Included by Cairn gates and validation |
| `cairn-policy/` | Nickel-authored Cairn policy source plus generated JSON | Parent repository owned | `README.md` and `docs/architecture.md` | Included by policy freshness checks |
| `config/` | Local workspace configuration | Optional local workspace root | Root `AGENTS.md` | Excluded from flake-source checks unless explicitly tracked |
| `docs/` | Architecture, evidence, and workflow documentation | Parent repository owned | `docs/architecture.md`, `docs/check-tiers.md`, and this checklist | Included by docs, evidence, and Cairn gates |
| `evidence/` | Legacy promoted evidence notes retained for historical review | Parent repository owned | `README.md` and `docs/architecture.md` | Excluded from new task citations; prefer `docs/evidence/` |
| `scripts/` | Compatibility shims and local automation | Parent repository owned | `README.md` | Included only when selected by a focused check |
| `tools/` | Rust/Steel validation tools and generators | Parent repository owned | `README.md` and `docs/check-tiers.md` | Included by focused tool and flake checks |

## Review rules

- Major owned component roots SHOULD have a local `AGENTS.md` or an explicit waiver in this checklist.
- Reference-only or external checkouts MUST state owner, command boundary, parent-repo interaction, and default-gate participation.
- Root-level nested Git directories MUST be listed above before they are treated as intentional.
- Default compatibility gates MUST NOT require reference-only checkouts unless a command explicitly opts into that reference.
- New component roots SHOULD update `AGENTS.md`, `README.md`, `docs/architecture.md`, and this checklist in the same change.
- New generated surfaces SHOULD carry a machine-owned marker that names the generator and source input.

## Current nested Git exceptions

- `hyperion/`: independent repository with separate jj/git workflow. Run Hyperion commands from inside `hyperion/`; do not use parent repo status as Hyperion status.
- `Leafish/`: reference-only client checkout retained for comparison and historical investigation. Do not include it in default compatibility gates, source-tree revision evidence, or parent-owned component scans unless a future Cairn explicitly reclassifies it.
