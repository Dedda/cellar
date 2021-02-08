pub struct Cell {
    content: String,
    pub rendered: String,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            rendered: String::new(),
        }
    }

    fn render(&mut self) {
        if self.content.starts_with("=") {
            // TODO: run actual function
            self.rendered = "### Functions are not implemented yet!".into();
        } else {
            self.rendered = self.content.clone();
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.render();
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}


#[cfg(test)]
mod tests {
    use crate::grid::cell::Cell;

    #[test]
    fn new_cell_is_empty() {
        assert!(Cell::new().content.is_empty());
        assert!(Cell::new().rendered.is_empty());
    }
}