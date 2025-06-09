use super::resource::{CellDirection, Level};
use crate::assets::LevelAssets;
use crate::gameplay::animation::AnimationFrameQueue;
use crate::prefabs::physics::GamePhysicsLayer as GPL;
use avian2d::prelude::{Collider, CollisionLayers, Friction, RigidBody};
use bevy::prelude::*;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_turborand::{DelegatedRng, GlobalRng};
use std::f32::consts::PI;

pub const WALL_TOTAL_WIDTH: f32 = 0.10;
pub const WALL_PICKABLE_WIDTH: f32 = 0.20;
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
pub struct PathNode {
    pub direction: CellDirection,
    pub prev_direction: CellDirection,
}

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
                            WallDirection::Right,
                        ) + Adjacent::new(
                            x,
                            y,
                            GeneralPosition::LeftRight,
                            ExactPosition::Wall(WallDirection::Right),
                        )
                        .store()
                        << wall(
                            level_assets,
                            (x as f32 - 0.5 + (WALL_TOTAL_WIDTH / 4.)) * LEVEL_SCALING,
                            y as f32 * LEVEL_SCALING,
                            WallDirection::Left,
                        ) + Adjacent::new(
                            x,
                            y,
                            GeneralPosition::LeftRight,
                            ExactPosition::Wall(WallDirection::Left),
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
        let mut last_direction = start_node.1;
        level = level
            << (node(
                start_node.0.x * LEVEL_SCALING,
                start_node.0.y * LEVEL_SCALING,
                start_node.1,
                last_direction,
            ) + StartNode.store()
                + AnimationFrameQueue::new(&[0, 1, 2, 3, 4]).store()
                + Sprite {
                    image: level_assets.enemy_spawner.clone(),
                    texture_atlas: Some(TextureAtlas::from(level_assets.spawner_layout.clone())),
                    custom_size: Some(Vec2::splat(LEVEL_SCALING * 0.8)),
                    color: Color::WHITE.with_alpha(0.95),
                    ..default()
                }
                .store());
        let mut path_iter = path_iter.rev();
        let last_node = path_iter.next().unwrap();
        let path_iter = path_iter.rev();

        level = level
            << (node(
                last_node.0.x * LEVEL_SCALING,
                last_node.0.y * LEVEL_SCALING,
                last_node.1,
                last_node.1,
            ) + EndNode.store());

        for node_i in path_iter {
            let (pos, direction) = node_i;
            level = level
                << node(
                    pos.x * LEVEL_SCALING,
                    pos.y * LEVEL_SCALING,
                    *direction,
                    last_direction,
                );
            last_direction = *direction;
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
        Architecture,
        Pickable::default(),
        Collider::rectangle(WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING, LEVEL_SCALING),
        CollisionLayers::new(GPL::Level, [GPL::Enemy, GPL::Default, GPL::Projectiles]),
        RigidBody::Static,
        Friction::new(0.3),
        Name::new("Wall"),
        Visibility::Inherited,
        Wall(direction),
    )
        .store()
        + pickable_rect(LEVEL_SCALING, WALL_PICKABLE_WIDTH * LEVEL_SCALING)
        + pos(x, y)
        << (
            Transform::from_xyz(0.0, 0.0, 0.1).with_rotation(Quat::from_rotation_z(PI / 2.0)),
            Pickable::IGNORE,
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
        Friction::new(0.),
        Name::new("Ceiling"),
        Visibility::Inherited,
        Ceiling,
    )
        .store()
        + pickable_rect(WALL_PICKABLE_WIDTH / 2. * LEVEL_SCALING, LEVEL_SCALING)
        + pos(x, y)
        << (
            Transform::from_xyz(0.0, -0.06, 0.0),
            Pickable::IGNORE,
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
        Friction::new(0.3),
        Name::new("Floor"),
        Visibility::Inherited,
        Floor,
    )
        .store()
        + pickable_rect(WALL_PICKABLE_WIDTH / 2. * LEVEL_SCALING, LEVEL_SCALING)
        + pos(x, y)
        << (
            Transform::from_xyz(0.0, 0.06, 0.0),
            Pickable::IGNORE,
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

pub fn node(
    x: f32,
    y: f32,
    direction: CellDirection,
    prev_direction: CellDirection,
) -> ComponentTree {
    PathNode::new(direction, prev_direction).store() + pos(x, y)
}

pub fn pickable_rect(h: f32, w: f32) -> ComponentTree {
    (
        Sprite {
            color: Color::WHITE.with_alpha(0.001),
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        Pickable::default(),
    )
        .store()
}

pub fn pos(x: f32, y: f32) -> ComponentTree {
    Transform::from_xyz(x, y, 0.).store()
}

impl PathNode {
    pub fn new(direction: CellDirection, prev_direction: CellDirection) -> Self {
        Self {
            direction,
            prev_direction,
        }
    }
}
