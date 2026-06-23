# Proposal: Define Valence admin permission ergonomics

## Why

Hyperion layers a permission group component and Clap-derived command ergonomics over its command system. Valence already has a command graph/scope system, so the useful integration is not a direct copy but a review of admin ergonomics: permission scopes, command visibility, command-tree refresh, and optional storage.

## What Changes

- Compare Hyperion permission/admin command behavior with Valence's command and scope crates.
- Define an optional permission/admin ergonomics model that fits Valence command scopes.
- Specify command visibility refresh, denied-command diagnostics, storage boundaries, and migration guidance.
- Add positive and negative tests for allowed commands, denied commands, missing permission data, stale command trees, invalid storage rows, and plugin-disabled behavior.
- Document how servers can layer moderation policy on top.

## Impact

- **Files**: `valence_command`, optional admin/permission plugin, docs/examples, tests, and Cairn artifacts.
- **Testing**: pure permission evaluator tests, command integration tests, storage fixtures if storage is added, plugin-off regressions, and Cairn gates/validation.
- **Non-claims**: this does not replace Valence's command graph, does not make a production moderation suite, and does not import Hyperion permission storage directly.
