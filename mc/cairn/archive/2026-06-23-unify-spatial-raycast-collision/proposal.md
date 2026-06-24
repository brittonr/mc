# Proposal: Unify spatial raycast and collision helpers

## Why

Valence already has `valence_spatial` and chunk/entity spatial structures, while Hyperion has useful geometry, ray traversal, and combined entity/block collision patterns. Integrating the stable concepts would give Valence a clearer reusable raycast/collision API for gameplay plugins without importing Hyperion's nightly or unsafe-heavy BVH code directly.

## What Changes

- Inventory Valence spatial APIs and Hyperion geometry/raycast/collision helpers.
- Define a stable Valence math/core API for entity hitboxes, block ray traversal, owner exclusion, and entity-vs-block collision ordering.
- Port concepts using stable, audited code and existing Valence math types where possible.
- Add positive and negative fixtures for axis-aligned rays, diagonal rays, inside-hitbox starts, boundary hits, NaN/invalid directions, owner exclusion, and block/entity tie cases.
- Document how gameplay plugins should use the new helpers.

## Impact

- **Files**: `valence_spatial`, `valence_server` interaction or layer helpers, tests, examples/docs, and Cairn artifacts.
- **Testing**: pure geometry fixtures, property-style edge cases if available, gameplay smoke tests, selected projectile/raycast mc-compat scenarios, and Cairn gates/validation.
- **Non-claims**: this does not replace Valence's BVH wholesale, does not import Hyperion's nightly-only code directly, and does not claim vanilla combat parity by itself.
