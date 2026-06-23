# Tasks

- [ ] [serial] Compare Hyperion and Valence paletted container behavior, representation states, encode paths, and unsafe/nightly dependencies. r[valence_hyperion_integration.palette_optimization.inventory]
- [ ] [depends:inventory] Define correctness invariants for indexing, palette transitions, unique queries, iteration, encoding, invalid indices, and mutation behavior. r[valence_hyperion_integration.palette_optimization.invariants]
- [ ] [depends:invariants] Add baseline correctness fixtures and benchmarks before changing Valence internals. r[valence_hyperion_integration.palette_optimization.baseline]
- [ ] [depends:baseline] Port measured stable-safe optimization concepts only where benchmarks and invariants justify the change. r[valence_hyperion_integration.palette_optimization.port]
- [ ] [depends:port] Add positive and negative tests for single, indirect, direct, palette growth, direct fallback, invalid indices, encode parity, and stale unique data. r[valence_hyperion_integration.palette_optimization.tests]
- [ ] [depends:tests] Run correctness tests, before/after benchmarks, selected chunk mc-compat dry runs if behavior changes, Cairn gates, and Cairn validation. r[valence_hyperion_integration.palette_optimization.validation]
