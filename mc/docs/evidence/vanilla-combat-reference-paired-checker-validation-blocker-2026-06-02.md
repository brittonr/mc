# Vanilla combat reference paired checker validation blocker — 2026-06-02

## Question

Can `tools/check_vanilla_combat_reference_parity.rs` be marked validated for `r[mc_compatibility.vanilla_combat_reference_paired_receipts.checker]`?

## Inspected evidence

- Authored Rust checker: `tools/check_vanilla_combat_reference_parity.rs`
- Active Cairn task: `cairn/changes/vanilla-combat-reference-paired-receipts/tasks.md`
- Static Steel token sanity check verified the checker contains the pure comparator, normalization path, CLI path, reference oracle/version constants, and required positive/negative diagnostics.
- Auxiliary Steel validation log: `docs/evidence/vanilla-combat-reference-paired-checker-steel-validation-2026-06-02.run.log`.
- Generated safe Steel validator: `steel_validate_vanilla_combat_checker`; saved positive/negative tests passed.

## Decision

No. The checker task must remain unchecked until a shell-capable validation run compiles the Rust file and runs:

```sh
nix develop --no-update-lock-file -c rustc --edition=2021 \
  tools/check_vanilla_combat_reference_parity.rs \
  -o target/check-vanilla-combat-reference-parity

target/check-vanilla-combat-reference-parity --self-test
```

The output must be copied to `docs/evidence/vanilla-combat-reference-paired-checker-2026-06-02.run.log`, include `exit_status=0`, and have a BLAKE3 sidecar before the checker task is marked complete.

A project-local Pi extension provides an allowlisted helper for this exact command shape after `/reload`:

```text
mc_validate_vanilla_combat_checker
```

The extension was updated after the first successful validation run to also write the `.b3` sidecar, so reload once more before rerunning if the sidecar is still missing.

Suggested tool parameters:

```json
{"repoDir":"/home/brittonr/git/mc","logPath":"docs/evidence/vanilla-combat-reference-paired-checker-2026-06-02.run.log"}
```

## Owner

agent

## Next action

Run the compile/self-test in a shell-capable session, record the log and `.b3`, then update the checker task evidence.
