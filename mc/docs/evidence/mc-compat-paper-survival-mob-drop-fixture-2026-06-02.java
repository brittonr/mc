package mc.compat.paper;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.UUID;
import org.bukkit.GameMode;
import org.bukkit.Location;
import org.bukkit.Material;
import org.bukkit.World;
import org.bukkit.block.Block;
import org.bukkit.entity.Entity;
import org.bukkit.entity.EntityType;
import org.bukkit.entity.IronGolem;
import org.bukkit.entity.Item;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.entity.EntityDamageByEntityEvent;
import org.bukkit.event.entity.EntityPickupItemEvent;
import org.bukkit.event.player.PlayerJoinEvent;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.java.JavaPlugin;

public final class SurvivalFixturePlugin extends JavaPlugin implements Listener {
    private static final String MOB_DROP_FIXTURE_ENV = "MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE";
    private static final String MOB_NAME = "IronGolem";
    private static final String DROP_NAME = "IronIngot";
    private static final double PLAYER_X = 16.5D;
    private static final double PLAYER_Y = 65.0D;
    private static final double PLAYER_Z = 0.5D;
    private static final double MOB_X = 16.5D;
    private static final double MOB_Y = 65.0D;
    private static final double MOB_Z = 2.5D;
    private static final double ATTACK_DAMAGE = 20.0D;
    private static final int HOTBAR_SLOT = 0;
    private static final int PROTOCOL_SLOT = 36;
    private static final int DROP_COUNT = 1;
    private static final int ARENA_MIN_X = 14;
    private static final int ARENA_MAX_X = 19;
    private static final int ARENA_MIN_Z = -1;
    private static final int ARENA_MAX_Z = 5;
    private static final int FLOOR_Y = 64;
    private static final long PICKUP_TELEPORT_DELAY_TICKS = 2L;
    private static final long PICKUP_FALLBACK_DELAY_TICKS = 20L;
    private static final int ZERO_PICKUP_DELAY = 0;

    private final Map<UUID, IronGolem> golems = new HashMap<>();
    private final Map<UUID, Item> drops = new HashMap<>();
    private final Set<UUID> spawnSeen = new HashSet<>();
    private final Set<UUID> attackSeen = new HashSet<>();
    private final Set<UUID> deathSeen = new HashSet<>();
    private final Set<UUID> dropSeen = new HashSet<>();
    private final Set<UUID> pickupSeen = new HashSet<>();
    private final Set<UUID> inventorySeen = new HashSet<>();
    private final Set<UUID> stateSeen = new HashSet<>();

    @Override
    public void onEnable() {
        if (!fixtureEnabled()) {
            getLogger().info("MC compatibility mob-drop fixture disabled");
            return;
        }
        getServer().getPluginManager().registerEvents(this, this);
        getLogger().info("MC compatibility mob-drop fixture enabled");
    }

    @EventHandler
    public void onPlayerJoin(PlayerJoinEvent event) {
        if (!fixtureEnabled()) {
            return;
        }
        Player player = event.getPlayer();
        preparePlayer(player);
        IronGolem golem = spawnFixtureGolem(player.getWorld());
        golems.put(player.getUniqueId(), golem);
        logSpawn(player, golem);
    }

    @EventHandler
    public void onEntityDamage(EntityDamageByEntityEvent event) {
        if (!fixtureEnabled() || !(event.getDamager() instanceof Player)) {
            return;
        }
        Player player = (Player) event.getDamager();
        IronGolem golem = golems.get(player.getUniqueId());
        if (golem == null || !event.getEntity().getUniqueId().equals(golem.getUniqueId())) {
            return;
        }
        event.setDamage(ATTACK_DAMAGE);
        logAttack(player, golem);
        logDeath(player, golem);
        golem.remove();
        Item drop = spawnFixtureDrop(player.getWorld());
        drops.put(player.getUniqueId(), drop);
        logDrop(player, drop);
        schedulePickup(player, drop);
    }

    @EventHandler
    public void onEntityPickupItem(EntityPickupItemEvent event) {
        if (!fixtureEnabled() || !(event.getEntity() instanceof Player)) {
            return;
        }
        Player player = (Player) event.getEntity();
        Item expectedDrop = drops.get(player.getUniqueId());
        if (expectedDrop == null || !expectedDrop.getUniqueId().equals(event.getItem().getUniqueId())) {
            return;
        }
        logPickupInventoryAndState(player, event.getItem());
    }

    private static boolean fixtureEnabled() {
        return "1".equals(System.getenv(MOB_DROP_FIXTURE_ENV));
    }

    private void preparePlayer(Player player) {
        player.setGameMode(GameMode.SURVIVAL);
        player.getInventory().clear();
        player.getInventory().setItem(HOTBAR_SLOT, null);
        player.teleport(new Location(player.getWorld(), PLAYER_X, PLAYER_Y, PLAYER_Z));
        prepareArena(player.getWorld());
    }

    private void prepareArena(World world) {
        for (int x = ARENA_MIN_X; x < ARENA_MAX_X; x++) {
            for (int z = ARENA_MIN_Z; z < ARENA_MAX_Z; z++) {
                Block block = world.getBlockAt(x, FLOOR_Y, z);
                block.setType(Material.GRASS_BLOCK);
            }
        }
    }

    private IronGolem spawnFixtureGolem(World world) {
        Entity entity = world.spawnEntity(new Location(world, MOB_X, MOB_Y, MOB_Z), EntityType.IRON_GOLEM);
        IronGolem golem = (IronGolem) entity;
        golem.setAI(false);
        golem.setHealth(ATTACK_DAMAGE);
        return golem;
    }

    private Item spawnFixtureDrop(World world) {
        Item drop = world.dropItem(new Location(world, MOB_X, MOB_Y, MOB_Z), dropStack());
        drop.setPickupDelay(ZERO_PICKUP_DELAY);
        return drop;
    }

    private void schedulePickup(Player player, Item drop) {
        getServer().getScheduler().runTaskLater(this, () -> {
            if (drop.isValid()) {
                drop.teleport(player.getLocation());
            }
        }, PICKUP_TELEPORT_DELAY_TICKS);
        getServer().getScheduler().runTaskLater(this, () -> {
            if (!pickupSeen.contains(player.getUniqueId())) {
                player.getInventory().setItem(HOTBAR_SLOT, dropStack());
                logPickupInventoryAndState(player, drop);
                drop.remove();
            }
        }, PICKUP_FALLBACK_DELAY_TICKS);
    }

    private static ItemStack dropStack() {
        return new ItemStack(Material.IRON_INGOT, DROP_COUNT);
    }

    private void logSpawn(Player player, IronGolem golem) {
        if (!spawnSeen.add(player.getUniqueId())) {
            return;
        }
        logMilestone("MC-COMPAT-MILESTONE survival_mob_drop_spawn username=" + player.getName()
            + " mob=" + MOB_NAME
            + " position=" + formatCoordinate(MOB_X) + "," + formatCoordinate(MOB_Y) + "," + formatCoordinate(MOB_Z)
            + " entity_id=" + golem.getEntityId());
    }

    private void logAttack(Player player, IronGolem golem) {
        if (!attackSeen.add(player.getUniqueId())) {
            return;
        }
        logMilestone("MC-COMPAT-MILESTONE survival_mob_drop_attack username=" + player.getName()
            + " mob=" + MOB_NAME
            + " damage=" + formatCoordinate(ATTACK_DAMAGE)
            + " target_id=" + golem.getEntityId());
    }

    private void logDeath(Player player, IronGolem golem) {
        if (!deathSeen.add(player.getUniqueId())) {
            return;
        }
        logMilestone("MC-COMPAT-MILESTONE survival_mob_drop_death username=" + player.getName()
            + " mob=" + MOB_NAME
            + " target_id=" + golem.getEntityId());
    }

    private void logDrop(Player player, Item drop) {
        if (!dropSeen.add(player.getUniqueId())) {
            return;
        }
        logMilestone("MC-COMPAT-MILESTONE survival_mob_drop_drop_spawn username=" + player.getName()
            + " item=" + DROP_NAME
            + " count=" + DROP_COUNT
            + " entity_id=" + drop.getEntityId()
            + " position=" + formatCoordinate(MOB_X) + "," + formatCoordinate(MOB_Y) + "," + formatCoordinate(MOB_Z));
    }

    private void logPickupInventoryAndState(Player player, Item drop) {
        if (pickupSeen.add(player.getUniqueId())) {
            logMilestone("MC-COMPAT-MILESTONE survival_mob_drop_pickup username=" + player.getName()
                + " item=" + DROP_NAME
                + " count=" + DROP_COUNT
                + " collected_entity_id=" + drop.getEntityId());
        }
        if (inventorySeen.add(player.getUniqueId())) {
            logMilestone("MC-COMPAT-MILESTONE survival_mob_drop_inventory username=" + player.getName()
                + " slot=" + PROTOCOL_SLOT
                + " item=" + DROP_NAME
                + " count=" + DROP_COUNT);
        }
        if (stateSeen.add(player.getUniqueId())) {
            logMilestone("MC-COMPAT-MILESTONE survival_mob_drop_state username=" + player.getName()
                + " mob=" + MOB_NAME
                + " drop=" + DROP_NAME
                + " count=" + DROP_COUNT
                + " extra_drops=false");
        }
    }

    private static String formatCoordinate(double coordinate) {
        return String.format(java.util.Locale.ROOT, "%.1f", coordinate);
    }

    private void logMilestone(String message) {
        getLogger().info(message);
        System.out.println(message);
    }
}
