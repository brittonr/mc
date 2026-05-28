# Design: Broad protocol 763 coverage proof

## Coverage strategy

Use a coverage ledger as the functional core of the proof. The ledger should join three sources: Valence protocol-763 packet metadata, Stevenarella protocol mappings/parsers, and existing receipt/scenario evidence. Each row must carry a status, owner, evidence path, and explicit non-claim when uncovered.

## Verification strategy

Promoted packet families need both positive and negative tests. Positive tests prove the reviewed wire id maps to the intended Stevenarella semantic and round-trips where applicable. Negative tests reject inherited fallback aliases, incompatible packet shapes, malformed payload fixtures, and overbroad claims without evidence.

## Evidence strategy

Broad coverage cannot be one receipt. It should advance by packet-family and scenario-family rows. The acceptance matrix may only promote a broad coverage row when the coverage ledger, tests, live receipts, BLAKE3 manifests, and current bundle all agree.

## Risks

- Packet-id coverage can hide parser-shape gaps. Mitigate with shape-reviewed fixtures before promotion.
- A broad proof can become stale quickly. Mitigate by depending on evidence freshness gates and generated metadata checks.
