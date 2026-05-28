# Agent Notes

## Scope
- This `mc/` directory is a workspace folder, not one buildable repo.
- `hyperion/` and `valence/` are independent nested Rust repos. Run commands from inside target repo, not from `mc/`.
- Keep changes scoped to one child repo unless user explicitly asks for cross-repo work.

## VCS boundaries
- `git rev-parse` from `mc/` resolves to parent repo `/home/brittonr/git`. Do not use parent repo status as status of `hyperion/` or `valence/`.
- `hyperion/` has both `.git/` and `.jj/`. Check `.jj/` before assuming git-only workflow.
- `valence/` is a plain git repo.

## Layout
- `hyperion/`: Minecraft engine/proxy workspace. Core crates live under `crates/`; event/game logic lives under `events/bedwars`; helper tools live under `tools/`. Repo-specific workflow now lives in `hyperion/AGENTS.md`.
- `valence/`: Minecraft server framework. Main crate in `src/`; workspace crates in `crates/*`; runnable examples in `examples/`; protocol/data extractor in `extractor/`; docs site in `website/`. Repo-specific workflow now lives in `valence/AGENTS.md`.
- `hyperion/` already has repo-local agent notes in `hyperion/.agent/napkin.md`. Keep workspace-wide notes here; prefer child-repo `AGENTS.md` files for repo-specific commands and conventions.

## Workflow
- Before editing inside a child repo, read that repo's `README.md`, `CONTRIBUTING.md`, and any local `AGENTS.md` or `.agent/napkin.md`.
- Prefer repo-local commands and toolchains. Root `mc/` has no shared Cargo workspace, test runner, or formatter.
- Avoid mixed commits across `hyperion/` and `valence/` unless user asks for cross-repo change.
- For Cairn evidence, do not leave review-critical receipts only under untracked `target/`; copy receipt/log artifacts into `docs/evidence/` and record BLAKE3 when tasks/docs cite them.
- If promoted evidence cites child-repo revisions that the receipt does not machine-record (for example Stevenarella client git rev), add a `docs/evidence/*oracle*` checkpoint with `## Question`, `## Inspected evidence`, `## Decision`, `## Owner`, and `## Next action`; otherwise reviewers cannot verify the child-revision claim from repo-local artifacts.
- Before archiving a Cairn, compare every checked task and proposal scope against promoted evidence rows. If a doc says a behavior remains a non-claim, do not mark the task as completed for that behavior; narrow the archive scope or leave/reopen the proof gap.
- Run mc Cairn validation/gates with the repo-pinned app from `mc/` (`nix run .#cairn -- ... --root .`). A newer sibling `/home/brittonr/git/cairn` binary can reject this repo's generated policy schema.

## Hyperion
- Toolchain pinned in `hyperion/rust-toolchain.toml`: `nightly-2025-02-22` with `rustfmt` and `clippy`.
- `hyperion/.cargo/config.toml` adds `--cfg tokio_unstable` and `-Ctarget-cpu=native`; run cargo from inside `hyperion/` so config applies.
- Fast checks: `just fmt`, `just lint`, `just test`, `just ci`.
- `just test` runs `cargo nextest run`.
- Common local loop: `cargo check -p bedwars`, `cargo run --bin bedwars`, and `cargo run --bin hyperion-proxy -- --server "127.0.0.1:35565" "0.0.0.0:25565"`.

## Valence
- CI copies playground template first: `cp tools/playground/src/playground.template.rs tools/playground/src/playground.rs`. Do same before fmt/clippy/test/doc if `playground.rs` is missing.
- `valence/rustfmt.toml` uses unstable rustfmt settings. Use `cargo +nightly fmt` locally.
- CI-equivalent checks:
  - `cargo clippy --workspace --no-deps --all-features --all-targets -- -D warnings`
  - `cargo test --workspace --all-features --all-targets`
  - `cargo test --workspace --all-features --doc`
  - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --document-private-items`
- Example smoke test: `cargo r -r --example parkour`.
- `extractor/` is a Gradle/Java project. CI builds it with Java 21 via `./gradlew build`.
- On Linux, docs/clippy/test jobs may need GUI and SSL development packages: `libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev libclang-dev libgtk-3-dev`.
