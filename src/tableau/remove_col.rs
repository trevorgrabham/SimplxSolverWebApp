use crate::tableau::Tableau;

impl Tableau {
    pub fn remove_col(&mut self, col_index: usize) {
        for row in self.A.iter_mut() {
            row.remove(col_index);
        }
        for index in self.basis_indecies.iter_mut() {
            if *index > col_index {
                *index -= 1;
            }
        }
        self.n -= 1;
    }
}