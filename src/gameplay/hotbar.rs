use crate::gameplay::messages::DisplayFlashMessage;
use crate::theme::palette::LABEL_TEXT;
use crate::{data::*, prelude::*, theme::prelude::*};
use bevy::color::palettes::tailwind;
use bevy::input::common_conditions::input_just_pressed;
use bevy::{ecs::spawn::*, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), on_enter_game);
    app.add_systems(
        Update,
        highlight_hovered_tile
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        Update,
        unset_cursor_state.run_if(input_just_pressed(KeyCode::Escape)),
    );

    app.add_systems(Update, watch_pointer_state);
}

#[derive(Component, Debug, Reflect)]
pub struct HotbarItem;

#[derive(Component)]
struct CancelInput;

fn on_enter_game(mut commands: Commands, assets: Res<UiAssets>) {
    let mut towers = Tower::all();
    towers.sort_by(|a, b| a.price().cmp(&b.price()));

    let hotbar_items: Vec<_> = towers
        .iter()
        .map(|t| {
            let asset = assets.hotbar_icons.get(t.ui_asset_key()).unwrap().clone();
            (*t, asset)
        })
        .collect();

    commands
        .spawn((
            StateScoped(Screen::Gameplay),
            spawn_hotbar(),
            Children::spawn(SpawnIter(
                hotbar_items
                    .into_iter()
                    .map(|(tower, icon)| spawn_hotbar_item(tower, icon)),
            )),
        ))
        .observe(hotbar_click_observer);

    commands
        .spawn((
            StateScoped(Screen::Gameplay),
            CancelInput,
            Visibility::Hidden,
            BackgroundColor(Color::Srgba(Srgba::RED.with_alpha(0.5))),
            BorderRadius::all(Val::Px(8.0)),
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                left: Val::Px(12.0),
                bottom: Val::Px(12.0),
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                ..default()
            },
            children![(Text::new("X"), TextFont::from_font_size(56.0))],
        ))
        .observe(on_click_cancel);
}

fn watch_pointer_state(
    mut changes: EventReader<StateTransitionEvent<PointerInteractionState>>,
    mut query: Query<&mut Visibility, With<CancelInput>>,
) {
    for change in changes.read() {
        if let Some(state) = change.entered {
            match state {
                PointerInteractionState::Placing(_) => {
                    for mut visibility in query.iter_mut() {
                        *visibility = Visibility::Inherited;
                    }
                }
                _ => {
                    for mut visibility in query.iter_mut() {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

fn on_click_cancel(
    _: Trigger<Pointer<Released>>,
    mut pointer_input_state: ResMut<NextState<PointerInteractionState>>,
) {
    pointer_input_state.set(PointerInteractionState::Selecting)
}

fn unset_cursor_state(mut pointer_input_state: ResMut<NextState<PointerInteractionState>>) {
    pointer_input_state.set(PointerInteractionState::Selecting)
}

fn spawn_hotbar() -> impl Bundle {
    (
        Name::new("Hotbar"),
        BorderRadius::all(Val::Px(8.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(8.0),
            margin: UiRect::horizontal(Val::Auto),
            padding: UiRect::axes(Val::Px(8.0), Val::Px(8.0)),
            display: Display::Flex,
            column_gap: Val::Px(12.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.75)),
    )
}

fn spawn_hotbar_item(tower: Tower, icon: Handle<Image>) -> impl Bundle {
    (
        Name::new(tower.name()),
        Button,
        Node {
            position_type: PositionType::Relative,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::WHITE.with_alpha(0.25)),
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(8.0)),
        HotbarItem,
        tower,
        TooltipParent,
        children![
            (
                Tooltip,
                BackgroundColor(Color::BLACK.with_alpha(0.85)),
                BorderRadius::all(Val::Px(4.0)),
                BorderColor(LABEL_TEXT),
                Node {
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    position_type: PositionType::Absolute,
                    row_gap: Val::Px(4.0),
                    bottom: Val::Px(84.0),
                    width: Val::Px(256.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                Children::spawn((
                    Spawn(widget::label(tower.name())),
                    Spawn(widget::body_text(tower.description())),
                    Spawn(widget::body_text(format!("Cost: {}", tower.price()))),
                )),
            ),
            (
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
                Pickable::IGNORE,
                ImageNode::new(icon)
            )
        ],
    )
}

fn highlight_hovered_tile(
    mut tile_query: Query<(&Interaction, &Tower, &mut BackgroundColor), With<HotbarItem>>,
    player_state: Res<PlayerState>,
) {
    for (interaction, tower, mut background_color) in &mut tile_query {
        background_color.0 = match interaction {
            Interaction::None => tailwind::SLATE_50.with_alpha(0.25).into(),
            _ => {
                if player_state.can_afford(tower.price()) {
                    tailwind::SLATE_50.with_alpha(0.8).into()
                } else {
                    tailwind::SLATE_50.with_alpha(0.25).into()
                }
            }
        }
    }
}

fn hotbar_click_observer(
    trigger: Trigger<Pointer<Click>>,
    mut pointer_input_state: ResMut<NextState<PointerInteractionState>>,
    mut commands: Commands,
    hotbar_items: Query<&Tower>,
    player_state: Res<PlayerState>,
) {
    let Ok(tower) = hotbar_items.get(trigger.target) else {
        return;
    };

    if !player_state.can_afford(tower.price()) {
        commands.trigger(DisplayFlashMessage::new("Insufficient funds"));
        return;
    }
    pointer_input_state.set(PointerInteractionState::Placing(*tower));
}
