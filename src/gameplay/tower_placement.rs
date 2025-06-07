use crate::gameplay::hotbar::HotbarItem;
use crate::level::components::{Adjacent, ExactPosition, LEVEL_SCALING};
use crate::prefabs::utils::GiveMeMesh;
use crate::{
    PausableSystems,
    assets::TowerSprites,
    data::*,
    gameplay::messages::DisplayFlashMessage,
    level::{
        components::{Ceiling, Floor, Wall, WallDirection},
        resource::CellDirection,
    },
    prefabs::{towers::tower, wizardry::add_observer_to_component},
    screens::Screen,
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
        tower_placement_change.run_if(on_event::<TowerPlacementEvent>),
    );

    app.insert_resource(TowerPreview::default());
    app.add_systems(Update, remove_preview);
    app.add_observer(observe_placeholder);

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

#[derive(Resource, Debug, Default, Clone, Copy, Reflect)]
struct TowerPreview {
    tower: Option<Tower>,
    position_entity: Option<Entity>,
    cell_direction: Option<CellDirection>,
}

#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
struct SpawnedPreview;

impl TowerPreview {
    fn reset(&mut self) {
        *self = TowerPreview::default()
    }
}

#[derive(Event, Debug, Clone, Copy, Reflect)]
struct SelectTower(pub Entity);

fn remove_preview(
    windows: Query<&Window>,
    spawned_previews: Query<(Entity, &GlobalTransform), With<SpawnedPreview>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut preview: ResMut<TowerPreview>,
    mut commands: Commands,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    let Some(window_cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(game_cursor_position) =
        camera.viewport_to_world_2d(camera_transform, window_cursor_position)
    else {
        return;
    };

    for (entity, global_transform) in spawned_previews {
        if global_transform
            .translation()
            .xy()
            .distance(game_cursor_position)
            > LEVEL_SCALING
        {
            commands.entity(entity).despawn();
            preview.reset();
        }
    }
}

fn tower_placement_change(
    mut tower_placement_events: EventReader<TowerPlacementEvent>,
    mut preview: ResMut<TowerPreview>,
    sprites: Option<Res<TowerSprites>>,
    spawned_previews: Query<(Entity, &GlobalTransform), With<SpawnedPreview>>,
    mut commands: Commands,
) {
    let Some(TowerPlacementEvent::Requested(tower, parent, placement)) =
        tower_placement_events.read().last()
    else {
        return;
    };

    let sprites = sprites.expect("GameAssets should be available during turret placement");

    for (entity, _) in spawned_previews {
        commands.entity(entity).despawn();
        preview.reset();
    }

    *preview = TowerPreview {
        tower: Some(*tower),
        position_entity: Some(*parent),
        cell_direction: Some(*placement),
    };

    commands.entity(*parent).with_children(|builder| {
        builder
            .spawn((
                sprites.tower_bundle(tower, placement),
                placement.sprite_offset(&tower),
                SpawnedPreview,
            ))
            .observe(observe_placeholder);
    });
}

fn observe_placeholder(
    trigger: Trigger<Pointer<Click>>,
    spawned_previews: Query<(), With<SpawnedPreview>>,
    mut commands: Commands,
    mut next_pointer_state: ResMut<NextState<PointerInteractionState>>,
    mut tower_placement_writer: EventWriter<TowerPlacementEvent>,
    mut player_state: ResMut<PlayerState>,
    input: Res<ButtonInput<KeyCode>>,
    preview: Res<TowerPreview>,
    relationships: Query<&Children>,
    towers: Query<(), With<Tower>>,
    adjacent_placements: Query<(Entity, &Adjacent)>,
    hotbar: Query<(), With<HotbarItem>>,
) {
    if let Ok(_) = hotbar.get(trigger.target) {
        return;
    }

    let total_previews = spawned_previews.iter().len();
    if total_previews == 0 || total_previews > 1 {
        return;
    }

    let Some(tower) = preview.tower else {
        return;
    };

    let Some(entity) = preview.position_entity else {
        return;
    };

    let Some(orientation) = preview.cell_direction else {
        return;
    };

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

    if tower.requires_adjecent_wall() {
        if let Ok((_, adjacent)) = adjacent_placements.get(entity) {
            if tower.requires_floor_placement() && adjacent.exact_position != ExactPosition::Floor {
                commands.trigger(DisplayFlashMessage::new(
                    "This tower must be placed on a floor panel",
                ));
                return;
            } else if tower.requires_floor_placement() && adjacent.id.unit_y == 0 {
                commands.trigger(DisplayFlashMessage::new(
                    "This tower cannot be placed on the bottom",
                ));
                return;
            }
            for (other_entity, other_adjacent_id) in adjacent_placements {
                if adjacent == other_adjacent_id {
                    if let Ok(children) = relationships.get(other_entity) {
                        for child in children {
                            if let Ok(_) = towers.get(*child) {
                                commands.trigger(DisplayFlashMessage::new(
                                    "This tower requires both sides of the wall",
                                ));
                                return;
                            }
                        }
                    }
                }
            }
        };
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
    previews: Res<TowerPreview>,
    relationships: Query<&Children>,
    mut tower_placement_writer: EventWriter<TowerPlacementEvent>,
) {
    let PointerInteractionState::Placing(tower) = *pointer_input_state.get() else {
        return;
    };

    let entity = trigger.target;

    if let Ok(children) = relationships.get(entity) {
        for child in children {
            if let Some(e) = previews.position_entity {
                if e == *child {
                    // Early return if the entity already has a preview
                    return;
                }
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
                            + orientation.sprite_offset(&tower).store(),
                    );
                });
            }
            _ => {}
        }
    }
}
