# Design: scenario fallback budget ratchet

## Scope

This change updates migration accounting for existing scenario manifest rows. It does not run scenarios, modify fixtures, or decide gameplay correctness. The only promoted claim is that the fallback budget baseline matches the current manifest and fails closed on future fallback regressions.

## Functional core

Use a pure evaluator over in-memory manifest rows and in-memory baseline entries. The evaluator returns a deterministic report with:

- active approved fallback rows;
- migrated rows removed from the fallback budget;
- stale baseline approvals that no longer correspond to current fallback rows;
- unapproved current fallback rows;
- missing waiver metadata;
- typed-event-ready rows that regressed to substring fallback.

If the current checker already computes these categories, this change should narrow to baseline refresh plus tests that lock the behavior. Any new logic stays filesystem-free and returns typed diagnostics rather than printing directly.

## Imperative shell

The checker shell reads the Nickel manifest/baseline surfaces, calls the pure evaluator, renders the report, and exits nonzero for new fallback debt, missing waiver metadata, or typed-event regressions. Generated docs are refreshed by the existing generated-surface command paths.

## Validation strategy

- Positive fixture: the ratcheted baseline passes against the current manifest.
- Positive fixture: a migrated row removed from fallback is reported as progress, not a failure.
- Negative fixture: a removed row reintroduced as substring fallback fails unless a new waiver is added.
- Negative fixture: a typed-event-ready row regressing to fallback fails.
- Closeout records generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation.

## Non-claims

Fallback-budget accounting does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, production readiness, full CTF correctness, full survival correctness, or broad Minecraft compatibility.
