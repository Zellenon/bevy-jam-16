use bevy::prelude::*;

use super::StatusEffect;

#[derive(Clone, Debug, Reflect, PartialEq, Eq)]
pub enum AttackType {
    EntireCell(Vec<AttackEffect>),
    Contact(Vec<AttackEffect>),
    DropsLiquid(LiquidType),
    ModifiesSelf,
}

#[derive(Clone, Debug, Reflect, PartialEq, Eq)]
pub enum AttackEffect {
    Damage(DamageType),
    Push,
    Status(StatusEffect),
}

#[derive(Component, Clone, Debug, Reflect, PartialEq, Eq)]
pub struct Lifetime(pub Timer);

#[derive(Component, Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub struct Droplet(pub LiquidType);

#[derive(Component, Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub struct Puddle(pub LiquidType);

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum LiquidType {
    Water,
    Oil,
    Acid,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Burning,
    Cold,
    Lightning,
    Chemical,
}

impl DamageType {
    pub fn status_effect(&self) -> StatusEffect {
        match self {
            DamageType::Physical => StatusEffect::Slowed,
            DamageType::Burning => StatusEffect::Burned,
            DamageType::Cold => StatusEffect::Frozen,
            DamageType::Lightning => StatusEffect::Electrified,
            DamageType::Chemical => StatusEffect::Acidic,
        }
    }
}

impl Lifetime {
    pub fn new(duration: f32) -> Self {
        Lifetime(Timer::from_seconds(duration, bevy::time::TimerMode::Once))
    }
}
