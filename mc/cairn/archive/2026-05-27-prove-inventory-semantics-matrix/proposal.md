# Proposal: Inventory semantics matrix proof

## Summary

Create a proof package for inventory semantics beyond the current bounded drop, pickup, click, container, and block-place rail.

## Motivation

The maintained inventory interaction receipt proves selected client/server milestones. It does not prove all inventory windows, state-id behavior, transaction acceptance/rejection, slot restoration, stack merging/splitting, carried-stack behavior, malformed click handling, or full item lifecycle semantics.

## Scope

- Define an inventory semantics matrix for player inventory, open containers, carried stacks, state ids, click modes, drop/pickup, and item lifecycle cases.
- Add positive scenarios for valid interactions and negative scenarios for invalid/stale/malformed interactions.
- Require Valence server correlation and Stevenarella client observation where the user-visible inventory state is claimed.
- Promote matrix rows only with live receipts and BLAKE3-backed evidence.

## Out of scope

- All equipment update semantics, armor mitigation, combat effects, and production load.
- Claiming broad protocol coverage outside inventory packet families.
