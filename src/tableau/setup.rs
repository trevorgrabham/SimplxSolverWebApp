use crate::tableau::Tableau;

impl Tableau {
    pub fn setup(&mut self) {
        self.find_basis_indecies();
        self.calc_reduced_cost();
    }
}