# Proposal: Add a Valence packet compose API

## Why

Hyperion's `Compose`-style packet API gives game systems a high-level way to build packet bundles and route them as unicast, global, local, or channel broadcasts. Valence exposes packet writing on clients, but higher-level broadcast composition would reduce duplicated packet fanout logic and prepare Valence for optional proxy mode.

## What Changes

- Define a Valence packet composition contract for bundling and routing outbound packets.
- Keep packet selection and delivery planning pure, with the flush shell responsible for client/proxy writes.
- Support direct-mode routing first, while leaving route intents compatible with future proxy backend work.
- Add positive and negative tests for packet bundles, ordering, exclusions, invalid routes, closed clients, and flush errors.
- Document when users should prefer direct client writes versus composed broadcast intents.

## Impact

- **Files**: `valence_server`, optional networking helpers, examples/docs, tests, and Cairn artifacts.
- **Testing**: pure planner tests, route-mode fixtures, direct flush regressions, examples or playground smoke tests, selected mc-compat dry runs, and Cairn gates/validation.
- **Non-claims**: this is an API/ergonomics layer; it does not require proxy mode, change default networking, or prove large-scale performance by itself.
