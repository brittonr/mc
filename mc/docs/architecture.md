# mc architecture

## Component layout

The `mc/` workspace is organized around product roles:

- `clients/stevenarella/`: core Rust Minecraft client used by compatibility rails and manual client checks.
- `servers/valence/`: core Rust Minecraft server framework used by compatibility rails.
- `compat/runner/`: compatibility runner that orchestrates client/server/Paper fixtures and writes receipts.
- `compat/config/`: typed mc-compat configuration and scenario manifests.
- `compat/fixtures/paper-survival/`: Paper reference fixture used for paired survival evidence.
- `cairn/`: lifecycle specs, active changes, and archived changes.
- `cairn-policy/`: Nickel-authored Cairn policy source plus generated JSON consumed by the pinned Cairn binary.
- `docs/evidence/`: promoted receipts, run logs, manifests, and review notes.
- `docs/layout-checklist.md`: current review checklist for component roots, local agent docs, and nested Git exceptions.

Stevenarella and Valence retain upstream ancestry, but they are not treated as passive vendor payloads. They are parent-owned core component trees, and source revision evidence is path-scoped to the resolved component root. Component-specific workflows live next to the code in `clients/stevenarella/AGENTS.md` and `servers/valence/AGENTS.md`.

Leafish is classified as a reference-only nested Git checkout at `Leafish/`. It is retained for comparison and historical investigation, excluded from default compatibility gates, and not treated as a parent-owned core client unless a future Cairn explicitly reclassifies it. Its parent-tracked local-doc waiver is recorded in `docs/layout-checklist.md`.

## Layout resolution

`compat/runner/src/layout.rs` is the central resolver for client, server, and compatibility roots. It accepts the final role-based layout and the old transition layout while this migration is active, and it fails closed for missing required roots, ambiguous duplicate roots, or nested Git directories inside core component trees.

Root-level nested Git checkouts are intentional only when named in `docs/layout-checklist.md`. Current exceptions are `hyperion/` as an independent engine/proxy repository and `Leafish/` as a reference-only client checkout.

Runner defaults, Valence worktree source detection, and validation tests should use this resolver rather than adding ad hoc path probes.

## Cairn policy boundary

`cairn-policy/` intentionally stays beside `cairn/` rather than under it because the repo-pinned Cairn policy exporter defaults to `cairn-policy/default.ncl` and `cairn-policy/generated/cairn-policy.json`. The source is Nickel with local contracts; the generated JSON is a checked runtime artifact. Use `nix run .#cairn -- policy export` to refresh it and `nix run .#cairn -- policy export --check` or the `mc-cairn-policy-fresh` flake check to prove freshness.

## Evidence boundaries

Receipts keep historical field names such as `client.git_rev`, `valence.git_rev_resolved`, and `stevenarella_child_revision` for schema compatibility. In this repository those fields mean parent-repository evidence scoped to the component path, not nested child-repo HEADs.

Promoted Cairn task evidence should cite copied artifacts under `docs/evidence/` with BLAKE3 manifests. Direct paths under `clients/stevenarella/`, `servers/valence/`, `hyperion/`, or `target/` are not reviewable evidence artifacts.
