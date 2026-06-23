# Design: Evaluate paletted container optimizations

## Context

Valence and Hyperion both store Minecraft chunk section block states in paletted forms. Hyperion's container includes direct helpers for unique block queries and transitions among storage forms, but uses nightly/unsafe patterns in adjacent code. Valence should use the implementation as a benchmark and design reference first.

## Decisions

### 1. Measure before porting

**Choice:** Add correctness fixtures and benchmarks before modifying Valence's container internals.

**Rationale:** Container changes can regress common chunk workloads despite looking faster in isolation.

### 2. Encode parity is required

**Choice:** Any optimized representation must produce byte-equivalent or semantically equivalent encoded chunk data for the same section state.

**Rationale:** Client-visible chunk data is a compatibility boundary.

### 3. Stable safe implementation first

**Choice:** Port concepts with stable safe Rust unless a separate audit approves unsafe or SIMD-specific code.

**Rationale:** Chunk storage is core infrastructure and must be maintainable.

### 4. Keep optimization claims evidence-bound

**Choice:** Performance notes name the benchmark fixture, dataset, and environment. No broad speedup claims are made without matching evidence.

**Rationale:** Paletted workload performance depends heavily on map shape and mutation pattern.

## Risks / Trade-offs

- More representation states can increase complexity; require invariants and tests.
- Benchmarks can overfit; include varied section distributions.
- Unique-block helpers can be useful but may add overhead; keep them lazy or derived when measured.
