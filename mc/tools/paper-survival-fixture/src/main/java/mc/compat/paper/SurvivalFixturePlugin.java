package mc.compat.paper;

import java.util.UUID;
import org.bukkit.GameMode;
import org.bukkit.Location;
import org.bukkit.Material;
import org.bukkit.World;
import org.bukkit.block.Block;
import org.bukkit.block.data.BlockData;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.block.BlockDamageEvent;
import org.bukkit.event.block.BlockPlaceEvent;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.event.inventory.InventoryOpenEvent;
import org.bukkit.event.inventory.InventoryType;
import org.bukkit.event.player.PlayerJoinEvent;
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

    @Override
    public void onEnable() {
        getServer().getPluginManager().registerEvents(this, this);
        getLogger().info("MC-COMPAT-MILESTONE survival_fixture_enabled");
    }

    @EventHandler
    public void onPlayerJoin(PlayerJoinEvent event) {
        Player player = event.getPlayer();
        getServer().getScheduler().runTask(this, () -> setupPlayer(player));
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
        player.getInventory().clear();
        player.setGameMode(GameMode.SURVIVAL);
        if (chestFixtureEnabled()) {
            player.setItemOnCursor(new ItemStack(Material.DIRT, ITEM_COUNT));
        }
        if (craftingFixtureEnabled()) {
            player.setItemOnCursor(new ItemStack(Material.OAK_PLANKS, CRAFTING_CURSOR_INPUT_COUNT));
        }
        player.teleport(new Location(world, SPAWN_X, SPAWN_Y, SPAWN_Z, SPAWN_YAW, SPAWN_PITCH));
        getLogger().info(
            "MC-COMPAT-MILESTONE survival_join username=" + player.getName()
                + " gamemode=Survival target=" + TARGET_X + "," + TARGET_Y + "," + TARGET_Z
        );
        if (chestFixtureEnabled()) {
            scheduleChestOpen(player);
        }
        if (craftingFixtureEnabled()) {
            scheduleCraftingOpen(player);
        }
        scheduleFixtureMilestones(player);
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

    private boolean chestFixtureEnabled() {
        return "1".equals(System.getenv(CHEST_FIXTURE_ENV));
    }

    private boolean craftingFixtureEnabled() {
        return "1".equals(System.getenv(CRAFTING_FIXTURE_ENV));
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

    private boolean isExpectedChestItem(ItemStack item) {
        return item != null && item.getType() == Material.DIRT && item.getAmount() == ITEM_COUNT;
    }

    private boolean isExpectedCraftingInput(ItemStack item) {
        return item != null && item.getType() == Material.OAK_PLANKS && item.getAmount() == CRAFTING_INPUT_COUNT;
    }

    private boolean isExpectedCraftingResult(ItemStack item) {
        return item != null && item.getType() == Material.STICK && item.getAmount() == CRAFTING_RESULT_COUNT;
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
