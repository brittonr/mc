# mc architecture

## Component layout

The `mc/` workspace is organized around product roles:

- `clients/stevenarella/`: core Rust Minecraft client used by compatibility rails and manual client checks.
- `servers/valence/`: core Rust Minecraft server framework used by compatibility rails.
- `compat/runner/`: compatibility runner that orchestrates client/server/Paper fixtures and writes receipts.
- `compat/config/`: typed mc-compat configuration, scenario manifests, and the workspace component registry (`component-registry.ncl`).
- `compat/fixtures/paper-survival/`: Paper reference fixture used for paired survival evidence.
- `cairn/`: lifecycle specs, active changes, and archived changes.
- `cairn-policy/`: Nickel-authored Cairn policy source plus generated JSON consumed by the pinned Cairn binary.
- `docs/evidence/`: promoted receipts, run logs, manifests, and review notes.
- `docs/layout-checklist.md`: current review checklist for component roots, local agent docs, and nested Git exceptions.

Stevenarella and Valence retain upstream ancestry, but they are not treated as passive vendor payloads. They are parent-owned core component trees, and source revision evidence is path-scoped to the resolved component root. Component-specific workflows live next to the code in `clients/stevenarella/AGENTS.md` and `servers/valence/AGENTS.md`.

Leafish is classified as a reference-only nested Git checkout at `Leafish/`. It is retained for comparison and historical investigation, excluded from default compatibility gates, and not treated as a parent-owned core client unless a future Cairn explicitly reclassifies it. Its parent-tracked local-doc waiver is recorded in `docs/layout-checklist.md`.

## Layout resolution

`compat/config/component-registry.ncl` is the typed check-time registry for component paths, roles, owners, VCS boundaries, command notes, default gate participation, and evidence policy. Runtime code remains Nickel-free: `compat/runner/src/layout.rs` uses checked-in Rust/static behavior and CLI inputs, while focused checks validate the registry and docs summaries.

`compat/runner/src/layout.rs` is the central resolver for client, server, and compatibility roots. It uses canonical role-based paths for active defaults and fails closed with migration diagnostics for old transition roots, missing required roots, ambiguous duplicate roots, or nested Git directories inside core component trees.

Root-level nested Git checkouts are intentional only when named in `compat/config/component-registry.ncl` and summarized in `docs/layout-checklist.md`. The current exception is `Leafish/` as a reference-only client checkout. The former local Hyperion checkout has been retired; historical Hyperion evidence remains under `docs/evidence/` and Cairn archives.

Runner defaults, Valence worktree source detection, and validation tests should use this resolver rather than adding ad hoc path probes.

## Compatibility runner core boundary

`compat/runner/src/main.rs` is only the binary entrypoint: it delegates to `mc_compat_runner::run_main()` and owns no runner policy. `compat/runner/src/lib.rs` is the application shell. It owns CLI parsing, environment and filesystem reads/writes, process execution, Docker/Paper lifecycle, sockets, clocks, logging, stdout/stderr, receipt file writes, and exit-code handling.

Pure deterministic runner logic belongs outside that shell. `compat/runner/src/scenario_catalog.rs` is data-only scenario vocabulary shared by the shell and core; `compat/runner/src/scenario_core.rs` owns scenario definitions, aliases, milestone and forbidden-pattern specs, behavior metadata, and static scenario validation; `compat/runner/src/runtime_config.rs` owns in-memory config normalization and validation. `compat/runner/src/receipt_validation.rs` owns in-memory receipt summary parsing, pair validation, and claim-boundary checks. These core modules must not import constants, helpers, or side-effecting functions from the binary entrypoint.

Compatibility shims may stay only when existing CLI names, flake app names, receipt schemas, scenario semantics, and non-claim boundaries remain unchanged.

## Cairn policy boundary

`cairn-policy/` intentionally stays beside `cairn/` rather than under it because the repo-pinned Cairn policy exporter defaults to `cairn-policy/default.ncl` and `cairn-policy/generated/cairn-policy.json`. The source is Nickel with local contracts; the generated JSON is a checked runtime artifact. Use `nix run .#cairn -- policy export` to refresh it and `nix run .#cairn -- policy export --check` or the `mc-cairn-policy-fresh` flake check to prove freshness.

## Artifact boundaries

Repository artifacts are classified before cleanup or citation rules change:

| Class | Allowed location | Tracking policy | Citation policy |
| --- | --- | --- | --- |
| Durable evidence | `docs/evidence/` | Tracked when it supports Cairn tasks, accepted specs, or review notes | May be cited from Cairn tasks when paired with a BLAKE3 manifest or inline BLAKE3 digest and command logs include `exit_status=0` |
| Generated checked-in output | Checked-in generated paths such as `compat/runner/src/scenario_manifest_generated.rs`, `compat/config/generated/`, `cairn-policy/generated/`, and generated evidence indexes | Tracked only when the owning generator/check says it is fresh | May be cited as source/generated state, not as a substitute for run evidence |
| Transient run/build output | Root `result`, root `result-*`, root `target/`, root `target-*.log`, root `*.run.log`, and local build directories | Ignored or removed locally; promote review-critical bytes before citation | Must not be cited directly from Cairn tasks; copy to `docs/evidence/` first |
| Local scratch | `.pi/`, interpreter caches, editor/runtime scratch, and untracked experiments | Local-only and ignored when a targeted pattern exists | Must not be cited as durable evidence |

Root `evidence/` is retired. Historical notes that need to remain reviewable live under `docs/evidence/` with a legacy filename, and new task evidence must use `docs/evidence/` directly. Root `config/` is not a checked configuration root; checked configuration lives under `compat/config/`, while local scratch should stay ignored outside source-controlled paths.

## Evidence partitions

New durable evidence SHOULD follow the partition rules in `docs/evidence/README.md`: command receipts under `receipts/<yyyy-mm-dd>/`, command output under `run-logs/<yyyy-mm-dd>/`, BLAKE3 manifests under `manifests/<yyyy-mm-dd>/`, promoted sidecar logs under `logs/<yyyy-mm-dd>/`, oracle notes under `oracles/<yyyy-mm-dd>/`, generated navigation under `indexes/` or existing root generated paths, fixtures under `fixtures/<yyyy-mm-dd>/`, and archive-only artifacts under `archive/<yyyy-mm-dd>/`. Existing flat `docs/evidence/` paths remain citation-stable until a focused migration updates every task/spec/note and manifest reference.

`docs/evidence/evidence-inventory.generated.md` classifies current artifacts, and `docs/evidence/evidence-index.generated.md` maps date/change/scenario keys to durable artifacts and covering manifests. `tools/check_evidence_partitions.rs` owns the freshness and path-safety checks for those generated surfaces.

## Evidence boundaries

Receipts keep historical field names such as `client.git_rev`, `valence.git_rev_resolved`, and `stevenarella_child_revision` for schema compatibility. In this repository those fields mean parent-repository evidence scoped to the component path, not nested child-repo HEADs.

Promoted Cairn task evidence should cite copied artifacts under `docs/evidence/` with BLAKE3 manifests. Direct paths under component trees, retired external checkouts, `target/`, `result`, `result-*`, root `target-*.log`, root `*.run.log`, or retired root `evidence/` are not reviewable evidence artifacts.
