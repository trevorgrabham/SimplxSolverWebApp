use crate::tableau::Tableau;

use num::rational::Ratio;

impl Tableau {
    pub fn select_leaving_var(&mut self) {
        match self.solve_algorithm.as_str() {
            "standard" => {
                if self.m <= 0 {
                    self.error = true;
                    self.error_message = String::from("Coefficient matrix is empty. Cannot solve an empty coefficient matrix.");
                    self.leaving_var_index = None;
                    return;
                }
                let entering_index = match self.entering_var_index {
                    None => {
                        self.error = true;
                        self.error_message = String::from("Something seems to have gone wrong. Standard solve requires to select an entering variable before a leaving variable can be selected.");
                        self.leaving_var_index = None;
                        return;
                    },
                    Some(index) => index
                };
                let ratios: Vec<Ratio<i64>> = self.A.iter()
                                                    .map(|row| &row[entering_index])
                                                    .zip(self.b.iter())
                                                    .map(|(a, b)| if a > &Ratio::from_integer(0i64) { b/a } else { Ratio::new(i64::MAX, 1) })
                                                    .collect();
                let min_ratio = ratios.iter()
                                      .min();
                let min_index = match min_ratio {
                    Some(ratio) => {
                        ratios.iter()
                              .position(|el| el == ratio)
                              .unwrap()
                    },
                    None => {
                        self.error = true;
                        self.error_message = String::from("Problem is unbounded.");
                        self.leaving_var_index = None;
                        return;
                    }
                };
                self.leaving_var_index = Some(min_index);
                return;
            },
            "dual" => {
                self.leaving_var_index = None;
                return;
            },
            _ => {
                self.error = true;
                self.error_message = String::from("Invalid type for solve algorithm selection");
                self.leaving_var_index = None;
                return;
            }
        }
    }
}