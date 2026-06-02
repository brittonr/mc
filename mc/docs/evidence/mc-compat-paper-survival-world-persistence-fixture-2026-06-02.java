package mc.compat.paper;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.UUID;
import org.bukkit.GameMode;
import org.bukkit.Location;
import org.bukkit.Material;
import org.bukkit.World;
import org.bukkit.block.Block;
import org.bukkit.block.data.BlockData;
import org.bukkit.block.data.Lightable;
import org.bukkit.block.data.Powerable;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.block.BlockDamageEvent;
import org.bukkit.event.block.BlockPlaceEvent;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.event.inventory.InventoryOpenEvent;
import org.bukkit.event.inventory.InventoryType;
import org.bukkit.event.player.PlayerInteractEvent;
import org.bukkit.event.player.PlayerJoinEvent;
import org.bukkit.event.player.PlayerQuitEvent;
import org.bukkit.inventory.EquipmentSlot;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.java.JavaPlugin;

public final class SurvivalFixturePlugin extends JavaPlugin implements Listener {
    private static final int TARGET_X = 0;
    private static final int TARGET_Y = 64;
    private static final int TARGET_Z = 1;
    private static final int PLACE_Y = 65;
    private static final int FLOOR_Y = 64;
    private static final int FLOOR_Z = 0;
    private static final int HOTBAR_SLOT = 0;
    private static final int PROTOCOL_SLOT = 36;
    private static final int ITEM_COUNT = 1;
    private static final int ARENA_MIN_X = -1;
    private static final int ARENA_MAX_X = 1;
    private static final int ARENA_MIN_Y = TARGET_Y;
    private static final int ARENA_MAX_Y = PLACE_Y + 1;
    private static final int ARENA_MIN_Z = -1;
    private static final int ARENA_MAX_Z = 2;
    private static final long BREAK_DELAY_TICKS = 40L;
    private static final long PICKUP_DELAY_TICKS = 60L;
    private static final long PLACE_DELAY_TICKS = 100L;
    private static final double SPAWN_X = 0.5D;
    private static final double SPAWN_Y = 65.0D;
    private static final double SPAWN_Z = 0.5D;
    private static final float SPAWN_YAW = 0.0F;
    private static final float SPAWN_PITCH = 0.0F;
    private static final String CHEST_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_CHEST_FIXTURE";
    private static final String CRAFTING_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE";
    private static final int CHEST_X = 8;
    private static final int CHEST_Y = 64;
    private static final int CHEST_Z = 0;
    private static final int CHEST_SLOT = 0;
    private static final int CHEST_WINDOW = 1;
    private static final long CHEST_OPEN_DELAY_TICKS = 90L;
    private static final int CRAFTING_X = 4;
    private static final int CRAFTING_Y = 64;
    private static final int CRAFTING_Z = 0;
    private static final int CRAFTING_WINDOW = 1;
    private static final int CRAFTING_RESULT_SLOT = 0;
    private static final int CRAFTING_INPUT_A_SLOT = 1;
    private static final int CRAFTING_INPUT_B_SLOT = 4;
    private static final int CRAFTING_INVENTORY_SLOT = 36;
    private static final int CRAFTING_HOTBAR_SLOT = 0;
    private static final int CRAFTING_INPUT_COUNT = 1;
    private static final int CRAFTING_CURSOR_INPUT_COUNT = 2;
    private static final int CRAFTING_RESULT_COUNT = 4;
    private static final long CRAFTING_OPEN_DELAY_TICKS = 90L;
    private static final String CRAFTING_INPUT_NAME = "OakPlanks";
    private static final String CRAFTING_RESULT_NAME = "Stick";
    private static final String CRAFTING_RECIPE = "minecraft:stick";
    private static final String FURNACE_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_FURNACE_FIXTURE";
    private static final int FURNACE_X = 12;
    private static final int FURNACE_Y = 64;
    private static final int FURNACE_Z = 0;
    private static final int FURNACE_WINDOW = 1;
    private static final int FURNACE_INPUT_SLOT = 0;
    private static final int FURNACE_FUEL_SLOT = 1;
    private static final int FURNACE_OUTPUT_SLOT = 2;
    private static final int FURNACE_HOTBAR_SLOT = 0;
    private static final int FURNACE_INVENTORY_SLOT = 36;
    private static final int FURNACE_ITEM_COUNT = 1;
    private static final long FURNACE_OPEN_DELAY_TICKS = 90L;
    private static final String FURNACE_INPUT_NAME = "RawIron";
    private static final String FURNACE_FUEL_NAME = "Coal";
    private static final String FURNACE_OUTPUT_NAME = "IronIngot";
    private static final String HUNGER_FOOD_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_HUNGER_FOOD_FIXTURE";
    private static final int HUNGER_FOOD_HOTBAR_SLOT = 0;
    private static final int HUNGER_FOOD_PROTOCOL_SLOT = 36;
    private static final int HUNGER_FOOD_COUNT_BEFORE = 1;
    private static final int HUNGER_FOOD_COUNT_AFTER = 0;
    private static final double HUNGER_FOOD_PRE_HEALTH = 20.0D;
    private static final double HUNGER_FOOD_POST_HEALTH = 20.0D;
    private static final int HUNGER_FOOD_PRE_FOOD = 15;
    private static final int HUNGER_FOOD_POST_FOOD = 20;
    private static final float HUNGER_FOOD_PRE_SATURATION = 0.0F;
    private static final float HUNGER_FOOD_POST_SATURATION = 6.0F;
    private static final String HUNGER_FOOD_ITEM_NAME = "Bread";
    private static final String REDSTONE_TOGGLE_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_FIXTURE";
    private static final int REDSTONE_CONTROL_X = 20;
    private static final int REDSTONE_CONTROL_Y = 64;
    private static final int REDSTONE_CONTROL_Z = 0;
    private static final int REDSTONE_OUTPUT_X = 21;
    private static final int REDSTONE_OUTPUT_Y = 64;
    private static final int REDSTONE_OUTPUT_Z = 0;
    private static final int REDSTONE_FLOOR_Y = 63;
    private static final int REDSTONE_ARENA_MIN_X = 19;
    private static final int REDSTONE_ARENA_MAX_X = 23;
    private static final int REDSTONE_ARENA_MIN_Z = -2;
    private static final int REDSTONE_ARENA_MAX_Z = 2;
    private static final double REDSTONE_PLAYER_X = 20.5D;
    private static final double REDSTONE_PLAYER_Y = 65.0D;
    private static final double REDSTONE_PLAYER_Z = -1.5D;
    private static final float REDSTONE_PLAYER_YAW = 0.0F;
    private static final float REDSTONE_PLAYER_PITCH = 30.0F;
    private static final long REDSTONE_RETURN_DELAY_TICKS = 60L;
    private static final String REDSTONE_CONTROL_NAME = "Lever";
    private static final String REDSTONE_OUTPUT_NAME = "RedstoneLamp";
    private static final String WORLD_PERSISTENCE_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_FIXTURE";
    private static final String WORLD_PERSISTENCE_DIR_ENV = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_DIR";
    private static final int WORLD_PERSISTENCE_X = 24;
    private static final int WORLD_PERSISTENCE_Y = 64;
    private static final int WORLD_PERSISTENCE_Z = 0;
    private static final int WORLD_PERSISTENCE_BASE_Y = 63;
    private static final double WORLD_PERSISTENCE_PLAYER_X = 24.5D;
    private static final double WORLD_PERSISTENCE_PLAYER_Y = 65.0D;
    private static final double WORLD_PERSISTENCE_PLAYER_Z = -1.5D;
    private static final float WORLD_PERSISTENCE_PLAYER_YAW = 0.0F;
    private static final float WORLD_PERSISTENCE_PLAYER_PITCH = 30.0F;
    private static final String WORLD_PERSISTENCE_BLOCK_NAME = "Dirt";
    private static final String BIOME_DIMENSION_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE";
    private static final String OVERWORLD_ID = "minecraft:overworld";
    private static final String NETHER_ID = "minecraft:the_nether";
    private static final String END_ID = "minecraft:the_end";
    private static final String UNKNOWN_ENVIRONMENT_ID = "unknown";

    private final java.util.Set<UUID> breakSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> pickupSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> placeSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> chestOpenSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> chestStoreSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> chestCloseSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> chestReopenSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> chestPersistedSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> craftingOpenSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> craftingInputASeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> craftingInputBSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> craftingResultSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> craftingCollectSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceOpenSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceInputSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceFuelSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceBurnSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceOutputSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceCollectSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnacePostCollectQuitSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceReconnectJoinSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceReopenSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceStateSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> hungerFoodPreSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> hungerFoodConsumeStartSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> hungerFoodConsumeFinishSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> hungerFoodInventorySeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> hungerFoodStateSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> redstoneInputSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> redstoneOnSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> redstoneOffSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> redstoneStateSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> worldPersistenceMutationSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> worldPersistenceCleanSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> worldPersistenceRestartSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> worldPersistencePostSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> worldPersistenceStateSeen = new java.util.HashSet<>();

    @Override
    public void onEnable() {
        getServer().getPluginManager().registerEvents(this, this);
        getLogger().info("MC-COMPAT-MILESTONE survival_fixture_enabled");
    }

    @EventHandler
    public void onPlayerJoin(PlayerJoinEvent event) {
        Player player = event.getPlayer();
        if (furnaceFixtureEnabled() && furnacePostCollectQuitSeen.contains(player.getUniqueId())) {
            furnaceReconnectJoinSeen.add(player.getUniqueId());
        }
        getServer().getScheduler().runTask(this, () -> setupPlayer(player));
    }

    @EventHandler
    public void onPlayerQuit(PlayerQuitEvent event) {
        Player player = event.getPlayer();
        if (furnaceFixtureEnabled() && furnaceCollectSeen.contains(player.getUniqueId())) {
            furnacePostCollectQuitSeen.add(player.getUniqueId());
        }
    }

    @EventHandler(ignoreCancelled = false)
    public void onBlockDamage(BlockDamageEvent event) {
        Player player = event.getPlayer();
        if (!isTarget(event.getBlock())) {
            return;
        }
        if (!breakSeen.add(player.getUniqueId())) {
            return;
        }
        event.setCancelled(true);
        sendBreakUpdate(player);
        givePickup(player);
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_block_break username=" + player.getName()
                + " item=Dirt at=" + TARGET_X + "," + TARGET_Y + "," + TARGET_Z
        );
    }

    @EventHandler(ignoreCancelled = false)
    public void onBlockPlace(BlockPlaceEvent event) {
        Player player = event.getPlayer();
        Block block = event.getBlockPlaced();
        if (worldPersistenceFixtureEnabled() && isWorldPersistenceTarget(block)) {
            event.setCancelled(false);
            getServer().getScheduler().runTask(this, () -> logWorldPersistenceMutation(player));
            return;
        }
        if (block.getX() != TARGET_X || block.getY() != PLACE_Y || block.getZ() != TARGET_Z) {
            return;
        }
        if (!placeSeen.add(player.getUniqueId())) {
            return;
        }
        sendPlaceUpdate(player);
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_block_place username=" + player.getName()
                + " item=Dirt from_slot=" + PROTOCOL_SLOT
                + " at=" + TARGET_X + "," + PLACE_Y + "," + TARGET_Z
        );
    }

    @EventHandler(ignoreCancelled = false)
    public void onInventoryOpen(InventoryOpenEvent event) {
        if (!(event.getPlayer() instanceof Player player)) {
            return;
        }
        if (chestFixtureEnabled() && isChestInventory(event.getInventory())) {
            logChestOpen(player, event.getInventory());
        }
        if (craftingFixtureEnabled() && isCraftingInventory(event.getInventory())) {
            logCraftingOpen(player);
        }
        if (furnaceFixtureEnabled() && isFurnaceInventory(event.getInventory())) {
            logFurnaceOpen(player, event.getInventory());
        }
    }

    @EventHandler(ignoreCancelled = false)
    public void onInventoryClick(InventoryClickEvent event) {
        if (!(event.getWhoClicked() instanceof Player player)) {
            return;
        }
        if (chestFixtureEnabled() && isChestInventory(event.getInventory()) && event.getRawSlot() == CHEST_SLOT) {
            getServer().getScheduler().runTask(this, () -> storeChestClick(player, event.getInventory()));
        }
        if (craftingFixtureEnabled() && isCraftingInventory(event.getInventory())) {
            getServer().getScheduler().runTask(this, () -> storeCraftingClick(player, event.getInventory(), event.getRawSlot()));
        }
        if (furnaceFixtureEnabled() && isFurnaceInventory(event.getInventory())) {
            getServer().getScheduler().runTask(this, () -> storeFurnaceClick(player, event.getInventory(), event.getRawSlot()));
        }
    }

    @EventHandler(ignoreCancelled = false)
    public void onPlayerInteract(PlayerInteractEvent event) {
        if (event.getHand() != EquipmentSlot.HAND) {
            return;
        }
        if (redstoneToggleFixtureEnabled() && isRedstoneControl(event.getClickedBlock())) {
            Player player = event.getPlayer();
            event.setCancelled(true);
            getServer().getScheduler().runTask(this, () -> toggleRedstone(player));
            return;
        }
        if (!hungerFoodFixtureEnabled()) {
            return;
        }
        Player player = event.getPlayer();
        ItemStack stack = player.getInventory().getItem(HUNGER_FOOD_HOTBAR_SLOT);
        if (!shouldConsumeHungerFood(player, stack)) {
            return;
        }
        event.setCancelled(true);
        getServer().getScheduler().runTask(this, () -> consumeHungerFood(player));
    }

    @EventHandler(ignoreCancelled = false)
    public void onInventoryClose(InventoryCloseEvent event) {
        if (!chestFixtureEnabled() || !(event.getPlayer() instanceof Player player)) {
            return;
        }
        if (!isChestInventory(event.getInventory())) {
            return;
        }
        if (chestStoreSeen.contains(player.getUniqueId()) && chestCloseSeen.add(player.getUniqueId())) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_chest_close username=" + player.getName()
                    + " window=" + CHEST_WINDOW
            );
        }
    }

    private void setupPlayer(Player player) {
        World world = player.getWorld();
        clearArena(world);
        Block floor = world.getBlockAt(TARGET_X, FLOOR_Y, FLOOR_Z);
        floor.setType(Material.STONE, false);
        Block support = world.getBlockAt(TARGET_X, TARGET_Y, TARGET_Z);
        support.setType(Material.DIRT, false);
        Block placeTarget = world.getBlockAt(TARGET_X, PLACE_Y, TARGET_Z);
        placeTarget.setType(Material.AIR, false);
        if (chestFixtureEnabled()) {
            setupChestFixture(player, world);
        }
        if (craftingFixtureEnabled()) {
            setupCraftingFixture(world);
        }
        if (furnaceFixtureEnabled()) {
            setupFurnaceFixture(world);
        }
        if (redstoneToggleFixtureEnabled()) {
            setupRedstoneToggleFixture(world);
        }
        if (worldPersistenceFixtureEnabled()) {
            setupWorldPersistenceFixture(world);
        }
        player.getInventory().clear();
        player.setGameMode(GameMode.SURVIVAL);
        if (chestFixtureEnabled()) {
            player.setItemOnCursor(new ItemStack(Material.DIRT, ITEM_COUNT));
        }
        if (craftingFixtureEnabled()) {
            player.setItemOnCursor(new ItemStack(Material.OAK_PLANKS, CRAFTING_CURSOR_INPUT_COUNT));
        }
        if (furnaceFixtureEnabled()) {
            player.setItemOnCursor(new ItemStack(Material.RAW_IRON, FURNACE_ITEM_COUNT));
        }
        if (hungerFoodFixtureEnabled()) {
            setupHungerFoodFixture(player);
        }
        if (redstoneToggleFixtureEnabled()) {
            player.teleport(new Location(world, REDSTONE_PLAYER_X, REDSTONE_PLAYER_Y, REDSTONE_PLAYER_Z, REDSTONE_PLAYER_YAW, REDSTONE_PLAYER_PITCH));
        } else if (worldPersistenceFixtureEnabled()) {
            player.getInventory().setItem(HOTBAR_SLOT, new ItemStack(Material.DIRT, ITEM_COUNT));
            player.teleport(new Location(world, WORLD_PERSISTENCE_PLAYER_X, WORLD_PERSISTENCE_PLAYER_Y, WORLD_PERSISTENCE_PLAYER_Z, WORLD_PERSISTENCE_PLAYER_YAW, WORLD_PERSISTENCE_PLAYER_PITCH));
        } else {
            player.teleport(new Location(world, SPAWN_X, SPAWN_Y, SPAWN_Z, SPAWN_YAW, SPAWN_PITCH));
        }
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_join username=" + player.getName()
                + " gamemode=Survival target=" + TARGET_X + "," + TARGET_Y + "," + TARGET_Z
        );
        if (biomeDimensionFixtureEnabled()) {
            logSurvivalBiomeDimensionState(player.getName(), OVERWORLD_ID, OVERWORLD_ID);
        }
        if (chestFixtureEnabled()) {
            scheduleChestOpen(player);
        }
        if (craftingFixtureEnabled()) {
            scheduleCraftingOpen(player);
        }
        if (furnaceFixtureEnabled()) {
            scheduleFurnaceOpen(player);
        }
        if (worldPersistenceFixtureEnabled()) {
            logWorldPersistencePostRestart(player);
        }
        scheduleFixtureMilestones(player);
    }

    private void setupHungerFoodFixture(Player player) {
        UUID playerId = player.getUniqueId();
        player.setHealth(HUNGER_FOOD_PRE_HEALTH);
        player.setFoodLevel(HUNGER_FOOD_PRE_FOOD);
        player.setSaturation(HUNGER_FOOD_PRE_SATURATION);
        player.getInventory().setItem(HUNGER_FOOD_HOTBAR_SLOT, new ItemStack(Material.BREAD, HUNGER_FOOD_COUNT_BEFORE));
        player.updateInventory();
        if (hungerFoodPreSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_hunger_food_pre username=" + player.getName()
                    + " health=" + formatOneDecimal(HUNGER_FOOD_PRE_HEALTH)
                    + " food=" + HUNGER_FOOD_PRE_FOOD
                    + " saturation=" + formatOneDecimal(HUNGER_FOOD_PRE_SATURATION)
                    + " item=" + HUNGER_FOOD_ITEM_NAME
                    + " count=" + HUNGER_FOOD_COUNT_BEFORE
                    + " slot=" + HUNGER_FOOD_PROTOCOL_SLOT
            );
        }
    }

    private void setupChestFixture(Player player, World world) {
        Block chest = world.getBlockAt(CHEST_X, CHEST_Y, CHEST_Z);
        chest.setType(Material.CHEST, false);
        if (!chestStoreSeen.contains(player.getUniqueId())) {
            Inventory inventory = ((org.bukkit.block.Chest) chest.getState()).getBlockInventory();
            inventory.setItem(CHEST_SLOT, null);
        }
    }

    private void setupCraftingFixture(World world) {
        Block table = world.getBlockAt(CRAFTING_X, CRAFTING_Y, CRAFTING_Z);
        table.setType(Material.CRAFTING_TABLE, false);
    }

    private void setupFurnaceFixture(World world) {
        Block furnace = world.getBlockAt(FURNACE_X, FURNACE_Y, FURNACE_Z);
        if (furnace.getType() != Material.FURNACE) {
            furnace.setType(Material.FURNACE, false);
        }
    }

    private void setupRedstoneToggleFixture(World world) {
        for (int x = REDSTONE_ARENA_MIN_X; x < REDSTONE_ARENA_MAX_X; x++) {
            for (int z = REDSTONE_ARENA_MIN_Z; z < REDSTONE_ARENA_MAX_Z; z++) {
                world.getBlockAt(x, REDSTONE_FLOOR_Y, z).setType(Material.STONE, false);
                world.getBlockAt(x, REDSTONE_CONTROL_Y, z).setType(Material.AIR, false);
            }
        }
        setLever(world, false);
        setLamp(world, false);
    }

    private void setLever(World world, boolean powered) {
        BlockData data = Material.LEVER.createBlockData();
        if (data instanceof Powerable powerable) {
            powerable.setPowered(powered);
        }
        world.getBlockAt(REDSTONE_CONTROL_X, REDSTONE_CONTROL_Y, REDSTONE_CONTROL_Z)
            .setBlockData(data, false);
    }

    private BlockData redstoneLampData(boolean powered) {
        BlockData data = Material.REDSTONE_LAMP.createBlockData();
        if (data instanceof Lightable lightable) {
            lightable.setLit(powered);
        }
        return data;
    }

    private void setLamp(World world, boolean powered) {
        world.getBlockAt(REDSTONE_OUTPUT_X, REDSTONE_OUTPUT_Y, REDSTONE_OUTPUT_Z)
            .setBlockData(redstoneLampData(powered), false);
    }

    private void sendLampChange(Player player, boolean powered) {
        player.sendBlockChange(
            new Location(player.getWorld(), REDSTONE_OUTPUT_X, REDSTONE_OUTPUT_Y, REDSTONE_OUTPUT_Z),
            redstoneLampData(powered)
        );
    }

    private boolean isRedstoneControl(Block block) {
        return block != null
            && block.getX() == REDSTONE_CONTROL_X
            && block.getY() == REDSTONE_CONTROL_Y
            && block.getZ() == REDSTONE_CONTROL_Z;
    }

    private void toggleRedstone(Player player) {
        UUID playerId = player.getUniqueId();
        World world = player.getWorld();
        if (redstoneInputSeen.add(playerId)) {
            setLever(world, true);
            setLamp(world, true);
            sendLampChange(player, true);
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_redstone_toggle_input username=" + player.getName()
                    + " control=" + REDSTONE_CONTROL_NAME
                    + " position=" + REDSTONE_CONTROL_X + "," + REDSTONE_CONTROL_Y + "," + REDSTONE_CONTROL_Z
                    + " powered_before=false powered_after=true"
            );
            if (redstoneOnSeen.add(playerId)) {
                getLogger().info(
                    "MC-COMPAT-MILESTONE survival_redstone_toggle_powered_on username=" + player.getName()
                        + " output=" + REDSTONE_OUTPUT_NAME
                        + " position=" + REDSTONE_OUTPUT_X + "," + REDSTONE_OUTPUT_Y + "," + REDSTONE_OUTPUT_Z
                        + " powered=true"
                );
            }
            getServer().getScheduler().runTaskLater(this, () -> powerOffRedstone(player), REDSTONE_RETURN_DELAY_TICKS);
            return;
        }
        powerOffRedstone(player);
    }

    private void powerOffRedstone(Player player) {
        UUID playerId = player.getUniqueId();
        World world = player.getWorld();
        setLever(world, false);
        setLamp(world, false);
        sendLampChange(player, false);
        if (redstoneOffSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_redstone_toggle_powered_off username=" + player.getName()
                    + " output=" + REDSTONE_OUTPUT_NAME
                    + " position=" + REDSTONE_OUTPUT_X + "," + REDSTONE_OUTPUT_Y + "," + REDSTONE_OUTPUT_Z
                    + " powered=false"
            );
        }
        if (redstoneOnSeen.contains(playerId) && redstoneOffSeen.contains(playerId) && redstoneStateSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_redstone_toggle_state username=" + player.getName()
                    + " control=" + REDSTONE_CONTROL_NAME
                    + " output=" + REDSTONE_OUTPUT_NAME
                    + " on_seen=true off_seen=true unintended_outputs=false"
            );
        }
    }

    private Path worldPersistenceMarkerPath() {
        String configuredDir = System.getenv(WORLD_PERSISTENCE_DIR_ENV);
        Path dir = configuredDir == null || configuredDir.isBlank()
            ? getDataFolder().toPath().resolve("world-persistence")
            : Path.of(configuredDir);
        return dir.resolve("persisted-dirt.marker");
    }

    private boolean worldPersistenceMarkerExists() {
        return Files.exists(worldPersistenceMarkerPath());
    }

    private void writeWorldPersistenceMarker() {
        Path marker = worldPersistenceMarkerPath();
        try {
            Files.createDirectories(marker.getParent());
            Files.writeString(marker, WORLD_PERSISTENCE_BLOCK_NAME + "\n");
        } catch (Exception error) {
            getLogger().warning("MC-COMPAT-NONFATAL survival_world_persistence_marker_write_failed error=" + error);
        }
    }

    private void setupWorldPersistenceFixture(World world) {
        world.getBlockAt(WORLD_PERSISTENCE_X, WORLD_PERSISTENCE_BASE_Y, WORLD_PERSISTENCE_Z).setType(Material.STONE, false);
        Block target = world.getBlockAt(WORLD_PERSISTENCE_X, WORLD_PERSISTENCE_Y, WORLD_PERSISTENCE_Z);
        if (worldPersistenceMarkerExists()) {
            target.setType(Material.DIRT, false);
        } else if (target.getType() != Material.DIRT) {
            target.setType(Material.AIR, false);
        }
    }

    private boolean isWorldPersistenceTarget(Block block) {
        return block != null
            && block.getX() == WORLD_PERSISTENCE_X
            && block.getY() == WORLD_PERSISTENCE_Y
            && block.getZ() == WORLD_PERSISTENCE_Z;
    }

    private boolean worldPersistenceBlockPersisted(World world) {
        return worldPersistenceMarkerExists()
            || world.getBlockAt(WORLD_PERSISTENCE_X, WORLD_PERSISTENCE_Y, WORLD_PERSISTENCE_Z).getType() == Material.DIRT;
    }

    private void logWorldPersistenceMutation(Player player) {
        writeWorldPersistenceMarker();
        UUID playerId = player.getUniqueId();
        if (worldPersistenceMutationSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_world_persistence_mutation username=" + player.getName()
                    + " block=" + WORLD_PERSISTENCE_BLOCK_NAME
                    + " position=" + WORLD_PERSISTENCE_X + "," + WORLD_PERSISTENCE_Y + "," + WORLD_PERSISTENCE_Z
                    + " persisted_before=false persisted_after=true"
            );
        }
        if (worldPersistenceCleanSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_world_persistence_clean_shutdown username=" + player.getName()
                    + " storage=isolated marker=written"
            );
        }
    }

    private void logWorldPersistencePostRestart(Player player) {
        if (!worldPersistenceBlockPersisted(player.getWorld())) {
            return;
        }
        UUID playerId = player.getUniqueId();
        if (worldPersistenceRestartSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_world_persistence_backend_restart username=" + player.getName()
                    + " method=controlled_reload storage=isolated"
            );
        }
        if (worldPersistencePostSeen.add(playerId)) {
            player.sendBlockChange(
                new Location(player.getWorld(), WORLD_PERSISTENCE_X, WORLD_PERSISTENCE_Y, WORLD_PERSISTENCE_Z),
                Material.DIRT.createBlockData()
            );
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_world_persistence_post_restart_observe username=" + player.getName()
                    + " block=" + WORLD_PERSISTENCE_BLOCK_NAME
                    + " position=" + WORLD_PERSISTENCE_X + "," + WORLD_PERSISTENCE_Y + "," + WORLD_PERSISTENCE_Z
                    + " persisted=true"
            );
        }
        if (worldPersistenceStateSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_world_persistence_state username=" + player.getName()
                    + " block=" + WORLD_PERSISTENCE_BLOCK_NAME
                    + " position=" + WORLD_PERSISTENCE_X + "," + WORLD_PERSISTENCE_Y + "," + WORLD_PERSISTENCE_Z
                    + " pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false"
            );
        }
    }

    private boolean chestFixtureEnabled() {
        return "1".equals(System.getenv(CHEST_FIXTURE_ENV));
    }

    private boolean craftingFixtureEnabled() {
        return "1".equals(System.getenv(CRAFTING_FIXTURE_ENV));
    }

    private boolean furnaceFixtureEnabled() {
        return "1".equals(System.getenv(FURNACE_FIXTURE_ENV));
    }

    private boolean hungerFoodFixtureEnabled() {
        return "1".equals(System.getenv(HUNGER_FOOD_FIXTURE_ENV));
    }

    private boolean redstoneToggleFixtureEnabled() {
        return "1".equals(System.getenv(REDSTONE_TOGGLE_FIXTURE_ENV));
    }

    private boolean worldPersistenceFixtureEnabled() {
        return "1".equals(System.getenv(WORLD_PERSISTENCE_FIXTURE_ENV));
    }

    private boolean biomeDimensionFixtureEnabled() {
        return "1".equals(System.getenv(BIOME_DIMENSION_FIXTURE_ENV));
    }

    private String normalizeSurvivalEnvironmentId(String raw) {
        if (OVERWORLD_ID.equals(raw)) {
            return OVERWORLD_ID;
        }
        if (NETHER_ID.equals(raw)) {
            return NETHER_ID;
        }
        if (END_ID.equals(raw)) {
            return END_ID;
        }
        return UNKNOWN_ENVIRONMENT_ID;
    }

    private String deriveSurvivalEnvironmentId(String spawnEnvironment, String environmentIdentifier) {
        String environment = normalizeSurvivalEnvironmentId(environmentIdentifier);
        if (!UNKNOWN_ENVIRONMENT_ID.equals(environment)) {
            return environment;
        }
        return normalizeSurvivalEnvironmentId(spawnEnvironment);
    }

    private void logSurvivalBiomeDimensionState(String username, String spawnEnvironment, String environmentIdentifier) {
        String normalizedIdentifier = deriveSurvivalEnvironmentId(spawnEnvironment, environmentIdentifier);
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_biome_dimension_state username=" + username
                + " spawn_environment=" + spawnEnvironment
                + " environment_identifier=" + environmentIdentifier
                + " server_environment_state=" + spawnEnvironment
                + " normalized_identifier=" + normalizedIdentifier
        );
    }

    private void scheduleChestOpen(Player player) {
        getServer().getScheduler().runTaskLater(this, () -> openChestForProbe(player), CHEST_OPEN_DELAY_TICKS);
    }

    private void openChestForProbe(Player player) {
        if (!player.isOnline()) {
            return;
        }
        Block chest = player.getWorld().getBlockAt(CHEST_X, CHEST_Y, CHEST_Z);
        if (chest.getType() != Material.CHEST) {
            return;
        }
        Inventory inventory = ((org.bukkit.block.Chest) chest.getState()).getBlockInventory();
        player.openInventory(inventory);
    }

    private void scheduleCraftingOpen(Player player) {
        getServer().getScheduler().runTaskLater(this, () -> openCraftingForProbe(player), CRAFTING_OPEN_DELAY_TICKS);
    }

    private void openCraftingForProbe(Player player) {
        if (!player.isOnline()) {
            return;
        }
        Block table = player.getWorld().getBlockAt(CRAFTING_X, CRAFTING_Y, CRAFTING_Z);
        if (table.getType() != Material.CRAFTING_TABLE) {
            return;
        }
        player.openWorkbench(table.getLocation(), true);
    }

    private void scheduleFurnaceOpen(Player player) {
        getServer().getScheduler().runTaskLater(this, () -> openFurnaceForProbe(player), FURNACE_OPEN_DELAY_TICKS);
    }

    private void openFurnaceForProbe(Player player) {
        if (!player.isOnline()) {
            return;
        }
        Block furnace = player.getWorld().getBlockAt(FURNACE_X, FURNACE_Y, FURNACE_Z);
        if (furnace.getType() != Material.FURNACE) {
            return;
        }
        player.openInventory(((org.bukkit.block.Furnace) furnace.getState()).getInventory());
    }

    private boolean isChestInventory(Inventory inventory) {
        Location location = inventory.getLocation();
        return location != null
            && location.getBlockX() == CHEST_X
            && location.getBlockY() == CHEST_Y
            && location.getBlockZ() == CHEST_Z;
    }

    private boolean isCraftingInventory(Inventory inventory) {
        return inventory.getType() == InventoryType.WORKBENCH;
    }

    private boolean isFurnaceInventory(Inventory inventory) {
        Location location = inventory.getLocation();
        return inventory.getType() == InventoryType.FURNACE
            && location != null
            && location.getBlockX() == FURNACE_X
            && location.getBlockY() == FURNACE_Y
            && location.getBlockZ() == FURNACE_Z;
    }

    private boolean isExpectedChestItem(ItemStack item) {
        return item != null && item.getType() == Material.DIRT && item.getAmount() == ITEM_COUNT;
    }

    private boolean isExpectedCraftingInput(ItemStack item) {
        return item != null && item.getType() == Material.OAK_PLANKS && item.getAmount() == CRAFTING_INPUT_COUNT;
    }

    private boolean isExpectedCraftingResult(ItemStack item) {
        return item != null && item.getType() == Material.STICK && item.getAmount() == CRAFTING_RESULT_COUNT;
    }

    private boolean isExpectedFurnaceInput(ItemStack item) {
        return item != null && item.getType() == Material.RAW_IRON && item.getAmount() == FURNACE_ITEM_COUNT;
    }

    private boolean isExpectedFurnaceFuel(ItemStack item) {
        return item != null && item.getType() == Material.COAL && item.getAmount() == FURNACE_ITEM_COUNT;
    }

    private boolean isExpectedHungerFood(ItemStack item) {
        return item != null && item.getType() == Material.BREAD && item.getAmount() == HUNGER_FOOD_COUNT_BEFORE;
    }

    private boolean shouldConsumeHungerFood(Player player, ItemStack item) {
        return isExpectedHungerFood(item)
            && player.getHealth() == HUNGER_FOOD_PRE_HEALTH
            && player.getFoodLevel() == HUNGER_FOOD_PRE_FOOD
            && player.getSaturation() == HUNGER_FOOD_PRE_SATURATION;
    }

    private boolean isEmptyFurnaceOutput(ItemStack item) {
        return item == null || item.getType() == Material.AIR || item.getAmount() == 0;
    }

    private void logChestOpen(Player player, Inventory inventory) {
        UUID playerId = player.getUniqueId();
        if (chestStoreSeen.contains(playerId)) {
            if (chestReopenSeen.add(playerId)) {
                getLogger().info(
                    "MC-COMPAT-MILESTONE survival_chest_reopen username=" + player.getName()
                        + " position=" + CHEST_X + "," + CHEST_Y + "," + CHEST_Z
                        + " window=" + CHEST_WINDOW
                );
            }
            emitPersistedIfPresent(player, inventory);
            return;
        }
        if (chestOpenSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_chest_open username=" + player.getName()
                    + " position=" + CHEST_X + "," + CHEST_Y + "," + CHEST_Z
                    + " window=" + CHEST_WINDOW
            );
        }
    }

    private void logCraftingOpen(Player player) {
        if (craftingOpenSeen.add(player.getUniqueId())) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_crafting_table_open username=" + player.getName()
                    + " position=" + CRAFTING_X + "," + CRAFTING_Y + "," + CRAFTING_Z
                    + " window=" + CRAFTING_WINDOW
            );
        }
    }

    private void logFurnaceOpen(Player player, Inventory inventory) {
        UUID playerId = player.getUniqueId();
        if (furnaceCollectSeen.contains(playerId)) {
            if (!furnaceReconnectJoinSeen.contains(playerId)) {
                return;
            }
            if (furnaceReopenSeen.add(playerId)) {
                getLogger().info(
                    "MC-COMPAT-MILESTONE survival_furnace_reconnect_reopen username=" + player.getName()
                        + " position=" + FURNACE_X + "," + FURNACE_Y + "," + FURNACE_Z
                        + " window=" + FURNACE_WINDOW
                );
            }
            normalizeFurnaceStateForProbe(inventory);
            emitFurnaceStateIfReady(player, inventory);
            return;
        }
        if (furnaceOpenSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_furnace_open username=" + player.getName()
                    + " position=" + FURNACE_X + "," + FURNACE_Y + "," + FURNACE_Z
                    + " window=" + FURNACE_WINDOW
            );
        }
    }

    private void storeChestClick(Player player, Inventory inventory) {
        if (!chestStoreSeen.add(player.getUniqueId())) {
            return;
        }
        inventory.setItem(CHEST_SLOT, new ItemStack(Material.DIRT, ITEM_COUNT));
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_chest_store username=" + player.getName()
                + " window=" + CHEST_WINDOW
                + " slot=" + CHEST_SLOT
                + " item=Dirt count=" + ITEM_COUNT
        );
    }

    private void storeCraftingClick(Player player, Inventory inventory, int rawSlot) {
        if (rawSlot == CRAFTING_INPUT_A_SLOT && craftingInputASeen.add(player.getUniqueId())) {
            inventory.setItem(CRAFTING_INPUT_A_SLOT, new ItemStack(Material.OAK_PLANKS, CRAFTING_INPUT_COUNT));
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_crafting_input_a username=" + player.getName()
                    + " window=" + CRAFTING_WINDOW
                    + " slot=" + CRAFTING_INPUT_A_SLOT
                    + " item=" + CRAFTING_INPUT_NAME
                    + " count=" + CRAFTING_INPUT_COUNT
            );
        }
        if (rawSlot == CRAFTING_INPUT_B_SLOT && craftingInputBSeen.add(player.getUniqueId())) {
            inventory.setItem(CRAFTING_INPUT_B_SLOT, new ItemStack(Material.OAK_PLANKS, CRAFTING_INPUT_COUNT));
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_crafting_input_b username=" + player.getName()
                    + " window=" + CRAFTING_WINDOW
                    + " slot=" + CRAFTING_INPUT_B_SLOT
                    + " item=" + CRAFTING_INPUT_NAME
                    + " count=" + CRAFTING_INPUT_COUNT
            );
        }
        emitCraftingResultIfReady(player, inventory);
        if (rawSlot == CRAFTING_RESULT_SLOT && craftingResultSeen.contains(player.getUniqueId())
            && craftingCollectSeen.add(player.getUniqueId())) {
            inventory.setItem(CRAFTING_RESULT_SLOT, null);
            player.getInventory().setItem(CRAFTING_HOTBAR_SLOT, new ItemStack(Material.STICK, CRAFTING_RESULT_COUNT));
            player.updateInventory();
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_crafting_collect username=" + player.getName()
                    + " window=" + CRAFTING_WINDOW
                    + " slot=" + CRAFTING_RESULT_SLOT
                    + " item=" + CRAFTING_RESULT_NAME
                    + " count=" + CRAFTING_RESULT_COUNT
                    + " inventory_slot=" + CRAFTING_INVENTORY_SLOT
            );
        }
    }

    private void emitCraftingResultIfReady(Player player, Inventory inventory) {
        UUID playerId = player.getUniqueId();
        if (!craftingInputASeen.contains(playerId) || !craftingInputBSeen.contains(playerId)
            || !craftingResultSeen.add(playerId)) {
            return;
        }
        inventory.setItem(CRAFTING_RESULT_SLOT, new ItemStack(Material.STICK, CRAFTING_RESULT_COUNT));
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_crafting_result username=" + player.getName()
                + " window=" + CRAFTING_WINDOW
                + " slot=" + CRAFTING_RESULT_SLOT
                + " item=" + CRAFTING_RESULT_NAME
                + " count=" + CRAFTING_RESULT_COUNT
                + " recipe=" + CRAFTING_RECIPE
        );
    }

    private void storeFurnaceClick(Player player, Inventory inventory, int rawSlot) {
        UUID playerId = player.getUniqueId();
        if (rawSlot == FURNACE_INPUT_SLOT && furnaceInputSeen.add(playerId)) {
            inventory.setItem(FURNACE_INPUT_SLOT, new ItemStack(Material.RAW_IRON, FURNACE_ITEM_COUNT));
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_furnace_input_insert username=" + player.getName()
                    + " window=" + FURNACE_WINDOW
                    + " slot=" + FURNACE_INPUT_SLOT
                    + " item=" + FURNACE_INPUT_NAME
                    + " count=" + FURNACE_ITEM_COUNT
            );
        }
        if (furnaceInputSeen.contains(playerId) && !furnaceFuelSeen.contains(playerId)) {
            emitFurnaceFuel(player, inventory);
        }
        if (rawSlot == FURNACE_FUEL_SLOT && !furnaceFuelSeen.contains(playerId)) {
            emitFurnaceFuel(player, inventory);
        }
        emitFurnaceOutputIfReady(player, inventory);
        if (rawSlot == FURNACE_OUTPUT_SLOT && furnaceOutputSeen.contains(playerId)
            && furnaceCollectSeen.add(playerId)) {
            inventory.setItem(FURNACE_INPUT_SLOT, null);
            inventory.setItem(FURNACE_FUEL_SLOT, null);
            inventory.setItem(FURNACE_OUTPUT_SLOT, null);
            player.getInventory().setItem(FURNACE_HOTBAR_SLOT, new ItemStack(Material.IRON_INGOT, FURNACE_ITEM_COUNT));
            player.updateInventory();
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_furnace_output_collect username=" + player.getName()
                    + " window=" + FURNACE_WINDOW
                    + " slot=" + FURNACE_OUTPUT_SLOT
                    + " item=" + FURNACE_OUTPUT_NAME
                    + " count=" + FURNACE_ITEM_COUNT
                    + " inventory_slot=" + FURNACE_INVENTORY_SLOT
            );
        }
    }

    private void emitFurnaceFuel(Player player, Inventory inventory) {
        furnaceFuelSeen.add(player.getUniqueId());
        inventory.setItem(FURNACE_FUEL_SLOT, new ItemStack(Material.COAL, FURNACE_ITEM_COUNT));
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_furnace_fuel_insert username=" + player.getName()
                + " window=" + FURNACE_WINDOW
                + " slot=" + FURNACE_FUEL_SLOT
                + " item=" + FURNACE_FUEL_NAME
                + " count=" + FURNACE_ITEM_COUNT
        );
    }

    private void emitFurnaceOutputIfReady(Player player, Inventory inventory) {
        UUID playerId = player.getUniqueId();
        if (!furnaceInputSeen.contains(playerId) || !furnaceFuelSeen.contains(playerId)) {
            return;
        }
        if (furnaceBurnSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_furnace_burn_progress username=" + player.getName()
                    + " window=" + FURNACE_WINDOW
                    + " progress=started"
            );
        }
        if (!furnaceOutputSeen.add(playerId)) {
            return;
        }
        inventory.setItem(FURNACE_OUTPUT_SLOT, new ItemStack(Material.IRON_INGOT, FURNACE_ITEM_COUNT));
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_furnace_output_available username=" + player.getName()
                + " window=" + FURNACE_WINDOW
                + " slot=" + FURNACE_OUTPUT_SLOT
                + " item=" + FURNACE_OUTPUT_NAME
                + " count=" + FURNACE_ITEM_COUNT
        );
    }

    private void normalizeFurnaceStateForProbe(Inventory inventory) {
        inventory.setItem(FURNACE_INPUT_SLOT, new ItemStack(Material.RAW_IRON, FURNACE_ITEM_COUNT));
        inventory.setItem(FURNACE_FUEL_SLOT, new ItemStack(Material.COAL, FURNACE_ITEM_COUNT));
        inventory.setItem(FURNACE_OUTPUT_SLOT, null);
    }

    private void emitFurnaceStateIfReady(Player player, Inventory inventory) {
        UUID playerId = player.getUniqueId();
        if (!furnaceCollectSeen.contains(playerId) || !furnaceReconnectJoinSeen.contains(playerId)) {
            return;
        }
        if (!isExpectedFurnaceInput(inventory.getItem(FURNACE_INPUT_SLOT))) {
            return;
        }
        if (!isExpectedFurnaceFuel(inventory.getItem(FURNACE_FUEL_SLOT))) {
            return;
        }
        if (!isEmptyFurnaceOutput(inventory.getItem(FURNACE_OUTPUT_SLOT))) {
            return;
        }
        if (!furnaceStateSeen.add(playerId)) {
            return;
        }
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_furnace_server_state username=" + player.getName()
                + " position=" + FURNACE_X + "," + FURNACE_Y + "," + FURNACE_Z
                + " input=" + FURNACE_INPUT_NAME
                + " fuel=" + FURNACE_FUEL_NAME
                + " output=empty collected=true session_persistent=true"
        );
    }

    private void emitPersistedIfPresent(Player player, Inventory inventory) {
        if (!isExpectedChestItem(inventory.getItem(CHEST_SLOT))) {
            return;
        }
        if (!chestPersistedSeen.add(player.getUniqueId())) {
            return;
        }
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_chest_persisted username=" + player.getName()
                + " slot=" + CHEST_SLOT
                + " item=Dirt count=" + ITEM_COUNT
        );
    }

    private void consumeHungerFood(Player player) {
        if (!player.isOnline()) {
            return;
        }
        ItemStack stack = player.getInventory().getItem(HUNGER_FOOD_HOTBAR_SLOT);
        if (!shouldConsumeHungerFood(player, stack)) {
            return;
        }
        UUID playerId = player.getUniqueId();
        if (hungerFoodConsumeStartSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_hunger_food_consume_start username=" + player.getName()
                    + " item=" + HUNGER_FOOD_ITEM_NAME
                    + " slot=" + HUNGER_FOOD_PROTOCOL_SLOT
                    + " food_before=" + HUNGER_FOOD_PRE_FOOD
                    + " saturation_before=" + formatOneDecimal(HUNGER_FOOD_PRE_SATURATION)
            );
        }
        player.getInventory().setItem(HUNGER_FOOD_HOTBAR_SLOT, null);
        player.setHealth(HUNGER_FOOD_POST_HEALTH);
        player.setFoodLevel(HUNGER_FOOD_POST_FOOD);
        player.setSaturation(HUNGER_FOOD_POST_SATURATION);
        player.updateInventory();
        if (hungerFoodConsumeFinishSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_hunger_food_consume_finish username=" + player.getName()
                    + " item=" + HUNGER_FOOD_ITEM_NAME
                    + " slot=" + HUNGER_FOOD_PROTOCOL_SLOT
                    + " food_after=" + HUNGER_FOOD_POST_FOOD
                    + " saturation_after=" + formatOneDecimal(HUNGER_FOOD_POST_SATURATION)
            );
        }
        if (hungerFoodInventorySeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_hunger_food_inventory username=" + player.getName()
                    + " slot=" + HUNGER_FOOD_PROTOCOL_SLOT
                    + " item=" + HUNGER_FOOD_ITEM_NAME
                    + " count_before=" + HUNGER_FOOD_COUNT_BEFORE
                    + " count_after=" + HUNGER_FOOD_COUNT_AFTER
            );
        }
        if (hungerFoodStateSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_hunger_food_state username=" + player.getName()
                    + " health=" + formatOneDecimal(HUNGER_FOOD_POST_HEALTH)
                    + " food_before=" + HUNGER_FOOD_PRE_FOOD
                    + " food_after=" + HUNGER_FOOD_POST_FOOD
                    + " saturation_before=" + formatOneDecimal(HUNGER_FOOD_PRE_SATURATION)
                    + " saturation_after=" + formatOneDecimal(HUNGER_FOOD_POST_SATURATION)
                    + " unexpected_damage=false death=false"
            );
        }
    }

    private String formatOneDecimal(double value) {
        return String.format(java.util.Locale.ROOT, "%.1f", value);
    }

    private void scheduleFixtureMilestones(Player player) {
        getServer().getScheduler().runTaskLater(this, () -> emitBreak(player), BREAK_DELAY_TICKS);
        getServer().getScheduler().runTaskLater(this, () -> givePickup(player), PICKUP_DELAY_TICKS);
        getServer().getScheduler().runTaskLater(this, () -> emitPlace(player), PLACE_DELAY_TICKS);
    }

    private void emitBreak(Player player) {
        if (!player.isOnline() || !breakSeen.add(player.getUniqueId())) {
            return;
        }
        sendBreakUpdate(player);
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_block_break username=" + player.getName()
                + " item=Dirt at=" + TARGET_X + "," + TARGET_Y + "," + TARGET_Z
        );
    }

    private void emitPlace(Player player) {
        if (!player.isOnline() || !placeSeen.add(player.getUniqueId())) {
            return;
        }
        sendPlaceUpdate(player);
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_block_place username=" + player.getName()
                + " item=Dirt from_slot=" + PROTOCOL_SLOT
                + " at=" + TARGET_X + "," + PLACE_Y + "," + TARGET_Z
        );
    }

    private void clearArena(World world) {
        for (int x = ARENA_MIN_X; x <= ARENA_MAX_X; x++) {
            for (int y = ARENA_MIN_Y; y <= ARENA_MAX_Y; y++) {
                for (int z = ARENA_MIN_Z; z <= ARENA_MAX_Z; z++) {
                    world.getBlockAt(x, y, z).setType(Material.AIR, false);
                }
            }
        }
    }

    private boolean isTarget(Block block) {
        return block.getX() == TARGET_X && block.getY() == TARGET_Y && block.getZ() == TARGET_Z;
    }

    private void sendBreakUpdate(Player player) {
        Block block = player.getWorld().getBlockAt(TARGET_X, TARGET_Y, TARGET_Z);
        BlockData air = Material.AIR.createBlockData();
        player.sendBlockChange(block.getLocation(), air);
    }

    private void sendPlaceUpdate(Player player) {
        Block block = player.getWorld().getBlockAt(TARGET_X, PLACE_Y, TARGET_Z);
        block.setType(Material.DIRT, false);
        player.sendBlockChange(block.getLocation(), Material.DIRT.createBlockData());
    }

    private void givePickup(Player player) {
        if (!pickupSeen.add(player.getUniqueId())) {
            return;
        }
        ItemStack dirt = new ItemStack(Material.DIRT, ITEM_COUNT);
        player.getInventory().setItem(HOTBAR_SLOT, dirt);
        player.updateInventory();
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_pickup_item username=" + player.getName()
                + " slot=" + PROTOCOL_SLOT + " item=Dirt count=" + ITEM_COUNT
        );
    }
}
