use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Particles".to_string(),
            width: 750.0,
            height: 750.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(setup)
        .add_system(spawn_boundaries)
        .run();
}

#[derive(Component)]
pub struct Static;

#[derive(Component)]
pub struct Gravity;

struct Sprites<'a> {
    foundation: Color,
    stone: Color,
    sand: &'a [Color],
}

const SCALE: f32 = 5.;

const SPRITES: Sprites = Sprites {
    foundation: Color::Rgba {
        red: 0.15,
        green: 0.15,
        blue: 0.15,
        alpha: 1.0,
    },
    stone: Color::Rgba {
        red: 0.7,
        green: 0.7,
        blue: 0.7,
        alpha: 1.0,
    },
    sand: &[
        Color::Rgba {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
            alpha: 1.0,
        },
        Color::Rgba {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
            alpha: 1.0,
        },
    ],
};

// Spawn the vertical and horizontal bounding walls
fn spawn_boundaries(mut commands: Commands) {
    let boundary_size = 65;
    // vertical walls
    for pos_y in -boundary_size..=boundary_size {
        spawn_particle(&mut commands, boundary_size as f32, pos_y as f32);
        spawn_particle(&mut commands, -boundary_size as f32, pos_y as f32);
    }

    // horizontal walls
    for pos_x in -(boundary_size - 1)..boundary_size {
        spawn_particle(&mut commands, pos_x as f32, boundary_size as f32);
        spawn_particle(&mut commands, pos_x as f32, -boundary_size as f32);
    }
}

// TODO: Take in particle type to spawn
fn spawn_particle(commands: &mut Commands, pos_x: f32, pos_y: f32) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SPRITES.foundation,
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform::from_xyz(pos_x, pos_y, 0.));
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= SCALE;
    commands.spawn_bundle(camera);
}
