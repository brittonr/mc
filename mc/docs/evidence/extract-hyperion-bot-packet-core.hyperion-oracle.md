# Hyperion bot packet core implementation oracle

## Question

Which source revision contains the Hyperion-local implementation for `extract-hyperion-bot-packet-core`, given the parent mc repository does not track Hyperion source files?

## Inspected evidence

- Hyperion nested repository jj commit: `2dc235d6` (`extract bot packet core for testable tooling boundaries`).
- Hyperion working copy after commit: clean (`docs/evidence/extract-hyperion-bot-packet-core.hyperion-bot-tests-final.run.log` and `docs/evidence/extract-hyperion-bot-packet-core.hyperion-clippy.run.log` were recorded before the commit).
- Parent Cairn archive path: `cairn/archive/2026-06-28-extract-hyperion-bot-packet-core/`.

## Decision

Treat Hyperion commit `2dc235d6` as the implementation revision for this Cairn archive. The parent commit records governance, accepted spec text, and durable evidence only; it does not vendor or claim ownership of Hyperion source files.

## Owner

`extract-hyperion-bot-packet-core`.

## Next action

Use the Hyperion nested repository at commit `2dc235d6` with the parent evidence logs when reviewing this archive. Any Valence adoption, porting, public-server safety claim, or broad compatibility claim requires a separate accepted Cairn.
