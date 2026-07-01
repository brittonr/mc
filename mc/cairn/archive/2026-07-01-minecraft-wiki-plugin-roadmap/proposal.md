# Proposal: Build a Minecraft Wiki-guided composable plugin roadmap

## Why

Valence is intentionally modular: vanilla mechanics, scripting, dedicated executables, and opinionated gameplay belong in optional plugins rather than in one always-on server core. The Minecraft Wiki is a useful index of vanilla domains and behavior seams, but using it ad hoc can create unstable plugin boundaries, target-version drift, and overbroad compatibility claims.

A reviewable roadmap should turn wiki domains into bounded plugin contracts, behavior cards, pure rule cores, thin Bevy system shells, and parity evidence requirements before implementation starts.

## What Changes

- Introduce a `vanilla-composable-plugins` lifecycle capability for wiki-guided Valence plugin work.
- Inventory Minecraft Wiki domain entry points such as blocks, items, crafting, smelting, enchanting, mobs, biomes, effects, commands, data packs/resource packs, and protocol documentation.
- Map wiki domains to candidate plugin groups, dependencies, version scope, non-claims, and follow-on implementation slices.
- Define a behavior-card template for each feature seam: source pages, target Java/protocol version, pure core, Bevy shell, positive tests, negative tests, schedule impact, and parity evidence.
- Require Paper/vanilla or extracted-data evidence before promoting behavior claims derived from wiki descriptions.

## Impact

- **Files**: Cairn change artifacts, a new accepted `vanilla-composable-plugins` spec on sync, roadmap/inventory docs under `docs/` or `docs/evidence/`, and later follow-on Cairns for individual plugins.
- **Testing**: Cairn proposal/design/tasks gates, Cairn validation, evidence-manifest checks for promoted docs, and no component test requirement until an implementation slice changes Valence or compatibility code.
- **Non-claims**: this does not implement plugins, change `DefaultPlugins`, change Valence behavior, prove vanilla parity, or authorize copying wiki text/tables into source without attribution review.
