use super::Cell;

pub struct Row {
    cells: Vec<Cell>,
}

impl Row {
    pub fn new() -> Self {
        Self {
            cells: vec![],
        }
    }

    pub fn get_cell(&self, x: usize) -> Option<&Cell> {
        self.cells.get(x)
    }

    pub fn get_cell_mut(&mut self, x: usize) -> Option<&mut Cell> {
        self.cells.get_mut(x)
    }

    pub fn generate_cells_until(&mut self, x: usize) {
        while self.cells.len() <= x {
            self.cells.push(Cell::new());
        }
    }
}