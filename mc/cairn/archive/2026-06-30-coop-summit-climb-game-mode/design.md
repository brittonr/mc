# Design: Add cooperative summit climb game mode

## Context

Hyperion's current game-specific event code lives under `hyperion/events/bedwars`, with `BedwarsPlugin` registering mode systems and shared Hyperion plugins. A summit-climb mode should follow that event-crate pattern while keeping experimental rules scoped away from Bedwars and core engine defaults.

Minecraft clients do not natively climb arbitrary solid faces. A faithful mode feel therefore cannot be achieved by only placing ladders, vines, scaffolding, or similar climbable blocks on the mountain. The server needs an explicit free-surface climb rule: when the player is in contact with an eligible climb face and provides valid movement input, the mode consumes stamina and applies vertical/along-face motion authoritatively.

## Decisions

### 1. Create a separate Hyperion event crate

**Choice:** Add a `summit_climb` event crate/plugin rather than extending `events/bedwars`.

**Rationale:** The mode has different win conditions, movement rules, inventory limits, and co-op failure semantics. Separate plugin ownership keeps Bedwars and shared engine behavior stable.

### 2. Make the runtime architecture Bevy-first

**Choice:** Express mode runtime state through Bevy primitives wherever they fit: components for player/team/arena/item/hazard/checkpoint state, resources for configuration and global mode indices, events for domain actions, `SystemSet`s for phase ordering, run conditions for mode gates, hierarchy relationships for arena-owned entities, observers for lifecycle hooks, and Bevy task-pool integration for async mountain generation or expensive route analysis if needed.

**Rationale:** Hyperion is already ECS-driven. Leaning on Bevy keeps the mode composable, observable, testable in small apps, and consistent with the rest of the engine instead of introducing a parallel game-loop manager.

### 3. Enable terrain through a mode-owned mountain source

**Choice:** Add a summit-climb terrain boundary that can load a dedicated hand-authored or pre-generated mountain save first, and can later swap in a mode-owned generator if it produces the same explicit terrain contract. The terrain source records spawn/base camp, staged regions, route corridors, campsites, summit goal volumes, hazard volumes, rescue affordances, and cleanup ownership.

**Rationale:** A purpose-built mountain is the fastest way to tune climbing feel and review safety. Keeping the source behind a mode-owned boundary avoids coupling summit climb to Bedwars maps, Valence terrain examples, or global Hyperion defaults.

### 4. Separate terrain affordance metadata from raw blocks

**Choice:** Treat blocks as geometry and typed terrain metadata as gameplay meaning. Route segments, rest shelves, campsite checkpoints, hazard zones, rescue/fallback paths, summit triggers, and no-build or cleanup volumes live in deterministic config/data that pure validators can inspect without a running server.

**Rationale:** Rock, ice, gravel, water, and decorative blocks are not enough to prove route quality. Explicit metadata lets tests reject impossible gaps, missing campsites, orphaned hazards, unsafe spawn placement, and accidental no-cost vanilla climbable dependencies.

### 5. Model free-surface climbing as a pure rules core

**Choice:** Implement climb eligibility and motion as pure functions over explicit inputs: contact state, surface classification, player input summary, stamina/ailment state, elapsed tick, and named config values. Bevy systems gather inputs, call the core, apply outputs, send packets, and render feedback.

**Rationale:** Free-surface climbing is the highest-risk rule. A pure core makes positive and negative behavior testable without a live server, packet session, or generated mountain, while Bevy owns scheduling, state, and side effects.

### 6. Allow vanilla climbables only as stamina-consuming assists

**Choice:** The primary climbing path must not require no-cost ladders, vines, scaffolding, bubble columns, water elevators, trapdoor tricks, or map-placed vanilla climbable block rails. When ladder, vine, scaffold, water, or bubble-column surfaces are explicitly configured as route aids or hazards, the mode still classifies them through the climb policy and consumes stamina while they contribute ascent or grip. Climbing assist items may create ropes or anchors as mode-owned entities/blocks, but ordinary mountain faces remain climbable through the mode's movement rule.

**Rationale:** The user explicitly requested climbing without those blocks, but making every ascent aid spend stamina preserves the expedition pressure and lets map designers use familiar Minecraft affordances without bypassing the mode.

### 7. Keep movement server-authoritative and fail closed

**Choice:** The shell applies climb results only while the player is alive, in the summit-climb arena, in contact with an eligible face, within stamina bounds, and below configured anti-teleport limits. Invalid contacts, exhausted stamina, wrong mode, spectator/admin states, stale player state, or forbidden surfaces produce no climb motion and may emit diagnostics.

**Rationale:** A custom movement rule can otherwise become an exploit path for vertical flight, cross-mode movement mutation, or rubber-band loops.

### 8. Use a cooperative expedition loop

**Choice:** The mode has a team expedition loop: spawn at crash/base camp, climb through named mountain regions, rest at campsites, use limited item slots, revive downed teammates, and win at the summit when required team conditions are met.

**Rationale:** The mode should be understandable to players and testable as staged rules instead of a collection of unrelated movement tricks.

### 9. Preserve IP and presentation boundaries

**Choice:** Use original mode, role, item, enemy, biome, and cosmetic names. Do not copy Peak names, art, UI, characters, audio, merch, event references, or branded terms into code, docs, or fixtures except as an external design motivation in proposal history.

**Rationale:** Mechanics can inspire a Minecraft mode, but protected presentation should remain out of the repo.

### 10. Isolate configuration and constants

**Choice:** All numeric tuning values, including team size, stamina capacity, drain rates, recovery rates, contact reach, climb speed, slip thresholds, region count, revive time, separation distance, and campsite restore amounts, live in named config/constants or test fixtures.

**Rationale:** The mode will require balance iteration and must obey the repository rule against magic numbers.

## Risks / Trade-offs

- Server-authoritative arbitrary climbing may feel jittery without client-side support; start with explicit feedback and bounded correction before optimizing feel.
- A Bevy-first model can become too fragmented if every tiny field becomes an entity. Use components/events where they improve ownership, scheduling, or tests; keep pure rule inputs compact.
- Movement hooks may require reusable Hyperion seams. If so, keep the reusable API generic and keep summit-climb policy in the event crate.
- Allowing natural surfaces to be climbable creates exploit risk. Surface classification, mode scoping, and negative tests must be stronger than the happy path.
- Terrain can imply more capability than implemented. Validation must distinguish hand-authored maps, generator experiments, route affordance checks, and non-claims about vanilla terrain parity.
- Assist items can become map-editing tools if not scoped. Rope and anchor placement must have ownership, lifetime, range, and cleanup rules.
