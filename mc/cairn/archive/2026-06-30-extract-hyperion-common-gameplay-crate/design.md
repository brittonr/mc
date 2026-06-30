# Design: Extract Hyperion common gameplay crate

## Context

The current `events/bedwars` crate owns mechanics that appear common: attacks, blocks, bows, chat, damage, regeneration, skin, spawn, stats, vanish, commands, maps, item interactions, permissions, and proxy integration. Some may still encode Bedwars assumptions.

## Decisions

### 1. Classify before moving

**Choice:** Classify modules as common, Bedwars-specific, or uncertain before any filesystem move.

**Rationale:** Mechanical relocation without ownership review can make Bedwars assumptions public.

### 2. Shared boundary should be named after gameplay, not Bedwars

**Choice:** Prefer a shared crate such as `hyperion-gameplay` or an explicitly common event module for reusable mechanics.

**Rationale:** Public paths should reflect ownership and intended reuse.

### 3. Move only stable common seams

**Choice:** Move mode-neutral plugin structs, pure cores, events, resources, and plugin-group exports first; leave uncertain or Bedwars-specific code in the event crate.

**Rationale:** A staged move reduces behavior risk and review noise.

### 4. Preserve default preset behavior

**Choice:** Existing default mode builders continue to install the same mechanics after imports are updated.

**Rationale:** This Cairn changes code ownership and API shape, not gameplay semantics.

## Risks / Trade-offs

- Crate extraction can cause dependency cycles; the inventory must identify dependencies before moving code.
- Moving many modules can create large diffs; scope may need staged commits by feature cluster.
- Some names may need compatibility re-exports to avoid downstream breakage.
