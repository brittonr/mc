# Proposal: Add Hyperion-derived load and packet tools

## Why

Hyperion includes practical tooling such as a load bot and packet inspector. Valence would benefit from comparable repo-owned tools for smoke testing, packet diagnostics, and load evidence, but these tools should remain outside the public server API and must not create compatibility claims without separate scenario evidence.

## What Changes

- Inventory Hyperion tools and Valence's existing tools to identify reusable concepts.
- Add or adapt load-bot and packet-inspection tools under Valence tooling boundaries.
- Keep tool configuration explicit and reproducible through repo-supported devshell/check commands.
- Add positive and negative tests for tool config parsing, safe connection targets, packet capture redaction, malformed captures, and load-run failure reporting.
- Document which evidence these tools can and cannot support.

## Impact

- **Files**: Valence `tools/`, docs, optional flake/check wrappers, evidence templates, tests, and Cairn artifacts.
- **Testing**: config parser tests, malformed capture fixtures, loopback smoke tests, failure-path tests, selected load dry runs, and Cairn gates/validation.
- **Non-claims**: tool output alone does not prove compatibility, production capacity, or vanilla parity.
