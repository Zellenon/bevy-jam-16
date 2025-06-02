use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Tower {
    Piston,
    SpikePit,
    Oil,
    TrapDoor,
    Tesla,
    Water,
}
