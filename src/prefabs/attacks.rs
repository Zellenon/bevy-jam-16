use super::physics::GamePhysicsLayer as GPL;
use super::utils::{DropletSprite, PuddleSprite};
use crate::data::projectiles::{Droplet, LiquidType, Puddle};
use crate::gameplay::shared_systems::Lifetime;
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, LinearVelocity, Mass, RigidBody, Sensor,
};
use bevy::prelude::*;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

pub fn droplet(liquid: LiquidType) -> ComponentTree {
    (
        DropletSprite(liquid),
        Visibility::Visible,
        Droplet(liquid),
        Collider::circle(1.5),
        CollisionLayers::new(GPL::Projectiles, [GPL::Enemy, GPL::Level]),
        RigidBody::Dynamic,
        Mass(0.1),
        CollisionEventsEnabled,
    )
        .store()
        + name("Droplet")
}

pub fn puddle(liquid: LiquidType) -> ComponentTree {
    (
        PuddleSprite(liquid),
        Visibility::Visible,
        Puddle(liquid),
        Collider::ellipse(3.5, 0.75),
        CollisionLayers::new(GPL::Projectiles, [GPL::Enemy, GPL::Level]),
        Sensor,
        RigidBody::Kinematic,
        LinearVelocity(Vec2::new(0., -20.)),
        CollisionEventsEnabled,
        Lifetime::new(4.0),
    )
        .store()
        + name("Puddle")
}
