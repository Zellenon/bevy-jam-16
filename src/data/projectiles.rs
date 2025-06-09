use super::status_effects::*;
use crate::level::resource::CellDirection;
use bevy::{color::palettes::css, prelude::*};
use std::fmt::Display;
use std::marker::ConstParamTy_;
use std::marker::UnsizedConstParamTy;

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum TowerAttackType {
    EntireCell(Vec<AttackSpecification>),
    Contact(Vec<AttackSpecification>),
    DropsLiquid(LiquidType),
    ModifiesSelf,
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum AttackSpecification {
    Damage(DamageType, usize),
    Push(f32),
    Status(StatusEnum),
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum AttackData {
    Damage {
        dmg_type: DamageType,
        strength: usize,
        damage: usize,
    },
    Push {
        direction: CellDirection,
        strength: usize,
        force: f32,
    },
    Status {
        status: StatusEnum,
        strength: usize,
    },
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

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq, UnsizedConstParamTy)]
pub enum DamageType {
    Physical,
    Burning,
    Cold,
    Lightning,
    Chemical,
}

impl ConstParamTy_ for DamageType {}

impl DamageType {
    pub fn color(&self) -> Color {
        match self {
            DamageType::Physical => css::PINK.into(),
            DamageType::Burning => css::ORANGE.into(),
            DamageType::Cold => css::AQUA.into(),
            DamageType::Lightning => css::YELLOW.into(),
            DamageType::Chemical => css::LIME.into(),
        }
    }
}

impl Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            DamageType::Physical => "Physical",
            DamageType::Burning => "Burning",
            DamageType::Cold => "Cold",
            DamageType::Lightning => "Lightning",
            DamageType::Chemical => "Chemical",
        };
        write!(f, "{}", string)
    }
}

impl LiquidType {
    pub fn contact_effects(&self) -> Vec<AttackSpecification> {
        match self {
            LiquidType::Water => vec![AttackSpecification::Status(StatusEnum::Wet)],
            LiquidType::Oil => vec![AttackSpecification::Status(StatusEnum::Oiled)],
            LiquidType::Acid => vec![
                AttackSpecification::Damage(DamageType::Chemical, 10),
                AttackSpecification::Status(StatusEnum::Acidified),
            ],
        }
    }
}
