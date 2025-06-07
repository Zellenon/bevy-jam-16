use super::status_effects::*;
use bevy::{color::palettes::css::*, prelude::*};

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
    Status(StatusEnum),
}

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
    pub fn color(&self) -> Color {
        match self {
            DamageType::Physical => GREY.into(),
            DamageType::Burning => ORANGE.into(),
            DamageType::Cold => AQUA.into(),
            DamageType::Lightning => YELLOW.into(),
            DamageType::Chemical => LIME.into(),
        }
    }
}

impl LiquidType {
    pub fn contact_effects(&self) -> Vec<AttackEffect> {
        match self {
            LiquidType::Water => vec![AttackEffect::Status(StatusEnum::Wet)],
            LiquidType::Oil => vec![AttackEffect::Status(StatusEnum::Oiled)],
            LiquidType::Acid => vec![
                AttackEffect::Damage(DamageType::Chemical),
                AttackEffect::Status(StatusEnum::Acidified),
            ],
        }
    }
}
