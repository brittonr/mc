# Design: Paired survival reference parity receipts

## Fixture strategy

Use the existing Stevenarella fixed-coordinate survival probe. Add a local reference backend with the same starting block, player spawn, and inventory expectations where possible.

## Comparator

Compare normalized fields instead of raw logs:

- join/render success;
- break target coordinate and resulting block state;
- pickup or inventory slot transition;
- place target coordinate and resulting block state;
- forbidden error patterns.

All promoted metrics for this first rail are exact-match metrics. Any server-specific behavior that cannot be normalized remains a non-claim.

## Evidence

Copy receipts and logs into `docs/evidence/`, record BLAKE3, and add a parity evidence doc that cites both child revisions and reference backend version.
