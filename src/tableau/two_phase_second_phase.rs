use crate::tableau::Tableau;
use crate::m::M;

use num::rational::Ratio;

impl Tableau {
    pub fn two_phase_second_phase(&mut self) {
        if self.obj != M::new(Ratio::new(0i64,1), Ratio::new(0i64,1)) {
            self.error = true;
            self.error_message = String::from("Optimal solution is non-zero, therefore it is impossible to solve the LP without an artificial variable. The underlying LP is infeasible.");
            return;
        }
        let artificial_vars_indecies:Vec<(usize,usize)> = (0..self.m).zip(self.basis_indecies.iter())
                                                                     .map(|(row_index, &col_index)| if self.c[col_index] == M::new(Ratio::new(-1i64,1), Ratio::new(0i64,1)) { (row_index, col_index) } else { (self.m, col_index) })
                                                                     .filter(|(row_index, _)| row_index < &self.m)
                                                                     .collect();
        if self.DEBUG {
            for (row_index, col_index) in &artificial_vars_indecies {
                print!("row: {}, col: {}", row_index, col_index);
            }
            println!("");
        }
        for (row_index, col_index) in artificial_vars_indecies {
            if self.DEBUG {
                self.print_table();
            }
            let entering_index = self.A[row_index].iter()
                                                  .position(|&el| el != Ratio::from_integer(0i64));
            if self.DEBUG {
                println!("Entering index: {:?}", entering_index);
            }
            match entering_index {
                Some(index) if index == col_index => {
                    self.remove_row(row_index);
                    self.remove_col(col_index);
                },
                Some(index) => {
                    self.entering_var_index = Some(index);
                    self.leaving_var_index = Some(col_index);
                    self.pivot();
                },
                None => {
                    self.error = true;
                    self.error_message = String::from("Something went wrong during the transition between phases in the two-phase simplex method.");
                    return;
                },
            }
        }
        self.has_artificial_vars = false;
        self.solved = false;
        self.calc_reduced_cost();
        self.solve();
    }
}