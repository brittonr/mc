# Proposal: Full CTF rule correctness proof

## Summary

Create a bounded CTF rule-cluster package for the rule paths currently evidenced by maintained receipts, and keep full CTF correctness plus invalid-action breadth blocked as non-claims.

## Motivation

The maintained evidence proves selected CTF flows: scoring, mirrored team scoring, flag-carrier death/return, reconnect flag-state, and supporting combat/inventory seams. It does not prove all CTF rules, edge cases, invalid actions, timing windows, or invariant preservation across multiple players and game states.

## Scope

- Inventory CTF rules and invariants that must be proven before any full CTF correctness claim.
- Add server/client oracle milestones for legal and illegal flag interactions.
- Exercise currently evidenced positive clusters: bounded scoring, flag-carrier death/return, and reconnect flag-state.
- Keep invalid pickup/capture attempts, same-team constraints, resets, score limits, and race windows unpromoted until live negative scenarios exist.
- Record live receipts, logs, BLAKE3 manifests, and matrix rows for each promoted rule cluster.

## Out of scope

- Broad Minecraft protocol compatibility.
- Production load or public-server safety.
- Exact vanilla combat/inventory parity except where needed as an input to a CTF rule proof.
