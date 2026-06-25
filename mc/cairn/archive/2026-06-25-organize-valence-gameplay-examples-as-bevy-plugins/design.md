# Design: Organize Valence gameplay examples as Bevy plugins

## Context

The examples already rely on Bevy ECS, but some compatibility fixtures and gameplay demos register many systems directly from `main`. A plugin boundary would make the imperative shell explicit while keeping pure fixture cores testable without a running Bevy world.

## Decisions

### 1. Inventory before refactor

**Choice:** Record the selected examples' systems, schedules, resources, events, env toggles, milestone emitters, and non-goals before moving code.

**Rationale:** Reviewers need to prove the refactor preserves fixture behavior and only changes organization.

### 2. Plugins own wiring, not decisions

**Choice:** Each extracted plugin registers resources, events, systems, and `SystemSet` ordering. Deterministic CTF, survival, inventory, combat, and terrain decisions stay in pure functions or data structures.

**Rationale:** Bevy should remain the shell around testable gameplay cores.

### 3. Named sets expose gameplay phases

**Choice:** Use public or module-local `SystemSet`s for phases such as input collection, rule decisions, ECS mutation, presentation, and cleanup.

**Rationale:** Named sets make schedule graphs reviewable and let user systems order against stable boundaries without depending on tuple order.

### 4. No default behavior change

**Choice:** Example plugins are added by the example binaries only. `DefaultPlugins` and Valence core plugin groups are unchanged unless a separate Cairn scopes that change.

**Rationale:** Organizing examples must not alter ordinary Valence users' runtime behavior.

## Risks / Trade-offs

- A plugin split can hide data flow if sets are too broad; keep set names concrete and document ordering.
- Compatibility milestones are receipt-stable; tests must prove strings and env behavior remain compatible.
- Over-generalizing examples into framework APIs is out of scope unless a later Cairn promotes them.
