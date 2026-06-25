# Check tier taxonomy

Use the smallest tier that covers the files and claim boundary you changed, then add any affected component-specific check named by the relevant subtree `AGENTS.md` or Cairn task. Tiers describe review coverage and expected evidence; they do not upgrade a dry-run, docs check, or manifest check into live gameplay compatibility evidence.

## Tier summary

| Tier ID | Name | Scope | Required evidence | Expected cost | Non-claims |
| --- | --- | --- | --- | --- | --- |
| `tier.docs-layout` | Docs and layout guidance | Markdown-only edits, layout checklist updates, local agent docs, and small Rust/Steel checkers for docs invariants | Checker output, Cairn gates, Cairn validation, BLAKE3 manifest for cited logs | Fast local | Does not prove runner behavior, component correctness, or live compatibility |
| `tier.generated` | Generated and policy freshness | Scenario manifest surfaces, Nickel exports, generated runner tables, Cairn policy output, and generated indexes | Freshness check or generator `--check`, plus relevant dry-run if generated command shape changes | Fast to medium | Does not prove live scenario success or semantic equivalence |
| `tier.evidence` | Evidence and receipt validation | `docs/evidence/` receipts, `.b3` manifests, partition inventory/index surfaces, task closeout evidence, promotion plans, and current evidence bundles | Evidence partition freshness, evidence manifest check, task-evidence gate when tasks change, affected receipt/schema checker | Medium | Does not create new gameplay evidence; validates durability and claim boundaries only |
| `tier.component` | Component behavior | Rust client/server/runner code, protocol parsing, Valence fixtures, and Stevenarella logic | Smallest affected Cargo test or flake check, plus relevant dry-run wrapper | Medium to high | Does not claim Paper/reference parity or production readiness unless paired evidence says so |
| `tier.live-manual` | Live/manual evidence | Bounded live rails, Paper/reference comparisons, public-server-authorized safety fixtures, and manual oracle notes | Promoted receipts/logs under `docs/evidence/`, BLAKE3 manifests, explicit non-claims, and paired comparator output where required | High/manual | Does not imply broad compatibility, adversarial safety, or unbounded production readiness |
| `tier.archive` | Cairn archive closeout | Active Cairn packages being synced and archived | Cairn proposal/design/tasks gates, sync/archive dry-run and execute receipts, Cairn validation, task evidence gate for checked tasks | Fast to medium | Does not repair missing implementation evidence; archive requires lower-tier evidence first |

## Inventory by tier

| Command or check | Tier | When to run | Evidence output |
| --- | --- | --- | --- |
| `tools/check_layout_boundaries.rs --self-test` | `tier.docs-layout` | Layout docs, local agent notes, nested Git exception changes | Positive/negative checker output |
| `tools/check_layout_boundaries.rs --root .` | `tier.docs-layout` | Layout docs, local agent notes, nested Git exception changes | Current-tree checker output |
| `tools/check_component_registry.rs --self-test` | `tier.docs-layout` | Typed component registry contracts, parser, fixtures, or diagnostics | Positive and negative registry fixture output |
| `tools/check_component_registry.rs --root .` | `tier.docs-layout` | Component registry inventory, docs summaries, nested Git exceptions, or gate participation metadata | Current-tree registry/docs sync output |
| `nix build .#checks.x86_64-linux.mc-compat-layout-boundaries --no-link -L` | `tier.docs-layout` | Layout guard core, component registry export/fixtures, registry docs summaries, or flake wiring changes | Focused flake check output |
| `tools/check_check_tiers.rs --self-test` | `tier.docs-layout` | Check-tier taxonomy/checker changes | Positive/negative checker output |
| `tools/check_check_tiers.rs --root .` | `tier.docs-layout` | Check-tier taxonomy/checker changes | Current-tree checker output |
| `nix run .#cairn -- validate --root .` | `tier.archive` | Any Cairn spec/change/archive mutation | Cairn validation JSON |
| `nix run .#cairn -- gate proposal <change> --root .` | `tier.archive` | Before and after Cairn proposal closeout | Cairn gate JSON |
| `nix run .#cairn -- gate design <change> --root .` | `tier.archive` | Before and after Cairn design closeout | Cairn gate JSON |
| `nix run .#cairn -- gate tasks <change> --root .` | `tier.archive` | Before and after task closeout | Cairn gate JSON |
| `nix run .#cairn -- sync <change> --root .` | `tier.archive` | Before accepted-spec mutation | Dry-run plan |
| `CAIRN_ARCHIVE_DATE=YYYY-MM-DD nix run .#cairn -- archive <change> --root . --execute` | `tier.archive` | Final archive execution | Archive mutation receipt |
| `nix run .#evidence-manifest-refresh -- --check` | `tier.evidence` | Any `docs/evidence/*.b3`, README, accepted spec, or cited artifact change | Manifest freshness report |
| `nix run .#evidence-manifest-refresh -- --refresh` | `tier.evidence` | When a check reports stale digests | Rewritten `.b3` manifests plus check report |
| `tools/check_evidence_partitions.rs --self-test` | `tier.evidence` | Evidence partition checker or generated inventory/index changes | Positive/negative partition, missing index row, and path escape fixtures |
| `tools/check_evidence_partitions.rs --root .` | `tier.evidence` | `docs/evidence/README.md`, generated evidence index/inventory, partitioned evidence paths, or partition docs change | Current-tree evidence partition/index freshness output |
| `nix build .#checks.x86_64-linux.mc-compat-cairn-task-evidence --no-link -L` | `tier.evidence` | Checked Cairn tasks or cited task evidence changes | Flake check build log |
| `nix build .#checks.x86_64-linux.mc-compat-evidence-manifest-refresh --no-link -L` | `tier.evidence` | Evidence manifest checker or manifest refresh changes | Flake check build log |
| `nix build .#checks.x86_64-linux.mc-compat-scenario-manifest --no-link -L` | `tier.generated` | Scenario manifest/config/generated-surface changes | Flake check build log |
| `nix build .#checks.x86_64-linux.mc-compat-generated-harness-surfaces --no-link -L` | `tier.generated` | Generated runner/index surface changes | Flake check build log |
| `nix run .#cairn -- policy export --check` | `tier.generated` | Cairn policy source/generated JSON changes | Policy freshness report |
| `nix build .#checks.x86_64-linux.mc-cairn-policy-fresh --no-link -L` | `tier.generated` | Cairn policy freshness changes | Flake check build log |
| `nix run .#stevenarella -- --dry-run` | `tier.component` | Stevenarella wrapper or client-command documentation changes | Dry-run plan |
| `nix run .#valence -- --dry-run` | `tier.component` | Valence wrapper or server-command documentation changes | Dry-run plan |
| `nix run .#mc-compat-smoke -- --dry-run --server-backend valence --scenario smoke` | `tier.component` | Runner command shape or smoke scenario dry-run changes | Dry-run plan |
| `nix build .#checks.x86_64-linux.mc-compat-maintained-dry-runs --no-link -L` | `tier.component` | Maintained scenario dry-run table changes | Flake check build log |
| `tools/check_octet_monorepo.rs --self-test` | `tier.component` | Aggregate Octet checker, workspace metadata, consumer `dylint.toml`, reviewed baseline, or exception documentation changes | Positive inventory plus negative lint-drift, missing-config, and new-finding fixture output |
| `tools/check_octet_monorepo.rs --root . --octet-source <pinned-octet> --run-octet` | `tier.component` | Octet enforced-scope code changes, reviewed baseline updates, or dynamic gate behavior changes | Per-workspace Octet status, finding counts, new/stale stable-ID comparison, and artifact paths |
| `nix build .#checks.x86_64-linux.mc-octet-monorepo --no-link -L` | `tier.component` | Octet lint inventory, workspace metadata, consumer `dylint.toml`, reviewed baseline, or flake wiring changes | Static lint-inventory/config/baseline drift output plus checker self-test output |
| `nix build .#checks.x86_64-linux.mc-compat-current-evidence-bundle --no-link -L` | `tier.evidence` | Current evidence bundle, promoted receipt, or index changes | Flake check build log |
| `nix build .#checks.x86_64-linux.mc-compat-full-survival-gate --no-link -L` | `tier.evidence` | Survival aggregate gate or row matrix changes | Flake check build log |
| `nix build .#checks.x86_64-linux.mc-compat-aggregate-claim-gates --no-link -L` | `tier.evidence` | Aggregate claim-boundary checker changes | Flake check build log |
| `nix run .#mc-compat-smoke -- --run --server-backend paper --scenario <scenario>` | `tier.live-manual` | Paper/reference side of a promoted parity row | Promoted receipt/log plus BLAKE3 manifest |
| `nix run .#mc-compat-smoke -- --run --server-backend valence --scenario <scenario>` | `tier.live-manual` | Valence side of a promoted parity row | Promoted receipt/log plus BLAKE3 manifest |

## Selecting the smallest sufficient tier

- Docs-only or local agent guidance: run `tier.docs-layout` plus `tier.archive` when closing a Cairn.
- Generated surfaces or Nickel exports: run `tier.generated`; add `tier.component` dry-runs if command shape changed.
- Runner code or scenario logic: run `tier.component`; add `tier.generated` when generated tables or manifest-derived docs change.
- Component code: run `tier.component` and the subtree-local affected Cargo tests; add `tier.live-manual` only when the Cairn task claims live evidence.
- Evidence manifests, receipts, or task citations: run `tier.evidence`; add `tier.archive` when tasks/specs move.
- Live/reference parity rows: run `tier.live-manual`, then `tier.evidence` and `tier.archive` for promoted artifacts.
- Archive-only closeout: run `tier.archive` after confirming lower-tier implementation evidence already exists.

## Freshness contract

`tools/check_check_tiers.rs --root .` verifies the tier IDs, required inventory rows, non-claim language, and root links to this document. It is intentionally a docs/index freshness check; it preserves existing public flake check names and does not add wrapper outputs or change evidence semantics.
