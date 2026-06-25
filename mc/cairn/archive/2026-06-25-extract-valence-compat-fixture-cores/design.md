# Design: Extract Valence compatibility fixture cores

## Context

The CTF and survival compatibility examples now encode rail behavior that the runner treats as evidence fixtures. They need stronger module boundaries than ordinary examples: pure rule decisions should be testable without Bevy ECS setup, while Bevy systems remain integration shells.

## Decisions

### 1. Separate fixture core from Bevy adapter

Move deterministic decisions into core modules: flag ownership transitions, score-limit behavior, race rejection, spawn reset, inventory event classification, survival container decisions, redstone toggles, persistence markers, and milestone string construction. Bevy systems pass explicit snapshots/events to those cores.

### 2. Avoid global mutable policy state

State currently held in globals should become Bevy resources, fixture state structs, or explicit inputs. If a temporary global remains during migration, it must be documented as a compatibility shim with a retirement task.

### 3. Keep examples runnable

The public example binaries should still run with the same commands and scenario env vars. The extraction should not change Valence core default gameplay behavior.

### 4. Test rules without the world

Pure fixture-core tests should cover valid transitions and invalid/rejected transitions without launching a server. Bevy smoke tests can remain focused on wiring.

## Risks / Trade-offs

- Extracting fixture logic can expose assumptions about Bevy event order; encode ordering explicitly in core inputs.
- Some milestone strings are receipt-stable; keep formatting in tested helpers.
- This work does not claim production CTF/survival behavior or vanilla parity.
