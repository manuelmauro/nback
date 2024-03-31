#[derive(Default, Debug)]
pub struct Answer {
    pub same_position: bool,
    pub same_color: bool,
}

impl Answer {
    pub fn same_position(&mut self) {
        self.same_position = true;
    }

    pub fn same_color(&mut self) {
        self.same_color = true;
    }

    pub fn reset(&mut self) {
        self.same_position = false;
        self.same_color = false;
    }
}
