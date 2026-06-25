#![allow(clippy::type_complexity)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use bevy_tasks::{block_on, poll_once, AsyncComputeTaskPool, Task, TaskPool};
use noise::{NoiseFn, SuperSimplex};
use tracing::{debug, info, warn};
use valence::prelude::*;
use valence::spawn::IsFlat;

const SPAWN_X: f64 = 0.0;
const SPAWN_Y: f64 = 200.0;
const SPAWN_Z: f64 = 0.0;
const SPAWN_POS: DVec3 = DVec3::new(SPAWN_X, SPAWN_Y, SPAWN_Z);

const DEFAULT_CHUNK_HEIGHT: u32 = 384;
const MIN_CHUNK_HEIGHT: u32 = 2;
const CHUNK_WIDTH_BLOCKS: u32 = 16;
const CHUNK_WIDTH_BLOCKS_I32: i32 = 16;
const SECONDS_PER_DAY: u64 = 86_400;
const WATER_HEIGHT: i32 = 55;
const WATER_SURFACE_OFFSET: i32 = 1;
const BLOCK_BELOW_OFFSET: u32 = 1;
const TALL_GRASS_UPPER_OFFSET: u32 = 1;
const GRASS_DECORATION_MIN_Y: u32 = 1;

const HILLY_SEED_OFFSET: u32 = 1;
const STONE_SEED_OFFSET: u32 = 2;
const GRAVEL_SEED_OFFSET: u32 = 3;
const GRASS_SEED_OFFSET: u32 = 4;

const HILLY_MIN: f64 = 0.1;
const HILLY_MAX: f64 = 1.0;
const HILLY_SCALE: f64 = 400.0;
const HILLY_EXPONENT: i32 = 2;
const TERRAIN_LOWER_BASE: f64 = 15.0;
const TERRAIN_BAND_HEIGHT: f64 = 100.0;
const DENSITY_SCALE: f64 = 100.0;
const DENSITY_OCTAVES: u32 = 4;
const DENSITY_LACUNARITY: f64 = 2.0;
const DENSITY_PERSISTENCE: f64 = 0.5;

const GRAVEL_SCALE: f64 = 10.0;
const GRAVEL_OCTAVES: u32 = 3;
const GRAVEL_LACUNARITY: f64 = 2.0;
const GRAVEL_PERSISTENCE: f64 = 0.5;
const GRAVEL_VARIATION_BLOCKS: f64 = 6.0;

const STONE_DEPTH_SCALE: f64 = 15.0;
const STONE_DEPTH_BLOCKS: f64 = 5.0;

const GRASS_SCALE: f64 = 5.0;
const GRASS_OCTAVES: u32 = 4;
const GRASS_LACUNARITY: f64 = 2.0;
const GRASS_PERSISTENCE: f64 = 0.7;
const GRASS_DENSITY_THRESHOLD: f64 = 0.55;
const TALL_GRASS_DENSITY_THRESHOLD: f64 = 0.7;

const UNIT_INTERVAL_MAX: f64 = 1.0;
const NOISE_UNIT_OFFSET: f64 = 1.0;
const NOISE_UNIT_DIVISOR: f64 = 2.0;

#[derive(Resource)]
struct GameState {
    /// Chunks that need to be generated or are currently owned by a Bevy task.
    pending: HashMap<ChunkPos, PendingChunkRequest>,
    noise: Arc<ChunkGenerationNoise>,
    generation_settings: ChunkGenerationSettings,
}

enum PendingChunkRequest {
    Queued(Priority),
    Generating(ChunkGenerationTask),
}

struct ChunkGenerationTask {
    task: Task<ChunkGenerationResult>,
}

#[derive(Debug)]
struct GeneratedChunk {
    pos: ChunkPos,
    chunk: UnloadedChunk,
}

type ChunkGenerationResult = Result<GeneratedChunk, ChunkGenerationError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ChunkGenerationInput {
    pos: ChunkPos,
    settings: ChunkGenerationSettings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ChunkGenerationSettings {
    height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChunkGenerationError {
    InvalidHeight { height: u32 },
    HeightTooTall { height: u32 },
}

struct ChunkGenerationNoise {
    density: SuperSimplex,
    hilly: SuperSimplex,
    stone: SuperSimplex,
    gravel: SuperSimplex,
    grass: SuperSimplex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RequestSlot {
    Vacant,
    Queued(Priority),
    Generating,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RequestUpdate {
    InsertQueued(Priority),
    UpdateQueued(Priority),
    KeepGenerating,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IncompleteTaskDecision {
    KeepPending,
    Cancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CompletionContext {
    expected_pos: ChunkPos,
    requested_now: bool,
    already_loaded: bool,
    shutting_down: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChunkCompletionStatus {
    Generated { pos: ChunkPos },
    Failed(ChunkGenerationError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChunkCompletionDecision {
    Insert,
    IgnoreShutdown,
    IgnoreFailure,
    IgnoreMismatchedPosition,
    IgnoreStale,
    IgnoreDuplicate,
}

/// The order in which chunks should be processed by the task pool. Smaller
/// values are sent first.
type Priority = u64;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (
                    init_clients,
                    remove_unviewed_chunks,
                    update_client_views,
                    run_chunk_tasks,
                )
                    .chain(),
                despawn_disconnected_clients,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let days_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / SECONDS_PER_DAY;
    let seed = seed_from_days(days_since_epoch);

    info!("current seed: {seed}");

    // Terrain uses Bevy's asynchronous compute task pool so the example models
    // Bevy-shaped background work without giving the generator ECS access.
    terrain_task_pool();

    commands.insert_resource(GameState {
        pending: HashMap::new(),
        noise: Arc::new(ChunkGenerationNoise::from_seed(seed)),
        generation_settings: ChunkGenerationSettings {
            height: DEFAULT_CHUNK_HEIGHT,
        },
    });

    let layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &mut IsFlat,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut is_flat,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set(SPAWN_POS);
        *game_mode = GameMode::Creative;
        is_flat.0 = true;
    }
}

fn remove_unviewed_chunks(mut layers: Query<&mut ChunkLayer>) {
    layers
        .single_mut()
        .retain_chunks(|_, chunk| chunk.viewer_count_mut() > 0);
}

fn update_client_views(
    mut layers: Query<&mut ChunkLayer>,
    mut clients: Query<(&mut Client, View, OldView)>,
    mut state: ResMut<GameState>,
) {
    let layer = layers.single_mut();

    for (client, view, old_view) in &mut clients {
        let view = view.get();
        let queue_pos = |pos: ChunkPos| {
            if layer.chunk(pos).is_none() {
                let priority = view.pos.distance_squared(pos);
                queue_chunk_request(&mut state.pending, pos, priority);
            }
        };

        // Queue all the new chunks in the view to be sent to the Bevy task pool.
        if client.is_added() {
            view.iter().for_each(queue_pos);
        } else {
            let old_view = old_view.get();
            if old_view != view {
                view.diff(old_view).for_each(queue_pos);
            }
        }
    }
}

fn run_chunk_tasks(
    mut layers: Query<&mut ChunkLayer>,
    clients: Query<View>,
    mut state: ResMut<GameState>,
) {
    let mut layer = layers.single_mut();
    let state = state.as_mut();

    poll_completed_chunk_tasks(&mut layer, &clients, state);
    spawn_queued_chunk_tasks(&layer, &clients, state);
}

fn poll_completed_chunk_tasks(
    layer: &mut ChunkLayer,
    clients: &Query<View>,
    state: &mut GameState,
) {
    let mut completed = Vec::new();
    let mut cancelled = Vec::new();

    for (pos, request) in &mut state.pending {
        let PendingChunkRequest::Generating(task) = request else {
            continue;
        };

        let requested_now = is_requested_by_current_view(*pos, clients);
        let maybe_completed = block_on(poll_once(&mut task.task));

        if let Some(result) = maybe_completed {
            completed.push((*pos, requested_now, result));
        } else if decide_incomplete_task(requested_now) == IncompleteTaskDecision::Cancel {
            cancelled.push(*pos);
        }
    }

    for pos in cancelled {
        state.pending.remove(&pos);
        debug!(?pos, "cancelled unviewed terrain generation task");
    }

    for (expected_pos, requested_now, result) in completed {
        let already_loaded = layer.chunk(expected_pos).is_some();
        let context = CompletionContext {
            expected_pos,
            requested_now,
            already_loaded,
            shutting_down: false,
        };

        handle_completed_chunk(layer, context, result);
        state.pending.remove(&expected_pos);
    }
}

fn spawn_queued_chunk_tasks(layer: &ChunkLayer, clients: &Query<View>, state: &mut GameState) {
    let mut stale = Vec::new();
    let mut to_spawn = Vec::new();

    for (pos, request) in &state.pending {
        let PendingChunkRequest::Queued(priority) = request else {
            continue;
        };

        if !is_requested_by_current_view(*pos, clients) || layer.chunk(*pos).is_some() {
            stale.push(*pos);
        } else {
            to_spawn.push((*priority, *pos));
        }
    }

    for pos in stale {
        state.pending.remove(&pos);
        debug!(?pos, "removed stale queued terrain chunk request");
    }

    to_spawn.sort_unstable_by_key(|(priority, _)| *priority);

    for (_, pos) in to_spawn {
        let task = spawn_chunk_generation_task(pos, state.noise.clone(), state.generation_settings);
        let previous = state
            .pending
            .insert(pos, PendingChunkRequest::Generating(task));
        debug_assert!(matches!(previous, Some(PendingChunkRequest::Queued(_))));
    }
}

fn queue_chunk_request(
    pending: &mut HashMap<ChunkPos, PendingChunkRequest>,
    pos: ChunkPos,
    priority: Priority,
) {
    match pending.entry(pos) {
        Entry::Occupied(mut occupied) => {
            let slot = request_slot(occupied.get());
            match plan_request_update(slot, priority) {
                RequestUpdate::InsertQueued(_) => unreachable!("occupied entry cannot be vacant"),
                RequestUpdate::UpdateQueued(next_priority) => {
                    *occupied.get_mut() = PendingChunkRequest::Queued(next_priority);
                }
                RequestUpdate::KeepGenerating => {}
            }
        }
        Entry::Vacant(vacant) => {
            if let RequestUpdate::InsertQueued(priority) =
                plan_request_update(RequestSlot::Vacant, priority)
            {
                vacant.insert(PendingChunkRequest::Queued(priority));
            }
        }
    }
}

fn request_slot(request: &PendingChunkRequest) -> RequestSlot {
    match request {
        PendingChunkRequest::Queued(priority) => RequestSlot::Queued(*priority),
        PendingChunkRequest::Generating(_) => RequestSlot::Generating,
    }
}

fn plan_request_update(slot: RequestSlot, priority: Priority) -> RequestUpdate {
    match slot {
        RequestSlot::Vacant => RequestUpdate::InsertQueued(priority),
        RequestSlot::Queued(existing_priority) => {
            RequestUpdate::UpdateQueued(existing_priority.min(priority))
        }
        RequestSlot::Generating => RequestUpdate::KeepGenerating,
    }
}

fn decide_incomplete_task(requested_now: bool) -> IncompleteTaskDecision {
    if requested_now {
        IncompleteTaskDecision::KeepPending
    } else {
        IncompleteTaskDecision::Cancel
    }
}

fn handle_completed_chunk(
    layer: &mut ChunkLayer,
    context: CompletionContext,
    result: ChunkGenerationResult,
) {
    let status = completion_status(&result);

    match decide_completed_chunk(status, context) {
        ChunkCompletionDecision::Insert => {
            let generated = result.expect("insert decisions require generated chunks");
            layer.insert_chunk(generated.pos, generated.chunk);
        }
        ChunkCompletionDecision::IgnoreShutdown => {
            debug!(?context.expected_pos, "ignored terrain chunk during shutdown");
        }
        ChunkCompletionDecision::IgnoreFailure => {
            warn!(?context.expected_pos, ?status, "terrain chunk generation failed");
        }
        ChunkCompletionDecision::IgnoreMismatchedPosition => {
            warn!(?context.expected_pos, ?status, "terrain chunk task returned a mismatched position");
        }
        ChunkCompletionDecision::IgnoreStale => {
            debug!(?context.expected_pos, "ignored stale terrain chunk completion");
        }
        ChunkCompletionDecision::IgnoreDuplicate => {
            debug!(?context.expected_pos, "ignored duplicate terrain chunk completion");
        }
    }
}

fn completion_status(result: &ChunkGenerationResult) -> ChunkCompletionStatus {
    match result {
        Ok(generated) => ChunkCompletionStatus::Generated { pos: generated.pos },
        Err(error) => ChunkCompletionStatus::Failed(*error),
    }
}

fn decide_completed_chunk(
    status: ChunkCompletionStatus,
    context: CompletionContext,
) -> ChunkCompletionDecision {
    if context.shutting_down {
        return ChunkCompletionDecision::IgnoreShutdown;
    }

    let ChunkCompletionStatus::Generated { pos } = status else {
        return ChunkCompletionDecision::IgnoreFailure;
    };

    if pos != context.expected_pos {
        return ChunkCompletionDecision::IgnoreMismatchedPosition;
    }

    if !context.requested_now {
        return ChunkCompletionDecision::IgnoreStale;
    }

    if context.already_loaded {
        return ChunkCompletionDecision::IgnoreDuplicate;
    }

    ChunkCompletionDecision::Insert
}

fn is_requested_by_current_view(pos: ChunkPos, clients: &Query<View>) -> bool {
    clients.iter().any(|view| view.get().contains(pos))
}

fn spawn_chunk_generation_task(
    pos: ChunkPos,
    noise: Arc<ChunkGenerationNoise>,
    settings: ChunkGenerationSettings,
) -> ChunkGenerationTask {
    let input = ChunkGenerationInput { pos, settings };
    let task = terrain_task_pool().spawn(async move {
        generate_chunk(input, &noise).map(|chunk| GeneratedChunk { pos, chunk })
    });

    ChunkGenerationTask { task }
}

fn terrain_task_pool() -> &'static AsyncComputeTaskPool {
    AsyncComputeTaskPool::get_or_init(TaskPool::new)
}

fn seed_from_days(days_since_epoch: u64) -> u32 {
    u32::try_from(days_since_epoch).unwrap_or(u32::MAX)
}

impl ChunkGenerationNoise {
    fn from_seed(seed: u32) -> Self {
        Self {
            density: SuperSimplex::new(seed),
            hilly: SuperSimplex::new(seed.wrapping_add(HILLY_SEED_OFFSET)),
            stone: SuperSimplex::new(seed.wrapping_add(STONE_SEED_OFFSET)),
            gravel: SuperSimplex::new(seed.wrapping_add(GRAVEL_SEED_OFFSET)),
            grass: SuperSimplex::new(seed.wrapping_add(GRASS_SEED_OFFSET)),
        }
    }
}

fn generate_chunk(
    input: ChunkGenerationInput,
    noise: &ChunkGenerationNoise,
) -> Result<UnloadedChunk, ChunkGenerationError> {
    validate_generation_settings(input.settings)?;

    let height_i32 =
        i32::try_from(input.settings.height).map_err(|_| ChunkGenerationError::HeightTooTall {
            height: input.settings.height,
        })?;
    let mut chunk = UnloadedChunk::with_height(input.settings.height);

    for offset_z in 0..CHUNK_WIDTH_BLOCKS {
        for offset_x in 0..CHUNK_WIDTH_BLOCKS {
            let x = i32::try_from(offset_x).expect("chunk column offset fits i32")
                + input.pos.x * CHUNK_WIDTH_BLOCKS_I32;
            let z = i32::try_from(offset_z).expect("chunk column offset fits i32")
                + input.pos.z * CHUNK_WIDTH_BLOCKS_I32;

            fill_terrain_column(&mut chunk, offset_x, offset_z, x, z, height_i32, noise);
            decorate_grass_column(&mut chunk, offset_x, offset_z, x, z, noise);
        }
    }

    Ok(chunk)
}

fn validate_generation_settings(
    settings: ChunkGenerationSettings,
) -> Result<(), ChunkGenerationError> {
    if settings.height < MIN_CHUNK_HEIGHT {
        return Err(ChunkGenerationError::InvalidHeight {
            height: settings.height,
        });
    }

    Ok(())
}

fn fill_terrain_column(
    chunk: &mut UnloadedChunk,
    offset_x: u32,
    offset_z: u32,
    x: i32,
    z: i32,
    height_i32: i32,
    noise: &ChunkGenerationNoise,
) {
    let mut in_terrain = false;
    let mut depth = 0;

    for y in (0..height_i32).rev() {
        let p = DVec3::new(f64::from(x), f64::from(y), f64::from(z));
        let block = terrain_block_at(noise, p, y, &mut in_terrain, &mut depth);

        chunk.set_block_state(offset_x, y as u32, offset_z, block);
    }
}

fn terrain_block_at(
    noise: &ChunkGenerationNoise,
    p: DVec3,
    y: i32,
    in_terrain: &mut bool,
    depth: &mut u32,
) -> BlockState {
    if has_terrain_at(noise, p) {
        return solid_terrain_block_at(noise, p, y, in_terrain, depth);
    }

    *in_terrain = false;
    *depth = 0;

    if y < WATER_HEIGHT {
        BlockState::WATER
    } else {
        BlockState::AIR
    }
}

fn solid_terrain_block_at(
    noise: &ChunkGenerationNoise,
    p: DVec3,
    y: i32,
    in_terrain: &mut bool,
    depth: &mut u32,
) -> BlockState {
    let gravel_height = WATER_HEIGHT
        - WATER_SURFACE_OFFSET
        - (fbm(
            &noise.gravel,
            p / GRAVEL_SCALE,
            GRAVEL_OCTAVES,
            GRAVEL_LACUNARITY,
            GRAVEL_PERSISTENCE,
        ) * GRAVEL_VARIATION_BLOCKS)
            .floor() as i32;

    if *in_terrain {
        return buried_terrain_block(y, gravel_height, depth);
    }

    *in_terrain = true;
    let n = noise01(&noise.stone, p / STONE_DEPTH_SCALE);
    *depth = (n * STONE_DEPTH_BLOCKS).round() as u32;

    surface_terrain_block(y, gravel_height)
}

fn buried_terrain_block(y: i32, gravel_height: i32, depth: &mut u32) -> BlockState {
    if *depth == 0 {
        return BlockState::STONE;
    }

    *depth -= 1;

    if y < gravel_height {
        BlockState::GRAVEL
    } else {
        BlockState::DIRT
    }
}

fn surface_terrain_block(y: i32, gravel_height: i32) -> BlockState {
    if y < gravel_height {
        BlockState::GRAVEL
    } else if y < WATER_HEIGHT - WATER_SURFACE_OFFSET {
        BlockState::DIRT
    } else {
        BlockState::GRASS_BLOCK
    }
}

fn decorate_grass_column(
    chunk: &mut UnloadedChunk,
    offset_x: u32,
    offset_z: u32,
    x: i32,
    z: i32,
    noise: &ChunkGenerationNoise,
) {
    for y in (GRASS_DECORATION_MIN_Y..chunk.height().saturating_sub(TALL_GRASS_UPPER_OFFSET)).rev()
    {
        if !can_place_grass_decoration(chunk, offset_x, y, offset_z) {
            continue;
        }

        let p = DVec3::new(f64::from(x), f64::from(y), f64::from(z));
        let density = fbm(
            &noise.grass,
            p / GRASS_SCALE,
            GRASS_OCTAVES,
            GRASS_LACUNARITY,
            GRASS_PERSISTENCE,
        );

        if density > GRASS_DENSITY_THRESHOLD {
            place_grass_decoration(chunk, offset_x, y, offset_z, density);
        }
    }
}

fn can_place_grass_decoration(chunk: &UnloadedChunk, offset_x: u32, y: u32, offset_z: u32) -> bool {
    chunk.block_state(offset_x, y, offset_z).is_air()
        && chunk.block_state(offset_x, y - BLOCK_BELOW_OFFSET, offset_z) == BlockState::GRASS_BLOCK
}

fn place_grass_decoration(
    chunk: &mut UnloadedChunk,
    offset_x: u32,
    y: u32,
    offset_z: u32,
    density: f64,
) {
    let upper_y = y + TALL_GRASS_UPPER_OFFSET;

    if density > TALL_GRASS_DENSITY_THRESHOLD
        && chunk.block_state(offset_x, upper_y, offset_z).is_air()
    {
        let upper = BlockState::TALL_GRASS.set(PropName::Half, PropValue::Upper);
        let lower = BlockState::TALL_GRASS.set(PropName::Half, PropValue::Lower);

        chunk.set_block_state(offset_x, upper_y, offset_z, upper);
        chunk.set_block_state(offset_x, y, offset_z, lower);
    } else {
        chunk.set_block_state(offset_x, y, offset_z, BlockState::GRASS);
    }
}

fn has_terrain_at(noise: &ChunkGenerationNoise, p: DVec3) -> bool {
    let hilly =
        lerp(HILLY_MIN, HILLY_MAX, noise01(&noise.hilly, p / HILLY_SCALE)).powi(HILLY_EXPONENT);

    let lower = TERRAIN_LOWER_BASE + TERRAIN_BAND_HEIGHT * hilly;
    let upper = lower + TERRAIN_BAND_HEIGHT * hilly;

    if p.y <= lower {
        return true;
    } else if p.y >= upper {
        return false;
    }

    let density = UNIT_INTERVAL_MAX - lerpstep(lower, upper, p.y);

    let n = fbm(
        &noise.density,
        p / DENSITY_SCALE,
        DENSITY_OCTAVES,
        DENSITY_LACUNARITY,
        DENSITY_PERSISTENCE,
    );

    n < density
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (UNIT_INTERVAL_MAX - t) + b * t
}

fn lerpstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    if x <= edge0 {
        0.0
    } else if x >= edge1 {
        UNIT_INTERVAL_MAX
    } else {
        (x - edge0) / (edge1 - edge0)
    }
}

fn fbm(noise: &SuperSimplex, p: DVec3, octaves: u32, lacunarity: f64, persistence: f64) -> f64 {
    let mut freq = UNIT_INTERVAL_MAX;
    let mut amp = UNIT_INTERVAL_MAX;
    let mut amp_sum = 0.0;
    let mut sum = 0.0;

    for _ in 0..octaves {
        let n = noise01(noise, p * freq);
        sum += n * amp;
        amp_sum += amp;

        freq *= lacunarity;
        amp *= persistence;
    }

    // Scale the output to [0, 1]
    sum / amp_sum
}

fn noise01(noise: &SuperSimplex, p: DVec3) -> f64 {
    (noise.get(p.to_array()) + NOISE_UNIT_OFFSET) / NOISE_UNIT_DIVISOR
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEED: u32 = 12_345;
    const TEST_HEIGHT: u32 = 64;
    const INVALID_TEST_HEIGHT: u32 = 0;
    const TEST_POS: ChunkPos = ChunkPos::new(0, 0);
    const OTHER_TEST_POS: ChunkPos = ChunkPos::new(1, 0);
    const NEAR_PRIORITY: Priority = 4;
    const FAR_PRIORITY: Priority = 9;

    #[test]
    fn pure_generator_produces_chunk_for_valid_input() {
        let noise = ChunkGenerationNoise::from_seed(TEST_SEED);
        let input = ChunkGenerationInput {
            pos: TEST_POS,
            settings: ChunkGenerationSettings {
                height: TEST_HEIGHT,
            },
        };

        let chunk = generate_chunk(input, &noise).unwrap();

        assert_eq!(chunk.height(), TEST_HEIGHT);
    }

    #[test]
    fn completion_decision_inserts_requested_unloaded_chunk() {
        let context = CompletionContext {
            expected_pos: TEST_POS,
            requested_now: true,
            already_loaded: false,
            shutting_down: false,
        };

        let decision =
            decide_completed_chunk(ChunkCompletionStatus::Generated { pos: TEST_POS }, context);

        assert_eq!(decision, ChunkCompletionDecision::Insert);
    }

    #[test]
    fn duplicate_request_keeps_running_task_and_minimizes_queued_priority() {
        let queued = plan_request_update(RequestSlot::Queued(FAR_PRIORITY), NEAR_PRIORITY);
        let generating = plan_request_update(RequestSlot::Generating, NEAR_PRIORITY);

        assert_eq!(queued, RequestUpdate::UpdateQueued(NEAR_PRIORITY));
        assert_eq!(generating, RequestUpdate::KeepGenerating);
    }

    #[test]
    fn incomplete_unrequested_task_is_cancelled() {
        let requested = decide_incomplete_task(true);
        let stale = decide_incomplete_task(false);

        assert_eq!(requested, IncompleteTaskDecision::KeepPending);
        assert_eq!(stale, IncompleteTaskDecision::Cancel);
    }

    #[test]
    fn stale_completion_is_ignored() {
        let context = CompletionContext {
            expected_pos: TEST_POS,
            requested_now: false,
            already_loaded: false,
            shutting_down: false,
        };

        let decision =
            decide_completed_chunk(ChunkCompletionStatus::Generated { pos: TEST_POS }, context);

        assert_eq!(decision, ChunkCompletionDecision::IgnoreStale);
    }

    #[test]
    fn worker_failure_fails_closed() {
        let noise = ChunkGenerationNoise::from_seed(TEST_SEED);
        let input = ChunkGenerationInput {
            pos: TEST_POS,
            settings: ChunkGenerationSettings {
                height: INVALID_TEST_HEIGHT,
            },
        };

        let err = generate_chunk(input, &noise).unwrap_err();
        let context = CompletionContext {
            expected_pos: TEST_POS,
            requested_now: true,
            already_loaded: false,
            shutting_down: false,
        };
        let decision = decide_completed_chunk(ChunkCompletionStatus::Failed(err), context);

        assert_eq!(
            err,
            ChunkGenerationError::InvalidHeight {
                height: INVALID_TEST_HEIGHT
            }
        );
        assert_eq!(decision, ChunkCompletionDecision::IgnoreFailure);
    }

    #[test]
    fn shutdown_completion_is_ignored_before_world_mutation() {
        let context = CompletionContext {
            expected_pos: TEST_POS,
            requested_now: true,
            already_loaded: false,
            shutting_down: true,
        };

        let decision =
            decide_completed_chunk(ChunkCompletionStatus::Generated { pos: TEST_POS }, context);

        assert_eq!(decision, ChunkCompletionDecision::IgnoreShutdown);
    }

    #[test]
    fn mismatched_completion_position_fails_closed() {
        let context = CompletionContext {
            expected_pos: TEST_POS,
            requested_now: true,
            already_loaded: false,
            shutting_down: false,
        };

        let decision = decide_completed_chunk(
            ChunkCompletionStatus::Generated {
                pos: OTHER_TEST_POS,
            },
            context,
        );

        assert_eq!(decision, ChunkCompletionDecision::IgnoreMismatchedPosition);
    }
}
