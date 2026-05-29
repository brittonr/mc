# Napkin

## Corrections
| Date | Source | What Went Wrong | What To Do Instead |
|------|--------|----------------|-------------------|
| 2026-04-14 | self | Wrote AGENTS guidance before re-checking every claimed command/path | Verify repo-specific commands and paths from repo files or listings before recording them in AGENTS or napkin |
| 2026-05-27 | self | Ran `git add -f mc` while fixing review evidence, which staged ignored target outputs and nested repos | Never force-add broad directories in this workspace; force-add only exact intended Cairn/docs evidence paths |
| 2026-05-27 | done-review | Archived proof Cairns with tasks marked complete even though evidence docs demoted key scope to non-claims | Before archive, compare checked tasks/proposal scope against promoted evidence rows; either add the missing receipt/inventory or amend/reopen so tasks describe only completed gates |
| 2026-05-28 | done-review | Pushed survival evidence that cited Valence `2663ed7` plus an untracked fixture diff even though the fixture was later committed at `1fac05a` | Re-run or update evidence against a committed child-repo revision containing the fixture before push; do not leave `plus current diff` as promoted evidence metadata |
| 2026-05-28 | done-review | Refresh fixed Valence revision but still claimed Stevenarella `9921e68` without repo-local receipt or oracle evidence for the client rev | When receipt schema lacks child client rev, add an oracle checkpoint doc with question/evidence/decision/owner/next-action before claiming the child revision |
| 2026-05-28 | done-review | Marked broad parser-fixture tasks complete by treating raw `Vec<u8>` byte-preservation as positive+negative parser-shape evidence | For byte-opaque raw packet consumers, claim only raw byte preservation and add explicit semantic malformed-payload non-claim; require true negative rejection fixtures only for structured parsers unless semantic decoders exist |
| 2026-05-29 | self | Ran `tools/check_evidence_manifests.py` with a `.b3` path, but the checker accepts only no args or `--self-test` | Run `nix develop --no-update-lock-file -c python3 tools/check_evidence_manifests.py` for the repo-wide manifest check; use `b3sum` directly for one manifest |
| 2026-05-29 | self | Wrote an evidence oracle with an unquoted shell heredoc containing backticks, causing command substitution and a corrupted doc | Use `write` for markdown with backticks or quote heredoc delimiters (`<<'EOF'`) when generating evidence docs |
| 2026-05-29 | self | Committed runner changes without force-adding ignored `tools/mc-compat-runner/src/scenario_manifest_generated.rs`, so Nix builds failed with missing module | Before any Nix flake/devshell check in `mc/`, force-add exact new generated/source files under ignored paths |
| 2026-05-29 | self | Live chest rail showed Stevenarella skipped all chest actions because `apply_mc_compat_active_probe` early-return guard omitted `survival_chest_probe_enabled` | When adding a Stevenarella probe flag, include it in both the active-probe early guard and movement gating as appropriate |
| 2026-05-29 | self | Rebuilt the shared Paper survival fixture jar in-place, which invalidated older `.b3` manifests that cited the same `target/.../mc-compat-paper-survival-fixture.jar` path | For new Paper evidence rows, build/cite a row-specific jar path or regenerate every manifest that intentionally follows the current fixture jar |
| 2026-05-29 | self | Piped `nix develop --command python3 tools/check_evidence_manifests.py` through `tee`; it exited 120 after the devshell banner | For manifest logs, redirect command output straight to the log file instead of piping through `tee` |
| 2026-05-29 | self | Manually appended accepted `cairn/specs/mc-compatibility/spec.md`, which made older `.b3` manifests stale because they included that accepted spec path | After accepted spec edits, run the evidence manifest checker and refresh every `.b3` that cites `cairn/specs/mc-compatibility/spec.md` |
| 2026-05-29 | self | Local evidence manifest check passed while Nix failed because `.b3` rows cited nested child repos and `target/` artifacts outside the parent source closure | Copy those exact bytes under `docs/evidence/`, update `.b3` rows to the copied paths, force-add new evidence artifacts, then rerun the Nix manifest check |
| 2026-05-29 | self | Tried direct host `rustc` while validating mc Cairn task evidence, but this environment exposes Rust through the mc dev shell | Use `nix develop --no-update-lock-file -c rustc ...` for standalone Rust checkers in `mc/` |

## User Preferences
- Keep replies terse and direct.

## Patterns That Work
- Start work in `mc/` by checking repo boundaries first. `git rev-parse` from `mc/` resolves to parent `/home/brittonr/git`, while `hyperion/` and `valence/` are nested repos with their own roots.
- Run build, test, and VCS commands from inside the target child repo, not from `mc/`.
- Verify command and path claims from actual repo files before writing AGENTS or napkin notes.
- Keep workspace-level notes in `mc/AGENTS.md`; put repo-specific workflow in `hyperion/AGENTS.md` and `valence/AGENTS.md`.

## Patterns That Don't Work
- Treat `mc/` as one buildable Rust repo.
- Assume child repos share one VCS workflow; `hyperion/` has both `.git/` and `.jj/`, `valence/` is git-only.
- For current-head live refresh, equipment-update and armor-mitigation runs can fail at the client/team milestone boundary even after build succeeds. Projectile-hit is the better representative rail when it has current HEAD evidence.
- Cairn gate CLI syntax is `cairn gate <proposal|design|tasks> <change> --root .`; do not use OpenSpec-style `--stage`/`--change` flags with the native binary.
- Running `nix run .#cairn -- archive ... --execute` under this environment may create `cairn/archive/1970-01-01-*`; rename to the real date before commit and rerun `cairn validate`.
- Do not cite untracked `target/...` live receipts as sole evidence in Cairn tasks/docs. Copy review-critical receipts/logs under `docs/evidence/` and record BLAKE3/checkpoint before committing.
- Parent `.gitignore` has broad `/mc/*` and `/mc/tools/*` ignores. Force-add new Cairn artifacts/tools before Nix flake builds; otherwise the flake source omits them even though they exist in the working tree.
- `nix develop --no-update-lock-file -c ...` in `mc/` prints devshell banner text to stdout before command output. When generating `.b3` files through it, pipe through `tail -n 1` or otherwise strip banner lines before committing.
- For mc Cairn checks, use repo-pinned `nix run --no-update-lock-file .#cairn -- ... --root .` from `/home/brittonr/git/mc`; sibling `/home/brittonr/git/cairn#cairn` can be newer and fail with policy schema drift such as `policy missing field steel_orchestration_policy`.
- When a gate log has its own `.b3` sidecar, do not run the manifest checker inside the log while the old sidecar exists; remove/regenerate the sidecar after writing the log, then run a separate manifest check.
- Projectile damage attribution evidence must pin Valence to `e5d18ad04010d92881267ac1ea43922ae91821f5` and use a fresh/matching Valence worktree; stale `/tmp/valence-compat-*` worktrees can silently point at the wrong revision if not checked.
- Armor/modifier evidence needs the `/tmp/mc-compat-valence.log` plus both `/tmp/mc-compat-client.*.log` files copied into `docs/evidence/` before another live rail overwrites them; the runner receipt alone does not carry raw mitigation math.
- Paper backend evidence needs `--keep-server` for final runs; otherwise runner removes the container and `docker logs` cannot be copied into `docs/evidence/`.
- Paper 1.20.1 chest fixture did not reliably open from Stevenarella's use-item-on-block packet alone; final rail uses the server fixture to schedule the chest open, then treats the received chest-slot click as the store trigger and sets/logs slot `0 = Dirt`.
- User wants checks/scripts authored in Rust or Steel Scheme only. Do not add new Python or Bash gates; migrate touched Python gates before extending them.

## Domain Notes
- `mc/` is a workspace directory with two independent Rust repos: `hyperion/` and `valence/`.
- `hyperion/` already has repo-local agent context in `hyperion/.agent/napkin.md` and a project-local skill in `hyperion/.pi/skills/`.
- `valence/` CI copies `tools/playground/src/playground.template.rs` to `tools/playground/src/playground.rs` before fmt/clippy/test/doc runs.
- Cairn `archive --execute` in mc currently writes archive dirs as `1970-01-01-<change>`; rename to current-date archive path (e.g. `2026-05-29-<change>`) and rerun `nix run --no-update-lock-file .#cairn -- validate --root .` before staging.
