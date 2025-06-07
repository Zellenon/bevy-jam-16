use crate::{
    assets::TowerSprites,
    data::*,
    gameplay::messages::DisplayFlashMessage,
    level::{
        components::{Ceiling, Floor, Wall, WallDirection},
        resource::CellDirection,
    },
    prelude::*,
    utils::destroy_entity,
};
use bevy::prelude::*;
use bevy_composable::app_impl::{ComplexSpawnable, ComponentTreeable};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<TowerPlacementEvent>();
    app.add_event::<SelectTower>();

    app.add_observer(on_turret_placement_hover);

    app.add_systems(
        Update,
        place_tower_preview.run_if(on_event::<TowerPlacementEvent>),
    );
    app.add_systems(
        OnEnter(PointerInteractionState::Selecting),
        destroy_entity::<TowerPreview>,
    );

    app.add_systems(
        Update,
        place_towers
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Event, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
enum TowerPlacementEvent {
    Requested(Tower, Entity, CellDirection),
    Accepted(Tower, Entity, CellDirection),
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
struct TowerPreview(Tower, Entity, CellDirection);

#[derive(Event, Debug, Clone, Copy, Reflect)]
struct SelectTower(pub Entity);

fn place_tower_preview(
    mut tower_placement_events: EventReader<TowerPlacementEvent>,
    previews: Query<Entity, With<TowerPreview>>,
    sprites: Option<Res<TowerSprites>>,
    mut commands: Commands,
) {
    let Some(TowerPlacementEvent::Requested(tower, parent, placement)) =
        tower_placement_events.read().last()
    else {
        return;
    };

    let sprites = sprites.expect("GameAssets should be available during turret placement");

    for entity in previews {
        commands.entity(entity).despawn()
    }
    commands.entity(*parent).with_children(|builder| {
        builder
            .spawn((
                sprites.tower_bundle(tower, placement),
                placement.sprite_offset(),
                TowerPreview(*tower, *parent, *placement),
                Pickable::default(),
            ))
            .observe(observe_placeholder);
    });
}

fn observe_placeholder(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut next_pointer_state: ResMut<NextState<PointerInteractionState>>,
    mut tower_placement_writer: EventWriter<TowerPlacementEvent>,
    mut player_state: ResMut<PlayerState>,
    input: Res<ButtonInput<KeyCode>>,
    previews: Query<&TowerPreview>,
    relationships: Query<&Children>,
    towers: Query<(), With<Tower>>,
) {
    let TowerPreview(tower, entity, orientation) = *previews.get(trigger.target).unwrap();

    // prevent placement if user cannot afford tower
    if !player_state.can_afford(tower.price()) {
        commands.trigger(DisplayFlashMessage::new("Insufficient funds"));
        return;
    }

    // prevent double placement
    if let Ok(children) = relationships.get(entity) {
        for child in children {
            if let Ok(_) = towers.get(*child) {
                commands.trigger(DisplayFlashMessage::new("There is already a tower here"));
                return;
            }
        }
    }

    player_state.money -= tower.price();
    tower_placement_writer.write(TowerPlacementEvent::Accepted(tower, entity, orientation));

    if !input.pressed(KeyCode::ShiftLeft) && !input.pressed(KeyCode::ShiftRight) {
        next_pointer_state.set(PointerInteractionState::Selecting);
    }
}

fn on_turret_placement_hover(
    trigger: Trigger<Pointer<Over>>,
    walls: Query<&Wall>,
    floors: Query<(), With<Floor>>,
    ceilings: Query<(), With<Ceiling>>,
    pointer_input_state: Res<State<PointerInteractionState>>,
    previews: Query<&TowerPreview>,
    relationships: Query<&Children>,
    mut tower_placement_writer: EventWriter<TowerPlacementEvent>,
) {
    let PointerInteractionState::Placing(tower) = *pointer_input_state.get() else {
        return;
    };

    let entity = trigger.target;

    if let Ok(children) = relationships.get(entity) {
        for child in children {
            if previews.get(*child).is_ok() {
                // Early return if the entity already has a preview
                return;
            }
        }
    }

    if let Ok(wall) = walls.get(entity) {
        let placement = match wall.0 {
            WallDirection::Left => CellDirection::Left,
            WallDirection::Right => CellDirection::Right,
        };
        tower_placement_writer.write(TowerPlacementEvent::Requested(tower, entity, placement));
    } else if ceilings.get(entity).is_ok() {
        tower_placement_writer.write(TowerPlacementEvent::Requested(
            tower,
            entity,
            CellDirection::Up,
        ));
    } else if floors.get(entity).is_ok() {
        tower_placement_writer.write(TowerPlacementEvent::Requested(
            tower,
            entity,
            CellDirection::Down,
        ));
    }
}

fn place_towers(mut place_events: EventReader<TowerPlacementEvent>, mut commands: Commands) {
    for event in place_events.read() {
        match *event {
            TowerPlacementEvent::Accepted(tower, entity, orientation) => {
                commands.entity(entity).with_children(|commands| {
                    commands.compose(
                        crate::prefabs::towers::tower(tower, orientation)
                            + orientation.sprite_offset().store(),
                    );
                });
            }
            _ => {}
        }
    }
}
