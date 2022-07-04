use bevy::{math::Vec2, prelude::Component};

use crate::Element;

#[derive(Component)]
pub struct Particle(pub Element);

#[derive(Component)]
pub struct Gravity;

// Fluid with dispersion, how far it will flow left or right
#[derive(Component)]
pub struct Fluid {
    pub dispersion: u8,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);
