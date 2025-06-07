use bevy::prelude::*;

use super::{
    StatusEffect,
    projectiles::{AttackEffect, AttackType, DamageType, LiquidType},
    status_effects::StatusEnum,
};

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum Tower {
    Piston,
    Fan,
    SpikePit,
    Oil,
    TrapDoor,
    Ice,
    Acid,
    Tesla,
    Water,
    Flame,
    Portal,
}

impl Tower {
    pub fn all() -> Vec<Tower> {
        vec![
            Tower::Piston,
            Tower::Fan,
            Tower::SpikePit,
            Tower::Oil,
            Tower::TrapDoor,
            Tower::Ice,
            Tower::Acid,
            Tower::Tesla,
            Tower::Water,
            Tower::Flame,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Tower::Piston => "Piston",
            Tower::Fan => "Fan",
            Tower::SpikePit => "Spike Pit",
            Tower::Oil => "Oil",
            Tower::TrapDoor => "Trap Door",
            Tower::Ice => "Ice",
            Tower::Acid => "Acid",
            Tower::Tesla => "Tesla Turret",
            Tower::Water => "Water Bucket",
            Tower::Flame => "Flame",
            Tower::Portal => "Portal",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Tower::Piston => "Pushes or crushes enemies with strong mechanical force.",
            Tower::Fan => "Slows enemies or redirects airborne effects using wind.",
            Tower::SpikePit => "Triggers hidden spikes to damage passing enemies.",
            Tower::Oil => "Covers the ground in oil, slowing enemies and increasing flammability.",
            Tower::TrapDoor => "Drops enemies through the floor, removing or rerouting them.",
            Tower::Ice => "Freezes the ground, significantly slowing movement speed.",
            Tower::Acid => "Applies corrosive damage over time to enemies it contacts.",
            Tower::Tesla => "Fires arcs of electricity that chain between nearby enemies.",
            Tower::Water => "Slows enemies and can clear or interact with terrain effects.",
            Tower::Flame => "Deals continuous fire damage to enemies in range.",
            Tower::Portal => "Teleports enemies backward along their path.",
        }
    }

    pub fn price(&self) -> i32 {
        match self {
            Tower::Piston => 100,
            Tower::Fan => 50,
            Tower::SpikePit => 75,
            Tower::Oil => 150,
            Tower::TrapDoor => 200,
            Tower::Ice => 125,
            Tower::Acid => 175,
            Tower::Tesla => 300,
            Tower::Water => 80,
            Tower::Flame => 120,
            Tower::Portal => 400,
        }
    }

    pub fn ui_asset_key(&self) -> &'static str {
        match self {
            Tower::Piston => "icon_piston",
            Tower::Fan => "icon_fan",
            Tower::SpikePit => "icon_spike_pit",
            Tower::Oil => "icon_oil",
            Tower::TrapDoor => "icon_trapdoor",
            Tower::Ice => "icon_ice",
            Tower::Acid => "icon_acid",
            Tower::Tesla => "icon_tesla",
            Tower::Water => "icon_water_bucket",
            Tower::Flame => "icon_flame",
            Tower::Portal => "icon_portal",
        }
    }

    pub fn has_trigger_zone(&self) -> bool {
        match self {
            Tower::Fan | Tower::SpikePit => false,
            _ => true,
        }
    }

    pub fn gravity_influences_trigger(&self) -> bool {
        match self {
            Tower::Oil | Tower::Acid | Tower::Water => true,
            _ => false,
        }
    }

    pub fn custom_trigger_zone(&self) -> Option<Vec2> {
        match self {
            Tower::TrapDoor => Some(Vec2::new(10.0, 10.0)),
            _ => None,
        }
    }

    pub fn attack_def(&self) -> AttackType {
        match self {
            Tower::Piston => AttackType::EntireCell(vec![
                AttackEffect::Damage(DamageType::Physical),
                AttackEffect::Push,
            ]),
            Tower::Fan => AttackType::EntireCell(vec![AttackEffect::Push]),
            Tower::SpikePit => {
                AttackType::Contact(vec![AttackEffect::Damage(DamageType::Physical)])
            }
            Tower::Oil => AttackType::DropsLiquid(LiquidType::Oil),
            Tower::TrapDoor => AttackType::ModifiesSelf,
            Tower::Ice => AttackType::EntireCell(vec![
                AttackEffect::Damage(DamageType::Cold),
                AttackEffect::Status(StatusEnum::Chilled),
            ]),
            Tower::Acid => AttackType::DropsLiquid(LiquidType::Acid),
            Tower::Tesla => {
                AttackType::EntireCell(vec![AttackEffect::Damage(DamageType::Lightning)])
            }
            Tower::Water => AttackType::DropsLiquid(LiquidType::Water),
            Tower::Flame => AttackType::EntireCell(vec![AttackEffect::Damage(DamageType::Burning)]),
            Tower::Portal => todo!(),
        }
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            Tower::Piston => 2.0,
            Tower::Fan => 0.,
            Tower::SpikePit => 0.32,
            Tower::Oil => 2.0,
            Tower::TrapDoor => 1.5,
            Tower::Ice => 0.67,
            Tower::Acid => 2.0,
            Tower::Tesla => 0.67,
            Tower::Water => 2.0,
            Tower::Flame => 0.67,
            Tower::Portal => 3.0,
        }
    }

    pub fn requires_adjecent_wall(&self) -> bool {
        match self {
            Tower::TrapDoor => true,
            _ => false,
        }
    }

    pub fn requires_floor_placement(&self) -> bool {
        match self {
            Tower::TrapDoor => true,
            _ => false,
        }
    }
}

#[derive(Component, Default)]
pub struct TowerCollision {
    pub slowdown: f32,
    pub damage: isize,
    pub iframe: Timer,
}

impl TowerCollision {
    fn new(slowdown: f32, damage: isize, iframe_time_s: f32) -> Self {
        Self {
            damage,
            slowdown,
            iframe: Timer::from_seconds(iframe_time_s, TimerMode::Repeating),
        }
    }
}

// These towers by themselves will cause damage upon collision
pub fn get_collision(tower: &Tower) -> Option<TowerCollision> {
    match tower {
        Tower::SpikePit => Some(TowerCollision::new(0.0, 15, 0.75)),
        _ => None,
    }
}
