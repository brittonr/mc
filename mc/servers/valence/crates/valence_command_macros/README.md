Simplify the creation of Valence commands with a derive macro.

## Scope and non-goals

`valence_command_macros` is optional ergonomics over Valence command APIs. It produces normal Valence command graph builder calls and keeps `valence_command::graph`, parser types, scopes, and handlers authoritative. Manual `CommandGraphBuilder` construction remains the baseline for advanced command trees.

Hyperion's Clap command framework was reviewed as reference-only design input. No Hyperion command code is copied, Valence command internals are not replaced, and existing command behavior changes only when a user opts into this derive or manual helper APIs.

| Hyperion source | Classification | Valence target | Reason | Non-claims |
| --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion-clap/src/lib.rs` | reference | `valence_command_macros` docs/tests | Clap derives show concise literal/argument/permission registration, but Valence graph builders remain authoritative. | No Hyperion command compatibility, runtime replacement, or production-scale claim. |
| `hyperion/crates/hyperion-clap-macros/src/lib.rs` | reference | derive diagnostics checklist | Permission derives motivate deterministic missing-attribute diagnostics only. | No copied macro implementation or permission model adoption. |
| `hyperion/events/bedwars/src/command/*.rs` | reference | examples and negative-test checklist | Bedwars commands demonstrate simple command ergonomics and permission annotations. | No Bedwars gameplay, GUI, combat, or default command behavior is imported. |
| `servers/valence/crates/valence_command/src/graph.rs` | Valence-owned | command graph builders | Valence graph nodes, parsers, suggestions, scopes, and handlers stay the output format. | No replacement of command graph internals. |

## Usage

```rust
#[derive(Command, Debug, Clone)]
#[paths("teleport", "tp")]
#[scopes("valence.command.teleport")]
enum TeleportCommand {
    #[paths = "{location}"]
    ExecutorToLocation { location: Vec3Parser },
    #[paths = "{target}"]
    ExecutorToTarget { target: EntitySelector },
    #[paths = "{from} {to}"]
    TargetToTarget {
        from: EntitySelector,
        to: EntitySelector,
    },
    #[paths = "{target} {location}"]
    TargetToLocation {
        target: EntitySelector,
        location: Vec3Parser,
    },
}

#[derive(Command, Debug, Clone)]
#[paths("gamemode", "gm")]
#[scopes("valence.command.gamemode")]
enum GamemodeCommand {
    #[paths("survival", "{/} gms")]
    Survival,
    #[paths("creative", "{/} gmc")]
    Creative,
    #[paths("adventure", "{/} gma")]
    Adventure,
    #[paths("spectator", "{/} gmsp")]
    Spectator,
}

#[derive(Command, Debug, Clone)]
#[paths("test", "t")]
#[scopes("valence.command.test")]
#[allow(dead_code)]
enum TestCommand {
    // 3 literals with an arg each
    #[paths("a {a} b {b} c {c}", "{a} {b} {c}")]
    A { a: String, b: i32, c: f32 },
    // 2 literals with an arg last being optional (Because of the greedy string before the end
    // this is technically unreachable)
    #[paths = "a {a} {b} b {c?}"]
    B {
        a: Vec3Parser,
        b: GreedyString,
        c: Option<String>,
    },
    // greedy string optional arg
    #[paths = "a {a} b {b?}"]
    C { a: String, b: Option<GreedyString> },
    // greedy string required arg
    #[paths = "a {a} b {b}"]
    D { a: String, b: GreedyString },
    // five optional args and an ending greedyString
    #[paths("options {a?} {b?} {c?} {d?} {e?}", "options {b?} {a?} {d?} {c?} {e?}")]
    E {
        a: Option<i32>,
        b: Option<QuotableString>,
        c: Option<Vec2Parser>,
        d: Option<Vec3Parser>,
        e: Option<GreedyString>,
    },
}
```

## Attributes

### `#[paths(...)]` or `#[paths = "..."]`

The `#[paths(...)]` or `#[paths = "..."]` attribute is used to specify the different paths that can be used to invoke
the command. The paths are specified as string literals, where any arguments are enclosed in curly braces `{}`.
The arguments are then mapped to fields in the command enum variant.

For example, in the `Teleport` enum, the `ExecutorToLocation` variant has a path of `{location}`, which means it expects
a single argument called `location` of type `Vec3Parser`. The `ExecutorToTarget` variant has a path of `{target}`, which
expects a single argument called `target` of type `EntitySelector`.

The paths attribute can have multiple values separated by commas, representing alternative paths that can be used to 
invoke the command. These alternative paths can have different argument orders, but they must have the same arguments.

Their are two special paths that can be used. The first is `{/}`, which represents the root command, this can only be 
used at the start of the command to specify it as a direct child of the root node. The second is `{<arg>?}`, which
represents an optional argument. The optional argument must only be followed by other optional arguments or the end of 
the path.

### `#[scopes(...)]` or `#[scopes = "..."]`

The `#[scopes(...)]` or `#[scopes = "..."]` attribute is used to specify the scopes that the command belongs to. Scopes
are used to specify who can use the command. Scope strings must be non-empty, contain no whitespace, and avoid empty dot
segments. The derive attaches scopes to generated literal nodes so Valence's scope registry and execution checks remain
authoritative.

The scopes attribute can have multiple values separated by commas, representing the different scopes that the command
belongs to.

### Suggestions and manual fallback

Client-side suggestion metadata is supported by manual `CommandGraphBuilder` construction via `with_suggestion(...)`.
The derive intentionally rejects `#[suggestions(...)]` for now with a deterministic diagnostic so generated command trees
do not hide unsupported suggestion semantics. Use manual graph construction when a command needs custom suggestions,
redirections, shared prefixes, dynamic parser selection, or graph shapes that should be inspected by hand.

## Diagnostics contract

The derive fails before registering an ambiguous or incomplete command graph when it can detect misuse:

- missing `#[paths(...)]` on a command type or enum variant reports a missing handler route;
- duplicate command paths report the duplicated route;
- invalid scopes with whitespace, empty strings, or empty dot segments are rejected;
- path arguments missing a matching struct or variant field are rejected;
- optional arguments may only be followed by other optional arguments;
- unsupported suggestion annotations are rejected with a manual-fallback message.

Positive tests compare generated graph data against equivalent manual `CommandGraphBuilder` output. Negative tests cover
duplicate paths, missing handlers, parser-field mistakes, invalid scopes, unsupported suggestions, optional-argument
ordering, and plugin-disabled behavior. These tests are command-helper evidence only; they do not claim broad Minecraft
compatibility, Hyperion compatibility, public-server safety, production readiness, or semantic equivalence.

## How do command graphs work anyway?

This is the core of the command system. It is a graph of `CommandNode`s that are connected by the `CommandEdgeType`. The
graph is used to determine what command to run when a command is entered. The graph is also used to generate the command
tree that is sent to the client. You can think of it as a tree where each leaf is part of a command, and the path to the
leaf is the command. See the documentation for `command.rs` in `valence_command` for more information.


### Our teleport command from the example (made with graphviz)
```text
                                              ┌────────────────────────────────┐
                                              │              Root              │ ─┐
                                              └────────────────────────────────┘  │
                                                │                                 │
                                                │ Child                           │
                                                ▼                                 │
                                              ┌────────────────────────────────┐  │
                                              │          Literal: tp           │  │
                                              └────────────────────────────────┘  │
                                                │                                 │
                                                │ Redirect                        │ Child
                                                ▼                                 ▼
┌──────────────────────────────────┐  Child   ┌──────────────────────────────────────────────────────────────────────────────┐
│  Argument: <destination:entity>  │ ◀─────── │                              Literal: teleport                               │
└──────────────────────────────────┘          └──────────────────────────────────────────────────────────────────────────────┘
                                                │                                           │
                                                │ Child                                     │ Child
                                                ▼                                           ▼
┌──────────────────────────────────┐  Child   ┌────────────────────────────────┐          ┌──────────────────────────────────┐
│ Argument: <destination:location> │ ◀─────── │   Argument: <target:entity>    │          │ Argument: <destination:location> │
└──────────────────────────────────┘          └────────────────────────────────┘          └──────────────────────────────────┘
                                                │
                                                │ Child
                                                ▼
                                              ┌────────────────────────────────┐
                                              │ Argument: <destination:entity> │
                                              └────────────────────────────────┘
```