# Design: Projectile damage attribution rail

## Approach

Start with feasibility inspection instead of direct implementation. The rail is acceptable only if the runner can require both sides of the evidence:

1. client-side projectile action/hit-or-damage observation from Stevenarella logs, and
2. server-side Valence CTF projectile damage attribution or equivalent deterministic damage marker.

If the existing logs expose only projectile use/loadout, stop at a checkpoint and do not claim collision or damage attribution. If feasible, add a new runner scenario with a dry-run receipt gate first, then run live evidence.

## Verification

Minimum verification for this change:

- feasibility checkpoint recording inspected files and decision,
- focused runner tests if scenario code changes,
- dry-run Nix check before live run,
- manifest/matrix/bundle checks if evidence docs change,
- Cairn validation.

## Risks

The primary risk is overclaiming from server loadout or client use events. The implementation must require actual projectile damage/collision evidence before calling the rail projectile damage attribution.
