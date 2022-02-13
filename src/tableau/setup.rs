use crate::tableau::Tableau;
use crate::m::M;

use num::rational::Ratio;

struct FuturePivot {
    entering_index: usize,
    leaving_index: usize,
}

impl Tableau {
    pub fn setup(&mut self) {
        match self.solve_algorithm.as_str() {
            "standard" => {
                if self.b.iter().find(|&el| el < &M::new(Ratio::new(0i64,1), Ratio::new(0i64,1))).is_some() { 
                    self.error = true;
                    self.error_message = String::from("Cannot have negative values for b. Please multiply any rows with a negative b value by -1.");
                    return;
                }
                self.find_basis_indecies();
                self.calc_reduced_cost();
            },
            "dual" => {
                self.find_basis_indecies();
                self.reduced_cost = self.c.iter()
                                          .map(|c| -c)
                                          .collect();
                self.obj = M::new(Ratio::new(0i64,1), Ratio::new(0i64,1));
                let mut future_pivots: Vec<FuturePivot> = Vec::with_capacity(self.m);
                let mut rows_to_remove: Vec<usize> = Vec::with_capacity(self.m);
                for (row, row_index) in self.A.iter().zip(0..self.m) {
                    if self.basis_indecies[row_index] == self.n {
                        let entering_index = row.iter()
                                                .position(|el| el != &Ratio::new(0i64,1));
                        match entering_index {
                            Some(index) => {
                                future_pivots.push(FuturePivot { entering_index: index, leaving_index: row_index });
                            }, 
                            None => {
                                if self.b[row_index] != M::new(Ratio::new(0i64,1), Ratio::new(0i64,1)) {
                                    self.error = true; 
                                    self.error_message = format!("Cannot satisfy the {}th constraint. Problem is infeasible.", row_index);
                                    return;
                                } else {
                                    rows_to_remove.push(row_index);
                                }
                            }
                        }
                    }
                }
                for row in rows_to_remove {
                    if self.DEBUG {
                        self.print_table();
                    }
                    self.remove_row(row);
                }
                for pivot in future_pivots {
                    if self.DEBUG {
                        self.print_table();
                    }
                    self.entering_var_index = Some(pivot.entering_index);
                    self.leaving_var_index = Some(pivot.leaving_index);
                    self.pivot();
                }
                self.calc_reduced_cost();
                if self.reduced_cost.iter().find(|&el| el < &M::new(Ratio::new(0i64,1), Ratio::new(0i64,1))).is_none() {
                    return;
                }
                self.A.push(vec![Ratio::new(1i64,1);self.n]);
                for &index in &self.basis_indecies {
                    self.A[self.m][index] = Ratio::new(0i64,1);
                }
                if self.DEBUG {
                    self.print_table();
                }
                self.m += 1;
                self.basis_indecies.push(self.m);
                self.b.push(M::new(Ratio::new(1i64,1), Ratio::new(0i64,1)));
                self.solution.push(M::new(Ratio::new(0i64,1), Ratio::new(-1i64,1)));
                self.add_col(self.m - 1, M::new(Ratio::new(0i64,1), Ratio::new(0i64,1)));
                self.reduced_cost.push(M::new(Ratio::new(0i64,1), Ratio::new(0i64,1)));
                if self.DEBUG {
                    self.print_table();
                }
                self.leaving_var_index = Some(self.m - 1);
                let min_value = self.reduced_cost.iter()
                                                 .zip(0..self.m)
                                                 .filter(|&(_rc, index)| self.basis_indecies.iter().find(|&i| i == &index).is_none())
                                                 .map(|(rc, _index)| rc)
                                                 .min()
                                                 .unwrap();
                self.entering_var_index = self.reduced_cost.iter()
                                                           .position(|el| el == min_value);
                self.pivot();
                return;
            },
            _ => {
                self.error = true; 
                self.error_message = String::from("Unknown selection for solve algorithm.");
                return;
            }
        }
    }
}