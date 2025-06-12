use crate::screens::Screen;
use crate::theme::widget;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<DisplayFlashMessage>();
    app.add_observer(show_message);
    app.add_systems(Update, fade_message);
}

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct DisplayFlashMessage(pub String);

impl DisplayFlashMessage {
    pub fn new<I: Into<String>>(input: I) -> Self {
        Self(input.into())
    }
}

#[derive(Component)]
pub struct FlashMessage {
    ttl: Timer,
}

fn show_message(
    trigger: Trigger<DisplayFlashMessage>,
    flash_message: Query<&ChildOf, With<FlashMessage>>,
    mut commands: Commands,
) {
    let message = &trigger.0;
    info!(message);

    if let Ok(entity) = flash_message.single() {
        commands.entity(entity.0).despawn();
    }

    commands.spawn((
        StateScoped(Screen::Gameplay),
        BackgroundColor(tailwind::SLATE_100.into()),
        Node {
            width: Val::Percent(75.0),
            top: Val::Percent(40.0),
            display: Display::Flex,
            left: Val::Percent(12.5),
            padding: UiRect::axes(Val::Px(24.0), Val::Px(8.0)),
            border: UiRect::bottom(Val::Px(4.0)),
            ..default()
        },
        children![(
            FlashMessage {
                ttl: Timer::from_seconds(1.0, TimerMode::Once),
            },
            Node {
                column_gap: Val::Px(32.0),
                display: Display::Flex,
                align_content: AlignContent::Center,
                ..default()
            },
            children![widget::ui_font(message)]
        )],
    ));
}

fn fade_message(
    time: Res<Time>,
    mut flash_messages: Query<(&ChildOf, &mut FlashMessage)>,
    mut nodes: Query<&mut BackgroundColor>,
    mut commands: Commands,
) {
    for (parent, mut flash_message) in flash_messages.iter_mut() {
        flash_message.ttl.tick(time.delta());

        let Ok(mut bg) = nodes.get_mut(parent.0) else {
            continue;
        };

        if flash_message.ttl.finished() {
            commands.entity(parent.0).despawn();
            continue;
        }

        bg.0.set_alpha(flash_message.ttl.fraction_remaining());
    }
}
