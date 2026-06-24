# Admin permission ergonomics scope checkpoint

## Question

How should Valence define admin permission ergonomics after comparing Hyperion's group/admin command behavior with Valence command scopes?

## Inspected evidence

- Hyperion permission group and refresh behavior: `hyperion/crates/hyperion-permission/src/lib.rs`.
- Hyperion permission storage boundary: `hyperion/crates/hyperion-permission/src/storage.rs`.
- Hyperion command permission diagnostics: `hyperion/crates/hyperion-clap/src/lib.rs`.
- Hyperion command permission derive: `hyperion/crates/hyperion-clap-macros/src/lib.rs`.
- Hyperion command registry shape: `hyperion/crates/hyperion-command/src/*`.
- Valence command scope registry and command manager: `servers/valence/crates/valence_command/src/scopes.rs` and `manager.rs`.

## Decision

Use Hyperion as reference/port input only. Valence keeps the existing command graph and scope registry as the authority for command visibility and execution denial. The implementation adds a Valence-owned pure admin permission evaluator and an optional plugin shell that maps explicit admin profiles into `CommandScopes`; it does not copy Hyperion code, storage, derive macros, or command framework behavior.

## Boundary classification

| Hyperion input | Classification | Valence decision |
| --- | --- | --- |
| Permission groups and command refresh observer | reference | Model role-to-scope ergonomics and preserve Valence command scopes as the integration point. |
| LMDB permission storage | reject | Keep storage caller-owned behind a trait and typed row adapter. |
| Clap command permission denial | port | Reimplement explicit allow/deny diagnostics over Valence metadata and scopes. |
| CommandPermission derive macro | reject | Do not add a parallel derive/macro permission system. |
| Hyperion command registry callbacks | reference | Retain Valence scoped graph traversal and command-tree refresh on `CommandScopes` changes. |

## Owner

`define-admin-permission-ergonomics`.

## Next action

Review the implementation and validation logs cited from the Cairn task list. This checkpoint is evidence for scope and boundary decisions only; it is not a broad Minecraft compatibility, Hyperion compatibility, production moderation, storage migration, or public-server safety claim.
