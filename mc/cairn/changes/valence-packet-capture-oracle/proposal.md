# Proposal: Valence packet capture oracle

## Why

Hyperion carries a packet-inspector tool that explains protocol failures. Valence compatibility evidence still often collapses failures into logs or disconnects. We need a packet capture oracle that tells operators which protocol state/packet boundary failed.

## What Changes

- Add a CLI/headless packet capture mode for Valence example compatibility runs.
- Normalize captured packet direction, state, id, and decode status into deterministic receipt fields.
- Correlate packet capture summaries with client/server scenario receipts.
- Document the operator workflow for using capture output during Stevenarella/Valence triage.

## Impact

- **Files**: A Valence-local tool, parent `mc` harness integration, README/operator docs, and focused packet-capture receipt tests.
- **Testing**: Unit tests for normalization/redaction, dry-run receipt gate, and one fixture/capture golden that does not require a live GUI.
