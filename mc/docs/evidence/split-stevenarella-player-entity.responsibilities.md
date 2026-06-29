# Split Stevenarella player entity responsibilities

## Question

What responsibilities did `clients/stevenarella/src/entity/player.rs` own before the split, and what boundaries should reviewers inspect after the refactor?

## Inspected evidence

- `clients/stevenarella/src/entity/player.rs` before implementation combined player entity construction, `PlayerModel` state, render-system shell/model assembly, `PlayerMovement` key state, movement update logic, collision resolution, and ECS system registration.
- The affected public API was `entity::player::add_systems`, `entity::player::create_local`, `entity::player::create_remote`, `entity::player::PlayerModel`, and `entity::player::PlayerMovement`.
- The pre-implementation focused baseline command was `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test entity::player -- --nocapture` from `clients/stevenarella`; it compiled and reported `0 passed; 0 failed; 332 filtered out` before player tests were added.

## Decision

The split keeps the public `entity::player::*` API stable while assigning focused ownership:

- `construction.rs`: local/remote player construction facts and ECS component attachment shell.
- `model_state.rs`: player model flags, animation state, visible part decisions, and pure animation-frame updates.
- `movement.rs`: movement key state and pure movement-vector calculation.
- `collision.rs`: pure AABB collision/slide decisions plus the world-query collision shell.
- `rendering.rs`: renderer shell, model assembly, renderer lifecycle planning, and calls into pure model-state decisions.
- `systems.rs`: ECS system registration and movement-system side effects.

This evidence is architectural/refactor evidence only. It does not promote broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claims.

## Owner

Stevenarella client subtree: `clients/stevenarella/`.

## Next action

Use the focused validation log and Cairn gates before syncing and archiving `split-stevenarella-player-entity`.
