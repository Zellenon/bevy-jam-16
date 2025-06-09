use crate::{
    PausableSystems,
    data::projectiles::{Droplet, Puddle},
    prefabs::wizardry::add_observer_to_component,
    screens::Screen,
};
use attacks::{
    ApplyAttackData, AttackEnemiesInContact, DropLiquid, animate_towers_on_attack,
    attack_contact_enemies, dispatch_attack_effects, do_tower_attacks, play_tower_sfx,
};
use bevy::prelude::*;
use common::*;
use directional::FireDirection;
use fan::{ForceField, do_forcefields, resolve_fancasters, spawn_fancasters};
use gravity_bullshit::{RangeDropper, drop_ranges, spawn_rangedroppers};
use liquids::{drop_liquids, puddle_attacks, splat_droplets, stop_dropping_puddles};
use piston::{Shove, do_shoves};
use trap_door::{DetectTrapDoor, OpenTrapDoor, close_trap_door, detect_trap_door, open_trap_door};

pub mod attacks;
pub mod common;
pub mod directional;
pub mod fan;
pub mod gravity_bullshit;
pub mod liquids;
pub mod piston;
pub mod trap_door;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TowerTriggerRange>()
        .register_type::<TowerTriggerNeedsGravity>()
        .register_type::<Cooldown>()
        .register_type::<TowerFired>()
        .register_type::<FireDirection>()
        .register_type::<ForceField>()
        .register_type::<TowerHasTargets>()
        .register_type::<RangeDropper>();

    app.add_event::<DropLiquid>()
        .add_event::<TowerFired>()
        .add_event::<Shove>()
        .add_event::<ApplyAttackData>()
        .add_event::<AttackEnemiesInContact>()
        .add_event::<ApplyAttackData>()
        .add_event::<DetectTrapDoor>()
        .add_event::<OpenTrapDoor>();

    app.add_observer(add_observer_to_component::<Puddle, _, _, _, _>(
        stop_dropping_puddles,
    ));
    app.add_observer(add_observer_to_component::<Puddle, _, _, _, _>(
        puddle_attacks,
    ));
    app.add_observer(add_observer_to_component::<Droplet, _, _, _, _>(
        splat_droplets,
    ));

    app.add_systems(
        Update,
        (
            (tick_cooldown, remove_cooldown).chain(),
            (
                towers_fire,
                do_tower_attacks,
                (
                    attack_contact_enemies,
                    drop_liquids,
                    detect_trap_door,
                    open_trap_door,
                    close_trap_door,
                ),
                (
                    dispatch_attack_effects,
                    do_shoves,
                    do_forcefields,
                    animate_towers_on_attack,
                    play_tower_sfx,
                ),
            )
                .chain(),
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        FixedUpdate,
        (
            add_tower_targets_from_zone,
            remove_tower_targets,
            spawn_rangedroppers,
            spawn_fancasters,
            drop_ranges,
            resolve_fancasters,
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}
