use crate::tableau::Tableau;

impl Tableau {
    pub fn get_solution(&mut self) {
        for (i, &basis_index) in self.basis_indecies.iter().enumerate() {
            self.solution[basis_index] = self.b[i];
        }
    }
}