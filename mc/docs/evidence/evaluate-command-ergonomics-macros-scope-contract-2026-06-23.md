# Command ergonomics macros scope and contract evidence

## Question

Can Valence use Hyperion command ergonomics as reference input while keeping Valence command graph internals authoritative?

## Inspected evidence

| Source | Classification | Decision |
| --- | --- | --- |
| `hyperion/crates/hyperion-clap/src/lib.rs` | reference | Clap-derived command registration, permission checks, and completions informed the ergonomics checklist only. No code was copied. |
| `hyperion/crates/hyperion-clap-macros/src/lib.rs` | reference | Permission derive diagnostics informed missing-attribute diagnostics only. No macro implementation was copied. |
| `hyperion/events/bedwars/src/command/*.rs` | reference | Simple command examples informed literal/argument/scope ergonomics only. No Bedwars gameplay, GUI, combat, or default behavior was imported. |
| `servers/valence/crates/valence_command/src/graph.rs` | Valence-owned | Valence graph nodes, parsers, suggestions, scopes, and handlers remain the helper output format. |
| `servers/valence/crates/valence_command_macros/src/expand.rs` | Valence-owned | The derive now keeps pure expansion/diagnostic validation separate from the proc-macro shell. |

## Decision

Hyperion remains reference-only. The implemented Valence helper contract is optional: derive output emits standard Valence command graph builder calls, manual graph construction remains the baseline for advanced command shapes, and suggestion metadata is supported through manual builder construction while derive-level suggestion annotations fail closed with a deterministic diagnostic.

Positive tests cover generated command graph output, manual graph parity, parser rejection, suggestion metadata, and plugin-disabled behavior. Negative tests cover duplicate paths, missing handler paths, parser-field mistakes, invalid scopes, unsupported suggestions, optional argument ordering, and missing command paths.

## Owner

Cairn change `evaluate-command-ergonomics-macros`.

## Non-claims

This evidence does not claim Hyperion command compatibility, replacement of Valence command graph internals, broad Minecraft compatibility, semantic equivalence, public-server safety, production readiness, or changes to default command behavior without opt-in helper usage.

## Next action

Use the focused test and Cairn validation logs in this change package as closeout evidence, then sync the accepted spec and archive the change if final gates pass.
