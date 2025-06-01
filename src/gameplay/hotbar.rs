use crate::prelude::*;
use bevy::ecs::spawn::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_turret_bar);
    app.add_systems(Update, highlight_hovered_tile);
}

#[derive(Component, Debug, Reflect)]
struct HotbarItem();

fn spawn_turret_bar(mut commands: Commands, assets: Res<UiAssets>) {
    let hotbar_items = vec![
        ("tesla turret", assets.hotbar_tesla_image.clone()),
        ("water bucket", assets.hotbar_water_image.clone()),
        ("trap door", assets.hotbar_trapdoor_image.clone()),
    ];

    commands.spawn((
        StateScoped(Screen::Gameplay),
        spawn_hotbar(),
        Children::spawn(SpawnIter(
            hotbar_items
                .into_iter()
                .map(|(name, icon)| spawn_hotbar_item(name, icon)),
        )),
    ));
}

fn spawn_hotbar() -> impl Bundle {
    (
        Name::new("Hotbar"),
        BorderRadius::all(Val::Px(8.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(8.0),
            margin: UiRect::horizontal(Val::Auto),
            height: Val::Px(80.0),
            padding: UiRect::axes(Val::Px(8.0), Val::Px(8.0)),
            display: Display::Flex,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.75)),
    )
}

fn spawn_hotbar_item(name: impl Into<String>, icon: Handle<Image>) -> impl Bundle {
    let owned_name = name.into().clone();
    (
        Name::new(owned_name.clone()),
        Button,
        Node {
            width: Val::Px(64.),
            height: Val::Px(64.),
            ..default()
        },
        BackgroundColor(Color::WHITE.with_alpha(0.25)),
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(8.0)),
        HotbarItem(),
        children![(
            Node {
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            ImageNode::new(icon)
        )],
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
