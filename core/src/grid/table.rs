use crate::grid::row::Row;
use crate::grid::cell::Cell;

pub struct Table {
    rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            rows: vec![],
        }
    }

    pub fn get_row(&self, y: usize) -> Option<&Row> {
        self.rows.get(y)
    }

    pub fn get_row_mut(&mut self, y: usize) -> Option<&mut Row> {
        self.rows.get_mut(y)
    }

    pub fn generate_cells_until(&mut self, x: usize, y: usize) {
        while self.rows.len() <= y {
            self.rows.push(Row::new());
        }
        self.get_row_mut(y).map(|r| r.generate_cells_until(x));
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.get_row(y).map(|r| r.get_cell(x)).flatten()
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.get_row_mut(y).map(|r| r.get_cell_mut(x)).flatten()
    }
}