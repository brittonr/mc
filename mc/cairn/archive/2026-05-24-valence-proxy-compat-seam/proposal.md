# Proposal: Valence proxy compatibility seam

## Why

Hyperion has explicit proxy/load architecture, multi-proxy support, and mTLS work. Valence already advertises proxy support, but our fork needs a testable compatibility seam for Velocity/ViaVersion-style paths before considering deeper proxy architecture changes.

## What Changes

- Document the first Valence fork proxy-compatibility boundary and non-goals.
- Add a deterministic fixture/check for proxy metadata/config expectations when possible.
- Define receipt fields for direct-vs-proxy path, observed protocol/version, and forwarding mode.
- Defer full Hyperion-style mTLS/multi-proxy architecture until the smaller compatibility seam is proven.

## Impact

- **Files**: Valence docs/tests and parent compatibility harness receipts; no wholesale proxy rewrite in the planning slice.
- **Testing**: Dry-run proxy-path receipt fixture first, then an owned local Velocity/ViaVersion smoke only when dependencies are present.
