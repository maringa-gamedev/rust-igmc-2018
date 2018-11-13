use amethyst::ecs::prelude::*;

#[derive(Debug)]
pub enum FlavorClass {
    Classic,
    Sherbert,
    Special,
}

#[derive(Debug)]
pub enum TeamMember {
    Captain,
    ScooperOne,
    ScooperTwo,
    Server,
}

#[derive(Debug)]
pub enum MemberCount {
    One,
    Two,
    Three,
}

#[derive(Debug)]
pub enum FilterQuantity {
    WholeTeam,
    One(TeamMember),
    Two(TeamMember, TeamMember),
    Three(TeamMember, TeamMember, TeamMember),
    Random(MemberCount),
}

#[derive(Debug)]
pub enum EffectFilter {
    Partner(FilterQuantity),
    Adversary(FilterQuantity),
}

pub type FlavorIndex = usize;
pub type ToppingIndex = usize;
pub type PreparationIndex = usize;

#[derive(Debug)]
pub enum EffectCondition {
    Alone,
    CombinedWith(Vec<FlavorIndex>),
}

#[derive(Debug)]
pub enum FlavorEffect {
    OrderTotalScore(f32),
    Speed(EffectFilter, f32),
    OrderMeltTimer(f32),
    GlobalMeltSpeed(EffectFilter, f32),
    PowerMeterFlatBonus(f32),
    Negate,
}

#[derive(Debug)]
pub struct Order {
    pub flavor_a: FlavorIndex,
    pub flavor_b: FlavorIndex,
    pub flavor_c: FlavorIndex,
    pub flavor_d: FlavorIndex,
    pub topping: ToppingIndex,
    pub preparation: PreparationIndex,
    pub base_worth: f32,
    pub calc_worth: f32,
    pub delivery_timer: f32,
}

#[derive(Debug)]
pub struct Flavor {
    pub index: FlavorIndex,
    pub base_worth: f32,
    pub effect: Vec<FlavorEffect>,
    pub condition: Vec<EffectCondition>,
    pub class: FlavorClass,
}

#[derive(Debug)]
pub struct Team {
    pub captain: Entity,
    pub scooper_one: Option<Entity>,
    pub scooper_two: Option<Entity>,
    pub server: Entity,
    pub score: isize,
    pub power_meter: f32,
    pub loadout: Vec<Flavor>,
    pub orders: Vec<Order>,
}

#[derive(Debug, Default)]
pub struct Match {
    pub teams: Vec<Team>,
    pub loadout: Vec<Flavor>,
}
