use super::common::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum FloatValue {
    Fixed(f32),
    Random(f32, f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DurationValue {
    Indeterminate,
    Fixed(f32),
    Random(f32, f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TeamMember {
    Captain,
    ScooperOne,
    ScooperTwo,
    Server,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MemberCount {
    One,
    Two,
    Three,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FilterQuantity {
    WholeTeam,
    One(TeamMember),
    Two(TeamMember, TeamMember),
    Three(TeamMember, TeamMember, TeamMember),
    Random(MemberCount),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TeamSide {
    Partner,
    Adversary,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectFilter {
    Carrier,
    Team(TeamSide, FilterQuantity),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectCondition {
    Alone,
    CombinedWithFlavor(Vec<FlavorIndex>),
    CombinedWithClass(Vec<FlavorClass>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScreenEffect {
    CreamClouds,
    Dizzy,
    Pulse(Color),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectDefinition {
    OrderTotalScore(FloatValue),
    Speed(EffectFilter, FloatValue, DurationValue),
    OrderMeltTimer(FloatValue),
    GlobalMeltSpeed(TeamSide, FloatValue),
    PowerMeterFlatBonus(TeamSide, FloatValue),
    BlockSpecial(DurationValue),
    Screen(ScreenEffect, TeamSide, DurationValue),
    Negate,
}
