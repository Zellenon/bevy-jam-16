//! Helper functions for creating common widgets.

use std::borrow::Cow;

use crate::theme::prelude::*;
use crate::theme::{interaction::InteractionPalette, palette::*};
use bevy::color::palettes::tailwind;
use bevy::sprite::Anchor;
use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A copy of root UI with minor changes
pub fn credits(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(12.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// Game title
pub fn title(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Title"),
        Text(text.into()),
        Anchor::Center,
        TextFont::from_font(TITLE_FONT).with_font_size(52.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font(LABEL_FONT).with_font_size(40.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font(LABEL_FONT).with_font_size(24.0),
        TextColor(LABEL_TEXT),
    )
}

/// Credits
pub fn credits_text(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Credits Text"),
        Text(text.into()),
        TextFont::from_font(LABEL_FONT).with_font_size(20.0),
        TextColor(LABEL_TEXT),
    )
}

pub fn ui_font(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Text"),
        Text(text.into()),
        TextFont::from_font(LABEL_FONT).with_font_size(18.0),
        TextColor(tailwind::STONE_900.into()),
    )
}

pub fn ui_font_color(text: impl Into<String>, color: Color) -> impl Bundle {
    (
        Name::new("Text"),
        Text(text.into()),
        TextFont::from_font(LABEL_FONT).with_font_size(18.0),
        TextColor(color),
    )
}

pub fn ui_font_with_node(text: impl Into<String>, node: Node) -> impl Bundle {
    (
        Name::new("Text"),
        Text(text.into()),
        TextFont::from_font(LABEL_FONT).with_font_size(18.0),
        TextColor(tailwind::STONE_900.into()),
        node,
        Pickable::IGNORE,
    )
}

/// A simple text label.
pub fn body_text(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font(BODY_FONT).with_font_size(16.0),
        TextColor(LABEL_TEXT),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: Px(380.0),
                height: Px(64.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::all(Px(8.0)),
        ),
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        Node {
            width: Px(30.0),
            height: Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font(LABEL_FONT).with_font_size(32.0),
                        TextColor(BUTTON_TEXT),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}

/// A large rounded button with text that is "Pickable"
pub fn global_observer_button(
    text: impl Into<String>,
    observer_bundle: impl Bundle,
) -> impl Bundle {
    global_observer_button_base(
        text,
        (
            observer_bundle,
            Pickable::default(),
            Node {
                width: Px(380.0),
                height: Px(64.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::all(Px(8.0)),
        ),
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn global_observer_button_base(text: impl Into<String>, button_bundle: impl Bundle) -> impl Bundle {
    let text = text.into();

    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font(LABEL_FONT).with_font_size(32.0),
                        TextColor(BUTTON_TEXT),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle);
        })),
    )
}
