use crate::data::*;
use crate::prelude::*;
use crate::theme::prelude::*;
use bevy::ecs::spawn::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), on_enter_game);
    app.add_systems(
        Update,
        (highlight_hovered_tile, on_press_hotbar)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(Update, watch_pointer_state);
}

#[derive(Component, Debug, Reflect)]
struct HotbarItem;

#[derive(Component)]
struct CancelInput;

fn on_enter_game(mut commands: Commands, assets: Res<UiAssets>) {
    let hotbar_items: Vec<_> = Tower::all()
        .iter()
        .map(|t| {
            (
                *t,
                assets.hotbar_icons.get(t.ui_asset_key()).unwrap().clone(),
            )
        })
        .collect();

    commands.spawn((
        StateScoped(Screen::Gameplay),
        spawn_hotbar(),
        Children::spawn(SpawnIter(
            hotbar_items
                .into_iter()
                .map(|(tower, icon)| spawn_hotbar_item(tower, icon)),
        )),
    ));

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

fn spawn_hotbar() -> impl Bundle {
    (
        Name::new("Hotbar"),
        BorderRadius::all(Val::Px(8.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(8.0),
            margin: UiRect::horizontal(Val::Auto),
            // height: Val::Px(88.0),
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
                BackgroundColor(Color::BLACK.with_alpha(0.75)),
                BorderRadius::all(Val::Px(8.0)),
                BorderColor(Color::WHITE),
                Node {
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-56.0),
                    max_width: Val::Px(256.0),
                    ..default()
                },
                Children::spawn((Spawn(Text(tower.name().into())),)),
            ),
            (
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
                ImageNode::new(icon)
            )
        ],
    )
}

fn highlight_hovered_tile(
    mut tile_query: Query<(&Interaction, &mut BackgroundColor), With<HotbarItem>>,
) {
    for (interaction, mut background_color) in &mut tile_query {
        background_color.0 = match interaction {
            Interaction::None => Color::WHITE.with_alpha(0.25),
            _ => Color::WHITE,
        }
    }
}

fn on_press_hotbar(
    current_pointer_input_state: Res<State<PointerInteractionState>>,
    mut pointer_input_state: ResMut<NextState<PointerInteractionState>>,
    mut tile_query: Query<(&Interaction, &Tower), With<HotbarItem>>,
) {
    for (interaction, tower) in &mut tile_query {
        match interaction {
            Interaction::Pressed => {
                if let PointerInteractionState::Placing(t) = **current_pointer_input_state {
                    if &t == tower {
                        continue;
                    }
                }
                pointer_input_state.set(PointerInteractionState::Placing(*tower));
            }
            _ => {}
        }
    }
}
