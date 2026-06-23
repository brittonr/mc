# Tasks

- [ ] [serial] Inventory Valence spatial APIs and Hyperion geometry/raycast/collision helpers, classifying code as adopt, port, reference, or reject. r[valence_hyperion_integration.spatial_raycast.inventory]
- [ ] [depends:inventory] Define deterministic raycast/collision semantics for entity hitboxes, block hits, owner exclusion, invalid rays, boundary hits, and tie ordering. r[valence_hyperion_integration.spatial_raycast.contract]
- [ ] [depends:contract] Implement pure math/core helpers and thin ECS/world shells using Valence-owned types. r[valence_hyperion_integration.spatial_raycast.core]
- [ ] [depends:core] Add positive and negative fixtures for axis-aligned, diagonal, inside-hitbox, boundary, NaN, zero-direction, owner-exclusion, and block/entity tie cases. r[valence_hyperion_integration.spatial_raycast.fixtures]
- [ ] [depends:fixtures] Wire optional gameplay/plugin helpers and docs without claiming vanilla combat parity. r[valence_hyperion_integration.spatial_raycast.wiring]
- [ ] [depends:wiring] Run geometry tests, gameplay smoke tests, selected projectile/raycast mc-compat scenarios, Cairn gates, and Cairn validation. r[valence_hyperion_integration.spatial_raycast.validation]
