# Proposal: Unify plugin contract metadata

## Why

Valence gameplay examples and Hyperion gameplay groups both record plugin metadata, but they use separate shapes and visibility boundaries. Reviewers and downstream code cannot ask one consistent set of questions: what does this plugin install, what phases does it expose, what resources/events does it own, what dependencies does it require, and what behavior is explicitly not claimed?

## What Changes

- Define a shared minimal plugin-contract vocabulary that both Valence and Hyperion can adapt to without coupling their crates.
- Preserve engine-local contract resources and public APIs while aligning field meaning and evidence expectations.
- Add adapters or helpers that expose schedule labels, phase order, install mode, scope model, owned resources/events, dependencies, and non-claim boundaries consistently.
- Add positive contract-inspection tests and negative stale/missing-contract tests.

## Impact

- **Files**: Valence example contract helpers, Hyperion gameplay inventory/contract surfaces, documentation/evidence for contract inventories, and selected tests.
- **Testing**: contract metadata unit tests, selected Valence example checks, Hyperion gameplay inventory checks, negative missing/stale contract tests, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests.
- **Non-claims**: this does not merge Valence and Hyperion crates, standardize all gameplay behavior, or add dynamic plugin loading; it aligns metadata and adapters only.
