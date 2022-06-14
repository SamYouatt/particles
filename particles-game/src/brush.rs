use itertools::Itertools;

pub enum BrushSize {
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

pub struct Brush(pub BrushSize);

// Return an iterator over the array of coordinates within a brush given a centre
pub fn get_brush_locations(
    cx: f32,
    cy: f32,
    brush: &BrushSize,
) -> impl Iterator<Item = (isize, isize)> {
    let delta: isize = match brush {
        BrushSize::Small => 0,
        BrushSize::Medium => 1,
        BrushSize::Large => 2,
        BrushSize::XLarge => 3,
        BrushSize::XXLarge => 4,
    };

    let x = cx as isize;
    let y = cy as isize;
    ((x - delta)..=(x + delta)).cartesian_product((y - delta)..=(y + delta))
}
