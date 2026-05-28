# Protocol-763 remaining Cairn validation checkpoint — 2026-05-27

## Purpose

Drain the review WARN that the validation/gate pass for the 10 active remaining-proof Cairns was not independently evidenced in the prior response.

## Commands

Run from `/home/brittonr/git/mc` with the repo-pinned Cairn app:

```sh
nix run --no-update-lock-file .#cairn -- validate --root .
for change in \
  prove-broad-protocol-763-coverage \
  prove-ctf-rule-correctness \
  prove-inventory-semantics-matrix \
  prove-equipment-slot-item-matrix \
  prove-armor-enchantment-status-matrix \
  prove-vanilla-combat-parity \
  prove-death-respawn-lifecycle \
  prove-projectile-travel-collision \
  prove-production-load-network-safety \
  harden-evidence-freshness-gates
 do
  nix run --no-update-lock-file .#cairn -- gate proposal "$change" --root .
  nix run --no-update-lock-file .#cairn -- gate design "$change" --root .
  nix run --no-update-lock-file .#cairn -- gate tasks "$change" --root .
done
```

## Result

- Validation receipt: `valid=true`, `changes=10`, `change_issues=[]`, `spec_issues=[]`, `issues=[]`, `specs_validated=11`.
- Gate receipts: 30 PASS verdicts total: proposal/design/tasks gates for each of the 10 active Cairn changes.
- Gate issue arrays: all `issues=[]`.
- Full run log: `docs/evidence/protocol-763-remaining-cairns-validation-2026-05-27.run.log`.
- BLAKE3 manifest: `docs/evidence/protocol-763-remaining-cairns-validation-2026-05-27.b3`.

## Decision

- Question: Are the 10 newly scoped remaining-proof Cairns valid active Cairn packages with passing proposal/design/tasks gates?
- Inspected evidence: tracked run log and BLAKE3 manifest above.
- Decision: Yes; validation and all stage gates passed. The Cairns remain active with unchecked implementation tasks.
- Decision owner: agent; maintainer can rerun the exact commands above from `/home/brittonr/git/mc`.
- Next action: drain each active Cairn independently when ready, starting with `harden-evidence-freshness-gates` or `prove-projectile-travel-collision` depending on ROI.

## Review note

Use `nix run --no-update-lock-file .#cairn -- ... --root .` from this repo. A newer sibling `/home/brittonr/git/cairn#cairn` binary can reject this repo's generated policy schema and is not the validation authority for this workspace.
