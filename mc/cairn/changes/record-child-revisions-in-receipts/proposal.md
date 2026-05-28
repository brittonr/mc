# Proposal: Record child revisions in compatibility receipts

## Summary

Promote child-repo revision capture from ad hoc oracle checkpoints into the `mc-compat-runner` receipt schema. Live receipts should record the resolved Valence and Stevenarella git revisions and cleanliness status used for a run.

## Motivation

The survival rail needed an oracle checkpoint because the receipt recorded `valence.rev` but not the Stevenarella git revision. Reviewers should not have to reconstruct child-repo state from transcript snippets or local commands. Promoted evidence needs machine-readable child revision fields in the receipt itself.

## Scope

- Add resolved child revision and status fields to live receipts.
- Include Stevenarella client revision/status and Valence requested/resolved revision/status.
- Update dry-run and tests so receipt shape remains deterministic.
- Update evidence checks to prefer machine-recorded child revisions and require an oracle only for legacy receipts.

## Non-goals

- No change to Minecraft compatibility semantics.
- No automatic remote push verification.
- No removal of existing legacy receipts.
