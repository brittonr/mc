# Design: Organize core Minecraft components by role

## Context

The repository recently absorbed Stevenarella and Valence into the parent source tree. The runner now scopes revision evidence to source paths rather than treating those trees as nested Git repositories, but the directory layout still exposes them as top-level names beside Cairn, docs, tools, and Hyperion.

Because these components are becoming core, a `vendor/` or `third_party/` boundary would be misleading. The durable boundary should be product role: client, server, and compatibility harness.

## Decisions

### Role-based component roots

**Choice:** Move core client implementations under `clients/` and core server implementations under `servers/`. Move compatibility-specific orchestration under `compat/` only where the path move can be made without weakening generated-surface or evidence checks.

**Rationale:** Role names describe how the project uses the code today. Upstream ancestry remains important metadata, but it is not the primary ownership model.

### Compatibility harness boundary

**Choice:** Treat `compat/` as the home for the mc-compat product: runner, scenario manifest, generated scenario surfaces, wrappers, and Paper/reference fixtures. Keep human evidence under `docs/evidence/` unless a later evidence re-layout change is created.

**Rationale:** The runner, manifest, fixtures, and generated surfaces form one product. Evidence has stronger archival and review semantics, so moving it should not be bundled into the first source-layout migration.

### Central layout resolver

**Choice:** Introduce one typed layout resolver consumed by runner startup, dry-run wrappers, generated docs, and checks. The resolver should accept transitional and final layouts during migration, then fail closed for missing or ambiguous component roots.

**Rationale:** Ad hoc path checks caused the previous nested-repo-to-vendored transition to need scattered updates. A central resolver makes the final move reviewable and testable.

### Path-scoped revision evidence remains canonical

**Choice:** Continue recording source-tree scoped revision and dirty-state evidence with parent repository commands, using the resolved component root as the scope.

**Rationale:** After absorption, parent `HEAD` alone is too broad and nested Git state is intentionally absent. Evidence must remain tied to the component path.

### Hyperion stays out of scope

**Choice:** Do not move or absorb `hyperion/` in this change.

**Rationale:** Hyperion remains an independent nested repo with its own workflow. Absorbing or re-homing it needs a separate decision and preservation plan.

## Risks / Trade-offs

- Large path moves can create noisy diffs; mitigate by separating pure moves from behavior edits where possible.
- Transition-path support can linger; mitigate by adding tasks to remove fallback acceptance once final paths are in place.
- Nix wrappers and evidence checkers may hide path assumptions; mitigate with generated-surface checks and negative layout fixtures.
- Documentation can accidentally call core components vendors or forks; mitigate with an explicit terminology update and reviewer-facing architecture map.
