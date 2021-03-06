use bevy::{prelude::*, render::camera::RenderTarget};
use constants::GRAVITY;
use constants::TERMINAL_VELOCITY;
use control::handle_click;
use control::handle_keyboard;
use rand::Rng;

mod components;
use brush::get_brush_locations;
use brush::Brush;
use brush::BrushSize;
use components::Fluid;
use components::Velocity;
use components::{Gravity, Particle};
mod sprites;
use constants::{BOUNDARY, SCALE};
use sprites::get_sprite_color;
use sprites::SPRITES;
use universe::Universe;
use wasm_bindgen::prelude::wasm_bindgen;

mod brush;
mod constants;
mod control;
mod universe;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Element {
    Empty,
    Foundation,
    Sand,
    Stone,
    Water,
}

pub struct Placing(Element);

#[wasm_bindgen]
pub fn run() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Particles".to_string(),
        width: 750.0,
        height: 750.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    app.add_startup_system(setup)
        .add_startup_system(spawn_boundaries)
        .add_system(handle_click)
        .add_system(handle_keyboard)
        .add_system(gravity)
        .add_system(fluid)
        .run();
}

// Spawn the vertical and horizontal bounding walls
fn spawn_boundaries(mut commands: Commands) {
    // vertical walls
    for pos_y in -BOUNDARY..=BOUNDARY {
        spawn_particle(
            &mut commands,
            BOUNDARY as f32,
            pos_y as f32,
            Element::Foundation,
        );
        spawn_particle(
            &mut commands,
            -BOUNDARY as f32,
            pos_y as f32,
            Element::Foundation,
        );
    }

    // horizontal walls
    for pos_x in -(BOUNDARY - 1)..BOUNDARY {
        spawn_particle(
            &mut commands,
            pos_x as f32,
            BOUNDARY as f32,
            Element::Foundation,
        );
        spawn_particle(
            &mut commands,
            pos_x as f32,
            -BOUNDARY as f32,
            Element::Foundation,
        );
    }
}

fn spawn_particle(commands: &mut Commands, pos_x: f32, pos_y: f32, element: Element) -> Entity {
    let sprite = match element {
        Element::Foundation => get_sprite_color(SPRITES.foundation),
        Element::Sand => get_sprite_color(SPRITES.sand),
        Element::Stone => get_sprite_color(SPRITES.stone),
        Element::Water => get_sprite_color(SPRITES.water),
        Element::Empty => get_sprite_color(SPRITES.none),
    };

    let particle = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: sprite,
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform::from_xyz(pos_x, pos_y, 0.))
        .insert(Particle(element))
        .id();

    if element == Element::Sand {
        commands
            .entity(particle)
            .insert(Gravity)
            .insert(Velocity(Vec2::new(0., 0.)));
    }

    if element == Element::Water {
        commands
            .entity(particle)
            .insert(Fluid { dispersion: 3 })
            .insert(Velocity(Vec2::new(0., 0.)));
    };

    particle
}

fn setup(mut commands: Commands) {
    // setup camera and add scaling
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= SCALE;
    commands.spawn_bundle(camera);

    // Setup universe as empty
    commands.insert_resource(Universe::new());

    // Setup default brush
    commands.insert_resource(Brush(BrushSize::Small));

    // Sets sand as default element
    commands.insert_resource(Placing(Element::Sand));
}

// Handle all gravity actions on particles affected by gravity
fn gravity(
    mut universe: ResMut<Universe>,
    mut query: Query<(&mut Transform, &Particle, &mut Velocity), With<Gravity>>,
) {
    let floor_limit = -((BOUNDARY - 1) as f32);
    let left_limit = -((BOUNDARY - 1) as f32);
    let right_limit = (BOUNDARY - 1) as f32;

    for (mut transform, particle, mut vel) in query.iter_mut() {
        // Check valid locations within velocity, starting at furthest

        let mut next_y = transform.translation.y;
        let mut next_x = transform.translation.x;

        // Check every place along its path and if its valid update its next position
        // Once done the furthest possible location along the path will be the next position
        for delta_y in 1..(vel.0.y.abs() as usize) {
            let sign = if vel.0.y < 0. { -1. } else { 1. };
            let check_y = transform.translation.y + (delta_y as f32 * sign);
            let element_at_next = universe.element_at_coord(transform.translation.x, check_y);

            // If valid position update next coord
            // Empty below - drop down
            if check_y > -(BOUNDARY) as f32 && element_at_next == Element::Empty {
                next_y = check_y;
            } else {
                // Slide left or right
                let element_right =
                    universe.element_at_coord(transform.translation.x + 1., check_y);
                let element_left = universe.element_at_coord(transform.translation.x - 1., check_y);

                if check_y > floor_limit
                    && transform.translation.x < right_limit
                    && element_right == Element::Empty
                {
                    next_y = check_y;
                    next_x = transform.translation.x + 1.;
                } else if check_y > floor_limit
                    && transform.translation.x > left_limit
                    && element_left == Element::Empty
                {
                    next_y = check_y;
                    next_x = transform.translation.x - 1.;
                } else {
                    // Nowhere for particle to go so stop trying
                    vel.0 = Vec2::new(0., 0.);
                    break;
                }
            }
        }

        // Only update if particle should move
        if next_y != transform.translation.y || next_x != transform.translation.x {
            universe.set_element_at_coord(
                transform.translation.x,
                transform.translation.y,
                Element::Empty,
            );
            universe.set_element_at_coord(next_x, next_y, particle.0);
            transform.translation.y = next_y;
            transform.translation.x = next_x;
        }

        if vel.0.y < TERMINAL_VELOCITY && vel.0.y > -TERMINAL_VELOCITY {
            // gravity acceleration
            vel.0.y += GRAVITY;
        }
    }
}

// Will check left and right for movement opportunities
fn fluid(mut universe: ResMut<Universe>, mut query: Query<(&mut Transform, &Particle, &Fluid)>) {
    let floor_limit = -((BOUNDARY - 1) as f32);
    let left_limit = -((BOUNDARY - 1) as f32);
    let right_limit = (BOUNDARY - 1) as f32;

    for (mut transform, particle, fluid) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;
        let element_below = universe.element_at_coord(x, y - 1.);

        if y > floor_limit && element_below == Element::Empty {
            // Straight down
            universe.set_element_at_coord(x, y, Element::Empty);
            universe.set_element_at_coord(x, y - 1., particle.0);
            transform.translation.y -= 1.;
        } else {
            // Slide left or right
            for delta in 1..fluid.dispersion {
                let element_right = universe.element_at_coord(x + (delta as f32), y);
                let element_left = universe.element_at_coord(x - (delta as f32), y);

                if y > floor_limit && x < right_limit && element_right == Element::Empty {
                    universe.set_element_at_coord(x, y, Element::Empty);
                    universe.set_element_at_coord(x + 1., y, particle.0);
                    // transform.translation.y -= 1.;
                    transform.translation.x += 1.;
                    break;
                } else if y > floor_limit && x > left_limit && element_left == Element::Empty {
                    universe.set_element_at_coord(x, y, Element::Empty);
                    universe.set_element_at_coord(x - 1., y, particle.0);
                    // transform.translation.y -= 1.;
                    transform.translation.x -= 1.;
                    break;
                }
            }
        }
    }
}

struct CoordPath {
    start: Vec2,
    end: Vec2,
    current: Vec2,
}

impl Iterator for CoordPath {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        // return the next coordinate along a path
        todo!()
    }
}

fn coordinate_path(start: Vec2, end: Vec2) -> CoordPath {
    CoordPath {
        start,
        end,
        current: start,
    }
}
