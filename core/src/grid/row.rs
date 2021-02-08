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

#[cfg(test)]
mod tests {
    use crate::grid::row::Row;
    use crate::grid::cell::Cell;

    #[test]
    fn new_row_is_empty() {
        assert!(Row::new().cells.is_empty());
    }

    #[test]
    fn row_generates_enough_cells() {
        let mut row = Row::new();
        row.generate_cells_until(5);
        assert_eq!(6, row.cells.len());
    }

    #[test]
    fn row_does_not_generate_too_many_cells() {
        let mut row = Row::new();
        row.cells = vec![Cell::new(), Cell::new()];
        row.generate_cells_until(6);
        assert_eq!(7, row.cells.len());
    }
}