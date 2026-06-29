pub fn redstone_power_transition(powered: bool) -> bool {
    !powered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redstone_transition_toggles_powered_state() {
        assert!(redstone_power_transition(false));
        assert!(!redstone_power_transition(true));
    }
}
