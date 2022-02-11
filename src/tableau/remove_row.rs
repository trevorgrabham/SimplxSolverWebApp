use crate::tableau::Tableau;

impl Tableau {
    pub fn remove_row(&mut self, row_index: usize) {
        self.A.remove(row_index);
        self.b.remove(row_index);
        self.basis_indecies.remove(row_index);
        self.m -= 1;
    }
}