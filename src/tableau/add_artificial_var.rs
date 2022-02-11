use crate::tableau::Tableau;

impl Tableau {
    pub fn add_artificial_var(&mut self, row_index: usize) {
        for row in &mut self.A {
            row.push(Ratio::new(0i64,1));
        }
        self.A[row_index][self.n] = Ratio::new(1i64,1);
        self.c.push(M::new(Ratio::new(-1i64,1), Ratio::new(0i64,1)));
        if self.big_M_solve_algorithm.as_str() == "two-phase" {
            self.two_phase_c.push(M::new(Ratio::new(0i64,1), Ratio::new(-1i64,1)));
        }
        self.basis_indecies[row_index] = self.n;
        self.n += 1;
        self.has_artificial_vars = true;
    }
}