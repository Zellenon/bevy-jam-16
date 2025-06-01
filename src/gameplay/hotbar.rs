use crate::prelude::*;
use bevy::ecs::spawn::*;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_turret_bar);
}

fn spawn_turret_bar(mut commands: Commands, assets: Res<UiAssets>) {
    let hotbar_items = vec![
        ("tesla turret", assets.hotbar_tesla_image.clone()),
        ("water bucket", assets.hotbar_tesla_image.clone()),
        ("crusher", assets.hotbar_tesla_image.clone()),
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
        Node {
            width: Val::Px(64.),
            height: Val::Px(64.),
            ..default()
        },
        BackgroundColor(Color::WHITE.with_alpha(0.5)),
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(8.0)),
        children![ImageNode::new(icon)],
    )
}
