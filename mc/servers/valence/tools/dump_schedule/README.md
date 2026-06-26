# `dump_schedule`

A simple debugging utility for visualizing Valence's schedule graph. Generates a SVG file.

1. Ensure that [Graphviz](https://graphviz.org/) is installed and the `dot` and `tred` commands are available.
2. Run the program with `cargo r -p dump_schedule -- PostUpdate`.
3. Open the generated `graph.svg` in your browser or other program, e.g. `chromium graph.svg`.

## Schedule hygiene gate

Use schedule hygiene evidence when a change intentionally affects Valence's Bevy schedule shape. The focused repository gate is:

```sh
tools/check_valence_schedule_hygiene.rs --root .
```

The Nix check form is:

```sh
nix build .#checks.x86_64-linux.mc-valence-schedule-hygiene --no-link -L
```

### Schedule evidence triggers

Record focused schedule evidence for changes that add, remove, or reorder Bevy plugins, schedules, `SystemSet`s, ordering constraints, event-loop phases, or default plugin membership. Non-schedule-impacting changes do not need a full graph review.

Useful selected schedule dumps include:

```sh
cargo r -p dump_schedule -- PostUpdate
cargo r -p dump_schedule -- EventLoopPreUpdate
```

When optional/default behavior is part of the contract, record a plugin configuration comparison. For example, compare the default configuration with a disabled-plugin comparison such as `DefaultPlugins.build().disable::<NetworkPlugin>()`, and assert the disabled plugin's systems or sets are absent.

### Focused receipt fields

A reviewable schedule receipt should name the command, schedule label, plugin configuration, expected sets/systems, disabled-plugin comparison when relevant, ambiguity policy, and non-claims. Store task-cited receipts, logs, or graph artifacts under `docs/evidence/` and cite a BLAKE3 manifest when a Cairn task depends on them.

The default graph output is diagnostic. It does not claim full schedule semantic equivalence, broad Minecraft compatibility, production readiness, public-server safety, full CTF correctness, or full survival correctness.
