# Proposal: Promote chunk biome data packet evidence

## Why

`ChunkBiomeDataS2CPacket` remains a named non-claim in the chunk/biome packet family despite existing bounded overworld dimension and chunk evidence. A narrow packet row would reduce a visible protocol gap without claiming all biome semantics.

## What Changes

- Add a bounded `chunk-biome-data-packet` row for one configured overworld biome data payload or parser fixture plus live receipt context.
- Require deterministic parser/fixture evidence and, when possible, client observation that the row does not break join/chunk/render milestones.
- Promote only the configured packet row, keeping all biome semantics, all chunks, all worldgen packets, dimension travel, full protocol coverage, and production readiness as non-claims.

## Impact

- **Files**: packet fixture/checker, packet inventory/current bundle docs, evidence KV/receipts/manifests, and Cairn specs/tasks.
- **Testing**: positive/negative parser/checker fixtures, scenario or fixture gates, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
