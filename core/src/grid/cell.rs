pub struct Cell {
    pub content: Option<String>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            content: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::grid::cell::Cell;

    #[test]
    fn new_cell_is_empty() {
        assert_eq!(Cell::new().content, None);
    }
}