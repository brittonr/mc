# Proposal: Guard Hyperion integration boundaries

## Why

Hyperion contains useful ideas, but also repo-specific runtime architecture, Bedwars game logic, custom combat behavior, nightly features, and unsafe-heavy implementations that should not be merged into Valence core by accident. A boundary Cairn gives reviewers explicit criteria for what may be adopted, ported, referenced, or rejected.

## What Changes

- Create a reviewable Hyperion integration inventory with adopt, port, reference, and reject classifications.
- Define forbidden core merges: Bedwars-specific logic, full Hyperion runtime replacement, custom combat as Valence core, and direct import of nightly/unsafe-heavy code without audit.
- Require optional-plugin boundaries and compatibility/reference evidence for any gameplay semantics such as combat.
- Add checks or review templates that force non-claim language and explicit evidence before integration Cairns are archived.
- Record validation logs for boundary classification and Cairn gates.

## Impact

- **Files**: Cairn specs, integration inventory docs, review/check templates if added, and any future integration Cairns that cite this boundary.
- **Testing**: docs/check validation, inventory consistency checks if implemented, negative fixtures for forbidden classifications, and Cairn gates/validation.
- **Non-claims**: this is a governance/boundary change; it does not implement any Valence feature by itself.
