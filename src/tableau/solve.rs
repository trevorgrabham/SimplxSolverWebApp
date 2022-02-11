use crate::tableau::Tableau;

impl Tableau {
    pub fn solve(&mut self) {
        for _ in 0..self.n + 1 {
            if self.solved || self.error { return; }
            if self.DEBUG {
                self.print_table();
            }
            self.iterate();
        }
    }
}