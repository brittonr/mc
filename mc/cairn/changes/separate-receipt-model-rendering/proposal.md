# Proposal: Separate mc-compat receipt model, rendering, and writing

## Why

Receipt construction currently mixes config lookup, fallback evidence derivation, JSON string rendering, and file-writing concerns. That makes receipt schemas harder to evolve and forces tests to inspect rendered strings for behavior that should be represented as typed data first.

## What Changes

- Introduce typed receipt input and receipt model structs before JSON rendering.
- Build scenario receipts through pure functions over explicit inputs and existing evidence values.
- Keep JSON rendering deterministic and separate from filesystem writes.
- Preserve existing receipt schemas, legacy fields, non-claims, and evidence boundaries.
- Add positive and negative schema/model tests before changing any receipt-writing shell.

## Impact

- **Files**: `compat/runner/src/receipts.rs`, `evidence_receipts.rs`, receipt validation tests, JSON helpers, file-writing shell code, docs/evidence schema notes if needed, and Cairn artifacts.
- **Testing**: baseline receipt fixtures, typed model positive/negative tests, JSON snapshot/schema tests, runner receipt tests, Cairn gates, and Cairn validation.
- **Non-claims**: schema-internal architecture only unless a later Cairn explicitly changes receipt schema fields.
