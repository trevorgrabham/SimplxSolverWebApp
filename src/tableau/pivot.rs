use crate::tableau::Tableau;

impl Tableau {
    pub fn pivot(&mut self) {
        match (self.entering_var_index, self.leaving_var_index) {
            (Some(entering_index), Some(leaving_index)) => {
                let leaving_row = &self.A[leaving_index];
                let entering_col: Vec<Ratio<i64>> = self.A.iter()
                                                          .map(|row| row.iter().cloned().nth(entering_index).unwrap())
                                                          .collect();
                let new_a: Vec<Vec<Ratio<i64>>> = self.A.iter()
                                                        .zip(0..self.m)
                                                        .zip(entering_col.iter())
                                                        .map(|((row, row_index), &entering_el)| row.iter()
                                                                                      .zip(leaving_row.iter())
                                                                                      .map(|(old_el, &leaving_el)| if row_index == leaving_index { 
                                                                                              old_el / entering_el 
                                                                                          } else {
                                                                                              old_el - leaving_el * entering_el / self.A[leaving_index][entering_index]
                                                                                          })
                                                                                      .collect::<Vec<Ratio<i64>>>())
                                                        .collect();
                let new_reduced_cost: Vec<M> = self.reduced_cost.iter()
                                                                .zip(leaving_row.iter())
                                                                .map(|(old_el, leaving_el)| old_el - leaving_el * &self.reduced_cost[entering_index] / &self.A[leaving_index][entering_index])
                                                                .collect();
                let new_b: Vec<Ratio<i64>> = self.b.iter()
                                                   .zip(entering_col.iter())
                                                   .zip(0..self.m)
                                                   .map(|((old_el, entering_el), row_index)| if row_index == leaving_index { 
                                                            old_el / &self.A[leaving_index][entering_index] 
                                                        } else { 
                                                            old_el - entering_el * &self.b[leaving_index] / &self.A[leaving_index][entering_index] 
                                                        })
                                                   .collect();
                self.obj -= &(&self.b[leaving_index] * &self.reduced_cost[entering_index] / &self.A[leaving_index][entering_index]);
                self.A = new_a;
                self.reduced_cost = new_reduced_cost;
                self.b = new_b;
                self.basis_indecies[leaving_index] = entering_index;
                return;
            },
            (Some(_), None) => {
                self.error = true;
                self.error_message = String::from("No leaving variable has been identified.");
                return;
            },
            (None, Some(_)) => {
                self.error = true;
                self.error_message = String::from("No entering variable has been identified.");
                return;
            }, 
            (None, None) => {
                self.error = true;
                self.error_message = String::from("No entering or leaving variables have been identified.");
                return;
            },
        }
    }
}