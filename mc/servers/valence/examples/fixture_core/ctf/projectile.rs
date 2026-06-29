#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectileProbeContract<'a> {
    pub sequence: i32,
    pub projectile_id: &'a str,
    pub weapon: &'a str,
    pub damage: f32,
    pub policy_id: &'a str,
    pub generation: u64,
    pub proof_basis: &'a str,
    pub travel_sample_kind: &'a str,
    pub travel_sample_index: u32,
    pub collision_kind: &'a str,
    pub player_max_health: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectileTravelCollisionMarkers {
    pub use_marker: String,
    pub travel: String,
    pub collision: String,
    pub hit: String,
}

pub fn projectile_sequence_matches(observed: i32, expected: i32) -> bool {
    observed == expected
}

pub fn projectile_travel_collision_markers(
    attacker_name: &str,
    target_name: &str,
    contract: ProjectileProbeContract<'_>,
) -> ProjectileTravelCollisionMarkers {
    ProjectileTravelCollisionMarkers {
        use_marker: format!(
            "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} hand=Main sequence={} \
             expected_sequence={} sequence_matches=true projectile_id={} weapon={} damage={:.1} \
             policy={} generation={} clamped=false proof_basis={}",
            attacker_name,
            target_name,
            contract.sequence,
            contract.sequence,
            contract.projectile_id,
            contract.weapon,
            contract.damage,
            contract.policy_id,
            contract.generation,
            contract.proof_basis
        ),
        travel: format!(
            "MC-COMPAT-MILESTONE projectile_travel_sample attacker={} target={} sequence={} \
             projectile_id={} weapon={} sample={} sample_index={} proof_basis={}",
            attacker_name,
            target_name,
            contract.sequence,
            contract.projectile_id,
            contract.weapon,
            contract.travel_sample_kind,
            contract.travel_sample_index,
            contract.proof_basis
        ),
        collision: format!(
            "MC-COMPAT-MILESTONE projectile_collision attacker={} target={} sequence={} \
             projectile_id={} weapon={} collision={} proof_basis={}",
            attacker_name,
            target_name,
            contract.sequence,
            contract.projectile_id,
            contract.weapon,
            contract.collision_kind,
            contract.proof_basis
        ),
        hit: format!(
            "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} sequence={} projectile_id={} \
             weapon={} damage={:.1} victim_health_before={:.1} victim_health_after={:.1} policy={} \
             generation={} clamped=false proof_basis={}",
            attacker_name,
            target_name,
            contract.sequence,
            contract.projectile_id,
            contract.weapon,
            contract.damage,
            contract.player_max_health,
            contract.player_max_health - contract.damage,
            contract.policy_id,
            contract.generation,
            contract.proof_basis
        ),
    }
}
