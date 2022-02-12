use crate::tableau::Tableau;
use crate::m::M;

use num::rational::Ratio;

impl Tableau {
    pub fn select_entering_var(&mut self) {
        match self.solve_algorithm.as_str() {
            "standard" => {
                match self.variable_select_type.as_str() {
                    "standard" => {
                        let min_value = self.reduced_cost.iter().min();
                        // match on 3 cases: we have a negative value for our min reduced cost, we have a positive value for our min reduced cost, or we have an empty reduced cost 
                        match min_value {
                            Some(min_v) if min_v < &M::new(Ratio::new(0i64,1), Ratio::new(0i64,1)) => {
                                let min_index = self.reduced_cost.iter()
                                                                .position(|value| value == min_v)
                                                                .unwrap();
                                self.entering_var_index = Some(min_index);
                                return;
                            },
                            Some(_) => {
                                self.solved = true;
                                self.entering_var_index = None;
                                return;
                            }
                            None => {
                                self.error = true;
                                self.error_message = String::from("Reduced cost vector is empty. ");
                                if self.n != 0 {
                                    self.error_message.push_str("Reduced cost vector is not the same dimensions as our coefficient matrix.");
                                } else {
                                    self.error_message.push_str("Coefficient matrix is empty. Cannot solve an empty coefficient matrix.");
                                }
                                self.entering_var_index = None;
                                return;
                            }
                        }
                    }, 
                    "bland" => {
                        match self.reduced_cost.iter().position(|el| el < &M::new(Ratio::from_integer(0i64), Ratio::from_integer(0i64))) {
                            Some(index) => {
                                self.entering_var_index = Some(index);
                                return;
                            },
                            None => {
                                self.solved = true;
                                self.entering_var_index = None;
                                return;
                            }
                        }
                    },
                    _ => {
                        self.error = true;
                        self.error_message = String::from("Invalid type for variable selection method");
                        self.entering_var_index = None;
                        return;
                    }
                }
            }, 
            "dual" => {
                let leaving_index = match self.leaving_var_index {
                    Some(index) => index, 
                    None => {
                        self.error = true;
                        self.error_message = String::from("Something seems to have gone wrong. Dual simplex method requires to select a leaving variable before an entering variable can be selected.");
                        self.entering_var_index = None;
                        return;
                    },
                };
                let ratios: Vec<M> = self.A[leaving_index].iter()
                                                          .zip(self.reduced_cost.iter())
                                                          .map(|(a, rc)| if a >= &Ratio::new(0i64,1) { M::new(Ratio::new(0i64,1), Ratio::new(i64::MIN,1)) } else { rc / a })
                                                          .collect();
                let max_value = ratios.iter()
                                      .max();
                let max_index = match max_value {
                    Some(value) if value == &M::new(Ratio::new(0i64,1), Ratio::new(i64::MIN,1)) => {
                        self.error = true;
                        self.error_message = String::from("Problem is unbounded.");
                        self.entering_var_index = None;
                        return;
                    },
                    Some(value) => {
                        ratios.iter()
                              .position(|el| el == value)
                              .unwrap()
                    }, 
                    None => {
                        self.error = true;
                        self.error_message = String::from("Unknown error. The reduced cost vector seems to be emtpy.");
                        self.entering_var_index = None;
                        return;
                    }
                };
                self.entering_var_index = Some(max_index);
                return;
            },
            _ => {
                self.error = true;
                self.error_message = String::from("Invalid type for solve algorithm selection");
                self.entering_var_index = None;
                return;
            }
        }
    }
}