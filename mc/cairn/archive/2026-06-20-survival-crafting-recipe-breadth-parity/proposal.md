# Proposal: Survival crafting recipe breadth parity

## Why

The current crafting row proves one crafting-table recipe only. Survival parity still needs bounded evidence for additional recipe shapes and collection modes before any broader crafting claim can be considered.

## What Changes

- Add a scoped `survival-crafting-recipe-breadth-parity` row covering a finite recipe matrix beyond the existing stick recipe.
- Require paired Paper/reference and Valence receipts for one shaped recipe, one shapeless recipe, one invalid/insufficient-input rejection, and one configured result collection mode.
- Add deterministic checker coverage that rejects Valence-only evidence, missing recipe identifiers, missing slot/result metrics, mismatched item counts, stale child revisions, and broad all-recipe overclaims.
- Promote only the bounded crafting-breadth row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner/client survival crafting rail, Paper survival fixture, Valence `survival_compat`, row checker, acceptance/current evidence docs, `docs/evidence/` receipts and manifests.
- **Testing**: positive and negative checker fixtures, dry-run shape coverage, paired Paper/Valence live receipts, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.

## Non-Claims

No all-recipes, recipe-book UI, recipe discovery breadth, all collection modes, shift-click/drag/split semantics beyond the configured row, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
