# Admin permission ergonomics

This note records the Hyperion comparison and the Valence-owned integration scope for optional admin permission ergonomics.

## Integration decision

Valence keeps command ownership in `valence_command`. Hyperion's group and Clap-derived command permission model is useful as a reference for ergonomics, but this work ports the idea into Valence scopes instead of adding a parallel command framework or importing Hyperion storage.

## Hyperion inventory

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion-permission/src/lib.rs` `Group`, `PermissionPlugin`, command refresh observer | reference | `define-admin-permission-ergonomics` | Shows role/group ergonomics and refresh-on-permission-change behavior; no code is copied. | `valence_command::admin_permissions` pure role-to-scope model and optional plugin shell | Avoids Hyperion ECS/storage coupling and keeps Valence command scopes authoritative. | `cargo test -p valence_command`; Cairn gates and validation logs under `docs/evidence/` | No Hyperion compatibility, production moderation suite, default Valence behavior change, or storage migration claim. |
| `hyperion/crates/hyperion-permission/src/storage.rs` LMDB permission storage | reject | `define-admin-permission-ergonomics` | Storage is server-specific and should remain behind a Valence-owned boundary. | Optional `AdminPermissionStorage` trait and typed row adapter only | No LMDB/heed dependency or persistence behavior is imported. Invalid and missing rows are tested as boundary fixtures. | Storage adapter unit tests in `valence_command` | No persistence backend, data migration, operational security, or production retention claim. |
| `hyperion/crates/hyperion-clap/src/lib.rs` command permission check and denial message | port | `define-admin-permission-ergonomics` | The explicit allow/deny diagnostic idea maps cleanly to Valence command metadata and scopes. | `AdminPermissionDecision`, `AdminPermissionDenial`, and existing scope checks in `manager.rs` | Reimplemented as pure deterministic decisions over metadata, profiles, role bindings, and `CommandScopeRegistry`; no Clap dependency is added. | Positive allowed-command and negative denied-command diagnostics tests | No Clap-derived Valence command framework or public moderation policy claim. |
| `hyperion/crates/hyperion-clap-macros/src/lib.rs` `CommandPermission` derive | reject | `define-admin-permission-ergonomics` | Macro-generated group checks would duplicate Valence's command graph builder and scope registry. | none | Avoids proc-macro API churn and keeps command metadata explicit in Valence scopes. | Scope comparison in this note | No derive-macro support claim. |
| `hyperion/crates/hyperion-command/src/*` registry and permission callback shape | reference | `define-admin-permission-ergonomics` | Confirms Hyperion handles visibility through command registration callbacks; Valence already has scoped graph traversal. | Existing `valence_command` scope filtering plus extracted pure evaluator | Keeps Valence command tree refresh tied to `Changed<CommandScopes>`. | `valence_command` tests and command manager refactor | No replacement of Valence command graph, packet parsing, or execution dispatch. |

## Valence model

- Command metadata stays in the existing command graph. Node scopes remain the visibility and execution gate.
- `admin_permissions` provides a pure evaluator over command metadata, effective scopes, profiles, role bindings, and explicit context.
- `AdminPermissionPlugin` is optional. It only maps an `AdminPermissionProfile` into `CommandScopes`; `CommandScopes` changes continue to trigger the existing command-tree refresh path.
- Storage is optional and caller-owned. Valence exposes a small `AdminPermissionStorage` trait and a pure row adapter for missing/invalid-row behavior.
- Plugin-disabled behavior is a no-op for command scopes, preserving existing command registration, parsing, execution, and suggestions.

## Moderation layering guidance

Servers can layer moderation policy by choosing role names, role-to-scope bindings, storage, audit logging, and user-facing denial text outside Valence core. Valence provides deterministic decision and scope-refresh ergonomics only.

## Non-claims

This work does not provide a production moderation suite, public-server safety policy, Hyperion compatibility, command macro parity, persistent permission database, or broad Minecraft compatibility claim.
