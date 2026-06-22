# mc architecture

## Component layout

The `mc/` workspace is organized around product roles:

- `clients/stevenarella/`: core Rust Minecraft client used by compatibility rails and manual client checks.
- `servers/valence/`: core Rust Minecraft server framework used by compatibility rails.
- `compat/runner/`: compatibility runner that orchestrates client/server/Paper fixtures and writes receipts.
- `compat/config/`: typed mc-compat configuration and scenario manifests.
- `compat/fixtures/paper-survival/`: Paper reference fixture used for paired survival evidence.
- `cairn/`: lifecycle specs, active changes, and archived changes.
- `docs/evidence/`: promoted receipts, run logs, manifests, and review notes.

Stevenarella and Valence retain upstream ancestry, but they are not treated as passive vendor payloads. They are parent-owned core component trees, and source revision evidence is path-scoped to the resolved component root.

## Layout resolution

`compat/runner/src/layout.rs` is the central resolver for client, server, and compatibility roots. It accepts the final role-based layout and the old transition layout while this migration is active, and it fails closed for missing required roots, ambiguous duplicate roots, or nested Git directories inside core component trees.

Runner defaults, Valence worktree source detection, and validation tests should use this resolver rather than adding ad hoc path probes.

## Evidence boundaries

Receipts keep historical field names such as `client.git_rev`, `valence.git_rev_resolved`, and `stevenarella_child_revision` for schema compatibility. In this repository those fields mean parent-repository evidence scoped to the component path, not nested child-repo HEADs.

Promoted Cairn task evidence should cite copied artifacts under `docs/evidence/` with BLAKE3 manifests. Direct paths under `clients/stevenarella/`, `servers/valence/`, `hyperion/`, or `target/` are not reviewable evidence artifacts.
