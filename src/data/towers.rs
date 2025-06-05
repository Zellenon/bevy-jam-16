use crate::level::resource::CellDirection;
use bevy::prelude::*;

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
            Tower::Fan | Tower::SpikePit | Tower::Ice => false,
            _ => true,
        }
    }

    pub fn gravity_influences_trigger(&self) -> bool {
        match self {
            Tower::Oil | Tower::Ice | Tower::Acid | Tower::Water => true,
            _ => false,
        }
    }
}

#[derive(Component, Default)]
pub struct TowerCollision {
    pub slowdown: f32,
    pub min_damage: f32,
    pub max_damage: f32,
    pub iframe: Timer,
}

impl TowerCollision {
    fn new(slowdown: f32, min_damage: f32, max_damage: f32, iframe_time_s: f32) -> Self {
        Self {
            min_damage,
            max_damage,
            slowdown,
            iframe: Timer::from_seconds(iframe_time_s, TimerMode::Repeating),
        }
    }
}

// These towers by themselves will cause damage upon collision
pub fn get_collison(tower: &Tower) -> TowerCollision {
    match tower {
        Tower::SpikePit => TowerCollision::new(0.0, 0.050, 0.150, 0.75),
        Tower::Acid => TowerCollision::new(0.0, 0.050, 0.100, 0.5),
        Tower::Flame => TowerCollision::new(0.0, 0.050, 0.100, 0.35),
        Tower::Ice => TowerCollision::new(0.0, 0.005, 0.010, 0.20),
        _ => TowerCollision::new(0.0, 0.0, 0.0, 1.0),
    }
}
