use crate::assets::SoundEffects;
use crate::audio::sound_effect;
use crate::data::levels::LevelData;
use crate::gameplay::wave_manager::WaveManager;
use crate::level::resource::{CurrentLoadedLevel, GotoNextLevel, LevelSelect, UnlockedLevels};
use crate::{
    assets::LevelAssets,
    data::PlayerState,
    demo::enemy_health::EnemyHealth,
    level::{
        components::{EndNode, LevelParent},
        resource::Level,
    },
    menus::Menu,
    prelude::*,
    screens::Screen,
};
use avian2d::prelude::{Collider, Sleeping};
use bevy::prelude::*;
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    wrappers::name,
};
use bevy_turborand::GlobalRng;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_enemy_on_goal
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(PreUpdate, pause_physics.run_if(in_state(Menu::Pause)));
    app.add_systems(PreUpdate, unpause_physics.run_if(in_state(Menu::None)));

    app.add_systems(Update, unlock_next_level.run_if(in_state(Screen::Gameplay)));
    app.add_systems(Update, goto_next_level.run_if(on_event::<GotoNextLevel>));
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
    level_select: Res<LevelSelect>,
    level_data: Res<LevelData>,
    level_assets: Res<LevelAssets>,
    rng: ResMut<GlobalRng>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let level_index = level_select.0;

    let Some(level_layout) = level_data.maps.get(level_index).cloned() else {
        next_screen.set(Screen::Title);
        return;
    };

    *level = Level::from_str(level_layout);
    commands.compose(
        LevelParent::from_data(&level, &level_assets, rng)
            + name("Level Parent")
            + StateScoped(Screen::Gameplay).store(),
    );
}

pub fn unlock_next_level(
    wave_manager: Res<WaveManager>,
    level_select: Res<LevelSelect>,
    current_loaded_level: Res<CurrentLoadedLevel>,
    mut unlocked_levels: ResMut<UnlockedLevels>,
    enemies: Query<(), With<EnemyHealth>>,
) {
    if current_loaded_level.0 != level_select.0 {
        return;
    }
    if wave_manager.remaining_waves() == 0
        && wave_manager.current_wave.is_none()
        && enemies.iter().len() == 0
    {
        let next_level = level_select.0 + 1;
        if !unlocked_levels.0.contains(&next_level) {
            unlocked_levels.0.push(next_level);
        }
    }
}

pub fn goto_next_level(
    mut _event: EventReader<GotoNextLevel>,
    mut level_select: ResMut<LevelSelect>,
    mut next_screen: ResMut<NextState<Screen>>,
    enemies: Query<(), With<EnemyHealth>>,
) {
    if enemies.iter().len() == 0 {
        level_select.0 += 1;
        next_screen.set(Screen::LevelTransition);
    }
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
