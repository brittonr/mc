## Context

`protocol-763-ctf-rule-ledger-2026-05-27.md` currently promotes one invalid pickup containment row and one invalid return/drop containment row. Both prove a single bounded path; neither proves all invalid actions, all owner states, all team/flag combinations, or adversarial behavior. The next work should add breadth deliberately through a maintained row family instead of hard-coding another isolated checker.

## Design

Create a row-family contract with a small pure validation core and thin shells around evidence loading:

- A matrix document names each invalid-action permutation, expected pre-state, expected rejection, postcondition, required client/server milestones, forbidden transitions, and non-claims.
- Runner scenarios or fixtures emit normalized row fields for the selected permutation. The first implementation target should be one additional bounded row, not full invalid-action coverage.
- A parameterized Rust checker validates row records against matrix entries in memory. The CLI shell reads receipt/log/doc paths and prints deterministic diagnostics.
- Existing one-off invalid pickup and invalid return/drop checks remain passing while the shared breadth contract is introduced. Migration can be incremental.
- Acceptance matrix/current-bundle updates must say exactly which new bounded row is covered and keep all-invalid-actions, full CTF correctness, adversarial security, public-server safety, and production readiness as non-claims.

## Initial candidate row

Prefer the next narrow row that maximizes contrast with the existing receipts while keeping owned-local execution simple: an invalid opponent-base return/drop attempt without carrier ownership, or a wrong-team/opponent-flag pickup attempt that must not transfer ownership or score. The final choice should be locked in the matrix before implementation.

## Non-goals

This change does not claim full CTF correctness, all invalid actions, all flag permutations, race safety, adversarial security, WAN/public-server safety, or production readiness. It does not require migrating every existing CTF checker before the first additional breadth row lands.

## Risks / Trade-offs

- A matrix can become stale if row checkers keep separate token lists. Mitigation: validate matrix entries directly or define named checker constants for canonical labels.
- Live rows can be expensive or flaky. Mitigation: start with deterministic owned-local fixtures/dry-runs, then promote only if reviewable live receipts and server logs are available.
- Adding breadth may imply broader correctness accidentally. Mitigation: keep row-level claims narrow and require current-bundle/ledger non-claims in the checker.
