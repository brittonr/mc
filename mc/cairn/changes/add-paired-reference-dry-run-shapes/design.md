# Design: paired-reference dry-run shapes

## Scope

This change adds deterministic dry-run shape coverage for paired reference comparator scenarios. It does not run Paper, Valence, or Stevenarella clients and does not change the promotion rule for live reference parity evidence.

## Receipt shape contract

Each dry-run receipt for the selected reference scenarios should include deterministic placeholder values for the same structural fields the live comparator requires:

- scenario identity;
- reference backend label;
- Valence backend label;
- metric names;
- tolerance fields;
- comparison status placeholder;
- non-claim text that distinguishes dry-run shape from live parity evidence.

The shape may use deterministic placeholder revision values rather than reading host source-tree state. It must not assert live metric equality or exact vanilla parity.

## Functional core

Add or reuse a pure validator that accepts a normalized dry-run receipt value and returns typed diagnostics. The validator checks required fields, allowed backend labels, metric/tolerance presence, scenario identity, and non-claim text. It rejects overbroad wording that would treat dry-run shape coverage as live Paper/Valence parity.

## Imperative shell

The runner shell creates dry-run receipts and writes them to the requested path. The scenario manifest checker invokes shape validation through existing generated-surface checks. Filesystem reads, writes, and process exits remain in the shell.

## Validation strategy

- Positive fixture: valid combat reference dry-run shape passes.
- Positive fixture: valid armor reference dry-run shape passes.
- Negative fixture: missing reference backend fields fails.
- Negative fixture: missing Valence backend fields fails.
- Negative fixture: missing tolerance fields fails.
- Negative fixture: dry-run receipt claiming live parity fails.
- Closeout records scenario manifest checks, generated surfaces, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation.

## Non-claims

Dry-run shape evidence is not live evidence, not a Paper/Valence comparator pass, not exact Mojang vanilla parity, not full combat parity, and not production-readiness or public-server safety evidence.
