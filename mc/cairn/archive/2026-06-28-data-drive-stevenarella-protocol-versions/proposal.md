# Proposal: Data-drive Stevenarella protocol version tables

## Why

`clients/stevenarella/protocol/src/protocol/versions.rs` manually maps protocol names to version numbers and dispatches packet translation to per-version modules. Protocol compatibility work repeatedly touches these tables and needs stronger validation that aliases, supported versions, fallback relationships, and packet translation modules stay consistent.

## What Changes

- Introduce a typed protocol-version manifest or generated surface that records supported names, numeric versions, aliases, translation module owners, and explicit fallback/reuse relationships.
- Generate or validate the Rust dispatch tables from that source of truth while preserving existing public functions.
- Add positive and negative validation for duplicate aliases, missing modules, unsupported fallback references, protocol-number mismatches, and packet-boundary regressions.
- Preserve existing protocol-name behavior, packet translation behavior, panic/error behavior where externally observed, and compatibility non-claims.

## Impact

- **Files**: `clients/stevenarella/protocol/src/protocol/versions.rs`, optional protocol manifest/config, generation/checker tools, focused protocol tests, docs if source-of-truth ownership is documented, and Cairn artifacts.
- **Testing**: baseline protocol tests, manifest/generated-surface checks, positive and negative protocol-version fixtures, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: protocol table maintainability only; this does not add new packet support or claim full protocol 763 compatibility.
