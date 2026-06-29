#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureGameMode {
    Survival,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureHand {
    Main,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureDirection {
    Up,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureDiggingState {
    Stop,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureInteraction {
    Attack,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureBlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureStack<'a> {
    pub item_name: &'a str,
    pub count: i8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureSlotChange<'a> {
    pub slot: i16,
    pub stack: FixtureStack<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureHungerProfile {
    pub event_prefix: &'static str,
    pub pre_health_tenths: i32,
    pub pre_food: i32,
    pub pre_saturation_tenths: i32,
    pub post_health_tenths: i32,
    pub post_food: i32,
    pub post_saturation_tenths: i32,
}
