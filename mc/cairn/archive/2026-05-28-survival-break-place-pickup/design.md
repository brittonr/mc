# Design: Survival break/place/pickup rail

## Approach

Use the existing receipt pattern, but split survival from CTF.

1. Runner adds `survival-break-place-pickup` as a one-client scenario.
2. Runner sets `VALENCE_EXAMPLE=survival_compat` through a new flake app and enables `MC_COMPAT_SURVIVAL_PROBE` for Stevenarella.
3. Stevenarella moves to fixed local coordinates, sends one survival dig stop against a known block, waits for server/client update evidence, selects the resulting hotbar item, and sends one placement packet.
4. Valence `survival_compat` starts offline in survival mode, creates a small flat world, accepts the fixed dig/place actions, gives the broken block item back through inventory state, emits an item-pickup animation, and logs deterministic `MC-COMPAT-MILESTONE survival_*` server milestones.
5. Runner dry-run check validates scenario shape, required client/server milestone names, receipt non-claims, and packet summary before live promotion.

## Verification

- Baseline runner tests before mutation: `nix develop --no-update-lock-file -c cargo test --manifest-path tools/mc-compat-runner/Cargo.toml`.
- Runner after mutation: same command plus focused scenario unit tests.
- Valence fixture compile: `cargo check --example survival_compat` from `valence/`.
- Nix dry-run check: `nix build .#checks.x86_64-linux.mc-compat-valence-survival-break-place-pickup-dry-run --no-link -L`.
- Cairn gates/validation before archive: proposal, design, tasks, and `nix run --no-update-lock-file .#cairn -- validate --root .`.

## Risks

- Client-visible block updates can be noisy; the client milestone should key on the fixed coordinate and server scenario should carry authoritative correlation.
- A dry-run receipt only proves shape. Matrix promotion requires live receipt/log copies and BLAKE3 manifests.
- Vanilla parity still requires paired reference-server receipts for the same survival rail; Valence-only receipts cannot satisfy parity.
