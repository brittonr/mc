## Why

Recent evidence gate runs showed avoidable token drift: row-specific checkers required exact legacy wording, while the evidence docs already described the same non-claims with newer canonical labels. That forced compatibility aliases into review docs and made aggregate gates stale when promoted row inventories changed.

## What Changes

- Canonicalize non-claim wording used by WAN and CTF evidence gates.
- Update row checkers so they require the canonical row labels instead of compatibility alias prose.
- Keep aggregate survival gates tied to their maintained row inventory rather than separate stale counts.
- Refresh evidence manifests and record focused gate evidence.

## Impact

- **Files**: `tools/check_*`, `docs/evidence/**`, and `cairn/changes/canonicalize-evidence-gate-wording/**`.
- **Testing**: focused baseline gates, checker self-tests through Nix checks, evidence manifest check, maintained dry-run aggregate gate, Cairn gates, and final validation.
