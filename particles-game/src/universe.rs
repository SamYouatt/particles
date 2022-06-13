use crate::constants::NUM_CELLS;
use crate::{Element, BOUNDARY};

pub struct Universe {
    elements: [Element; NUM_CELLS],
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            elements: [Element::Empty; NUM_CELLS],
        }
    }

    pub fn element_at_coord(&self, x: f32, y: f32) -> Element {
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

    pub fn set_element_at_coord(&mut self, x: f32, y: f32, element: Element) {
        let index = Universe::index_from_xy(x, y);
        self.elements[index] = element;
    }
}
