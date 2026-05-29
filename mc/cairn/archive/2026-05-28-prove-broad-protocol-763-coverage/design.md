# Design: Broad protocol-763 coverage proof

## Proof layers

1. Packet inventory completeness: every Valence protocol-763 packet row is present.
2. Mapping review: each Stevenarella mapping is explicit, not inherited by accident.
3. Parser-shape fixture: structured promoted packet families have positive and negative decode/encode fixtures; byte-opaque raw consumers have positive byte-preservation fixtures plus explicit semantic non-claim rationale.
4. Scenario evidence: live or deterministic receipts exercise representative packet families.
5. Non-claim guard: full Minecraft compatibility remains separate from protocol packet coverage.

## Checker behavior

The checker should fail closed on fallback aliases, missing parser-shape evidence, malformed structured-fixture acceptance, missing raw-consumer non-claim rationale, missing owners, or stale packet inventory.

## Evidence

Promoted broad coverage requires a reviewable evidence doc, BLAKE3 manifest, and receipt/checker output copied under `docs/evidence/`.
