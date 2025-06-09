use crate::define_status_effect;
use bevy::{color::palettes::css::*, prelude::*, reflect::GetTypeRegistration};
use std::marker::PhantomData;

use super::projectiles::DamageType;

pub trait StatusEffectTrait:
    'static + Send + Sync + Reflect + Copy + Sized + TypePath + GetTypeRegistration
{
    fn name() -> &'static str;
    fn color() -> Color;
    fn ui_text() -> impl Bundle {
        (Text::new(Self::name()), TextColor(Self::color()))
    }
    fn base_duration() -> f32;
    fn damage_element() -> DamageType;
    fn as_effect(strength: usize, duration: f32) -> StatusEffect<Self> {
        StatusEffect::new(strength, duration)
    }
    fn corresponding_enum() -> StatusEnum;
}

#[derive(Component, Eq, PartialEq, Debug, Reflect, Clone)]
pub struct StatusEffect<T: StatusEffectTrait> {
    pub strength: usize,
    pub duration: Timer,
    #[reflect(ignore)]
    _phantom: PhantomData<T>,
}

impl<T: StatusEffectTrait> StatusEffect<T> {
    pub fn new(strength: usize, duration: f32) -> StatusEffect<T> {
        StatusEffect {
            strength,
            duration: Timer::from_seconds(duration, TimerMode::Once),
            _phantom: PhantomData,
        }
    }
}

define_status_effect!(Wet, "Wet", BLUE.into(), 4., DamageType::Cold);
define_status_effect!(Ignited, "Ignited", RED.into(), 3., DamageType::Burning);
define_status_effect!(Burned, "Burned", ORANGE.into(), 4., DamageType::Burning);
define_status_effect!(Chilled, "Chilled", AQUA.into(), 3.0, DamageType::Cold);
define_status_effect!(Frozen, "Frozen", AQUA.into(), 3.0, DamageType::Cold);
define_status_effect!(
    Electrocuted,
    "Electrocuted",
    YELLOW.into(),
    0.9,
    DamageType::Lightning
);
define_status_effect!(
    Acidified,
    "Acidified",
    LIME.into(),
    4.0,
    DamageType::Chemical
);
define_status_effect!(Oiled, "Oiled", BROWN.into(), 4.0, DamageType::Chemical);

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq)]
pub enum StatusEnum {
    Wet,
    Ignited,
    Burned,
    Chilled,
    Frozen,
    Electrocuted,
    Acidified,
    Oiled,
}

#[macro_export]
macro_rules! define_status_effect {
    ( $structname:ident, $name:expr, $color:expr , $base_duration: expr, $element: expr) => {
        #[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
        pub struct $structname;

        impl StatusEffectTrait for $structname {
            fn name() -> &'static str {
                $name
            }

            fn color() -> Color {
                $color
            }

            fn base_duration() -> f32 {
                $base_duration
            }

            fn damage_element() -> DamageType {
                $element
            }

            fn corresponding_enum() -> StatusEnum {
                StatusEnum::$structname
            }
        }
    };
}

pub fn duration_multiplier(strength: usize) -> f32 {
    match strength {
        0 => 0.,
        1 => 1.,
        2 => 1.67,
        3 => 2.5,
        4 => 3.,
        _ => 4.,
    }
}

pub fn damage_multiplier(strength: usize) -> f32 {
    match strength {
        0 => 0.,
        1 => 0.67,
        2 => 1.25,
        3 => 2.0,
        4 => 2.67,
        _ => 4.,
    }
}
