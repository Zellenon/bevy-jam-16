use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::ecs::system::{Res, ResMut};
use bevy::image::TextureAtlas;
use bevy::math::Quat;
use bevy::picking::Pickable;
use bevy::prelude::Name;
use bevy::{
    color::Color, ecs::component::Component, math::Vec2, prelude::info, reflect::Reflect,
    render::view::Visibility, sprite::Sprite, transform::components::Transform, utils::default,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_turborand::DelegatedRng;
use bevy_turborand::GlobalRng;
use std::f32::consts::PI;

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
                            level_assets,
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
                            level_assets,
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
                            level_assets,
                            x as f32 * LEVEL_SCALING,
                            ((y as f32) - 0.5 - FLOOR_TOTAL_HEIGHT / 4.) * LEVEL_SCALING,
                        ) + Adjacent::new(x, y, GeneralPosition::UpDown, ExactPosition::Ceiling)
                            .store()
                        << floor(
                            level_assets,
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

pub fn wall(
    level_assets: &Res<LevelAssets>,
    x: f32,
    y: f32,
    direction: WallDirection,
) -> ComponentTree {
    (
        Wall(direction),
        Architecture,
        Pickable::default(),
        Collider::rectangle(WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING, LEVEL_SCALING),
        CollisionLayers::new(GPL::Level, [GPL::Enemy, GPL::Default, GPL::Projectiles]),
        RigidBody::Static,
        Name::new("Wall"),
        Visibility::Inherited,
    )
        .store()
        + pos(x, y)
        << (
            Wall(direction),
            Pickable::default(),
            Transform::from_xyz(0.0, 0.0, 0.1).with_rotation(Quat::from_rotation_z(PI / 2.0)),
            Visibility::Inherited,
            Sprite {
                image: level_assets.level.clone(),
                custom_size: Some(Vec2::new(LEVEL_SCALING, LEVEL_SCALING / 16.)),
                texture_atlas: Some(TextureAtlas {
                    layout: level_assets.level_layout.clone(),
                    index: 2,
                }),
                ..default()
            },
        )
            .store()
}

pub fn ceiling(level_assets: &Res<LevelAssets>, x: f32, y: f32) -> ComponentTree {
    (
        Architecture,
        Collider::rectangle(LEVEL_SCALING, WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING),
        CollisionLayers::new(GPL::Level, [GPL::Enemy, GPL::Default, GPL::Projectiles]),
        RigidBody::Static,
        Name::new("Ceiling"),
        Visibility::Inherited,
    )
        .store()
        + pos(x, y)
        << (
            Ceiling,
            Transform::from_xyz(0.0, -0.06, 0.0),
            Pickable::default(),
            Visibility::Inherited,
            Sprite {
                image: level_assets.level.clone(),
                custom_size: Some(Vec2::new(LEVEL_SCALING, LEVEL_SCALING / 16.)),
                texture_atlas: Some(TextureAtlas {
                    layout: level_assets.level_layout.clone(),
                    index: 1,
                }),
                ..default()
            },
        )
            .store()
}

pub fn floor(level_assets: &Res<LevelAssets>, x: f32, y: f32) -> ComponentTree {
    (
        Architecture,
        Collider::rectangle(LEVEL_SCALING, WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING),
        CollisionLayers::new(GPL::Level, [GPL::Enemy, GPL::Default, GPL::Projectiles]),
        RigidBody::Static,
        Name::new("Floor"),
        Visibility::Inherited,
    )
        .store()
        + pos(x, y)
        << (
            Floor,
            Transform::from_xyz(0.0, 0.06, 0.0),
            Pickable::default(),
            Visibility::Inherited,
            Sprite {
                image: level_assets.level.clone(),
                custom_size: Some(Vec2::new(LEVEL_SCALING, LEVEL_SCALING / 16.)),
                texture_atlas: Some(TextureAtlas {
                    layout: level_assets.level_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
        )
            .store()
}

pub fn node(x: f32, y: f32, direction: CellDirection) -> ComponentTree {
    PathNode(direction).store() + pos(x, y)
}

pub fn pos(x: f32, y: f32) -> ComponentTree {
    Transform::from_xyz(x, y, 0.).store()
}
