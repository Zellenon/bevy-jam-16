use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StatusEffect {
    Wet,
    Burning,
    Frozen,
    Electrified,
    Acidic,
    Oiled,
    Slowed,
    Pushed,
}

impl StatusEffect {
    pub fn name(&self) -> &'static str {
        match self {
            StatusEffect::Wet => "Wet",
            StatusEffect::Burning => "Burning",
            StatusEffect::Frozen => "Frozen",
            StatusEffect::Electrified => "Electrified",
            StatusEffect::Acidic => "Acidic",
            StatusEffect::Oiled => "Oiled",
            StatusEffect::Slowed => "Slowed",
            StatusEffect::Pushed => "Pushed",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            StatusEffect::Wet => BLUE.into(),
            StatusEffect::Burning => RED.into(),
            StatusEffect::Frozen => AQUA.into(),
            StatusEffect::Electrified => YELLOW.into(),
            StatusEffect::Acidic => GREEN.into(),
            StatusEffect::Oiled => BROWN.into(),
            StatusEffect::Slowed => PURPLE.into(),
            StatusEffect::Pushed => ORANGE.into(),
        }
    }

    pub fn ui_text(&self) -> impl Bundle {
        (Text::new(self.name()), TextColor(self.color()))
    }

    pub fn ailments(&self) -> Ailments {
        match self {
            StatusEffect::Wet => Ailments::new(0.0, 0.0, 0.0, 8., 8.),
            StatusEffect::Burning => Ailments::new(0.0, 0.005, 0.010, 7., 1.5),
            StatusEffect::Frozen => Ailments::new(1.0, 0.003, 0.005, 8., 2.),
            StatusEffect::Electrified => Ailments::new(0.0, 0.006, 0.012, 12., 1.),
            StatusEffect::Acidic => Ailments::new(0.0, 0.008, 0.016, 9., 2.5),
            StatusEffect::Oiled => Ailments::new(0.0, 0.0, 0.0, 15., 15.),
            StatusEffect::Slowed => Ailments::new(0.7, 0.0, 0.0, 10., 10.),
            StatusEffect::Pushed => Ailments::new(1.0, 0.0, 0.0, 0.25, 0.25),
        }
    }
}

#[derive(Component, Default)]
pub struct Ailments {
    slowdown: f32,
    min_damage: f32,
    max_damage: f32,
    damage_timer: Timer,
    ailment_timer: Timer,
}

impl Ailments {
    fn new(
        slowdown: f32,
        min_damage: f32,
        max_damage: f32,
        ailment_time_s: f32,
        damage_time_s: f32,
    ) -> Self {
        Self {
            slowdown,
            min_damage,
            max_damage,
            ailment_timer: Timer::from_seconds(ailment_time_s, TimerMode::Once),
            damage_timer: Timer::from_seconds(damage_time_s, TimerMode::Repeating),
        }
    }
}
