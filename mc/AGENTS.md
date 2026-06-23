# Agent Notes

## Scope
- This `mc/` directory is a workspace folder, not one buildable repo.
- `hyperion/` remains an independent nested Rust repo. Run Hyperion commands from inside `hyperion/`, not from `mc/`.
- `servers/valence/` and `clients/stevenarella/` are core component trees owned by the parent `/home/brittonr/git` repository, not submodules or nested Git repos.
- Keep changes scoped to the affected subtree unless user explicitly asks for cross-repo work.

## VCS boundaries
- `git rev-parse` from `mc/` resolves to parent repo `/home/brittonr/git`; this is now the owning repo for `mc/servers/valence` and `mc/clients/stevenarella`.
- `hyperion/` has both `.git/` and `.jj/`. Check `.jj/` before assuming git-only workflow, and do not use parent repo status as Hyperion status.

## Layout
- `hyperion/`: Minecraft engine/proxy workspace. Core crates live under `crates/`; event/game logic lives under `events/bedwars`; helper tools live under `tools/`. Repo-specific workflow now lives in `hyperion/AGENTS.md`.
- `compat/`: mc-compat runner, Nickel/Steel config, generated runner surfaces, and Paper/reference fixtures.
- `servers/valence/`: core Minecraft server framework. Main crate in `src/`; workspace crates in `crates/*`; runnable examples in `examples/`; protocol/data extractor in `extractor/`; docs site in `website/`. Repo-specific workflow lives in `servers/valence/AGENTS.md`.
- `clients/stevenarella/`: core Rust Minecraft client used by mc-compat rails and manual client checks. It has no repo-local AGENTS notes yet; use the mc devshell for Cargo and native UI dependencies.
- `hyperion/` already has repo-local agent notes in `hyperion/.agent/napkin.md`. Keep workspace-wide notes here; prefer subtree-local `AGENTS.md` files for repo-specific commands and conventions.

## Workflow
- Before editing inside a major subtree, read that subtree's `README.md`, `CONTRIBUTING.md`, and any local `AGENTS.md` or `.agent/napkin.md`.
- Prefer repo-local commands and toolchains. Root `mc/` has no shared Cargo workspace, test runner, or formatter.
- For `clients/stevenarella/`, run Cargo through the mc devshell, for example `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test world::tests -- --nocapture` from the Stevenarella repo.
- New checks/scripts for this workspace should be Rust or Steel Scheme, not Python or Bash. Existing Python gates may remain until touched; migrate touched gates before extending them.
- Avoid mixed commits across `hyperion/` and the parent-owned `servers/valence/`/`clients/stevenarella/` trees unless user asks for cross-repo change.
- For Cairn evidence, do not leave review-critical receipts only under untracked `target/`; copy receipt/log artifacts into `docs/evidence/` and record BLAKE3 when tasks/docs cite them.
- Paper backend containers are removed after runner exit unless `--keep-server` is used; use `--keep-server`, copy `docker logs <server>` into `docs/evidence/`, then `docker rm -f <server>` for reviewable Paper server logs.
- If promoted evidence cites source-tree revisions that the receipt does not machine-record (for example a core Stevenarella or Valence subtree rev), add a `docs/evidence/*oracle*` checkpoint with `## Question`, `## Inspected evidence`, `## Decision`, `## Owner`, and `## Next action`; otherwise reviewers cannot verify the revision claim from repo-local artifacts.
- Before archiving a Cairn, compare every checked task and proposal scope against promoted evidence rows. If a doc says a behavior remains a non-claim, do not mark the task as completed for that behavior; narrow the archive scope or leave/reopen the proof gap.
- Accepted spec edits can stale existing `.b3` manifests that include `cairn/specs/mc-compatibility/spec.md`; run the evidence manifest checker and refresh every cited-spec digest before final validation.
- Nix evidence-manifest checks see only the parent repo source closure; `.b3` rows must cite tracked parent files (prefer `docs/evidence/` copies) rather than independent nested-repo paths or `target/` outputs.
- Run mc Cairn validation/gates with the repo-pinned app from `mc/` (`nix run .#cairn -- ... --root .`). A newer sibling `/home/brittonr/git/cairn` binary can reject this repo's generated policy schema.
- When writing `.b3` manifests through `nix develop`, redirect inside the devshell command (`nix develop ... -c bash -lc 'b3sum ... > file'`); redirecting the outer `nix develop` command captures the shell-hook banner and corrupts the manifest.
- Task-cited `.run.log` files must contain an explicit `exit_status=0` for `tools/check_cairn_task_evidence.rs`; avoid citing the task-evidence gate log in the task whose gate writes that same file, or the checker reads the incomplete log during the run.
- After Cairn input updates, keep `cairn-policy/default.ncl`, `cairn-policy/contracts.ncl`, and `cairn-policy/generated/cairn-policy.json` schema-compatible with the pinned binary; run `nix run .#cairn -- policy export --check` or `.#checks.x86_64-linux.mc-cairn-policy-fresh`. Current policy needs disabled `steel_orchestration_policy` and explicit `probe_policy` blocks.

## Hyperion
- Toolchain pinned in `hyperion/rust-toolchain.toml`: `nightly-2025-02-22` with `rustfmt` and `clippy`.
- `hyperion/.cargo/config.toml` adds `--cfg tokio_unstable` and `-Ctarget-cpu=native`; run cargo from inside `hyperion/` so config applies.
- Fast checks: `just fmt`, `just lint`, `just test`, `just ci`.
- `just test` runs `cargo nextest run`.
- Common local loop: `cargo check -p bedwars`, `cargo run --bin bedwars`, and `cargo run --bin hyperion-proxy -- --server "127.0.0.1:35565" "0.0.0.0:25565"`.

## Stevenarella
- On 1.20.1, under-map CTF symptoms can come from missing dimension-codec bounds: JoinGame gives `dimension_codec` plus `dimension_type_name`; the client must apply the selected type's `min_y`/`height` before parsing `ChunkData_AndLight` sections.

## Valence
- CI copies playground template first: `cp tools/playground/src/playground.template.rs tools/playground/src/playground.rs`. Do same before fmt/clippy/test/doc if `playground.rs` is missing.
- `servers/valence/rustfmt.toml` uses unstable rustfmt settings. Use `cargo +nightly fmt` locally.
- CI-equivalent checks:
  - `cargo clippy --workspace --no-deps --all-features --all-targets -- -D warnings`
  - `cargo test --workspace --all-features --all-targets`
  - `cargo test --workspace --all-features --doc`
  - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --document-private-items`
- Example smoke test: `cargo r -r --example parkour`.
- `extractor/` is a Gradle/Java project. CI builds it with Java 21 via `./gradlew build`.
- On Linux, docs/clippy/test jobs may need GUI and SSL development packages: `libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev libclang-dev libgtk-3-dev`.
