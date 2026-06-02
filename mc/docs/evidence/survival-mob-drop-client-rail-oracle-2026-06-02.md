# Survival mob-drop client rail oracle

## Question

Does the runner/client task have reviewable evidence for the Stevenarella client-side `survival-mob-drop` probe even though the parent mc repo does not track the nested Stevenarella source?

## Inspected evidence

- Stevenarella child repo commit `fd7e229` (`add mob-drop client probe rail`) commits only `src/server/mod.rs` for the mob-drop probe.
- Focused validation log `docs/evidence/survival-mob-drop-runner-client-2026-06-02.run.log` records `cargo test server::tests::survival_mob_drop -- --nocapture` passing after the probe code was written.
- The Stevenarella checkout still has unrelated pre-existing dirty files: `blocks/src/lib.rs`, `src/model/mod.rs`, `src/render/mod.rs`, and `src/resources.rs`. Those files were not staged in `fd7e229` and are outside this mob-drop probe task.

## Decision

Treat `fd7e229` plus the focused validation log as the client-side runner/probe evidence for `r[mc_compatibility.survival_mob_drop.runner]`. Do not claim paired reference receipts or clean child-repo evidence from this task; those remain scoped to later fixture/receipt tasks.

## Owner

Agent / future Cairn drain.

## Next action

Implement Valence and Paper fixture instrumentation, then produce paired receipts with explicit child revision metadata or a new oracle if the receipt schema still cannot record the nested Stevenarella revision.
