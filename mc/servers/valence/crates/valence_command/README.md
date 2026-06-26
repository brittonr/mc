# Valence Command

This plugin manages the command system for a valence server. It is responsible for parsing, storing, managing and
dispatching commands.

#### This plugin manages the following:

- Registering commands to a Command Graph which is used parse commands.
- Receiving commands from the client and turning them into events.
- Parsing commands and dispatching them in the registered executable format.
- Sending the command graph to clients.

See the module level documentation for more information.

Optional admin permission ergonomics are documented in [ADMIN_PERMISSIONS.md](ADMIN_PERMISSIONS.md).

## Typed command packet event

`CommandPlugin` decodes `CommandExecutionC2s` once in `EventLoopPreUpdate` during `EventLoopSet::TypedAdapters` and emits `CommandExecutionPacketEvent` with the source command client, raw packet arrival timestamp, and decoded command string. `emit_command_execution_events` consumes that typed packet event in the domain phase and preserves the existing public `CommandExecutionEvent` consumed by command parsing and user systems.

Wrong packet IDs, decode failures, partial decodes, malformed command payloads, and stale entities without `CommandScopes` emit no typed packet event and no public command execution event. Raw `PacketEvent` access remains public for low-level users and unsupported packet semantics. This does not change command parsing semantics, permission policy, vanilla parity, broad Minecraft compatibility, public-server safety, or production-readiness claims.
