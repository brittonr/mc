# Tasks

- [x] [serial] Compare Hyperion and Valence paletted container behavior, representation states, encode paths, and unsafe/nightly dependencies. r[valence_hyperion_integration.palette_optimization.inventory]
  - Evidence: docs/evidence/paletted-container-optimization-inventory.md; docs/evidence/paletted-container-pre-gate-proposal.run.log; docs/evidence/paletted-container-optimization.b3.
- [x] [depends:inventory] Define correctness invariants for indexing, palette transitions, unique queries, iteration, encoding, invalid indices, and mutation behavior. r[valence_hyperion_integration.palette_optimization.invariants]
  - Evidence: docs/evidence/paletted-container-optimization-inventory.md; docs/evidence/paletted-container-baseline-fixture-tests.run.log; docs/evidence/paletted-container-optimization.b3.
- [x] [depends:invariants] Add baseline correctness fixtures and benchmarks before changing Valence internals. r[valence_hyperion_integration.palette_optimization.baseline]
  - Evidence: docs/evidence/paletted-container-existing-baseline-test.run.log; docs/evidence/paletted-container-baseline-fixture-tests.run.log; docs/evidence/paletted-container-baseline-bench.run.log; docs/evidence/paletted-container-optimization.b3.
- [x] [depends:baseline] Port measured stable-safe optimization concepts only where benchmarks and invariants justify the change. r[valence_hyperion_integration.palette_optimization.port]
  - Evidence: docs/evidence/paletted-container-final-chunk-tests.run.log; docs/evidence/paletted-container-final-bench.run.log; docs/evidence/paletted-container-optimization.b3.
- [x] [depends:port] Add positive and negative tests for single, indirect, direct, palette growth, direct fallback, invalid indices, encode parity, and stale unique data. r[valence_hyperion_integration.palette_optimization.tests]
  - Evidence: docs/evidence/paletted-container-final-chunk-tests.run.log; docs/evidence/paletted-container-optimization.b3.
- [x] [depends:tests] Run correctness tests, before/after benchmarks, selected chunk mc-compat dry runs if behavior changes, Cairn gates, and Cairn validation. r[valence_hyperion_integration.palette_optimization.validation]
  - Evidence: docs/evidence/paletted-container-valence-fmt.run.log; docs/evidence/paletted-container-final-chunk-tests.run.log; docs/evidence/paletted-container-final-bench.run.log; docs/evidence/paletted-container-final-cairn-gates.run.log; docs/evidence/paletted-container-final-cairn-validate.run.log; docs/evidence/paletted-container-optimization.b3.
