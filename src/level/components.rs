use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::ecs::system::{Res, ResMut};
use bevy::image::TextureAtlas;
use bevy::picking::Pickable;
use bevy::{
    color::Color, ecs::component::Component, math::Vec2, prelude::info, reflect::Reflect,
    render::view::Visibility, sprite::Sprite, transform::components::Transform, utils::default,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_turborand::DelegatedRng;
use bevy_turborand::GlobalRng;

use crate::assets::{GameAssets, LevelAssets, game_assets};
use crate::prefabs::physics::GamePhysicsLayer as GPL;

use super::resource::{CellDirection, Level};

pub const WALL_TOTAL_WIDTH: f32 = 0.10;
pub const FLOOR_TOTAL_HEIGHT: f32 = 0.10;
pub const LEVEL_SCALING: f32 = 10.;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum WallDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct LevelParent;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Architecture;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Wall(pub WallDirection);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Floor;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Ceiling;

#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect)]
pub struct PathNode(pub CellDirection);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct StartNode;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct EndNode;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum GeneralPosition {
    UpDown,
    LeftRight,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum ExactPosition {
    Floor,
    Ceiling,
    Wall(WallDirection),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Adjacent {
    pub id: AdjacentId,
    pub exact_position: ExactPosition,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct AdjacentId {
    pub unit_x: usize,
    pub unit_y: usize,
    pub general_position: GeneralPosition,
}

impl Adjacent {
    pub fn new(
        unit_x: usize,
        unit_y: usize,
        position: GeneralPosition,
        exact: ExactPosition,
    ) -> Self {
        Self {
            id: AdjacentId {
                unit_x,
                unit_y,
                general_position: position,
            },
            exact_position: exact,
        }
    }
}

impl LevelParent {
    pub fn from_data(
        level_data: &Level,
        level_assets: &Res<LevelAssets>,
        mut rng: ResMut<GlobalRng>,
    ) -> ComponentTree {
        let mut level = (LevelParent, Transform::default(), Visibility::default()).store();

        for x in 0..=level_data.width as i32 {
            for y in 0..=level_data.height as i32 {
                level = level
                    << (
                        Sprite {
                            image: level_assets.floortiles.clone(),
                            custom_size: Some(Vec2::new(1.0 * LEVEL_SCALING, 1.0 * LEVEL_SCALING)),
                            texture_atlas: Some(TextureAtlas {
                                layout: level_assets.floortiles_layout.clone(),
                                index: rng.usize(0..8),
                            }),
                            ..default()
                        },
                        Transform::from_translation(
                            Vec2::new(
                                x as f32 * LEVEL_SCALING - LEVEL_SCALING / 2.,
                                y as f32 * LEVEL_SCALING - LEVEL_SCALING / 2.,
                            )
                            .extend(-10.0),
                        ),
                    )
                        .store();
            }
        }

        for x in 0..(level_data.width + 1) {
            for y in 0..level_data.height {
                if level_data.walls[x][y] {
                    level = level
                        << wall(
                            (x as f32 - 0.5 - (WALL_TOTAL_WIDTH / 4.)) * LEVEL_SCALING,
                            y as f32 * LEVEL_SCALING,
                            WallDirection::Left,
                        ) + Adjacent::new(
                            x,
                            y,
                            GeneralPosition::LeftRight,
                            ExactPosition::Wall(WallDirection::Left),
                        )
                        .store()
                        << wall(
                            (x as f32 - 0.5 + (WALL_TOTAL_WIDTH / 4.)) * LEVEL_SCALING,
                            y as f32 * LEVEL_SCALING,
                            WallDirection::Right,
                        ) + Adjacent::new(
                            x,
                            y,
                            GeneralPosition::LeftRight,
                            ExactPosition::Wall(WallDirection::Right),
                        )
                        .store();
                }
            }
        }
        for x in 0..level_data.width {
            for y in 0..(level_data.height + 1) {
                if level_data.floors[x][y] {
                    level = level
                        << ceiling(
                            x as f32 * LEVEL_SCALING,
                            ((y as f32) - 0.5 - FLOOR_TOTAL_HEIGHT / 4.) * LEVEL_SCALING,
                        ) + Adjacent::new(x, y, GeneralPosition::UpDown, ExactPosition::Ceiling)
                            .store()
                        << floor(
                            x as f32 * LEVEL_SCALING,
                            ((y as f32) - 0.5 + FLOOR_TOTAL_HEIGHT / 4.) * LEVEL_SCALING,
                        ) + Adjacent::new(x, y, GeneralPosition::UpDown, ExactPosition::Floor)
                            .store();
                }
            }
        }
        let mut path_iter = level_data.path.iter();
        let start_node = path_iter.next().unwrap();
        level = level
            << (node(
                start_node.0.x * LEVEL_SCALING,
                start_node.0.y * LEVEL_SCALING,
                start_node.1,
            ) + StartNode.store());
        let mut path_iter = path_iter.rev();
        let last_node = path_iter.next().unwrap();
        let path_iter = path_iter.rev();

        level = level
            << (node(
                last_node.0.x * LEVEL_SCALING,
                last_node.0.y * LEVEL_SCALING,
                last_node.1,
            ) + EndNode.store());

        for node_i in path_iter {
            let (pos, direction) = node_i;
            level = level << node(pos.x * LEVEL_SCALING, pos.y * LEVEL_SCALING, *direction);
        }

        level
    }
}

pub fn wall(x: f32, y: f32, direction: WallDirection) -> ComponentTree {
    (
        Wall(direction),
        Architecture,
        Pickable::default(),
        Collider::rectangle(WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING, LEVEL_SCALING),
        CollisionLayers::new(GPL::Level, [GPL::Enemy, GPL::Default, GPL::Projectiles]),
        RigidBody::Static,
    )
        .store()
        + name("Wall")
        + rect_sprite(
            x,
            y,
            LEVEL_SCALING,
            WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING,
            match direction {
                WallDirection::Left => Color::srgba(0.9, 0.1, 0.1, 1.0),
                WallDirection::Right => Color::srgba(0.8, 0.3, 0.0, 1.0),
            },
        )
}

pub fn ceiling(x: f32, y: f32) -> ComponentTree {
    (
        Ceiling,
        Architecture,
        Collider::rectangle(LEVEL_SCALING, WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING),
        CollisionLayers::new(GPL::Level, [GPL::Enemy, GPL::Default, GPL::Projectiles]),
        RigidBody::Static,
        Pickable::default(),
    )
        .store()
        + name("Ceiling")
        + rect_sprite(
            x,
            y,
            FLOOR_TOTAL_HEIGHT / 2. * LEVEL_SCALING,
            LEVEL_SCALING,
            Color::srgba(0.0, 0.2, 0.8, 1.0),
        )
}

pub fn floor(x: f32, y: f32) -> ComponentTree {
    (
        Floor,
        Architecture,
        Collider::rectangle(LEVEL_SCALING, WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING),
        CollisionLayers::new(GPL::Level, [GPL::Enemy, GPL::Default, GPL::Projectiles]),
        RigidBody::Static,
        Pickable::default(),
    )
        .store()
        + name("Floor")
        + rect_sprite(
            x,
            y,
            FLOOR_TOTAL_HEIGHT / 2. * LEVEL_SCALING,
            LEVEL_SCALING,
            Color::srgba(0.4, 0.4, 0.0, 1.0),
        )
}

pub fn rect_sprite(x: f32, y: f32, h: f32, w: f32, color: Color) -> ComponentTree {
    (
        Sprite {
            color,
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        Visibility::Visible,
    )
        .store()
        + pos(x, y)
}

pub fn node(x: f32, y: f32, direction: CellDirection) -> ComponentTree {
    (PathNode(direction)).store() + pos(x, y)
}

pub fn pos(x: f32, y: f32) -> ComponentTree {
    Transform::from_xyz(x, y, 0.).store()
}
