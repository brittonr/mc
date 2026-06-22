# Design: scenario manifest fallback budget gate

## Scope

The gate tracks migration accounting for scenario manifest rows. It does not run scenarios, inspect live receipts, or decide gameplay correctness. It only evaluates manifest rows, migration states, and waiver metadata.

## Functional core

Add a pure fallback-budget evaluator that accepts in-memory manifest rows plus an in-memory baseline allowlist and returns a structured report:

- rows that remain approved fallback entries
- rows that left fallback and can be removed from the baseline
- rows newly marked substring fallback
- rows missing owner/reason/non-claim/next-action waiver text
- rows that regressed from typed-event-ready to fallback

The core has no filesystem, environment, process, or time access. It compares row names and waiver fields exactly and returns deterministic diagnostics.

## Imperative shell

A thin checker shell reads the Nickel-exported/generated manifest surfaces and the checked-in fallback baseline, calls the pure evaluator, renders a report, and exits nonzero on new fallback debt or typed-event regression. The flake check invokes this shell as part of the focused mc-compat validation surface.

## Validation strategy

- Positive fixture: current baseline passes and reports unchanged fallback rows.
- Positive fixture: removing a fallback row passes and reports it as migration progress.
- Negative fixture: adding an unlisted fallback row fails.
- Negative fixture: omitting waiver text fails.
- Negative fixture: moving a typed-event-ready row back to fallback fails.
- Run generated-surface freshness, scenario manifest checks, evidence manifests, Cairn gates, and Cairn validation.

## Non-claims

The gate is migration accounting only. It does not claim typed-event coverage for fallback rows, live compatibility, semantic equivalence, production readiness, public-server safety, or any new gameplay behavior.
