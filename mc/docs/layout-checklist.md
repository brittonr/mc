# Repository layout checklist

Use this checklist for layout-only reviews. Run `tools/check_layout_boundaries.rs --self-test` and `tools/check_layout_boundaries.rs --root .` through the script's pinned Nix shell to verify the current component registry, documented nested Git boundaries, root artifact boundaries, subtree docs, and generated-surface markers.

## Layout guard contract

### Diagnostic classes

- `undocumented_root_directory` (required): a non-hidden root directory is present but absent from the component registry tables below.
- `undocumented_nested_git` (required): a nested `.git` boundary is present but the owning root is not listed as an intentional nested Git exception.
- `root_transient_artifact` (advisory): root scratch artifacts such as `result`, `result-*`, `target/`, `target-*.log`, root `target-*live.log`, or root `*.run.log` are present. These are not review evidence unless copied under `docs/evidence/` with a BLAKE3 manifest.
- `missing_subtree_documentation` (required): a registry row names local notes that are absent, or root docs stop linking to required subtree guidance.
- `generated_marker_drift` (required): a machine-owned generated surface is missing its ownership marker.
- `component_registry_mismatch` (required): observed layout conflicts with registry ownership, such as a parent-owned component root missing or containing an unexpected nested Git boundary.

### Waiver model

Waivers live in the component registry row that needs them, not in hidden tool state. A waiver MUST state owner, command boundary, parent-repo interaction, default-gate participation, and next action. Reference-only or external checkouts use this registry row as their waiver; parent-owned component rows SHOULD instead point at local `AGENTS.md` guidance.

### Source-of-truth inputs

The editable component registry lives at `compat/config/component-registry.ncl` with contracts in `compat/config/component-registry-contracts.ncl`. The Markdown tables in this checklist are static registry summaries with `Path`, `Role`, `Ownership`, `Local notes`, and `Default gates` columns; `tools/check_component_registry.rs` checks that they stay aligned with the typed registry, and `tools/check_layout_boundaries.rs` continues to consume the checked-in summaries. Artifact-boundary rules are the artifact classes below, the transient root patterns named in `root_transient_artifact`, and the evidence boundary in `docs/architecture.md`. Generated-surface ownership markers are read from the checked-in generated files and must match their generator comments.

### Non-claims

The layout guard does not claim live compatibility, semantic parity, production readiness, public-server safety, or evidence correctness outside layout policy. It only reports repository structure and documentation hygiene.

## Artifact classes

| Class | Allowed location | Tracking policy | Citation policy |
| --- | --- | --- | --- |
| Durable evidence | `docs/evidence/` using `docs/evidence/README.md` partitions such as `run-logs/<yyyy-mm-dd>/`, `manifests/<yyyy-mm-dd>/`, `receipts/<yyyy-mm-dd>/`, `logs/<yyyy-mm-dd>/`, `oracles/<yyyy-mm-dd>/`, `indexes/`, `fixtures/<yyyy-mm-dd>/`, and `archive/<yyyy-mm-dd>/` | Tracked when a Cairn task, accepted spec, review note, or evidence index depends on it; existing flat paths stay citation-stable until migrated with references | Cairn tasks may cite it when a copied artifact, command `.run.log` with `exit_status=0`, and BLAKE3 manifest or inline BLAKE3 digest are present |
| Generated checked-in output | Owner-specific generated paths such as `compat/runner/src/scenario_manifest_generated.rs`, `compat/config/generated/`, `cairn-policy/generated/`, and generated evidence indexes | Tracked only with a freshness check or generator marker | May support source/layout claims, but does not replace command or live evidence |
| Transient run/build output | Root `result`, root `result-*`, root `target/`, root `target-*.log`, root `*.run.log`, and component build outputs | Ignored or removed locally unless explicitly promoted | Must be copied under `docs/evidence/` before citation |
| Local scratch | `.pi/`, interpreter caches, editor/runtime scratch, and untracked experiments | Local-only; ignored by targeted rules where possible | Must not be cited as durable review evidence |

Retired root evidence notes have been moved under `docs/evidence/` with legacy filenames. Root `config/` is not reserved for checked configuration; use `compat/config/` for repository-owned configuration and keep local experiments out of source-controlled roots.

## Major component roots

| Path | Role | Ownership | Local notes | Default gates |
| --- | --- | --- | --- | --- |
| `clients/stevenarella/` | Core client | Parent repository owned | `clients/stevenarella/AGENTS.md` and `clients/stevenarella/README.md` | Included when selected by mc-compat scenario or client checks |
| `servers/valence/` | Core server | Parent repository owned | `servers/valence/AGENTS.md`, `servers/valence/README.md`, and `servers/valence/CONTRIBUTING.md` | Included when selected by mc-compat or Valence checks |
| `compat/` | Compatibility harness/config/fixtures root | Parent repository owned | Root `AGENTS.md` plus `docs/architecture.md` | Included by runner, generated-surface, and evidence checks |
| `Leafish/` | Reference client checkout | Reference-only nested Git checkout | `Leafish/README.md`; Waived here because the nested checkout is not parent-owned | Excluded from default gates unless explicitly selected |

## Registry subcomponents

| Path | Role | Ownership | Local notes | Default gates |
| --- | --- | --- | --- | --- |
| `compat/runner/` | Compatibility runner | Parent repository owned | `README.md`, `docs/architecture.md`, and `docs/check-tiers.md` | Included by runner and dry-run checks |
| `compat/config/` | Compatibility configuration and manifests | Parent repository owned | `README.md`, `docs/architecture.md`, and `docs/check-tiers.md` | Included by Nickel, generated-surface, and layout checks |
| `compat/fixtures/paper-survival/` | Paper reference fixture | Parent repository owned | `README.md` and `docs/check-tiers.md` | Included when selected by Paper/reference checks |
| `docs/evidence/` | Durable evidence root | Parent repository owned | `docs/evidence/README.md`, `docs/architecture.md`, `docs/check-tiers.md`, `docs/evidence/evidence-index.generated.md`, and `docs/evidence/evidence-inventory.generated.md` | Included by evidence partition, evidence manifest, and task-evidence checks |

## Intentional root directories

| Path | Role | Ownership | Local notes | Default gates |
| --- | --- | --- | --- | --- |
| `cairn/` | Cairn lifecycle specs, changes, and archives | Parent repository owned | Root `AGENTS.md` and `README.md` | Included by Cairn gates and validation |
| `cairn-policy/` | Nickel-authored Cairn policy source plus generated JSON | Parent repository owned | `README.md` and `docs/architecture.md` | Included by policy freshness checks |
| `docs/` | Architecture, evidence, and workflow documentation | Parent repository owned | `docs/architecture.md`, `docs/check-tiers.md`, and this checklist | Included by docs, evidence, and Cairn gates |
| `nix/` | Nix flake apps, packages, checks, and generated wrapper wiring | Parent repository owned | `README.md`, `docs/architecture.md`, and `docs/check-tiers.md` | Included by flake and layout checks |
| `scripts/` | Compatibility shims and local automation | Parent repository owned | `README.md` and root `AGENTS.md` | Included only when selected by a focused check |
| `tools/` | Rust/Steel validation tools and generators | Parent repository owned | `README.md` and `docs/check-tiers.md` | Included by focused tool and flake checks |

## Review rules

- Major owned component roots SHOULD have a local `AGENTS.md` or an explicit waiver in this checklist.
- Reference-only or external checkouts MUST state owner, command boundary, parent-repo interaction, and default-gate participation.
- Root-level nested Git directories MUST be listed above before they are treated as intentional.
- Default compatibility gates MUST NOT require reference-only checkouts unless a command explicitly opts into that reference.
- New component roots SHOULD update `compat/config/component-registry.ncl`, `AGENTS.md`, `README.md`, `docs/architecture.md`, and this checklist in the same change.
- New generated surfaces SHOULD carry a machine-owned marker that names the generator and source input.

## Current nested Git exceptions

- `Leafish/`: reference-only client checkout retained for comparison and historical investigation. Do not include it in default compatibility gates, source-tree revision evidence, or parent-owned component scans unless a future Cairn explicitly reclassifies it.
