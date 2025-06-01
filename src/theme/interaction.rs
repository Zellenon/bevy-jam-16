use crate::assets::ui_assets::UiAssets;
use crate::audio::sound_effect;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(Update, apply_interaction_palette);

    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn play_on_hover_sound_effect(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    interaction_assets: Option<Res<UiAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.target()) {
        commands.spawn(sound_effect(interaction_assets.button_hover_sound.clone()));
    }
}

fn play_on_click_sound_effect(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    interaction_assets: Option<Res<UiAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.target()) {
        commands.spawn(sound_effect(interaction_assets.button_click_sound.clone()));
    }
}
