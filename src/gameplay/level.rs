use avian2d::prelude::{Collider, Sleeping};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    wrappers::name,
};
use bevy_turborand::GlobalRng;

use crate::assets::SoundEffects;
use crate::audio::sound_effect;
use crate::{
    assets::LevelAssets,
    audio::music,
    data::PlayerState,
    demo::enemy_health::EnemyHealth,
    level::{
        components::{EndNode, LEVEL_SCALING, LevelParent},
        resource::{Level, MAP_TEXT},
    },
    menus::Menu,
    prelude::*,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_enemy_on_goal
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(PreUpdate, pause_physics.run_if(in_state(Menu::Pause)));
    app.add_systems(PreUpdate, unpause_physics.run_if(in_state(Menu::None)));
}

fn pause_physics(mut commands: Commands, colliders: Query<Entity, With<Collider>>) {
    for entity in colliders {
        commands.entity(entity).insert(Sleeping);
    }
}

fn unpause_physics(mut commands: Commands, colliders: Query<Entity, With<Collider>>) {
    for entity in colliders {
        commands.entity(entity).remove::<Sleeping>();
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    mut level: ResMut<Level>,
    game_assets: Res<GameAssets>,
    level_assets: Res<LevelAssets>,
    rng: ResMut<GlobalRng>,
) {
    *level = Level::from_str(MAP_TEXT);
    commands
        .compose(
            LevelParent::from_data(&level, &level_assets, rng)
                + name("Level Parent")
                + StateScoped(Screen::Gameplay).store(),
        )
        .with_children(|p| {
            p.spawn((
                Name::new("Gameplay Music"),
                music(game_assets.tubamusic.clone()),
            ));
        });
}

pub fn despawn_enemy_on_goal(
    mut commands: Commands,
    mut game_state: ResMut<PlayerState>,
    enemies: Query<(Entity, &Transform), With<EnemyHealth>>,
    goal: Query<&Transform, With<EndNode>>,
    sfx: Res<SoundEffects>,
) {
    if let Ok(goal_pos) = goal.single() {
        let goal_pos = goal_pos.translation.xy();
        for (e, pos) in enemies.iter() {
            if pos.translation.xy().distance(goal_pos) < 7. {
                commands.get_entity(e).unwrap().despawn();
                game_state.health -= 1;
                commands.spawn(sound_effect(sfx.took_damage.clone()));
                println!("Damage Taken!");
            }
        }
    }
}
