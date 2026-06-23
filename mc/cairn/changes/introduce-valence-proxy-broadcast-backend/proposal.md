# Proposal: Introduce a Valence proxy broadcast backend

## Why

Hyperion demonstrates a useful large-scale pattern: keep game-state simulation in one Valence-like server process while moving player socket fanout, regional broadcast selection, and per-player packet delivery into horizontally scalable proxies. Valence currently treats direct client networking as the normal path. An optional proxy backend would let high-player-count servers adopt Hyperion's broadcast architecture without changing the default Valence API or claiming full Hyperion runtime compatibility.

## What Changes

- Review Hyperion's proxy message model, Valence's `valence_network`/`valence_server` packet flow, and the compatibility seams between them.
- Define a proxy message contract for unicast, global broadcast, local broadcast, channel broadcast, subscription updates, player position updates, and shutdown.
- Implement a pure routing/planning core that maps server messages plus proxy-observed player state to delivery decisions.
- Add an optional Valence network backend or plugin that can use the proxy transport while preserving existing direct-mode behavior.
- Add positive and negative tests for routing, ordering, exclusions, stale subscriptions, disconnected streams, backpressure, and malformed proxy messages.

## Impact

- **Files**: Valence networking crates, optional proxy crate/plugin, compatibility docs, focused tools/tests, and Cairn artifacts.
- **Testing**: routing core unit tests, proxy message fixtures, direct-mode regression tests, proxy smoke tests, malformed-message rejection, selected mc-compat dry runs, and Cairn gates/validation.
- **Non-claims**: this does not merge Hyperion's runtime into Valence, does not make proxy mode the default, and does not claim full large-scale production readiness until load evidence exists.
