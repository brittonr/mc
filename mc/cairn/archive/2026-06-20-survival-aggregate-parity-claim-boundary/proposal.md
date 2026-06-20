# Proposal: Survival aggregate parity claim boundary

## Why

The survival matrix now has all current row slots covered, but the docs still correctly keep full survival compatibility and broad vanilla parity as non-claims. Before any aggregate claim is attempted, the repo needs an explicit gate that distinguishes row coverage from aggregate survival parity.

## What Changes

- Add a scoped `survival-aggregate-parity-claim-boundary` change that defines the evidence threshold for any future aggregate survival claim.
- Require the gate to enumerate required row families, reject missing breadth rows, reject stale manifests, and reject docs that claim full survival or broad vanilla parity without the aggregate evidence bundle.
- Add positive and negative fixtures for all-row-present success, missing row failure, stale evidence failure, Valence-only row failure, and broad-overclaim failure.
- Keep current docs as non-claims until the aggregate gate and evidence bundle pass.

## Impact

- **Files**: aggregate survival gate checker, survival coverage matrix, current bundle, acceptance matrix, evidence manifests, Cairn artifacts.
- **Testing**: positive and negative checker fixtures, current docs validation, evidence manifest checks, task-evidence gate, Cairn gates, and Cairn validation.

## Non-Claims

This change does not itself claim full survival compatibility, exact Mojang vanilla parity, production readiness, public-server safety, or completion of any survival breadth row. It defines the claim boundary only.
