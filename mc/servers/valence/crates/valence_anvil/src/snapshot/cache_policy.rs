/// Policy for deciding whether snapshot cache state can be reused.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SnapshotCachePolicy {
    /// Reuse cached chunk data when timestamps prove it is fresh.
    #[default]
    ReuseFresh,
    /// Ignore cache state and reload available source data.
    ReloadAll,
}

/// Timestamp metadata for a cached snapshot chunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotCacheEntry {
    /// Cached chunk timestamp in seconds since the epoch.
    pub timestamp: u32,
}

/// Availability metadata for snapshot source data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotSourceState {
    /// Source chunk timestamp in seconds since the epoch, when known.
    pub timestamp: Option<u32>,
    /// True when source chunk bytes are available to the shell.
    pub available: bool,
}

/// Input to the pure snapshot cache planner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotCachePlanInput {
    /// Current cached metadata, if a cache entry exists.
    pub cached: Option<SnapshotCacheEntry>,
    /// Current source metadata supplied by the filesystem shell.
    pub source: SnapshotSourceState,
    /// Cache policy selected by the caller.
    pub policy: SnapshotCachePolicy,
}

/// Cache action selected before a shell reads or evicts data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotCacheAction {
    /// No cache entry exists and no source chunk is available.
    Miss,
    /// Reuse the cache entry without reading source data.
    ReuseCached,
    /// Load source data because no reusable cache entry exists.
    LoadSource,
    /// Evict stale cached data before loading source data.
    EvictStale,
}

/// Plans a snapshot cache action from explicit cache and source metadata.
pub fn plan_snapshot_cache(input: SnapshotCachePlanInput) -> SnapshotCacheAction {
    if !input.source.available {
        return plan_unavailable_source(input.cached, input.policy);
    }

    match input.policy {
        SnapshotCachePolicy::ReloadAll => SnapshotCacheAction::LoadSource,
        SnapshotCachePolicy::ReuseFresh => plan_reuse_fresh(input.cached, input.source),
    }
}

fn plan_unavailable_source(
    cached: Option<SnapshotCacheEntry>,
    policy: SnapshotCachePolicy,
) -> SnapshotCacheAction {
    match (cached, policy) {
        (Some(_), SnapshotCachePolicy::ReuseFresh) => SnapshotCacheAction::ReuseCached,
        (Some(_), SnapshotCachePolicy::ReloadAll) | (None, _) => SnapshotCacheAction::Miss,
    }
}

fn plan_reuse_fresh(
    cached: Option<SnapshotCacheEntry>,
    source: SnapshotSourceState,
) -> SnapshotCacheAction {
    let Some(cached) = cached else {
        return SnapshotCacheAction::LoadSource;
    };
    let Some(source_timestamp) = source.timestamp else {
        return SnapshotCacheAction::LoadSource;
    };
    if cached.timestamp >= source_timestamp {
        SnapshotCacheAction::ReuseCached
    } else {
        SnapshotCacheAction::EvictStale
    }
}
