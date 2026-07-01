# Proposal: Expose plugin diagnostics registry

## Why

When plugin composition fails or produces surprising behavior, users must inspect Bevy state, tests, or panic messages to understand which modes/features were installed, disabled, replaced, or skipped. Hyperion needs a structured diagnostics registry that reports composition facts and dependency decisions for preset builders and selected plugin groups.

## What Changes

- Add a Hyperion plugin diagnostics registry/resource populated by validated builders and selected plugin groups.
- Record selected mode, default gameplay inclusion, installed features, disabled features, replacements, custom plugin slots, dependency decisions, contract metadata, and non-claim boundaries.
- Expose diagnostics to tests and optional receipts/logs without requiring live proxy startup.
- Add positive diagnostics tests and negative stale/missing diagnostics tests.

## Impact

- **Files**: Hyperion game-mode composition core/shell, Bedwars app builders, gameplay inventory adapters, tests, and evidence logs.
- **Testing**: diagnostics registry unit tests, preset builder tests, disabled/replacement diagnostics tests, negative stale/missing diagnostics tests, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests.
- **Non-claims**: diagnostics are local app/test metadata, not a remote admin protocol, metrics backend, hot-reload manager, or security sandbox.
