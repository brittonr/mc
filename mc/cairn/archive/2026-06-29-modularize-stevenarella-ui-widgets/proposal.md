# Proposal: Modularize Stevenarella UI widgets

## Why

`clients/stevenarella/src/ui/mod.rs` contains layout regions, containers, image batches, text rendering, formatted text, buttons, text boxes, focus/input behavior, and tests in one module. UI widgets are reusable but currently coupled through one broad owner.

## What Changes

- Split UI code into modules for layout regions, containers/element trees, image/batch rendering, text and formatted text, buttons, text boxes, and input/focus helpers.
- Extract pure layout, attachment, focus, text-formatting, and widget-state decisions.
- Keep renderer calls, clipboard access, input event side effects, and resource access in shells.
- Preserve widget behavior, public UI builder APIs, text formatting behavior, input semantics, and non-claims.

## Impact

- **Files**: `clients/stevenarella/src/ui/mod.rs`, new UI modules, focused UI tests, and Cairn artifacts.
- **Testing**: baseline UI/client tests, positive and negative widget-core tests, Cairn gates, and Cairn validation.
- **Non-claims**: UI architecture only; no compatibility or rendering evidence is promoted.
