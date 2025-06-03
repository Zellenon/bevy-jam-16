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
}
