# Protocol 763 reference-parity expansion checkpoint

## Policy

Compatibility evidence rows now use three claim classes:

- `reference-parity-covered`: paired Paper/reference and Valence artifacts exist, a comparator checks normalized metrics, and the matrix/current bundle keeps bounded non-claims.
- `valence-only-containment`: Valence-only evidence may prove one bounded local behavior or rejection, but cannot claim vanilla/reference parity.
- `explicit-non-claim`: no promoted parity claim exists; docs must name the missing artifacts or owner change.

Acceptance/current-bundle rows stay bounded. A row cannot be labeled reference parity unless it has reference receipt, reference client/server logs, Valence receipt, Valence client/server logs, BLAKE3 coverage, and comparator output under `docs/evidence/`.

## Survival inventory

| Row | Class | Owner | Evidence standard | Next action |
| --- | --- | --- | --- | --- |
| Break/place/pickup | `reference-parity-covered` | archived survival parity work | `tools/check_survival_reference_parity.py` over paired Paper/Valence receipts/logs. | Keep matrix/current-bundle label narrow. |
| Chest persistence | `explicit-non-claim` until receipts land | active `prove-survival-chest-persistence` Cairn | The active change owns the bounded chest contract, checker, fixture instrumentation, and future paired receipts. | Do not promote a competing chest row here. |
| Crafting/furnace | `explicit-non-claim` | none | Needs paired reference and Valence fixture plus normalized item transform metrics. | Candidate next survival rail after chest if deterministic fixture support appears. |
| Hunger/food | `explicit-non-claim` | none | Needs paired health/food/saturation metrics and time-bounded tick control. | Blocked on deterministic probe support. |
| Mob drops | `explicit-non-claim` | none | Needs deterministic mob spawn/kill/drop fixture under Paper and Valence. | Blocked on deterministic fixture support. |
| Redstone/biome/dimension/world persistence | `explicit-non-claim` | none | Too broad for current harness. | Split into narrower rows before any promotion. |

## Comparator contracts

Survival comparator contract:

- Required artifacts: Paper/reference receipt, Paper client log, Paper server log, Valence receipt, Valence client log, Valence server log.
- Required behavior: fail closed on Valence-only evidence, missing metrics, wrong backend, or mismatched normalized metrics.
- Current implementation/evidence: `tools/check_survival_reference_parity.py`; paired break/place/pickup evidence in `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md`.

Combat comparator contract:

- Required artifacts: Paper/reference receipt, Valence receipt, reference version, bounded metric names, numeric tolerance, and explicit non-claims.
- Required behavior: fail closed on missing reference receipt, wrong reference version, missing metric, Valence-only evidence, or out-of-tolerance values.
- Current implementation/evidence: `tools/check_vanilla_combat_parity.py`; parity remains blocked by `docs/evidence/protocol-763-vanilla-combat-parity-2026-05-27.md`.

## Next survival row decision

Chest persistence is the next survival row in progress and remains owned by `prove-survival-chest-persistence`. This change does not promote another survival row because doing so would duplicate active chest work or skip the highest-ROI active row. The next row after chest is crafting/furnace only if both Paper and Valence deterministic fixtures exist; until then it stays an explicit non-claim.

## Combat parity oracle checkpoint

### Question
Can this change promote vanilla combat parity for damage, knockback, armor mitigation, or projectile damage?

### Inspected evidence
- `docs/evidence/protocol-763-vanilla-combat-parity-2026-05-27.md` records `reference_receipt: none` and `valence_reference_pair: none`.
- `tools/check_vanilla_combat_parity.py --self-test` rejects missing reference receipts, wrong reference version, missing values, Valence-only evidence, and out-of-tolerance values.
- Acceptance/current-bundle rows explicitly keep exact vanilla damage/knockback/mitigation parity as non-claims.

### Decision
Do not promote vanilla combat parity. Existing combat rows remain Valence-only bounded compatibility/containment evidence, not reference parity.

### Owner
Future parity change owner.

### Next action
Produce paired Paper/reference and Valence combat receipts with reference version and tolerance fields, then run the comparator before changing matrix labels.

## Matrix/current-bundle labels

Current labels for review:

- `reference-parity-covered`: Survival break/place/pickup only.
- `valence-only-containment`: CTF scoring, inventory, combat, projectile, reconnect, latency/jitter, load/network safety rows.
- `explicit-non-claim`: chest persistence until active receipts land; exact vanilla combat parity; broad survival; full Minecraft/CTF/protocol correctness.

These labels preserve existing non-claims and avoid reclassifying Valence-only rows as reference parity.
