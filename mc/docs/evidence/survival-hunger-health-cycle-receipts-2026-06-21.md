# Survival hunger/health-cycle paired receipts — 2026-06-21

This checkpoint closes the receipt task for `survival-hunger-health-cycle-parity`.

Evidence produced:

- Paper live run: `docs/evidence/survival-hunger-health-cycle-paper-2026-06-21.receipt.json`, run log, client log, server log, typed events, and normalized key/value evidence.
- Valence live run: `docs/evidence/survival-hunger-health-cycle-valence-2026-06-21.receipt.json`, run log, client log, server log, typed events, and normalized key/value evidence.
- Row comparator: `tools/check_survival_breadth_contracts.rs --row survival-hunger-health-cycle-parity --paper docs/evidence/survival-hunger-health-cycle-paper-2026-06-21.evidence --valence docs/evidence/survival-hunger-health-cycle-valence-2026-06-21.evidence` passed with `exit_status=0` in `docs/evidence/survival-hunger-health-cycle-row-parity-2026-06-21.run.log`.

Observed matching normalized metrics:

- Food item: `Bread` from inventory slot `36` with a one-item decrement.
- Pre-checkpoint: health `18.0`, food `15`, saturation `0.0`.
- Post-checkpoint: health `20.0`, food `20`, saturation `6.0`.
- Both backends recorded client milestones for item observation, pre-state, consume send, recovery, and inventory update, plus server milestones for pre-state, consume start/finish, inventory, and final health-cycle state.

Child revisions recorded in receipts/evidence:

- Valence: `53ec70c527796b158463d087fbbb9d0826bc52c5`.
- Stevenarella: `c6bafb754e2e4d713c819d6f78cdbb45a6082bd7`.
- Paper backend: `1.20.1`; the Paper receipt records clean revision status and server revision `8ad9c8587a3273ec59b0ec4edae0bf790bdf403b`.

Non-claims: this proves only the configured Bread consume health-cycle row with the listed health, food, saturation, and inventory checkpoints in the owned local fixture. It does not claim all foods, all exhaustion sources, starvation loops, potion/effect interactions, offhand consumption, natural regeneration breadth, full survival compatibility, broad vanilla parity, public-server safety, production readiness, or semantic equivalence.
