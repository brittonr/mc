# Protocol-763 RED/BLUE scoring soak historical oracle — 2026-05-27

## Purpose

Record the explicit historical/oracle decision for the only maintained acceptance-matrix row that still cites a mutable `target/` receipt path: RED/BLUE scoring soak.

## Question

Can the RED/BLUE scoring soak row remain indexed as bounded maintained evidence even though `target/mc-compat-blue-soak/blue-flag-score-600s.json` is no longer the original live receipt and has been overwritten by a dry-run fixture?

## Inspected evidence

- Acceptance matrix row: `docs/evidence/protocol-763-acceptance-matrix.md`.
- Historical evidence doc: `docs/evidence/stevenarella-valence-763-blue-flag-600s-soak-2026-05-25.md`.
- Historical command recorded there:

```sh
MC_COMPAT_BLUE_SOAK_RECEIPT=target/mc-compat-blue-soak/blue-flag-score-600s.json \
  nix run .#mc-compat-valence-ctf-blue-600s-soak \
  > target/mc-compat-blue-soak/blue-flag-score-600s.run.log 2>&1
```

- Historical live receipt BLAKE3 recorded there: `b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de`.
- Historical live receipt result recorded there: `status: pass`, `classification: timeout-success-evidence`, BLUE team milestones observed, server milestones observed, and hygiene scan reported zero forbidden matches.
- Current mutable `target/mc-compat-blue-soak/blue-flag-score-600s.json` is not used as review evidence for the row because it no longer hashes to the historical live digest.

## Decision

The row may remain as historical bounded evidence with the existing scoped claim and explicit non-claims, but reviewers must treat `docs/evidence/stevenarella-valence-763-blue-flag-600s-soak-2026-05-25.md` as the reviewable evidence source and must not treat the current `target/` file as the live receipt.

## Owner

Agent. Maintainer can request a fresh live rerun if the row needs a current reviewable receipt copy instead of historical evidence.

## Next action

Before changing the RED/BLUE scoring soak hash or broadening its claim, rerun the BLUE soak live rail, copy the receipt/log under `docs/evidence/`, add a BLAKE3 manifest, update the matrix/current bundle together, and run the evidence freshness promotion gate.

## Non-claims

This oracle does not prove full CTF correctness, production load, public-server safety, unbounded soak, broad Minecraft compatibility, or current live freshness of the original target receipt.
