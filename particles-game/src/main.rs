use bevy::{prelude::*, render::camera::RenderTarget};

const SCALE: f32 = 5.;
const BOUNDARY: i32 = 65;
const NUM_CELLS: usize = 17161;

mod components;
use components::{Gravity, Particle};
mod sprites;
use sprites::SPRITES;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Element {
    Empty,
    Foundation,
    Sand,
    Stone,
}

struct Universe {
    elements: [Element; NUM_CELLS],
}

impl Universe {
    fn new() -> Universe {
        Universe {
            elements: [Element::Empty; NUM_CELLS],
        }
    }

    fn element_at_coord(&self, x: f32, y: f32) -> Element {
        let index = Universe::index_from_xy(x, y);
        self.elements[index]
    }

    fn index_from_xy(x: f32, y: f32) -> usize {
        let radius = BOUNDARY - 1;
        let shifted_x = (x + radius as f32) as usize;
        let shifted_y = (y + radius as f32) as usize;
        let width = (radius * 2 + 1) as usize;
        let index: usize = shifted_x + (width * shifted_y);

        index
    }

    fn set_element_at_coord(&mut self, x: f32, y: f32, element: Element) {
        let index = Universe::index_from_xy(x, y);
        self.elements[index] = element;
    }
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

fn spawn_particle(commands: &mut Commands, pos_x: f32, pos_y: f32, element: Element) {
    let sprite = match element {
        Element::Foundation => SPRITES.foundation,
        Element::Sand => SPRITES.sand[0],
        Element::Stone => SPRITES.stone,
        Element::Empty => SPRITES.none,
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
        .insert(Particle)
        .id();

    if element == Element::Sand {
        commands.entity(particle).insert(Gravity);
    }
}

fn setup(mut commands: Commands) {
    // setup camera and add scaling
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= SCALE;
    commands.spawn_bundle(camera);

    // Setup universe as empty
    commands.insert_resource(Universe::new());
}

// Get mouse coordinate in world space and place a particle if within the
// boundaries.
// TODO: Spawn in brush area, shouldn't spawn outside
fn handle_click(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut universe: ResMut<Universe>,
) {
    let (camera, camera_transform) = q_camera.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        if mouse_input.pressed(MouseButton::Left) {
            let x = world_pos.x.round();
            let y = world_pos.y.round();
            let limit = BOUNDARY as f32;

            if x < limit && x > -limit && y < limit && y > -limit {
                spawn_particle(&mut commands, x, y, Element::Sand);
                universe.element_at_coord(x, y);
                universe.set_element_at_coord(x, y, Element::Sand);
            }
        }
    }
}

// Handle all gravity actions on particles affected by gravity
// TODO: prevent collisions with other particles
fn gravity(mut query: Query<&mut Transform, (With<Gravity>, With<Particle>)>) {
    for mut transform in query.iter_mut() {
        if transform.translation.y > -((BOUNDARY - 1) as f32) {
            transform.translation.y -= 1.;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Particles".to_string(),
            width: 750.0,
            height: 750.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_boundaries)
        .add_system(handle_click)
        .add_system(gravity)
        .run();
}
