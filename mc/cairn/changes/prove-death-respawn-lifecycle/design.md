# Design: Death and respawn lifecycle proof

## State strategy

Model the lifecycle as explicit states: alive, lethal damage observed, dead, respawn requested/accepted, respawned, and post-respawn playable. The proof should assert allowed transitions and forbidden transitions for each scenario row.

## Oracle strategy

Valence server evidence is authoritative for death, respawn, flag/score state, and inventory state. Stevenarella evidence is required for user-visible death/health/respawn observations. Negative fixtures must reject duplicate, missing, or out-of-order lifecycle milestones.

## Evidence strategy

Promoted rows need dry-run fixtures, live receipt, Valence/client logs, BLAKE3 sidecars, and matrix/current-bundle updates. Each row should state whether it covers ordinary death, flag-carrier death, repeated death, reconnect during death, or post-respawn state.

## Risks

- Existing flag-carrier death evidence may be mistaken for full lifecycle coverage. Keep previous rows as inputs only and require explicit lifecycle state assertions.
