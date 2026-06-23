# `valence_entity`

Components and systems concerning Minecraft entities. This includes "zombie", "chicken", "player", etc.

Bundles of components are used to spawn entities. Each entity type gets its own module here.

## Metadata diff tracking

Entity metadata is encoded through `TrackedData`. Generated metadata systems remain the source of truth for index and type IDs, while `TrackedData` owns packet-ready byte caches for spawn initialization and incremental updates.

The metadata policy is final-state-per-index within one update flush window: repeated updates for the same metadata index replace the earlier queued update and emit the final encoded value once. Initial spawn metadata suppresses default values by removing that index from the initialization cache; incremental updates still send an explicit default value when a visible client must be reset. Invalid metadata index `0xff` is reserved for the packet terminator and fails closed before mutating packet bytes.

Ordering remains explicit: spawn packets are written before initialization metadata, incremental metadata updates are written from the entity update path before status and animation packets, and `ClearEntityChangesSet` clears queued update bytes after the update shell has had a chance to write them. Despawn cleanup removes despawned entity IDs from `EntityManager` so stale metadata is not associated with a recycled protocol ID.

Non-claims: this policy does not add new entity types, does not claim full vanilla entity behavior parity, and does not claim Hyperion compatibility beyond the bounded metadata diff audit evidence.
