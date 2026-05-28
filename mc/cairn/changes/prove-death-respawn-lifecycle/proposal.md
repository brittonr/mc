# Proposal: Death and respawn lifecycle proof

## Summary

Create a proof package for death/respawn lifecycle correctness beyond the existing bounded flag-carrier death/return and combat health-update rails.

## Motivation

Current evidence covers selected death-adjacent paths: combat damage, flag-carrier death/drop/return, and bounded respawn health restoration. It does not prove full death-message behavior, respawn state reset, inventory/flag/score consistency across deaths, invalid respawn timing, repeated deaths, or reconnect during death/respawn transitions.

## Scope

- Define death/respawn lifecycle states and invariants for Valence CTF as observed by Stevenarella.
- Add positive scenarios for lethal damage, death observation, respawn, state reset, and post-respawn playability.
- Add negative scenarios for invalid early actions, missing respawn, duplicate death, and forbidden score/flag transitions.
- Promote only lifecycle rows with correlated client/server receipts.

## Out of scope

- Full combat parity, full CTF correctness, broad protocol coverage, and production soak.
