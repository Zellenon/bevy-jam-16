use attacks::{
    ApplyAttackEffect, AttackEnemiesInContact, DropLiquid, attack_contact_enemies,
    dispatch_attack_effects, do_tower_attacks,
};
use bevy::{
    app::{App, FixedUpdate, Update},
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Res},
    },
    math::{Quat, Vec3},
    state::{condition::in_state, state::OnEnter},
    transform::components::Transform,
};
use bevy_composable::app_impl::{ComplexSpawnable, ComponentTreeable};
use gravity_bullshit::{RangeDropper, drop_ranges, spawn_rangedroppers};
use liquids::{
    drop_liquids, puddle_attacks, splat_droplets, stop_dropping_puddles, tick_lifetimes,
    timeout_lifetimes,
};

use std::f32;
use trap_door::{DetectTrapDoor, OpenTrapDoor, close_trap_door, detect_trap_door, open_trap_door};

use super::{stats::StatSet, status_effects::common::status_debuff_multiplier};
use crate::{
    PausableSystems,
    assets::TowerSprites,
    data::{
        Tower,
        projectiles::{Droplet, Puddle},
        stats::MoveSpeed,
        status_effects::Frozen,
    },
    level::{components::pos, resource::CellDirection},
    prefabs::{towers::tower, wizardry::add_observer_to_component},
    screens::Screen,
};
use common::*;

pub mod attacks;
pub mod common;
pub mod gravity_bullshit;
pub mod liquids;
pub mod tesla;
pub mod trap_door;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TowerTriggerRange>()
        .register_type::<TowerTriggerNeedsGravity>()
        .register_type::<Cooldown>()
        .register_type::<TowerFired>()
        .register_type::<TowerHasTargets>()
        .register_type::<RangeDropper>();

    app.add_event::<DropLiquid>()
        .add_event::<TowerFired>()
        .add_event::<ApplyAttackEffect>()
        .add_event::<AttackEnemiesInContact>()
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
            (status_debuff_multiplier::<Frozen, MoveSpeed>(0.)).in_set(StatSet::Modify),
            (tick_lifetimes, timeout_lifetimes).chain(),
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
                dispatch_attack_effects,
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
            drop_ranges,
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}
