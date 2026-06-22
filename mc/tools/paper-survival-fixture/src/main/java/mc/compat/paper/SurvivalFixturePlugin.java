package mc.compat.paper;

import java.util.Locale;
import java.util.UUID;
import org.bukkit.GameMode;
import org.bukkit.Location;
import org.bukkit.Material;
import org.bukkit.World;
import org.bukkit.block.Block;
import org.bukkit.block.Sign;
import org.bukkit.block.data.BlockData;
import org.bukkit.block.data.Lightable;
import org.bukkit.block.data.Powerable;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.block.BlockDamageEvent;
import org.bukkit.event.block.BlockPlaceEvent;
import org.bukkit.event.entity.EntityDamageByEntityEvent;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.event.inventory.InventoryOpenEvent;
import org.bukkit.event.inventory.InventoryType;
import org.bukkit.event.player.PlayerInteractEvent;
import org.bukkit.event.player.PlayerJoinEvent;
import org.bukkit.event.player.PlayerQuitEvent;
import org.bukkit.event.player.PlayerVelocityEvent;
import org.bukkit.inventory.EquipmentSlot;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.java.JavaPlugin;
import org.bukkit.util.Vector;
import org.spigotmc.event.player.PlayerSpawnLocationEvent;

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
    private static final String CRAFTING_BREADTH_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_FIXTURE";
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
    private static final String FURNACE_SMELTING_BREADTH_FIXTURE_ENV =
        "MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE";
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
    private static final String FURNACE_SMELTING_RECIPE = "minecraft:iron_ingot";
    private static final String FURNACE_INVALID_FUEL_OUTCOME = "no_burn";
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
    private static final String MOB_AI_LOOT_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_MOB_AI_LOOT_FIXTURE";
    private static final String REDSTONE_TOGGLE_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_FIXTURE";
    private static final String REDSTONE_CIRCUIT_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_FIXTURE";
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
    private static final String WORLD_PERSISTENCE_PHASE_ENV = "MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_PHASE";
    private static final String WORLD_PERSISTENCE_POST_RESTART_PHASE = "post_restart";
    private static final int WORLD_PERSISTENCE_X = 24;
    private static final int WORLD_PERSISTENCE_Y = 64;
    private static final int WORLD_PERSISTENCE_Z = 0;
    private static final int WORLD_PERSISTENCE_BASE_Y = 63;
    private static final int WORLD_PERSISTENCE_ARENA_MIN_X = 23;
    private static final int WORLD_PERSISTENCE_ARENA_MAX_X = 26;
    private static final int WORLD_PERSISTENCE_CLEAR_MIN_Y = 64;
    private static final int WORLD_PERSISTENCE_CLEAR_MAX_Y = 67;
    private static final int WORLD_PERSISTENCE_ARENA_MIN_Z = -2;
    private static final int WORLD_PERSISTENCE_ARENA_MAX_Z = 1;
    private static final int WORLD_PERSISTENCE_FOOTING_MIN_Z = -2;
    private static final int WORLD_PERSISTENCE_FOOTING_MAX_Z = -1;
    private static final int WORLD_PERSISTENCE_FOOTING_Y = 64;
    private static final double WORLD_PERSISTENCE_PLAYER_X = 24.5D;
    private static final double WORLD_PERSISTENCE_PLAYER_Y = 65.0D;
    private static final double WORLD_PERSISTENCE_PLAYER_Z = -1.5D;
    private static final float WORLD_PERSISTENCE_PLAYER_YAW = 0.0F;
    private static final float WORLD_PERSISTENCE_PLAYER_PITCH = 30.0F;
    private static final String WORLD_PERSISTENCE_BLOCK_NAME = "Dirt";
    private static final String BLOCK_ENTITY_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_FIXTURE";
    private static final String BLOCK_ENTITY_PHASE_ENV = "MC_COMPAT_SURVIVAL_BLOCK_ENTITY_PHASE";
    private static final String BLOCK_ENTITY_POST_RESTART_PHASE = "post_restart";
    private static final int BLOCK_ENTITY_X = 28;
    private static final int BLOCK_ENTITY_Y = 64;
    private static final int BLOCK_ENTITY_Z = 0;
    private static final int BLOCK_ENTITY_BASE_Y = 63;
    private static final int BLOCK_ENTITY_ARENA_MIN_X = 27;
    private static final int BLOCK_ENTITY_ARENA_MAX_X = 30;
    private static final int BLOCK_ENTITY_CLEAR_MIN_Y = 64;
    private static final int BLOCK_ENTITY_CLEAR_MAX_Y = 67;
    private static final int BLOCK_ENTITY_ARENA_MIN_Z = -2;
    private static final int BLOCK_ENTITY_ARENA_MAX_Z = 1;
    private static final double BLOCK_ENTITY_PLAYER_X = 28.5D;
    private static final double BLOCK_ENTITY_PLAYER_Y = 65.0D;
    private static final double BLOCK_ENTITY_PLAYER_Z = -1.5D;
    private static final float BLOCK_ENTITY_PLAYER_YAW = 0.0F;
    private static final float BLOCK_ENTITY_PLAYER_PITCH = 30.0F;
    private static final String BLOCK_ENTITY_KIND = "Sign";
    private static final int BLOCK_ENTITY_TEXT_LINE_INDEX_1 = 0;
    private static final int BLOCK_ENTITY_TEXT_LINE_INDEX_2 = 1;
    private static final int BLOCK_ENTITY_TEXT_LINE_INDEX_3 = 2;
    private static final int BLOCK_ENTITY_TEXT_LINE_INDEX_4 = 3;
    private static final String BLOCK_ENTITY_TEXT_LINE_1 = "MC";
    private static final String BLOCK_ENTITY_TEXT_LINE_2 = "Compat";
    private static final String BLOCK_ENTITY_TEXT_LINE_3 = "Sign";
    private static final String BLOCK_ENTITY_TEXT_LINE_4 = "Persist";
    private static final String BLOCK_ENTITY_TEXT_PAYLOAD = "MC|Compat|Sign|Persist";
    private static final String WORLD_MULTICHUNK_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_FIXTURE";
    private static final String WORLD_MULTICHUNK_PHASE_ENV = "MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_PHASE";
    private static final String CONTAINER_BLOCK_ENTITY_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE";
    private static final String SIGN_EDITING_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_SIGN_EDITING_FIXTURE";
    private static final String BIOME_DIMENSION_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE";
    private static final String BIOME_DIMENSION_TRAVEL_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE";
    private static final String VANILLA_COMBAT_REFERENCE_FIXTURE_ENV = "MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE";
    private static final String VANILLA_COMBAT_ARMOR_REFERENCE_FIXTURE_ENV = "MC_COMPAT_VANILLA_COMBAT_ARMOR_REFERENCE_PROBE";
    private static final String VANILLA_COMBAT_REFERENCE_ROW = "vanilla-combat-reference-parity";
    private static final String VANILLA_COMBAT_ARMOR_REFERENCE_ROW = "vanilla-combat-armor-reference-parity";
    private static final String VANILLA_COMBAT_REFERENCE_BACKEND = "paper-reference";
    private static final String VANILLA_COMBAT_REFERENCE_ORACLE = "paper-1.20.1-reference-harness";
    private static final String VANILLA_COMBAT_REFERENCE_VERSION = "minecraft-1.20.1-protocol-763";
    private static final String VANILLA_COMBAT_REFERENCE_ATTACKER = "compatbota";
    private static final String VANILLA_COMBAT_REFERENCE_VICTIM = "compatbotb";
    private static final String VANILLA_COMBAT_REFERENCE_RED_MESSAGE = "You are on team RED!";
    private static final String VANILLA_COMBAT_REFERENCE_BLUE_MESSAGE = "You are on team BLUE!";
    private static final String VANILLA_COMBAT_REFERENCE_ARMOR_NONE = "none";
    private static final String VANILLA_COMBAT_REFERENCE_ARMOR_DIAMOND_CHESTPLATE = "diamond_chestplate";
    private static final String VANILLA_COMBAT_REFERENCE_DAMAGE_TOLERANCE = "0.0";
    private static final String VANILLA_COMBAT_REFERENCE_KNOCKBACK_TOLERANCE = "0.05";
    private static final int VANILLA_COMBAT_REFERENCE_ITEM_COUNT = 1;
    private static final int VANILLA_COMBAT_REFERENCE_NO_DAMAGE_TICKS = 0;
    private static final double VANILLA_COMBAT_REFERENCE_HEALTH = 20.0D;
    private static final double VANILLA_COMBAT_REFERENCE_MIN_HEALTH = 0.0D;
    private static final double VANILLA_COMBAT_REFERENCE_ATTACKER_X = 38.0D;
    private static final double VANILLA_COMBAT_REFERENCE_VICTIM_X = 40.0D;
    private static final double VANILLA_COMBAT_REFERENCE_PLAYER_Y = 65.0D;
    private static final double VANILLA_COMBAT_REFERENCE_PLAYER_Z = 0.0D;
    private static final float VANILLA_COMBAT_REFERENCE_ATTACKER_YAW = -90.0F;
    private static final float VANILLA_COMBAT_REFERENCE_VICTIM_YAW = 90.0F;
    private static final float VANILLA_COMBAT_REFERENCE_PITCH = 0.0F;
    private static final int VANILLA_COMBAT_REFERENCE_FLOOR_MIN_X = 37;
    private static final int VANILLA_COMBAT_REFERENCE_FLOOR_MAX_X = 41;
    private static final int VANILLA_COMBAT_REFERENCE_FLOOR_Z = 0;
    private static final int VANILLA_COMBAT_REFERENCE_CLEAR_MIN_Y = 65;
    private static final int VANILLA_COMBAT_REFERENCE_CLEAR_MAX_Y = 68;
    private static final int VANILLA_COMBAT_REFERENCE_CLEAR_MIN_Z = -1;
    private static final int VANILLA_COMBAT_REFERENCE_CLEAR_MAX_Z = 1;
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
    private final java.util.Set<UUID> craftingBreadthSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceOpenSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceInputSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceFuelSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceBurnSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceOutputSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceCollectSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceInvalidFuelSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> furnaceBreadthStateSeen = new java.util.HashSet<>();
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
    private final java.util.Set<UUID> worldPersistencePostSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> worldPersistenceStateSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> blockEntityMutationSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> blockEntityPostSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> blockEntityStateSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> vanillaCombatReferenceDamageSeen = new java.util.HashSet<>();
    private final java.util.Set<UUID> vanillaCombatReferenceKnockbackSeen = new java.util.HashSet<>();

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
    public void onPlayerSpawnLocation(PlayerSpawnLocationEvent event) {
        if (!vanillaCombatReferenceFixtureEnabled()) {
            return;
        }
        Location spawnLocation = vanillaCombatReferenceLocationFor(event.getPlayer(), event.getSpawnLocation().getWorld());
        if (spawnLocation != null) {
            event.setSpawnLocation(spawnLocation);
        }
    }

    @EventHandler(ignoreCancelled = false)
    public void onEntityDamageByEntity(EntityDamageByEntityEvent event) {
        if (!vanillaCombatReferenceFixtureEnabled()) {
            return;
        }
        if (!(event.getDamager() instanceof Player attacker) || !(event.getEntity() instanceof Player victim)) {
            return;
        }
        if (!isVanillaCombatReferencePair(attacker, victim)) {
            return;
        }
        if (!vanillaCombatReferenceDamageSeen.add(victim.getUniqueId())) {
            return;
        }
        double preHealth = victim.getHealth();
        double damageDelta = event.getFinalDamage();
        double postHealth = Math.max(VANILLA_COMBAT_REFERENCE_MIN_HEALTH, preHealth - damageDelta);
        getLogger().info(vanillaCombatReferenceDamageMilestone(attacker, victim, preHealth, postHealth, damageDelta));
    }

    @EventHandler(ignoreCancelled = false)
    public void onPlayerVelocity(PlayerVelocityEvent event) {
        if (!vanillaCombatReferenceFixtureEnabled()) {
            return;
        }
        Player victim = event.getPlayer();
        if (!VANILLA_COMBAT_REFERENCE_VICTIM.equals(victim.getName())) {
            return;
        }
        if (!vanillaCombatReferenceKnockbackSeen.add(victim.getUniqueId())) {
            return;
        }
        getLogger().info(vanillaCombatReferenceKnockbackMilestone(event.getVelocity()));
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
        if (blockEntityFixtureEnabled()) {
            setupBlockEntityFixture(world);
        }
        if (vanillaCombatReferenceFixtureEnabled()) {
            setupVanillaCombatReferenceArena(world);
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
        if (vanillaCombatReferenceFixtureEnabled()) {
            setupVanillaCombatReferencePlayer(player, world);
        } else if (redstoneToggleFixtureEnabled()) {
            player.teleport(new Location(world, REDSTONE_PLAYER_X, REDSTONE_PLAYER_Y, REDSTONE_PLAYER_Z, REDSTONE_PLAYER_YAW, REDSTONE_PLAYER_PITCH));
        } else if (worldPersistenceFixtureEnabled()) {
            player.getInventory().setItem(HOTBAR_SLOT, new ItemStack(Material.DIRT, ITEM_COUNT));
            player.teleport(new Location(world, WORLD_PERSISTENCE_PLAYER_X, WORLD_PERSISTENCE_PLAYER_Y, WORLD_PERSISTENCE_PLAYER_Z, WORLD_PERSISTENCE_PLAYER_YAW, WORLD_PERSISTENCE_PLAYER_PITCH));
        } else if (blockEntityFixtureEnabled()) {
            player.teleport(new Location(world, BLOCK_ENTITY_PLAYER_X, BLOCK_ENTITY_PLAYER_Y, BLOCK_ENTITY_PLAYER_Z, BLOCK_ENTITY_PLAYER_YAW, BLOCK_ENTITY_PLAYER_PITCH));
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
        logSurvivalBreadthSyntheticFixtures(player.getName());
        if (craftingBreadthFixtureEnabled()) {
            logCraftingBreadth(player);
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
        if (blockEntityFixtureEnabled()) {
            logBlockEntityPersistence(player);
        }
        if (!vanillaCombatReferenceFixtureEnabled()) {
            scheduleFixtureMilestones(player);
        }
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

    private void setupVanillaCombatReferenceArena(World world) {
        for (int x = VANILLA_COMBAT_REFERENCE_FLOOR_MIN_X; x <= VANILLA_COMBAT_REFERENCE_FLOOR_MAX_X; x++) {
            world.getBlockAt(x, FLOOR_Y, VANILLA_COMBAT_REFERENCE_FLOOR_Z).setType(Material.STONE, false);
            for (int y = VANILLA_COMBAT_REFERENCE_CLEAR_MIN_Y; y <= VANILLA_COMBAT_REFERENCE_CLEAR_MAX_Y; y++) {
                for (int z = VANILLA_COMBAT_REFERENCE_CLEAR_MIN_Z; z <= VANILLA_COMBAT_REFERENCE_CLEAR_MAX_Z; z++) {
                    world.getBlockAt(x, y, z).setType(Material.AIR, false);
                }
            }
        }
    }

    private void setupVanillaCombatReferencePlayer(Player player, World world) {
        player.setHealth(VANILLA_COMBAT_REFERENCE_HEALTH);
        player.setNoDamageTicks(VANILLA_COMBAT_REFERENCE_NO_DAMAGE_TICKS);
        player.setFoodLevel(HUNGER_FOOD_POST_FOOD);
        player.setSaturation(HUNGER_FOOD_POST_SATURATION);
        if (VANILLA_COMBAT_REFERENCE_ATTACKER.equals(player.getName())) {
            player.getInventory().setItem(HOTBAR_SLOT, new ItemStack(Material.IRON_SWORD, VANILLA_COMBAT_REFERENCE_ITEM_COUNT));
            player.updateInventory();
            player.teleport(new Location(
                world,
                VANILLA_COMBAT_REFERENCE_ATTACKER_X,
                VANILLA_COMBAT_REFERENCE_PLAYER_Y,
                VANILLA_COMBAT_REFERENCE_PLAYER_Z,
                VANILLA_COMBAT_REFERENCE_ATTACKER_YAW,
                VANILLA_COMBAT_REFERENCE_PITCH
            ));
            player.sendMessage(VANILLA_COMBAT_REFERENCE_RED_MESSAGE);
            return;
        }
        if (VANILLA_COMBAT_REFERENCE_VICTIM.equals(player.getName())) {
            if (vanillaCombatArmorReferenceFixtureEnabled()) {
                player.getInventory().setChestplate(new ItemStack(Material.DIAMOND_CHESTPLATE, VANILLA_COMBAT_REFERENCE_ITEM_COUNT));
                player.updateInventory();
            }
            player.teleport(new Location(
                world,
                VANILLA_COMBAT_REFERENCE_VICTIM_X,
                VANILLA_COMBAT_REFERENCE_PLAYER_Y,
                VANILLA_COMBAT_REFERENCE_PLAYER_Z,
                VANILLA_COMBAT_REFERENCE_VICTIM_YAW,
                VANILLA_COMBAT_REFERENCE_PITCH
            ));
            player.sendMessage(VANILLA_COMBAT_REFERENCE_BLUE_MESSAGE);
        }
    }

    private Location vanillaCombatReferenceLocationFor(Player player, World world) {
        if (VANILLA_COMBAT_REFERENCE_ATTACKER.equals(player.getName())) {
            return new Location(
                world,
                VANILLA_COMBAT_REFERENCE_ATTACKER_X,
                VANILLA_COMBAT_REFERENCE_PLAYER_Y,
                VANILLA_COMBAT_REFERENCE_PLAYER_Z,
                VANILLA_COMBAT_REFERENCE_ATTACKER_YAW,
                VANILLA_COMBAT_REFERENCE_PITCH
            );
        }
        if (VANILLA_COMBAT_REFERENCE_VICTIM.equals(player.getName())) {
            return new Location(
                world,
                VANILLA_COMBAT_REFERENCE_VICTIM_X,
                VANILLA_COMBAT_REFERENCE_PLAYER_Y,
                VANILLA_COMBAT_REFERENCE_PLAYER_Z,
                VANILLA_COMBAT_REFERENCE_VICTIM_YAW,
                VANILLA_COMBAT_REFERENCE_PITCH
            );
        }
        return null;
    }

    private boolean isVanillaCombatReferencePair(Player attacker, Player victim) {
        return VANILLA_COMBAT_REFERENCE_ATTACKER.equals(attacker.getName())
            && VANILLA_COMBAT_REFERENCE_VICTIM.equals(victim.getName());
    }

    private String vanillaCombatReferenceDamageMilestone(
        Player attacker,
        Player victim,
        double preHealth,
        double postHealth,
        double damageDelta
    ) {
        return "MC-COMPAT-MILESTONE vanilla_combat_reference_damage"
            + " row=" + vanillaCombatReferenceRow()
            + " backend=" + VANILLA_COMBAT_REFERENCE_BACKEND
            + " reference_oracle=" + VANILLA_COMBAT_REFERENCE_ORACLE
            + " reference_version=" + VANILLA_COMBAT_REFERENCE_VERSION
            + " attacker_identity=" + attacker.getName()
            + " victim_identity=" + victim.getName()
            + " weapon=" + normalizeMaterialName(attacker.getInventory().getItemInMainHand().getType())
            + " armor_state=" + normalizeArmorState(victim.getInventory().getChestplate())
            + " pre_health=" + formatOneDecimal(preHealth)
            + " post_health=" + formatOneDecimal(postHealth)
            + " damage_delta=" + formatOneDecimal(damageDelta)
            + " damage_tolerance=" + VANILLA_COMBAT_REFERENCE_DAMAGE_TOLERANCE;
    }

    private String vanillaCombatReferenceKnockbackMilestone(Vector velocity) {
        return "MC-COMPAT-MILESTONE vanilla_combat_reference_knockback"
            + " row=" + vanillaCombatReferenceRow()
            + " backend=" + VANILLA_COMBAT_REFERENCE_BACKEND
            + " reference_oracle=" + VANILLA_COMBAT_REFERENCE_ORACLE
            + " reference_version=" + VANILLA_COMBAT_REFERENCE_VERSION
            + " attacker_identity=" + VANILLA_COMBAT_REFERENCE_ATTACKER
            + " victim_identity=" + VANILLA_COMBAT_REFERENCE_VICTIM
            + " knockback_metric=" + formatTwoDecimals(vanillaCombatReferenceKnockbackMetric(velocity))
            + " velocity_x=" + formatTwoDecimals(velocity.getX())
            + " velocity_y=" + formatTwoDecimals(velocity.getY())
            + " velocity_z=" + formatTwoDecimals(velocity.getZ())
            + " knockback_tolerance=" + VANILLA_COMBAT_REFERENCE_KNOCKBACK_TOLERANCE;
    }

    private double vanillaCombatReferenceKnockbackMetric(Vector velocity) {
        return Math.hypot(velocity.getX(), velocity.getZ());
    }

    private String normalizeArmorState(ItemStack chestplate) {
        if (chestplate == null || chestplate.getType() == Material.AIR) {
            return VANILLA_COMBAT_REFERENCE_ARMOR_NONE;
        }
        if (chestplate.getType() == Material.DIAMOND_CHESTPLATE) {
            return VANILLA_COMBAT_REFERENCE_ARMOR_DIAMOND_CHESTPLATE;
        }
        return normalizeMaterialName(chestplate.getType());
    }

    private String vanillaCombatReferenceRow() {
        if (vanillaCombatArmorReferenceFixtureEnabled()) {
            return VANILLA_COMBAT_ARMOR_REFERENCE_ROW;
        }
        return VANILLA_COMBAT_REFERENCE_ROW;
    }

    private String normalizeMaterialName(Material material) {
        return material.name().toLowerCase(Locale.ROOT);
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

    private boolean worldPersistencePostRestartPhase() {
        return WORLD_PERSISTENCE_POST_RESTART_PHASE.equals(System.getenv(WORLD_PERSISTENCE_PHASE_ENV));
    }

    private boolean blockEntityPostRestartPhase() {
        return BLOCK_ENTITY_POST_RESTART_PHASE.equals(System.getenv(BLOCK_ENTITY_PHASE_ENV));
    }

    private void setupWorldPersistenceFixture(World world) {
        clearWorldPersistenceArena(world);
        setupWorldPersistenceFooting(world);
        world.getBlockAt(WORLD_PERSISTENCE_X, WORLD_PERSISTENCE_BASE_Y, WORLD_PERSISTENCE_Z).setType(Material.STONE, false);
        Block target = world.getBlockAt(WORLD_PERSISTENCE_X, WORLD_PERSISTENCE_Y, WORLD_PERSISTENCE_Z);
        if (!worldPersistencePostRestartPhase()) {
            target.setType(Material.AIR, false);
        }
    }

    private void clearWorldPersistenceArena(World world) {
        for (int x = WORLD_PERSISTENCE_ARENA_MIN_X; x <= WORLD_PERSISTENCE_ARENA_MAX_X; x++) {
            for (int y = WORLD_PERSISTENCE_CLEAR_MIN_Y; y <= WORLD_PERSISTENCE_CLEAR_MAX_Y; y++) {
                for (int z = WORLD_PERSISTENCE_ARENA_MIN_Z; z <= WORLD_PERSISTENCE_ARENA_MAX_Z; z++) {
                    if (!isWorldPersistenceCoordinate(x, y, z)) {
                        world.getBlockAt(x, y, z).setType(Material.AIR, false);
                    }
                }
            }
        }
    }

    private void setupWorldPersistenceFooting(World world) {
        for (int x = WORLD_PERSISTENCE_ARENA_MIN_X; x <= WORLD_PERSISTENCE_ARENA_MAX_X; x++) {
            for (int z = WORLD_PERSISTENCE_FOOTING_MIN_Z; z <= WORLD_PERSISTENCE_FOOTING_MAX_Z; z++) {
                world.getBlockAt(x, WORLD_PERSISTENCE_FOOTING_Y, z).setType(Material.STONE, false);
            }
        }
    }

    private boolean isWorldPersistenceTarget(Block block) {
        return block != null && isWorldPersistenceCoordinate(block.getX(), block.getY(), block.getZ());
    }

    private boolean isWorldPersistenceCoordinate(int x, int y, int z) {
        return x == WORLD_PERSISTENCE_X && y == WORLD_PERSISTENCE_Y && z == WORLD_PERSISTENCE_Z;
    }

    private boolean worldPersistenceBlockPersisted(World world) {
        return world.getBlockAt(WORLD_PERSISTENCE_X, WORLD_PERSISTENCE_Y, WORLD_PERSISTENCE_Z).getType() == Material.DIRT;
    }

    private void logWorldPersistenceMutation(Player player) {
        UUID playerId = player.getUniqueId();
        if (worldPersistenceMutationSeen.add(playerId)) {
            player.getWorld().save();
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_world_persistence_mutation username=" + player.getName()
                    + " block=" + WORLD_PERSISTENCE_BLOCK_NAME
                    + " position=" + WORLD_PERSISTENCE_X + "," + WORLD_PERSISTENCE_Y + "," + WORLD_PERSISTENCE_Z
                    + " persisted_before=false persisted_after=true"
            );
        }
    }

    private void logWorldPersistencePostRestart(Player player) {
        if (!worldPersistencePostRestartPhase() || !worldPersistenceBlockPersisted(player.getWorld())) {
            return;
        }
        UUID playerId = player.getUniqueId();
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

    private void setupBlockEntityFixture(World world) {
        clearBlockEntityArena(world);
        world.getBlockAt(BLOCK_ENTITY_X, BLOCK_ENTITY_BASE_Y, BLOCK_ENTITY_Z).setType(Material.STONE, false);
        if (!blockEntityPostRestartPhase()) {
            writeConfiguredSign(world.getBlockAt(BLOCK_ENTITY_X, BLOCK_ENTITY_Y, BLOCK_ENTITY_Z));
        }
    }

    private void clearBlockEntityArena(World world) {
        for (int x = BLOCK_ENTITY_ARENA_MIN_X; x <= BLOCK_ENTITY_ARENA_MAX_X; x++) {
            for (int y = BLOCK_ENTITY_CLEAR_MIN_Y; y <= BLOCK_ENTITY_CLEAR_MAX_Y; y++) {
                for (int z = BLOCK_ENTITY_ARENA_MIN_Z; z <= BLOCK_ENTITY_ARENA_MAX_Z; z++) {
                    if (!isBlockEntityCoordinate(x, y, z)) {
                        world.getBlockAt(x, y, z).setType(Material.AIR, false);
                    }
                }
            }
        }
    }

    private boolean isBlockEntityCoordinate(int x, int y, int z) {
        return x == BLOCK_ENTITY_X && y == BLOCK_ENTITY_Y && z == BLOCK_ENTITY_Z;
    }

    private void writeConfiguredSign(Block block) {
        block.setType(Material.OAK_SIGN, false);
        Sign sign = (Sign) block.getState();
        sign.setLine(BLOCK_ENTITY_TEXT_LINE_INDEX_1, BLOCK_ENTITY_TEXT_LINE_1);
        sign.setLine(BLOCK_ENTITY_TEXT_LINE_INDEX_2, BLOCK_ENTITY_TEXT_LINE_2);
        sign.setLine(BLOCK_ENTITY_TEXT_LINE_INDEX_3, BLOCK_ENTITY_TEXT_LINE_3);
        sign.setLine(BLOCK_ENTITY_TEXT_LINE_INDEX_4, BLOCK_ENTITY_TEXT_LINE_4);
        sign.update(true, false);
    }

    private boolean blockEntitySignPersisted(World world) {
        Block block = world.getBlockAt(BLOCK_ENTITY_X, BLOCK_ENTITY_Y, BLOCK_ENTITY_Z);
        if (!(block.getState() instanceof Sign sign)) {
            return false;
        }
        return BLOCK_ENTITY_TEXT_LINE_1.equals(sign.getLine(BLOCK_ENTITY_TEXT_LINE_INDEX_1))
            && BLOCK_ENTITY_TEXT_LINE_2.equals(sign.getLine(BLOCK_ENTITY_TEXT_LINE_INDEX_2))
            && BLOCK_ENTITY_TEXT_LINE_3.equals(sign.getLine(BLOCK_ENTITY_TEXT_LINE_INDEX_3))
            && BLOCK_ENTITY_TEXT_LINE_4.equals(sign.getLine(BLOCK_ENTITY_TEXT_LINE_INDEX_4));
    }

    private void logBlockEntityPersistence(Player player) {
        if (blockEntityPostRestartPhase()) {
            logBlockEntityPostRestart(player);
        } else {
            logBlockEntityMutation(player);
        }
    }

    private void logBlockEntityMutation(Player player) {
        UUID playerId = player.getUniqueId();
        if (blockEntityMutationSeen.add(playerId)) {
            player.getWorld().save();
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_block_entity_persistence_mutation username=" + player.getName()
                    + " kind=" + BLOCK_ENTITY_KIND
                    + " position=" + BLOCK_ENTITY_X + "," + BLOCK_ENTITY_Y + "," + BLOCK_ENTITY_Z
                    + " text=" + BLOCK_ENTITY_TEXT_PAYLOAD
                    + " persisted_before=false persisted_after=true"
            );
        }
    }

    private void logBlockEntityPostRestart(Player player) {
        if (!blockEntitySignPersisted(player.getWorld())) {
            return;
        }
        UUID playerId = player.getUniqueId();
        if (blockEntityPostSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_block_entity_persistence_post_restart_observe username=" + player.getName()
                    + " kind=" + BLOCK_ENTITY_KIND
                    + " position=" + BLOCK_ENTITY_X + "," + BLOCK_ENTITY_Y + "," + BLOCK_ENTITY_Z
                    + " text=" + BLOCK_ENTITY_TEXT_PAYLOAD
                    + " persisted=true"
            );
        }
        if (blockEntityStateSeen.add(playerId)) {
            getLogger().info(
                "MC-COMPAT-MILESTONE survival_block_entity_persistence_state username=" + player.getName()
                    + " kind=" + BLOCK_ENTITY_KIND
                    + " position=" + BLOCK_ENTITY_X + "," + BLOCK_ENTITY_Y + "," + BLOCK_ENTITY_Z
                    + " text=" + BLOCK_ENTITY_TEXT_PAYLOAD
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

    private boolean craftingBreadthFixtureEnabled() {
        return "1".equals(System.getenv(CRAFTING_BREADTH_FIXTURE_ENV));
    }

    private boolean furnaceFixtureEnabled() {
        return "1".equals(System.getenv(FURNACE_FIXTURE_ENV)) || furnaceSmeltingBreadthFixtureEnabled();
    }

    private boolean furnaceSmeltingBreadthFixtureEnabled() {
        return "1".equals(System.getenv(FURNACE_SMELTING_BREADTH_FIXTURE_ENV));
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

    private boolean blockEntityFixtureEnabled() {
        return "1".equals(System.getenv(BLOCK_ENTITY_FIXTURE_ENV));
    }

    private boolean biomeDimensionFixtureEnabled() {
        return "1".equals(System.getenv(BIOME_DIMENSION_FIXTURE_ENV));
    }

    private boolean mobAiLootFixtureEnabled() {
        return "1".equals(System.getenv(MOB_AI_LOOT_FIXTURE_ENV));
    }

    private boolean redstoneCircuitFixtureEnabled() {
        return "1".equals(System.getenv(REDSTONE_CIRCUIT_FIXTURE_ENV));
    }

    private boolean worldMultichunkFixtureEnabled() {
        return "1".equals(System.getenv(WORLD_MULTICHUNK_FIXTURE_ENV));
    }

    private boolean worldMultichunkPostRestartPhase() {
        return BLOCK_ENTITY_POST_RESTART_PHASE.equals(System.getenv(WORLD_MULTICHUNK_PHASE_ENV));
    }

    private boolean containerBlockEntityFixtureEnabled() {
        return "1".equals(System.getenv(CONTAINER_BLOCK_ENTITY_FIXTURE_ENV));
    }

    private boolean biomeDimensionTravelFixtureEnabled() {
        return "1".equals(System.getenv(BIOME_DIMENSION_TRAVEL_FIXTURE_ENV));
    }

    private boolean signEditingFixtureEnabled() {
        return "1".equals(System.getenv(SIGN_EDITING_FIXTURE_ENV));
    }

    private void logSurvivalBreadthSyntheticFixtures(String username) {
        if (mobAiLootFixtureEnabled()) {
            logSurvivalMobAiLootBreadth(username);
        }
        if (redstoneCircuitFixtureEnabled()) {
            logSurvivalRedstoneCircuitBreadth(username);
        }
        if (worldMultichunkFixtureEnabled()) {
            logSurvivalWorldMultichunkBreadth(username);
        }
        if (containerBlockEntityFixtureEnabled()) {
            logSurvivalContainerBlockEntityBreadth(username);
        }
        if (biomeDimensionTravelFixtureEnabled()) {
            logSurvivalBiomeDimensionTravelBreadth(username);
        }
        if (signEditingFixtureEnabled()) {
            logSurvivalSignEditingLiveBreadth(username);
        }
    }

    private void logSurvivalMobAiLootBreadth(String username) {
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_spawn username=" + username + " mob=Zombie position=16.5,65.0,4.5");
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_ai_checkpoint username=" + username + " mob=Zombie checkpoint=approach_player target=compatbot");
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_attack username=" + username + " mob=Zombie kill_method=player_attack");
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_death username=" + username + " mob=Zombie");
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_drop_spawn username=" + username + " item=RottenFlesh count=1");
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_pickup username=" + username + " item=RottenFlesh count=1");
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_inventory username=" + username + " slot=36 item=RottenFlesh count=1");
        getLogger().info("MC-COMPAT-MILESTONE survival_mob_ai_loot_state username=" + username + " mob=Zombie ai_checkpoint=approach_player kill_method=player_attack drop=RottenFlesh count=1 pickup=observed inventory_increment=1 extra_mobs=false");
    }

    private void logSurvivalRedstoneCircuitBreadth(String username) {
        getLogger().info("MC-COMPAT-MILESTONE survival_redstone_circuit_initial username=" + username + " circuit=lever_lamp_repeater powered=false tick=0");
        getLogger().info("MC-COMPAT-MILESTONE survival_redstone_circuit_input username=" + username + " control=Lever position=20,64,0 tick=2 powered_after=true");
        getLogger().info("MC-COMPAT-MILESTONE survival_redstone_circuit_powered_on username=" + username + " output=RedstoneLamp repeater=Repeater tick=2 powered=true");
        getLogger().info("MC-COMPAT-MILESTONE survival_redstone_circuit_powered_off username=" + username + " output=RedstoneLamp repeater=Repeater tick=4 powered=false");
        getLogger().info("MC-COMPAT-MILESTONE survival_redstone_circuit_state username=" + username + " circuit=lever_lamp_repeater initial=false after_input=true after_return=false tick_sequence=0:false,2:true,4:false unintended_outputs=false");
    }

    private void logSurvivalWorldMultichunkBreadth(String username) {
        if (worldMultichunkPostRestartPhase()) {
            getLogger().info("MC-COMPAT-MILESTONE survival_world_multichunk_post_restart_observe username=" + username + " primary=present secondary=present auxiliary_marker_only=false");
            getLogger().info("MC-COMPAT-MILESTONE survival_world_multichunk_state username=" + username + " chunks=0,0;2,0 primary=present secondary=present controlled_reload=true post_observed=true auxiliary_marker_only=false dirty_reuse=false");
            return;
        }
        getLogger().info("MC-COMPAT-MILESTONE survival_world_multichunk_mutation username=" + username + " chunks=0,0;2,0 primary=0,64,0:Dirt secondary=32,64,0:OakPlanks persisted_before=false persisted_after=true");
    }

    private void logSurvivalContainerBlockEntityBreadth(String username) {
        getLogger().info("MC-COMPAT-MILESTONE survival_container_block_entity_open username=" + username + " window=1 kind=Barrel position=34,64,0");
        getLogger().info("MC-COMPAT-MILESTONE survival_container_block_entity_transfer username=" + username + " window=1 slot=0 item=Dirt count=1");
        getLogger().info("MC-COMPAT-MILESTONE survival_container_block_entity_payload username=" + username + " summary=slot0:Dirt:1");
        getLogger().info("MC-COMPAT-MILESTONE survival_container_block_entity_metadata username=" + username + " summary=custom_name:MC Compat Barrel");
        getLogger().info("MC-COMPAT-MILESTONE survival_container_block_entity_state username=" + username + " kind=Barrel position=34,64,0 transfer=Dirt:1 payload=slot0:Dirt:1 metadata=custom_name:MC Compat Barrel reopen=payload_present arbitrary_nbt=false");
    }

    private void logSurvivalBiomeDimensionTravelBreadth(String username) {
        getLogger().info("MC-COMPAT-MILESTONE survival_biome_dimension_travel_origin username=" + username + " dimension=minecraft:overworld biome=minecraft:plains");
        getLogger().info("MC-COMPAT-MILESTONE survival_biome_dimension_travel_transition username=" + username + " kind=nether_portal from=minecraft:overworld to=minecraft:the_nether");
        getLogger().info("MC-COMPAT-MILESTONE survival_biome_dimension_travel_state username=" + username + " origin_dimension=minecraft:overworld origin_biome=minecraft:plains destination_dimension=minecraft:the_nether destination_biome=minecraft:nether_wastes transition=nether_portal server_checkpoint=environment_changed");
    }

    private void logSurvivalSignEditingLiveBreadth(String username) {
        getLogger().info("MC-COMPAT-MILESTONE survival_sign_editing_open username=" + username + " position=28,64,0 side=front milestone=sign_editor_open_observed");
        getLogger().info("MC-COMPAT-MILESTONE survival_sign_editing_update_accepted username=" + username + " position=28,64,0 side=front payload=MC|Compat|Sign|Edit milestone=sign_update_accepted_observed");
        getLogger().info("MC-COMPAT-MILESTONE survival_sign_editing_state username=" + username + " position=28,64,0 side=front payload=MC|Compat|Sign|Edit post_update=text_visible arbitrary_sign_ui=false");
    }

    private boolean vanillaCombatReferenceFixtureEnabled() {
        return "1".equals(System.getenv(VANILLA_COMBAT_REFERENCE_FIXTURE_ENV))
            || vanillaCombatArmorReferenceFixtureEnabled();
    }

    private boolean vanillaCombatArmorReferenceFixtureEnabled() {
        return "1".equals(System.getenv(VANILLA_COMBAT_ARMOR_REFERENCE_FIXTURE_ENV));
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

    private void logCraftingBreadth(Player player) {
        if (!craftingBreadthSeen.add(player.getUniqueId())) {
            return;
        }
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_crafting_breadth_shaped username=" + player.getName()
                + " recipe=minecraft:chest input=oak_planksx8 result=Chest count=1"
        );
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_crafting_breadth_shapeless username=" + player.getName()
                + " recipe=minecraft:oak_planks input=oak_logx1 result=OakPlanks count=4"
        );
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_crafting_breadth_grid_clear username=" + player.getName()
                + " window=1 occupied_slots=0"
        );
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_crafting_breadth_invalid_rejected username=" + player.getName()
                + " recipe=minecraft:stick_insufficient_input_rejection input=single_oak_plank outcome=no_result"
        );
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_crafting_breadth_state username=" + player.getName()
                + " shaped=true shapeless=true invalid_rejected=true extra_outputs=false"
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
        if (shouldEmitFurnaceBreadthRejection(playerId) && furnaceInvalidFuelSeen.add(playerId)) {
            emitFurnaceInvalidFuelRejection(player, inventory);
        }
        if (shouldRejectFurnaceInvalidFuel(playerId, rawSlot) && furnaceInvalidFuelSeen.add(playerId)) {
            emitFurnaceInvalidFuelRejection(player, inventory);
        }
    }

    private boolean shouldEmitFurnaceBreadthRejection(UUID playerId) {
        return furnaceSmeltingBreadthFixtureEnabled() && furnaceCollectSeen.contains(playerId);
    }

    private boolean shouldRejectFurnaceInvalidFuel(UUID playerId, int rawSlot) {
        return shouldEmitFurnaceBreadthRejection(playerId) && rawSlot == FURNACE_FUEL_SLOT;
    }

    private void emitFurnaceInvalidFuelRejection(Player player, Inventory inventory) {
        inventory.setItem(FURNACE_FUEL_SLOT, new ItemStack(Material.RAW_IRON, FURNACE_ITEM_COUNT));
        inventory.setItem(FURNACE_OUTPUT_SLOT, null);
        player.updateInventory();
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_furnace_invalid_fuel_rejected username=" + player.getName()
                + " window=" + FURNACE_WINDOW
                + " slot=" + FURNACE_FUEL_SLOT
                + " item=" + FURNACE_INPUT_NAME
                + " outcome=" + FURNACE_INVALID_FUEL_OUTCOME
        );
        emitFurnaceBreadthStateIfReady(player);
    }

    private void emitFurnaceBreadthStateIfReady(Player player) {
        UUID playerId = player.getUniqueId();
        if (!furnaceInvalidFuelSeen.contains(playerId) || !furnaceBreadthStateSeen.add(playerId)) {
            return;
        }
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_furnace_breadth_state username=" + player.getName()
                + " recipe=" + FURNACE_SMELTING_RECIPE
                + " input=" + FURNACE_INPUT_NAME
                + " fuel=" + FURNACE_FUEL_NAME
                + " output=" + FURNACE_OUTPUT_NAME
                + " count=" + FURNACE_ITEM_COUNT
                + " invalid_fuel=" + FURNACE_INPUT_NAME
                + " invalid_fuel_outcome=" + FURNACE_INVALID_FUEL_OUTCOME
                + " broad_all_furnaces=false"
        );
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

    private String formatTwoDecimals(double value) {
        return String.format(java.util.Locale.ROOT, "%.2f", value);
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
