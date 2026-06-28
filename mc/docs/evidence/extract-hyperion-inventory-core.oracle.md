# Extract Hyperion inventory core oracle

## Question

Can `extract-hyperion-inventory-core` be closed as Hyperion-owned inventory modularity work without implying Valence adoption or broader Minecraft compatibility claims?

## Inspected evidence

- Parent workflow docs: `AGENTS.md`, `README.md`, `docs/check-tiers.md`, and `docs/hyperion-integration-boundaries.md`.
- Hyperion workflow docs: `hyperion/README.md`, `hyperion/CONTRIBUTING.md`, `hyperion/AGENTS.md`, and `hyperion/.agent/napkin.md`.
- Inventory source responsibilities before the change:
  - `hyperion/crates/hyperion/src/simulation/inventory.rs` owned Bevy/ECS observers and systems, packet emission/resync, drop-event emission, selected-slot handling, equipment broadcasts, and the click/drag/shift/hotbar/drop transition logic.
  - `hyperion/crates/hyperion-inventory/src/lib.rs` owned inventory state/access types, slot storage, cursor/hand helpers, item insertion helpers, armor-slot helpers, and window-id state.
- Hyperion nested implementation commit: `f4ad334649482d1acd42a222f07f859bdf8646c8` (`extract inventory transitions for testable shell boundaries`).
- Final Hyperion validation log: `docs/evidence/extract-hyperion-inventory-core.hyperion-final.run.log`.
- Cairn pre-implementation gates log: `docs/evidence/extract-hyperion-inventory-core.pre-gates.run.log`.

## Decision

Classification: `reference` for Valence and Hyperion-owned for implementation. No Hyperion code or behavior was adopted, ported, or targeted into Valence. The implementation stayed inside the independent Hyperion nested repository:

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion/src/simulation/inventory.rs` | `reference` | `extract-hyperion-inventory-core` | Shell responsibilities informed the extraction boundary, but the shell remains Hyperion-local. | none | No Valence runtime/API change; ECS mutation, packet emission, scheduling, tracing/logging, and network/proxy side effects remain in the Hyperion shell. | `docs/evidence/extract-hyperion-inventory-core.hyperion-final.run.log` | No Valence adoption, vanilla parity, production safety, public-server safety, broad Minecraft compatibility, full CTF correctness, or full survival correctness claim. |
| `hyperion/crates/hyperion-inventory/src/transaction.rs` | `reference` | `extract-hyperion-inventory-core` | Pure transaction core was added for Hyperion inventory tests and shell reuse only. | none | Pure over explicit request/slot/cursor inputs; no Bevy runtime, network IO, proxy state, packet emission, tracing, or filesystem side effects. | `docs/evidence/extract-hyperion-inventory-core.hyperion-final.run.log` | No Valence adoption, vanilla parity, production safety, public-server safety, broad Minecraft compatibility, full CTF correctness, or full survival correctness claim. |

## Owner

`extract-hyperion-inventory-core` Cairn change, implemented in the Hyperion nested repository and closed from the parent `mc` Cairn lifecycle.

## Next action

Use the parent Cairn sync/archive flow only after task evidence cites tracked `docs/evidence/` logs and a BLAKE3 manifest. Any future Valence use MUST create a separate integration Cairn and classify inspected Hyperion sources under `docs/hyperion-integration-boundaries.md` before implementation.
