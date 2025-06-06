use crate::demo::enemy_health::{DoDamageToEnemy, EnemyHealth};
use crate::prelude::*;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Gameplay), spawn_damage_container);
    app.add_systems(
        Update,
        (show_damage_numbers, animate_damage_numbers).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
struct DamageNumber;

#[derive(Component)]
struct DamageNumberLifetime {
    timer: Timer,
    velocity: Vec3,
}

// pub fn spawn_damage_container(mut commands: Commands) {
//     commands.spawn((StateScoped(Screen::Gameplay), DamageNumberContainer));
// }

pub fn show_damage_numbers(
    mut events: EventReader<DoDamageToEnemy>,
    enemies: Query<&Transform, With<EnemyHealth>>,
    // tag_parent: Single<Entity, With<DamageNumberContainer>>,
    mut rng: ResMut<GlobalRng>,

    mut commands: Commands,
) {
    for event in events.read() {
        let Ok(transform) = enemies.get(event.enemy) else {
            continue;
        };
        let mut translation = transform.translation.clone();
        translation.x += (rng.f32() - 0.5) * 3.0;
        translation.y += (rng.f32() - 0.5) * 2.0;

        commands.spawn((
            StateScoped(Screen::Gameplay),
            Text2d::new(((event.damage * 100.) as i32).to_string()),
            TextColor(event.damage_type.status_effect().color()),
            TextFont::from_font_size(18.0).with_font(BASE_FONT),
            DamageNumber,
            DamageNumberLifetime {
                timer: Timer::from_seconds(0.6, TimerMode::Once),
                velocity: Vec3::Y * 0.8, // upward movement
            },
            Transform::from_translation(translation).with_scale(Vec3::splat(0.1)),
        ));

        // println!("Damage numbers {:?}", transform.translation);
    }
}

pub fn animate_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut TextColor,
            &mut DamageNumberLifetime,
        ),
        With<DamageNumber>,
    >,
) {
    for (entity, mut transform, mut color, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());

        // Move upward
        transform.translation += lifetime.velocity * time.delta_secs();

        // Fade out
        let progress = lifetime.timer.elapsed_secs() / lifetime.timer.duration().as_secs_f32();
        let alpha = 1.0 - progress.clamp(0.0, 1.0); // Clamp just in case        let alpha = 1.0 - progress;
        color.0.set_alpha(alpha);
        // if let Some(section) = text.sections.get_mut(0) {
        //     section.style.color.set_a(alpha);
        // }

        // Despawn when done
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
