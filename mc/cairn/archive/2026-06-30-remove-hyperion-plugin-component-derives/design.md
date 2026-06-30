# Design: Remove Hyperion plugin Component derives

## Context

`BedwarsPlugin`, `DayzPlugin`, `HardcoreFactionsPlugin`, and other plugin structs should be Bevy plugin values. ECS entity state should use marker components such as player mode markers, not plugin structs.

## Decisions

### 1. Inventory first

**Choice:** Search for plugin structs deriving `Component` and any insertion/query uses before removing derives.

**Rationale:** If a type is truly used as ECS state, it should be renamed or split intentionally.

### 2. Separate plugin types from markers

**Choice:** Plugin structs keep plugin-only derives; ECS marker components get their own names and tests.

**Rationale:** This makes composition APIs and ECS ownership easier to understand.

### 3. No behavior change

**Choice:** Removing unused `Component` derives should not alter plugin installation or app behavior.

**Rationale:** The change is hygiene that reduces confusion before larger marker work.

### 4. Guard with negative tests

**Choice:** Tests or compile checks should fail if code assumes plugin structs are queryable entity components after cleanup.

**Rationale:** The point is to prevent plugin-as-component coupling from returning.

## Risks / Trade-offs

- If downstream code uses plugin structs as markers, compatibility aliases or replacement marker components may be needed.
- This cleanup may be easiest after marker/run-condition work defines replacement marker names.
- Compile-only tests may be sufficient if no runtime marker behavior changes.
