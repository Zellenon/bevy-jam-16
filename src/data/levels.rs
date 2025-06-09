use crate::gameplay::wave_manager::Wave;
use crate::prefabs::enemies::{basic_trooper, chonkus_trooper, turbo_trooper};
use bevy::prelude::*;
use std::collections::VecDeque;

#[allow(unused)]
pub const MAP_TEXT: &'static str = ">>>
";

pub const MAP_TEXT1: &'static str = "^<<v<
>>^v^
^<<<^
>>>>^
";

pub const MAP_TEXT3: &'static str = "^<<<<
>>>>^
^<<<<
>>>>^
";

pub const MAP_TEXT2: &'static str = "^<<<<<<<<<<<<<
>>>>>>>>>>>>>^";

pub const MAP_TEXT4: &'static str = "^v<<<v<
^<>v^<^
>>^>>>^
";

pub const MAP_TEXT5: &'static str = "^<<<<
>v>v^
^>^>^
";

pub const MAP_TEXT6: &'static str = ">v>^v<
^v^v<^
^v^v>^
^v^<^<
^>>>>^
";

pub const MAP_TEXT7: &'static str = "^>>>>>v
x^v<<<v
x^v>>^v
^<<^<<v
>>>>v^v
^<<<v^v
>>>^>^>
";

#[derive(Resource, Clone)]
pub struct LevelData {
    pub maps: Vec<&'static str>,
    pub enemies: Vec<VecDeque<Wave>>,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            maps: vec![
                MAP_TEXT1, MAP_TEXT2, MAP_TEXT3, MAP_TEXT4, MAP_TEXT5, MAP_TEXT6, MAP_TEXT7,
            ],
            enemies: vec![map1(), map2(), map3(), map4(), map5(), map6(), map7()],
        }
    }
}

pub fn map1() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![basic_trooper()], 2.),
            (vec![basic_trooper(), turbo_trooper()], 0.),
        ]
        .into(),
        // Wave 2
        vec![
            (vec![basic_trooper(), turbo_trooper()], 2.),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![basic_trooper()], 2.),
            (vec![basic_trooper(), turbo_trooper()], 0.),
        ]
        .into(),
        // Wave 4
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 2.),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
        ]
        .into(),
    ]
    .into()
}

pub fn map2() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper()], 2.),
            (vec![basic_trooper(), turbo_trooper()], 0.),
        ]
        .into(),
        //
        // Wave 2
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ]
        .into(),
        // Wave 4
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ]
        .into(),
    ]
    .into()
}

pub fn map3() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
        ]
        .into(),
        // Wave 2
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 4
        vec![
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.0),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
    ]
    .into()
}

pub fn map4() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
        ]
        .into(),
        // Wave 2
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
        ]
        .into(),
        // Wave 4
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 5
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 6
        vec![
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
    ]
    .into()
}

pub fn map5() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
        ]
        .into(),
        // Wave 2
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 4
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 5
        vec![
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 6
        vec![
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 7
        vec![
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
    ]
    .into()
}

pub fn map6() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
        ]
        .into(),
        // Wave 2
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 4
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 5
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 5
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 6
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
    ]
    .into()
}

pub fn map7() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
        ]
        .into(),
        // Wave 2
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 4
        vec![
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 1.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 5
        vec![
            (vec![turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 6
        vec![
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
        // Wave 6
        vec![
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                0.5,
            ),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![chonkus_trooper(), basic_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
            (
                vec![chonkus_trooper(), basic_trooper(), turbo_trooper()],
                1.5,
            ),
            (vec![chonkus_trooper()], 1.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 1.5),
            (vec![turbo_trooper()], 2.5),
            (vec![basic_trooper(), turbo_trooper()], 1.0),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![basic_trooper()], 0.5),
            (vec![turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.75),
            (vec![turbo_trooper()], 0.5),
        ]
        .into(),
    ]
    .into()
}

/*

(vec![chonkus_trooper(), basic_trooper(), turbo_trooper()], 0.5),

(vec![chonkus_trooper(), basic_trooper()], 0.5),

(vec![chonkus_trooper()], 0.5),
(vec![ basic_trooper(), turbo_trooper()], 1.5),
(vec![ turbo_trooper()], 2.5),


(vec![ basic_trooper(), turbo_trooper()], 1.0),
(vec![ basic_trooper(), turbo_trooper()], 0.75),

(vec![ basic_trooper()], 0.5),

(vec![ turbo_trooper()], 0.5),
*/
