# Proposal: Broad protocol 763 coverage proof

## Summary

Create a reviewable protocol-763 coverage inventory and non-overclaiming gate for Stevenarella ⇄ Valence, without claiming broad/full compatibility.

## Motivation

The current evidence matrix proves bounded local Valence CTF flows. It does not prove full Minecraft compatibility, every protocol-763 packet/state transition, semantic parser correctness for all packet shapes, or all client/server feature surfaces. A broad claim needs an explicit coverage inventory, gap ledger, positive/negative packet fixtures, and live scenario evidence before any wording can be promoted.

## Scope

- Build a protocol-763 coverage inventory across Valence generated packet definitions, Stevenarella mappings/parsers, and maintained compatibility scenarios.
- Classify each packet/state as covered, intentionally unsupported, fixture-only, or requiring a new proof rail.
- Define deterministic positive and negative mapping/parser checks required before any packet family is newly promoted.
- Add bounded live receipts only for scenario groups whose protocol coverage is claimed.
- Keep all broad/full compatibility claims false until every required coverage gate is satisfied.

## Out of scope

- Production load, public-server compatibility, full CTF correctness, combat fidelity, inventory fidelity, and projectile physics; those remain separate Cairn packages.
- Claiming full Minecraft compatibility from packet-id mapping alone.
