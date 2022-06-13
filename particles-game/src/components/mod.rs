use bevy::prelude::Component;

use crate::Element;

#[derive(Component)]
pub struct Particle(pub Element);

#[derive(Component)]
pub struct Gravity;
