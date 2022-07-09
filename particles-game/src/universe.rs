use bevy::prelude::Entity;

use crate::constants::NUM_CELLS;
use crate::{Element, BOUNDARY};

// Track the contents of every placeable cell
pub struct Universe {
    cells: [Cell; NUM_CELLS],
}

#[derive(Copy, Clone)]
pub struct Cell {
    element: Element,
    eid: Option<Entity>,
}

impl Cell {
    pub fn new(element: Element, eid: Option<Entity>) -> Cell {
        Cell { element, eid }
    }
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            cells: [Cell {
                element: Element::Empty,
                eid: None,
            }; NUM_CELLS],
        }
    }

    pub fn element_at_coord(&self, x: f32, y: f32) -> Element {
        let index = Universe::index_from_xy(x, y);
        self.cells[index].element
    }

    pub fn eid_at_coord(&self, x: f32, y: f32) -> Option<Entity> {
        let index = Universe::index_from_xy(x, y);
        self.cells[index].eid
    }

    pub fn cell_at_coord(&self, x: f32, y: f32) -> Cell {
        let index = Universe::index_from_xy(x, y);
        self.cells[index]
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
        self.cells[index].element = element;
    }

    pub fn set_eid_at_coord(&mut self, x: f32, y: f32, eid: Option<Entity>) {
        let index = Universe::index_from_xy(x, y);
        self.cells[index].eid = eid;
    }

    pub fn set_cell_at_coord(&mut self, x: f32, y: f32, cell: Cell) {
        let index = Universe::index_from_xy(x, y);
        self.cells[index] = cell;
    }

    pub fn remove_eid_at_coord(&mut self, x: f32, y: f32) {
        let index = Universe::index_from_xy(x, y);
        self.cells[index].eid = None;
    }
}
