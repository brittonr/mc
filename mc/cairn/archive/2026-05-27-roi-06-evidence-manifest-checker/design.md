# Design: Evidence manifest checker

## Approach

Add `tools/check_evidence_manifests.py` alongside existing lightweight evidence checkers. The checker has a pure validation core over manifest text, file bytes, and receipt text, plus a thin CLI shell that reads `docs/evidence`, runs `b3sum`, and prints actionable errors.

The checker validates:

1. Every tracked `docs/evidence/*.b3` entry has a 64-hex BLAKE3 digest and a repo-relative path.
2. Each referenced file exists under the repo root.
3. `b3sum --check` succeeds for every manifest.
4. JSON receipt evidence does not contain stale milestone names that conflict with current runner semantics.

A self-test mode covers both positive and negative fixtures so future edits keep failure behavior reviewable.

## Verification

- `python3 tools/check_evidence_manifests.py --self-test`
- `python3 tools/check_evidence_manifests.py`
- Existing matrix and bundle checks.
- `nix run .#cairn -- validate --root .`
- New Nix check wired into the maintained dry-run aggregate.

## Risks

- Avoid overclaiming: this only verifies manifests, file integrity, and stale local marker names.
- Keep the stale marker deny-list narrow and explicit so it does not become a broad protocol oracle.
