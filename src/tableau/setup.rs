use crate::tableau::Tableau;

use num::rational::Ratio;

impl Tableau {
    pub fn setup(&mut self) {
        if self.b.iter().find(|&&el| el < Ratio::new(0i64,1)).is_some() { 
            self.error = true;
            self.error_message = String::from("Cannot have negative values for b. Please multiply any rows with a negative b value by -1.");
            return;
        }
        self.find_basis_indecies();
        self.calc_reduced_cost();
    }
}