use crate::data::status_effects::StatusEffect;
use crate::theme::widget;
use bevy::ecs::relationship::RelatedSpawner;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
            Tower::Portal,
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

    pub fn tooltip(&self, parent: &mut RelatedSpawner<ChildOf>) {
        let _ = match self {
            Tower::Piston => {
                parent.spawn(widget::label("Smash your enemies with a piston!"));
            }
            Tower::Fan => {
                parent.spawn(widget::label("Blow your enemies away with a fan! Applies "));
                parent.spawn(StatusEffect::Pushed.ui_text());
            }
            _ => {
                parent.spawn(widget::label(format!("This is a {} tower.", self.name())));
            }
        };
    }
}
