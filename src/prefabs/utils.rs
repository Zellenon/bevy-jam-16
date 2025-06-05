use bevy::{
    app::{Plugin, Update},
    asset::{Asset, Assets, Handle},
    ecs::system::IntoObserverSystem,
    image::TextureAtlasLayout,
    math::UVec2,
    prelude::{Bundle, Commands, Component, Event, Image, OnAdd, Query, Res, Trigger},
    reflect::Reflect,
    render::{
        mesh::{Mesh, Mesh2d},
        view::Visibility,
    },
    sprite::{ColorMaterial, MeshMaterial2d, Sprite},
    time::Time,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};
use std::sync::Arc;

use super::{enemies::ShowDelay, wizardry::GimmieFn};
use crate::{
    assets::{GameAssets, TowerSprites},
    data::Tower,
    level::resource::CellDirection,
};

#[derive(Component, Clone)]
pub struct GiveMeImage(pub Arc<dyn GimmieFn<Image, GameAssets>>);

#[derive(Component, Clone)]
pub struct GiveMeLayout(pub Arc<dyn GimmieFn<TextureAtlasLayout, GameAssets>>);

#[derive(Component, Clone)]
pub struct GiveMeMesh(pub Arc<dyn GimmieFn<Mesh, GameAssets>>);

#[derive(Component, Clone)]
pub struct GiveMeColor(pub Arc<dyn GimmieFn<ColorMaterial, GameAssets>>);

#[derive(Component, Clone, Debug, Reflect)]
pub struct TowerSprite(pub Tower, pub CellDirection);

pub fn image(image: impl GimmieFn<Image, GameAssets>) -> ComponentTree {
    GiveMeImage(Arc::new(image)).store()
}

pub fn layout(layout: impl GimmieFn<TextureAtlasLayout, GameAssets>) -> ComponentTree {
    GiveMeLayout(Arc::new(layout)).store()
}

pub fn mesh(mesh: impl GimmieFn<Mesh, GameAssets>) -> ComponentTree {
    GiveMeMesh(Arc::new(mesh)).store()
}

pub fn color(color: impl GimmieFn<ColorMaterial, GameAssets>) -> ComponentTree {
    GiveMeColor(Arc::new(color)).store()
}

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_observer(give_images);
    app.add_observer(give_layouts);
    app.add_observer(give_meshes);
    app.add_observer(give_colors);
    app.add_observer(give_tower_sprite);
    app.add_systems(Update, show_delay);
}

pub fn show_delay(mut query: Query<(&mut Visibility, &mut ShowDelay)>, time: Res<Time>) {
    for (mut vis, mut debug) in query.iter_mut() {
        debug.0.tick(time.delta());
        if debug.0.just_finished() {
            *vis = Visibility::Visible;
        }
    }
}

pub fn give_images(
    trigger: Trigger<OnAdd, GiveMeImage>,
    images: Res<GameAssets>,
    requests: Query<&GiveMeImage>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    commands
        .get_entity(entity)
        .unwrap()
        .insert(Sprite {
            image: requests.get(entity).unwrap().0(&images),
            ..Default::default()
        })
        .remove::<GiveMeImage>();
}

pub fn give_layouts(
    trigger: Trigger<OnAdd, GiveMeLayout>,
    assets: Res<GameAssets>,
    mut requests: Query<(&GiveMeLayout, &mut Sprite)>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    if let Ok((gimmie, mut sprite)) = requests.get_mut(entity) {
        sprite.texture_atlas = Some(gimmie.0(&assets).into());
        commands
            .get_entity(entity)
            .unwrap()
            .remove::<GiveMeLayout>();
    }
}

pub fn give_meshes(
    trigger: Trigger<OnAdd, GiveMeMesh>,
    meshes: Res<GameAssets>,
    requests: Query<&GiveMeMesh>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    commands
        .get_entity(entity)
        .unwrap()
        .insert(Mesh2d(requests.get(entity).unwrap().0(&meshes)))
        .remove::<GiveMeMesh>();
}

pub fn give_colors(
    trigger: Trigger<OnAdd, GiveMeColor>,
    colors: Res<GameAssets>,
    requests: Query<&GiveMeColor>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    commands
        .get_entity(entity)
        .unwrap()
        .insert(MeshMaterial2d(requests.get(entity).unwrap().0(&colors)))
        .remove::<GiveMeColor>();
}

pub fn give_tower_sprite(
    trigger: Trigger<OnAdd, TowerSprite>,
    sprites: Res<TowerSprites>,
    requests: Query<&TowerSprite>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    let tower = requests.get(entity).unwrap();
    commands
        .get_entity(entity)
        .unwrap()
        .insert(sprites.tower_bundle(&tower.0, &tower.1.into()))
        .remove::<TowerSprite>();
}
