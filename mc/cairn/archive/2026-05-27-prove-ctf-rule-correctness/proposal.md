# Proposal: Full CTF rule correctness proof

## Summary

Create a proof package for Valence CTF rule correctness as observed by Stevenarella clients, separate from the existing bounded scoring, reconnect, inventory, and combat seams.

## Motivation

The maintained evidence proves selected CTF flows: scoring, mirrored team scoring, flag-carrier death/return, reconnect flag-state, and supporting combat/inventory seams. It does not prove all CTF rules, edge cases, invalid actions, timing windows, or invariant preservation across multiple players and game states.

## Scope

- Inventory CTF rules and invariants that must be proven before any full CTF correctness claim.
- Add server/client oracle milestones for legal and illegal flag interactions.
- Exercise positive and negative scenarios: captures, returns, invalid pickup/capture attempts, same-team constraints, disconnects, deaths, and resets.
- Record live receipts, logs, BLAKE3 manifests, and matrix rows for each promoted rule cluster.

## Out of scope

- Broad Minecraft protocol compatibility.
- Production load or public-server safety.
- Exact vanilla combat/inventory parity except where needed as an input to a CTF rule proof.
