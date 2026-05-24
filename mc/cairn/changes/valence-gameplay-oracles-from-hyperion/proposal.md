# Proposal: Valence gameplay oracles from Hyperion

## Why

Hyperion Bedwars logic provides useful semantic milestones—team assignment, flag/score, respawn, combat, inventory—that map well to Valence CTF evidence. We should turn those ideas into Valence harness oracles rather than copying game code.

## What Changes

- Catalog the Hyperion Bedwars milestones that correspond to Valence `ctf` or other examples.
- Define Valence scenario receipts for team, inventory, combat/death/respawn, flag score, reconnect, and multi-client load claims.
- Add server-side correlation markers or normalized logs where Valence examples currently make milestones ambiguous.
- Record explicit non-claims for full CTF correctness, broad protocol support, and unbounded soak.

## Impact

- **Files**: Valence example instrumentation/tests, parent `mc` scenario receipt code, README/evidence docs.
- **Testing**: Focused unit tests for milestone classification plus dry-run scenario gates, then bounded live receipts for selected scenarios.
