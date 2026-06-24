# Proposal: Upstream a byte-backed Valence protocol path

## Why

Hyperion depends on a forked Valence protocol path that keeps packet bodies and selected string/byte fields backed by shared bytes so ingress events can be cheap to move into ECS systems. Valence should own this capability directly if it wants Hyperion-style high packet throughput, but the integration must be audited, stable, and tested before it becomes a public protocol surface.

## What Changes

- Audit Hyperion's byte-backed protocol usage and the current Valence protocol/event path.
- Define a byte-backed API boundary for raw packet bodies and validated string/byte fields.
- Port the framing and decode rules through a pure core where possible, keeping I/O and concurrency in thin shells.
- Add positive fixtures for complete frames, split frames, compressed frames, borrowed byte fields, and valid text fields.
- Add negative fixtures for malformed VarInts, zero-length packets, oversized packets, invalid compression, invalid UTF data, and lifetime/ownership regressions.

## Impact

- **Files**: `valence_protocol`, `valence_network`, event-loop packet surfaces, focused protocol fixtures, and Cairn artifacts.
- **Testing**: protocol unit tests, packet framing fixtures, event-loop regressions, direct-mode mc-compat dry runs, and Cairn gates/validation.
- **Non-claims**: this does not require Valence to adopt Hyperion's full packet channel, proxy runtime, or nightly-only implementation details.
