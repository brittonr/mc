# Proposal: Add optional observability and profiling hooks

## Why

Hyperion uses tracing, stats, and profiler-oriented workflows to understand high-player-count behavior. Valence would benefit from optional observability hooks for tick phases, networking, chunk egress, entity updates, and plugin systems, but these hooks should be disabled or lightweight by default and must not introduce a required profiler dependency.

## What Changes

- Review Hyperion observability/profiling patterns and Valence's current tracing/logging surfaces.
- Define an optional observability contract for spans, counters, histograms or summaries, tick phase labels, redaction, and overhead budgets.
- Implement pure event/metric classification where possible, with exporter/profiler integration as shell adapters.
- Add positive and negative tests for disabled hooks, enabled hook labels, redaction, unknown metric names, exporter failure, and low-overhead default behavior.
- Document how to collect evidence without claiming production capacity from local traces alone.

## Impact

- **Files**: optional observability crate/plugin, Valence tick/network/chunk/entity instrumentation points, docs, tests, and Cairn artifacts.
- **Testing**: classification tests, plugin-disabled regressions, exporter failure fixtures, smoke traces, overhead checks if claims are made, and Cairn gates/validation.
- **Non-claims**: this does not make profiling mandatory and does not prove large-scale capacity without separate load evidence.
