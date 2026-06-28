# Design: Stevenarella login functional core

## Context

The current login path branches over protocol-version packet variants and online/offline server modes directly inside `Server::connect`. A focused login core can normalize those packet variants into decisions while the connection shell continues to own actual packet reads/writes and cryptographic/session side effects.

## Decisions

### 1. Normalize login packet variants

**Choice:** Add pure helper types such as login events, login decisions, and login outcomes that abstract over string UUID, UUID, and UUID-with-properties success packets, compression packets, encryption requests, and disconnect packets.

**Rationale:** Protocol-version differences should be handled once, then downstream code can work with normalized login facts.

### 2. Keep cryptographic and network side effects in the shell

**Choice:** The core may decide that encryption is required or that login succeeded, but it does not generate random secrets, call Mojang session joins, encrypt buffers, read/write packets, spawn threads, or construct live connections.

**Rationale:** The login state machine becomes testable with in-memory events while side effects remain explicit.

### 3. Preserve milestone and error semantics

**Choice:** Existing milestone names, disconnect behavior, wrong-packet diagnostics, compression threshold propagation, and unsupported-version behavior remain stable unless a separate compatibility Cairn changes them.

**Rationale:** This is an architecture extraction, not a protocol behavior change.

### 4. Extract in two phases

**Choice:** First add normalized helper functions used by `Server::connect`, then move the helpers into a `server::login` module after parity tests pass.

**Rationale:** This reduces risk in a critical connection path and keeps the diff reviewable.

## Risks / Trade-offs

- Some legacy branches panic today for unsupported FML network versions; preserve behavior first, then improve diagnostics only under a separate Cairn.
- The shell must still coordinate read/write connection state after encryption; tests should cover the core decisions while component checks cover integration.
- Packet types live in generated protocol modules, so helper APIs should avoid leaking too many generated variants into unrelated modules.
