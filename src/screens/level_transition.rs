//! A splash screen that plays briefly at startup.

use crate::assets::UiAssets;
use crate::data::levels::LevelData;
use crate::level::resource::{CurrentLoadedLevel, LevelSelect};
use crate::{screens::Screen, theme::prelude::*, AppSystems};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    // Spawn splash screen.
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));

    // Add splash timer.
    app.register_type::<ScreenTimer>();
    app.add_systems(OnEnter(Screen::LevelTransition), spawn_screen);
    app.add_systems(OnEnter(Screen::LevelTransition), insert_splash_timer);
    app.add_systems(OnExit(Screen::LevelTransition), remove_splash_timer);

    app.add_systems(
        Update,
        (
            tick_screen_timer.in_set(AppSystems::TickTimers),
            check_splash_timer.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::LevelTransition)),
    );

    // Exit the splash screen early if the player hits escape.
    app.add_systems(
        Update,
        skip_screen
            .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::LevelTransition))),
    );

    app.add_systems(
        Update,
        skip_screen
            .run_if(input_just_pressed(MouseButton::Left).and(in_state(Screen::LevelTransition))),
    );
    // // Animate splash screen.
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(AppSystems::TickTimers),
            apply_fade_in_out.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::LevelTransition)),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

fn spawn_screen(
    mut commands: Commands,
    assets: Res<UiAssets>,
    mut current_loaded_level: ResMut<CurrentLoadedLevel>,
    level_select: Res<LevelSelect>,
) {
    current_loaded_level.0 = level_select.0;

    if level_select.0 == 0 {
        commands.spawn((
            widget::ui_root("Controls"),
            GlobalZIndex(2),
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            StateScoped(Screen::LevelTransition),
            children![
                widget::ui_font_color("Controls:", Color::srgb(0.88, 0.88, 0.88)),
                widget::ui_font_color(
                    "Move Map: Middle Click and Drag",
                    Color::srgb(0.88, 0.88, 0.88)
                ),
                widget::ui_font_color(
                    "Zoom: Shift + Middle Click and Drag",
                    Color::srgb(0.88, 0.88, 0.88)
                ),
                widget::ui_font_color("Pause: Spacebar", Color::srgb(0.88, 0.88, 0.88)),
                widget::ui_font_color("Interact: Left Click to Select and Place Items", Color::srgb(0.88, 0.88, 0.88)),
                widget::ui_font_color("\n", Color::srgb(0.88, 0.88, 0.88)),
                widget::ui_font_color("Set up your towers and click 'Next Wave'.\nTry to set up towers of elements that react with other elements.\nFor example, setup water next to electcity for a bigger impact.\nGood luck! ", Color::srgb(0.88, 0.88, 0.88)),
                widget::ui_font_color("\n", Color::srgb(0.88, 0.88, 0.88)),
                widget::ui_font_color("Click to Continue", Color::srgb(0.88, 0.88, 0.88)),
            ],
        ));
    } else {
        commands.spawn((
            widget::ui_root("Splash Screen"),
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            StateScoped(Screen::LevelTransition),
            children![(
                Name::new("Splash image"),
                Node {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                assets.intro_bundle(),
                // ImageNode::new(asset_server.load("images/splash.png")),
                ImageNodeFadeInOut {
                    total_duration: SPLASH_DURATION_SECS,
                    fade_duration: SPLASH_FADE_DURATION_SECS,
                    t: 0.0,
                },
            )],
        ));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl ImageNodeFadeInOut {
    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_fade_in_out(time: Res<Time>, mut animation_query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

fn apply_fade_in_out(mut animation_query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct ScreenTimer(Timer);

impl Default for ScreenTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<ScreenTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<ScreenTimer>();
}

fn tick_screen_timer(
    time: Res<Time>,
    mut timer: ResMut<ScreenTimer>,
    level_select: Res<LevelSelect>,
) {
    if level_select.0 == 0 {
        return;
    }
    timer.0.tick(time.delta());
}

fn check_splash_timer(
    timer: ResMut<ScreenTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
    level_select: Res<LevelSelect>,
    level_data: Res<LevelData>,
) {
    if timer.0.just_finished() {
        if let Some(_) = level_data.maps.get(level_select.0).cloned() {
            next_screen.set(Screen::Gameplay);
        } else {
            // Next level is unavailable (usually end of game)
            next_screen.set(Screen::Title);
        }
    }
}

// Early Exit from screen
fn skip_screen(
    mut next_screen: ResMut<NextState<Screen>>,
    level_select: Res<LevelSelect>,
    level_data: Res<LevelData>,
) {
    if let Some(_) = level_data.maps.get(level_select.0).cloned() {
        next_screen.set(Screen::Gameplay);
    } else {
        // Next level is unavailable (usually end of game)
        next_screen.set(Screen::Title);
    }
}
