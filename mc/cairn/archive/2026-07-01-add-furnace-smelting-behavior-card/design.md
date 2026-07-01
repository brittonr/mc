## Context

`docs/vanilla-composable-plugins-roadmap.md` sequences bounded survival stats first and already sketches furnace smelting as a good first seam. The accepted `vanilla-composable-plugins` spec requires behavior cards, pure deterministic rule cores, thin Bevy/ECS shells, positive and negative tests, and explicit evidence before wiki-guided behavior claims are promoted.

This change is behavior-card-only. It prepares a reviewable contract for a future implementation Cairn without changing Valence code, runtime behavior, or default plugin membership.

## Decisions

### 1. Start with standard furnace selected-row behavior

**Choice:** Scope the card to one future standard-furnace selected-row implementation slice for Java Edition 1.20.1 / protocol 763.

**Rationale:** Standard furnace smelting reuses deterministic state transition logic, target-version recipe/fuel data, and block-entity tick scheduling, while avoiding broader smoker, blast furnace, hopper, XP, recipe-book, and chunk-unload behavior.

### 2. Make wiki pages discovery inputs, not proof

**Choice:** Record Minecraft Wiki Smelting, Block entity, and Java Edition 1.20.1 pages as untrusted guidance, and require extracted data plus Paper/vanilla receipts before any vanilla behavior claim.

**Rationale:** Current wiki pages drift across releases. The card may guide vocabulary and seams, but target-version behavior needs repo-local evidence.

### 3. Validate card structure with a focused checker

**Choice:** Add `tools/check_furnace_smelting_behavior_card.rs` as a small Rust script with a pure validation core and an imperative shell for file reads/CLI exit handling.

**Rationale:** The checker proves the card contains source scope, target version, bounded claim, non-claims, pure core, ECS shell, positive tests, negative tests, evidence requirements, and stop conditions. Self-tests cover both valid and invalid cards.

## Risks / Trade-offs

- A behavior-card-only change does not reduce implementation work; it deliberately prevents overclaiming before data extraction and parity evidence exist.
- The card names future code shapes without binding Valence to exact public API names.
- Additional furnace behavior such as hoppers, XP, smoker, blast furnace, recipe book, and chunk unloading remains out of scope until separate cards and receipts exist.
