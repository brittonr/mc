# Proposal: Survival break/place/pickup rail

## Summary

Create the first bounded survival-compatibility rail for the local protocol-763 Stevenarella ⇄ Valence harness: join a dedicated survival fixture, break a block, observe a survival pickup/inventory delta, place the block back, and record reviewable client/server milestones.

## Motivation

The current compatibility evidence is CTF-focused. It covers login/play, scoring, inventory, combat, reconnect, and selected protocol seams, but broad Minecraft/survival compatibility remains a non-claim. Survival work needs its own fixture and receipt path so new claims do not overfit to CTF mechanics or silently strengthen existing rows.

## Scope

- Add a narrowly named `survival-break-place-pickup` scenario to `tools/mc-compat-runner`.
- Add Stevenarella probe milestones required to drive one deterministic survival block break/place loop.
- Add a Valence `survival_compat` example fixture with server-side milestone logging.
- Wire a Nix app/check for deterministic dry-run receipt shape.
- Keep live evidence and matrix promotion scoped to this single rail.

## Non-goals

- No full vanilla parity claim from this rail.
- No broad protocol-763 or full Minecraft compatibility claim.
- No production/public/WAN/load claim.
- No crafting, furnace, mob AI, redstone, biome, dimension, or full inventory semantics claim in this change.
