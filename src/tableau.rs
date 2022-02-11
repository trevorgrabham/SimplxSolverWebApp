#![allow(non_snake_case)]

use crate::m::M;
use crate::tableau_data::TableauData;
use num::rational::Ratio;

#[derive(Debug)]
pub struct Tableau {
    DEBUG: bool,
    pub A: Vec<Vec<Ratio<i64>>>,
    pub b: Vec<Ratio<i64>>,
    pub c: Vec<M>,
    pub m: usize,
    pub n: usize,
    pub obj: M,
    pub basis_indecies: Vec<usize>,
    pub reduced_cost: Vec<M>,
    two_phase_c: Vec<M>,
    pub has_artificial_vars: bool,
    variable_select_type: String,
    solve_algorithm: String,
    pub big_M_solve_algorithm: String,
    pub solved: bool,
    pub error: bool, 
    pub error_message: String,
    entering_var_index: Option<usize>,
    leaving_var_index: Option<usize>,
    pub solution: Vec<Ratio<i64>>,
}

impl Tableau {
    pub fn new(t: TableauData) -> Tableau {
        let a: Vec<Vec<Ratio<i64>>> = t.A_numerators.into_iter()
                                                    .zip(t.A_denominators.into_iter())
                                                    .map(|(num_row, den_row)| num_row.into_iter()
                                                                                     .zip(den_row.into_iter())
                                                                                     .map(|(num, den)| Ratio::new(num, den))
                                                                                     .collect())
                                                    .collect();
        let b: Vec<Ratio<i64>> = t.b_numerators.into_iter()
                                               .zip(t.b_denominators.into_iter())
                                               .map(|(num, den)| Ratio::new(num, den))
                                               .collect();
        let c: Vec<M> = t.c_m_numerators.into_iter()
                                        .zip(t.c_m_denominators.into_iter())
                                        .zip(t.c_numerators.into_iter()
                                                           .zip(t.c_denominators.into_iter()))
                                        .map(|((m_num, m_den), (const_num, const_den))| M::new(Ratio::new(m_num, m_den), Ratio::new(const_num, const_den)))
                                        .collect();
        Tableau {
            DEBUG: true,
            A: a,
            b: b,
            c: c,
            m: t.m,
            n: t.n,
            obj: M::new(Ratio::from_integer(0i64), Ratio::from_integer(-1i64)),
            basis_indecies: vec![t.n;t.m],
            reduced_cost: Vec::with_capacity(t.n),
            two_phase_c: vec![M::new(Ratio::from_integer(0i64), Ratio::from_integer(0i64));t.n],
            has_artificial_vars: false,
            variable_select_type: t.variable_select_type,
            solve_algorithm: t.solve_algorithm,
            big_M_solve_algorithm: t.big_M_solve_algorithm,
            solved: false,
            error: false,
            error_message: String::from(""),
            entering_var_index: None,
            leaving_var_index: None,
            solution: vec![Ratio::from_integer(0i64);t.n],
        }

    }

    pub fn error_tableau(t: Tableau) -> Tableau {
        Tableau {
            DEBUG: true,
            A: vec![Vec::with_capacity(0);1],
            b: Vec::with_capacity(0),
            c: Vec::with_capacity(0),
            m: 0,
            n: 0,
            reduced_cost: Vec::with_capacity(0),
            two_phase_c: Vec::with_capacity(0),
            obj: M::new(Ratio::from_integer(0i64), Ratio::from_integer(-1i64)),
            basis_indecies: Vec::with_capacity(0),
            has_artificial_vars: false,
            variable_select_type: String::from(""),
            solve_algorithm: String::from(""),
            big_M_solve_algorithm: String::from(""),
            solved: false,
            error: t.error,
            error_message: t.error_message,
            entering_var_index: None,
            leaving_var_index: None,
            solution: Vec::with_capacity(0),
        }
    }

    fn find_basis_indecies(&mut self) {
        let mut a_cols = vec![Vec::with_capacity(self.m);self.n];
        for row in 0..self.m {
            for col in 0..self.n {
                a_cols[col].push(self.A[row][col]);
            }
        }
        let mut I = vec![Ratio::new(0i64,1);self.m];
        I[0] = Ratio::new(1i64,1);
        for i in 0..self.m {
            let res = a_cols.iter().position(|col| col == &I);
            match res {
                Some(index) => {
                    self.basis_indecies[i] = index;
                },
                None => {
                    self.add_artificial_var(i);
                }
            }
            I.rotate_right(1);
        }
    }

    fn calc_reduced_cost(&mut self) {
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

    fn select_entering_var(&mut self) {
        match self.variable_select_type.as_str() {
            "standard" => {
                match self.solve_algorithm.as_str() {
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
                    "dual" => {
                        self.entering_var_index = Some(self.n);
                        return;
                    },
                    _ => {
                        self.error = true;
                        self.error_message = String::from("Invalid type for solve algorithm selection");
                        self.entering_var_index = None;
                        return;
                    }
                }
            },
            "bland" => {
                match self.solve_algorithm.as_str() {
                    "standard" => {
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
                    "dual" => {
                        self.entering_var_index = Some(self.n);
                        return;
                    },
                    _ => {
                        self.error = true;
                        self.error_message = String::from("Invalid type for solve algorithm selection");
                        self.entering_var_index = None;
                        return;
                    },
                }
            },
            _ => {
                self.error = true;
                self.error_message = String::from("Invalid type for variable selection method");
                self.entering_var_index = None;
                return;
            }
        }
    }

    fn select_leaving_var(&mut self) {
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

    fn pivot(&mut self) {
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

    fn iterate(&mut self) {
        match self.solve_algorithm.as_str() {
            "standard" => {
                self.select_entering_var(); 
                if self.solved || self.error {
                    return;
                }
                self.select_leaving_var(); 
                if self.solved || self.error {
                    return;
                }
                if self.DEBUG {
                    println!("Entering index: {:?}\nLeaving index: {:?}\n", self.entering_var_index, self.leaving_var_index);
                }
                self.pivot();
            },
            "dual" => {
                self.select_leaving_var(); 
                if self.solved || self.error {
                    return;
                }
                self.select_entering_var(); 
                if self.solved || self.error {
                    return;
                }
                self.pivot();
            },
            _ => { 
                self.error = true;
                self.error_message = String::from("Invalid solve algorithm selection.");
            }
        }
    }

    pub fn setup(&mut self) {
        self.find_basis_indecies();
        self.calc_reduced_cost();
    }

    fn print_table(&self) {
        for row in 0..self.m {
            print!("[\t");
            for col in 0..self.n {
                print!("{}\t", self.A[row][col]);
            }
            print!("|\t{}\t", self.b[row]);
            println!("]");
        }
        for _ in 0..(self.n+3) {
            print!("________");
        } 
        print!("\n[\t");
        for col in 0..self.n {
            print!("{}\t", self.reduced_cost[col]);
        }
        print!("|\t{}\t", self.obj);
        println!("]\n");
    }

    pub fn get_solution(&mut self) {
        for (i, &basis_index) in self.basis_indecies.iter().enumerate() {
            self.solution[basis_index] = self.b[i];
        }
    }

    pub fn solve(&mut self) {
        for _ in 0..self.n + 1 {
            if self.solved || self.error { return; }
            if self.DEBUG {
                self.print_table();
            }
            self.iterate();
        }
    }

    fn add_artificial_var(&mut self, row_index: usize) {
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

    fn remove_row(&mut self, row_index: usize) {
        self.A.remove(row_index);
        self.b.remove(row_index);
        self.basis_indecies.remove(row_index);
        self.m -= 1;
    }

    fn remove_col(&mut self, col_index: usize) {
        for row in self.A.iter_mut() {
            row.remove(col_index);
        }
        for index in self.basis_indecies.iter_mut() {
            if *index > col_index {
                *index -= 1;
            }
        }
        self.n -= 1;
    }

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