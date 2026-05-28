# Valence combat-loop Steel-compatible policy implementation excerpts — 2026-05-28

## Scope

Review-readable excerpts for archived change `cairn/archive/2026-05-28-valence-combat-loop-steel-policy`.

## Source spans

Source file: `valence/examples/ctf.rs`.

### `valence_ctf.combat_event_projectile_probe` (`1842..1922`)

```rust
let projectile_probe_hit = projectile_probe_enabled()
    && attacker.username.as_str() == "compatbota"
    && victim.username.as_str() == "compatbotb";
let arrow_damage_decision = if projectile_probe_hit {
    Some(projectile_probe_damage_decision())
} else {
    None
};
let damage = arrow_damage_decision
    .as_ref()
    .map(|decision| decision.damage)
    .unwrap_or_else(|| (base_damage - armor_mitigation).max(0.0));

if projectile_probe_hit {
    let decision = arrow_damage_decision
        .as_ref()
        .expect("projectile probe hit has arrow decision");
    let projectile_use = format!(
        "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} item={:?} \
         policy={} generation={} clamped={}",
        attacker.username.as_str(),
        victim.username.as_str(),
        stack.item,
        decision.policy_id,
        decision.generation,
        decision.clamped
    );
    info!("{}", projectile_use);
    println!("{}", projectile_use);
}

victim.health.0 -= damage;
let milestone = format!(
    "MC-COMPAT-MILESTONE combat_damage attacker={} victim={} damage={:.1} \
     victim_health_before={:.1} victim_health_after={:.1} attacker_item={:?}",
    attacker.username.as_str(),
    victim.username.as_str(),
    damage,
    victim.health.0 + damage,
    victim.health.0,
    stack.item
);
info!("{}", milestone);
println!("{}", milestone);
if projectile_probe_hit {
    let decision = arrow_damage_decision
        .as_ref()
        .expect("projectile probe hit has arrow decision");
    let projectile_hit = format!(
        "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} damage={:.1} \
         victim_health_before={:.1} victim_health_after={:.1} policy={} \
         generation={} clamped={}",
        attacker.username.as_str(),
        victim.username.as_str(),
        damage,
        victim.health.0 + damage,
        victim.health.0,
        decision.policy_id,
        decision.generation,
        decision.clamped
    );
    info!("{}", projectile_hit);
    println!("{}", projectile_hit);
}
```

### `valence_ctf.projectile_interaction_probe` (`1986..2036`)

```rust
for event in interact_item.read() {
    let Ok((_, _, attacker_username, _, attacker_team)) = clients.get(event.client) else {
        continue;
    };
    let attacker_name = attacker_username.as_str().to_owned();
    let attacker_team = *attacker_team;
    let victim_ent = clients.iter().find_map(|(entity, _, _, _, team)| {
        if *team != attacker_team { Some(entity) } else { None }
    });
    let Some(victim_ent) = victim_ent else { continue; };
    let Ok((_, mut victim_client, victim_username, mut victim_health, _)) =
        clients.get_mut(victim_ent)
    else { continue; };

    let decision = projectile_probe_damage_decision();
    let before = victim_health.0;
    victim_health.0 -= decision.damage;
    victim_client.trigger_status(EntityStatus::PlayAttackSound);
    let milestone = format!(
        "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} hand={:?} \
         sequence={} damage={:.1} policy={} generation={} clamped={}",
        attacker_name,
        victim_username.as_str(),
        event.hand,
        event.sequence,
        decision.damage,
        decision.policy_id,
        decision.generation,
        decision.clamped
    );
    info!("{}", milestone);
    println!("{}", milestone);
    let hit = format!(
        "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} \
         victim_health_before={:.1} victim_health_after={:.1} policy={} generation={} \
         clamped={}",
        attacker_name,
        victim_username.as_str(),
        before,
        victim_health.0,
        decision.policy_id,
        decision.generation,
        decision.clamped
    );
    info!("{}", hit);
    println!("{}", hit);
}
```

## Receipt summaries

Positive receipt: `docs/evidence/valence-combat-loop-steel-policy-live-equivalent-2026-05-28.receipt.json`.

- protocol: `763`
- policy source: restricted Steel-compatible literal subset
- config paths: `combat.arrow.base_damage`, `combat.arrow.velocity_multiplier`, `combat.arrow.max_damage`
- non-default `arrow-base-damage`: `4.0`
- call sites:
  - `valence_ctf.combat_event_projectile_probe`: damage `4.0`, health `20.0 -> 16.0`, clamped `false`
  - `valence_ctf.projectile_interaction_probe`: damage `4.0`, health `20.0 -> 16.0`, clamped `false`

Negative receipt: `docs/evidence/valence-combat-loop-steel-policy-negative-reload-2026-05-28.receipt.json`.

- active policy before rejection: generation `0`, base damage `3.0`
- rejected candidate kinds: range-invalid, malformed-policy, capability-token-invalid
- active generation after each rejection: `0`
- post-reject decision: damage `3.0`, victim health after `17.0`

## Checker coverage

`tools/check_runtime_steel_config.rs` validates that each TSV call-site span contains the policy helper, projectile use/hit milestones, and milestone fields for damage, victim health before/after, policy id, generation, and clamped flag.
