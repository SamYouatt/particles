use bevy::{prelude::*, render::camera::RenderTarget};
use rand::Rng;

use crate::{
    brush::{get_brush_locations, Brush, BrushSize},
    constants::BOUNDARY,
    spawn_particle,
    universe::{Cell, Universe},
    Element, Placing,
};

// Get mouse coordinate in world space and place a particle if within the
// boundaries.
pub fn handle_click(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    brush: Res<Brush>,
    mut universe: ResMut<Universe>,
    placing: Res<Placing>,
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

            get_brush_locations(x, y, &brush.0).for_each(|(dx, dy)| {
                let dx = dx as f32;
                let dy = dy as f32;

                if dx < limit
                    && dx > -limit
                    && dy < limit
                    && dy > -limit
                    && universe.element_at_coord(dx, dy) == Element::Empty
                {
                    // Randomise the placement of certain particles for nicer brush
                    let mut rng = rand::thread_rng();

                    match placing.0 {
                        Element::Sand => {
                            if rng.gen::<f32>() < 0.8 {
                                let eid = spawn_particle(&mut commands, dx, dy, placing.0);
                                universe.set_element_at_coord(dx, dy, placing.0);
                                universe.set_eid_at_coord(dx, dx, Some(eid));
                            }
                        }
                        Element::Empty => {}
                        _ => {
                            let eid = spawn_particle(&mut commands, dx, dy, placing.0);
                            universe.set_eid_at_coord(dx, dy, Some(eid));
                            universe.set_element_at_coord(dx, dy, placing.0);
                        }
                    }
                } else if dx < limit
                    && dx > -limit
                    && dy < limit
                    && dy > -limit
                    && placing.0 == Element::Empty
                {
                    if let Some(eid) = universe.eid_at_coord(dx, dy) {
                        commands.entity(eid).despawn();
                    }

                    let cell = Cell::new(Element::Empty, None);
                    universe.set_cell_at_coord(dx, dy, cell);
                }
            })
        }
    }
}

pub fn handle_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut brush: ResMut<Brush>,
    mut placing: ResMut<Placing>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        println!("Brush size increased");
        match brush.0 {
            BrushSize::Small => brush.0 = BrushSize::Medium,
            BrushSize::Medium => brush.0 = BrushSize::Large,
            BrushSize::Large => brush.0 = BrushSize::XLarge,
            BrushSize::XLarge => brush.0 = BrushSize::XXLarge,
            BrushSize::XXLarge => (), // max size
        }
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        println!("Brush size decreased");
        match brush.0 {
            BrushSize::Small => (), // min size
            BrushSize::Medium => brush.0 = BrushSize::Small,
            BrushSize::Large => brush.0 = BrushSize::Medium,
            BrushSize::XLarge => brush.0 = BrushSize::Large,
            BrushSize::XXLarge => brush.0 = BrushSize::XLarge,
        }
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        println!("Switched to sand");
        placing.0 = Element::Sand;
    }

    if keyboard_input.just_pressed(KeyCode::W) {
        println!("Switched to water");
        placing.0 = Element::Water;
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        println!("Switched to stone");
        placing.0 = Element::Stone;
    }

    if keyboard_input.just_pressed(KeyCode::E) {
        println!("Swithed to empty");
        placing.0 = Element::Empty;
    }
}
