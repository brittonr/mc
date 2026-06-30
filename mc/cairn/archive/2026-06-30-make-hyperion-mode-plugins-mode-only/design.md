# Design: Make Hyperion mode plugins mode-only

## Context

The existing mode plugins set `ActiveGameType`, add `CommonGameplayPlugin`, and register a mode-specific `OnAdd<packet_state::Play>` observer. This makes them convenient presets but prevents independent composition of shared mechanics and mode identity.

## Decisions

### 1. Mode plugin owns only mode-local setup

**Choice:** Each mode plugin inserts its active mode identity and registers only mode-specific observers/resources/components.

**Rationale:** A Bevy plugin named after a mode should be safe to add alongside custom shared gameplay choices.

### 2. App builders preserve defaults

**Choice:** Existing entrypoints compose the public default gameplay group plus the selected mode to retain runtime behavior.

**Rationale:** Users of current `init_game*` and `build_game_app*` APIs should not lose mechanics from this refactor.

### 3. Naming separates mode from preset

**Choice:** Documentation and code distinguish `BedwarsModePlugin`-style mode plugins from preset/app-builder surfaces that include shared gameplay.

**Rationale:** Clear names reduce accidental double-installation and make tests easier to reason about.

### 4. Mode-only tests assert absence

**Choice:** Tests should verify both what mode plugins install and what they intentionally do not install.

**Rationale:** The key regression risk is shared gameplay creeping back into mode plugins.

## Risks / Trade-offs

- Existing downstream code may expect `BedwarsPlugin` to be a preset; compatibility aliases or deprecation windows may be needed.
- Splitting install ownership can expose missing dependency assumptions in shared gameplay.
- The active-game resource still represents one selected mode until exclusive-mode validation changes it.
