use crate::{
    PausableSystems,
    assets::TowerSprites,
    data::*,
    level::{
        components::{Architecture, Ceiling, Floor, Wall, WallDirection},
        resource::CellDirection,
    },
    prefabs::{towers::tower, wizardry::add_observer_to_component},
    screens::Screen,
    utils::destroy_entity,
};
use bevy::prelude::*;
use bevy_composable::app_impl::{ComplexSpawnable, ComponentTreeable};
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<TowerPlacementState>();

    app.add_event::<SelectTower>();
    app.add_event::<PlaceTower>();

    app.add_observer(on_turret_placement_hover);
    app.add_observer(add_observer_to_component::<Architecture, _, _, _, _>(
        click_tower,
    ));

    app.add_systems(
        Update,
        tower_placement_change.run_if(state_changed::<TowerPlacementState>),
    );
    app.add_systems(
        OnEnter(PointerInteractionState::Selecting),
        destroy_entity::<TowerPreview>,
    );

    app.add_systems(
        Update,
        (place_towers)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
enum TowerPlacementState {
    #[default]
    None,
    Placing(Tower, Entity, CellDirection),
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
struct TowerPreview;

#[derive(Event, Debug, Clone, Copy, Reflect)]
struct PlaceTower(pub Entity, pub Tower, pub CellDirection);

#[derive(Event, Debug, Clone, Copy, Reflect)]
struct SelectTower(pub Entity);

fn tower_placement_change(
    tower_placement_state: Res<State<TowerPlacementState>>,
    previews: Query<Entity, With<TowerPreview>>,
    sprites: Option<Res<TowerSprites>>,
    mut commands: Commands,
) {
    let tower_placement_state = tower_placement_state.get();
    let TowerPlacementState::Placing(tower, parent, placement) = tower_placement_state else {
        return;
    };
    let sprites = sprites.expect("GameAssets should be available during turret placement");

    for entity in previews {
        commands.entity(entity).despawn()
    }

    commands.entity(*parent).with_child((
        sprites.tower_bundle(tower, placement),
        placement.sprite_offset(),
        TowerPreview,
    ));
}

fn on_turret_placement_hover(
    trigger: Trigger<Pointer<Over>>,
    walls: Query<&Wall>,
    floors: Query<(), With<Floor>>,
    ceilings: Query<(), With<Ceiling>>,
    pointer_input_state: Res<State<PointerInteractionState>>,
    tower_placement_state: Res<State<TowerPlacementState>>,
    mut next_tower_placement_state: ResMut<NextState<TowerPlacementState>>,
) {
    let PointerInteractionState::Placing(tower) = pointer_input_state.get() else {
        return;
    };

    let entity = trigger.target;

    if let TowerPlacementState::Placing(t, e, _) = tower_placement_state.get() {
        if *t == *tower && *e == entity {
            return;
        }
    }

    if let Ok(wall) = walls.get(entity) {
        let placement = match wall.0 {
            WallDirection::Left => CellDirection::Left,
            WallDirection::Right => CellDirection::Right,
        };
        next_tower_placement_state.set(TowerPlacementState::Placing(*tower, entity, placement));
    } else if ceilings.get(entity).is_ok() {
        next_tower_placement_state.set(TowerPlacementState::Placing(
            *tower,
            entity,
            CellDirection::Up,
        ));
    } else if floors.get(entity).is_ok() {
        next_tower_placement_state.set(TowerPlacementState::Placing(
            *tower,
            entity,
            CellDirection::Down,
        ));
    }
}

fn click_tower(
    trigger: Trigger<Pointer<Pressed>>,
    pointer_input_state: Res<State<PointerInteractionState>>,
    tower_placement_state: Res<State<TowerPlacementState>>,
    mut select_events: EventWriter<SelectTower>,
    mut place_events: EventWriter<PlaceTower>,
) {
    match **pointer_input_state {
        PointerInteractionState::Selecting => {
            println!("Selecting!");
            select_events.write(SelectTower(trigger.target));
        }
        PointerInteractionState::Placing(tower) => {
            println!("Spawning!");
            let entity = trigger.target;

            let (_, _, orientation) = (match **tower_placement_state {
                TowerPlacementState::None => None,
                TowerPlacementState::Placing(tower, entity, tower_placement) => {
                    Some((tower, entity, tower_placement))
                }
            })
            .unwrap();

            place_events.write(PlaceTower(entity, tower, orientation));
        }
    }
}

pub fn place_towers(mut place_events: EventReader<PlaceTower>, mut commands: Commands) {
    for PlaceTower(entity, tower_type, orientation) in place_events.read() {
        commands.entity(*entity).with_children(|commands| {
            commands.compose(
                tower(*tower_type, *orientation) + orientation.sprite_offset().store(),
            );
        });
    }
}
