use crate::tableau::Tableau;
use crate::m::M;

use num::rational::Ratio;

impl Tableau {
    pub fn add_col(&mut self, row_index: usize, c: M) {
        if &c.M != &Ratio::new(0i64,1) {
            self.has_artificial_vars = true;
        }
        for row in &mut self.A {
            row.push(Ratio::new(0i64,1));
        }
        self.A[row_index][self.n] = Ratio::new(1i64,1);
        self.c.push(c);
        if self.big_M_solve_algorithm.as_str() == "two-phase" {
            self.two_phase_c.push(M::new(Ratio::new(0i64,1), Ratio::new(-1i64,1)));
        }
        self.basis_indecies[row_index] = self.n;
        self.n += 1;
    }
}