# Design: Post-drain evidence index

## Approach

Keep this as a documentation/checker alignment change. The source of truth remains the already tracked receipt files and BLAKE3 manifests under `docs/evidence/`. The acceptance matrix gains three new bounded rows, and the bundle checker continues to enforce that bundle rows mirror matrix seams and hashes.

The residual combat catalog moves ROI 01–03 from "next independently drainable seams" into covered rails, while keeping precise remaining non-claims:

- no all equipment slot/item matrix,
- no all armor loadouts or enchantment/status-effect modifiers,
- no projectile travel/collision/damage physics,
- no full combat correctness.

## Verification

- `python3 tools/check_acceptance_matrix.py`
- `python3 tools/check_current_evidence_bundle.py`
- `python3 tools/check_evidence_manifests.py`
- `nix build .#checks.x86_64-linux.mc-compat-evidence-manifests --no-link -L --no-update-lock-file --option builders ''`
- `nix run --no-update-lock-file .#cairn -- validate --root .`

## Risks

The main risk is overclaiming from receipt names like `projectile-hit`. Matrix and catalog text must describe observed milestones exactly and retain non-claims for unproven projectile collision, travel, and damage attribution.
