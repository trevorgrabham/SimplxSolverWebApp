use crate::tableau::Tableau;

impl Tableau {
    pub fn calc_reduced_cost(&mut self) {
        let basis_cost:Vec<&M>;
        if self.has_artificial_vars && self.big_M_solve_algorithm.as_str() == "two-phase" {
            basis_cost = self.basis_indecies.iter()
                                            .map(|&index| &self.two_phase_c[index])
                                            .collect();
        } else {
            basis_cost = self.basis_indecies.iter()
                                            .map(|&index| &self.c[index])
                                            .collect();
        }
        if self.DEBUG {
            print!("basis_cost: [");
            for el in &basis_cost {
                print!("{}, ", el);
            }
            print!("]\n");
        }
        for col in 0..self.n {
            self.reduced_cost.push(M::new(Ratio::new(0i64, 1), Ratio::new(0i64, 1)));
            for row in 0..self.m {
                self.reduced_cost[col] += &(&self.A[row][col] * basis_cost[row]);
            }
            if self.has_artificial_vars && self.big_M_solve_algorithm.as_str() == "two-phase" {
                self.reduced_cost[col] -= &self.two_phase_c[col];
            } else {
                self.reduced_cost[col] -= &self.c[col];
            }
        }
        self.obj = basis_cost.iter()
                             .zip(self.b.iter())
                             .map(|(&c, b)| b*c )
                             .sum();
    }
}