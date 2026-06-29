# Proposal: Modularize Hyperion packet inspector UI

## Why

`hyperion/tools/packet-inspector/src/app/packet_list.rs` concentrates packet-list state, filtering, selection, rendering, and UI interaction logic. Packet inspector is Hyperion tooling, so UI state/filter decisions should be pure and Hyperion-local while rendering shells remain separate.

## What Changes

- In Hyperion's packet inspector, split packet-list code into modules for list state, filters, selection, sorting/grouping, render models, and UI rendering shell.
- Extract pure filter, selection, sorting, grouping, and render-model decisions.
- Keep terminal/UI framework rendering, event loop integration, IO, and logging in shells.
- Preserve packet inspector UI behavior, keyboard/mouse interactions, filter semantics, public tool behavior, and non-claims.

## Impact

- **Files**: Hyperion packet-inspector modules under `hyperion/`, Hyperion tool tests, optional parent evidence notes, and Cairn artifacts.
- **Testing**: Hyperion baseline/focused packet-inspector tests from `hyperion/`, Cairn gates, and Cairn validation.
- **Non-claims**: Hyperion tool UI architecture only; no Valence adoption or compatibility evidence claim is promoted.
