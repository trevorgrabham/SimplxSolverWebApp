#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use num::rational::Ratio;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct TableauData {
    A_numerators: Vec<Vec<i64>>,
    A_denominators: Vec<Vec<i64>>,
    b_numerators: Vec<i64>,
    b_denominators: Vec<i64>,
    c_numerators: Vec<i64>,
    c_denominators: Vec<i64>,
    m: usize,
    n: usize,
    solve_algorithm: String,
    variable_select_type: String,
    error: bool,
    error_message: String,
    reduced_cost_numerators: Vec<i64>,
    reduced_cost_denominators: Vec<i64>,
    basis_indecies: Vec<usize>,
    obj_numerator: i64,
    obj_denominator: i64,
    solved: bool,
}

#[derive(Debug)]
struct Tableau {
    A: Vec<Vec<Ratio<i64>>>,
    b: Vec<Ratio<i64>>,
    c: Vec<Ratio<i64>>,
    m: usize,
    n: usize,
    obj: Ratio<i64>,
    basis_indecies: Vec<usize>,
    reduced_cost: Vec<Ratio<i64>>,
    has_artificial_vars: bool,
    variable_select_type: String,
    solve_algorithm: String,
    solved: bool,
    error: bool, 
    error_message: String,
    entering_var_index: Option<usize>,
    leaving_var_index: Option<usize>,
}

impl TableauData {
    fn new(t: Tableau) -> TableauData {
        let (a_num, a_den): (Vec<Vec<i64>>, Vec<Vec<i64>>) = t.A.into_iter()
                                                                .map(|row| row.into_iter()
                                                                              .map(|el| (*el.numer(), *el.denom()))
                                                                              .unzip())
                                                                .unzip();
        let (b_num, b_den): (Vec<i64>, Vec<i64>) = t.b.into_iter()
                                                      .map(|el| (*el.numer(), *el.denom()))
                                                      .unzip();
        let (c_num, c_den): (Vec<i64>, Vec<i64>) = t.c.into_iter()
                                                      .map(|el| (*el.numer(), *el.denom()))
                                                      .unzip();
        let (reduced_cost_num, reduced_cost_den): (Vec<i64>, Vec<i64>) = t.reduced_cost.into_iter()
                                                                                      .map(|el| (*el.numer(), *el.denom()))
                                                                                      .unzip();
        TableauData {
            A_numerators: a_num,
            A_denominators: a_den,
            b_numerators: b_num,
            b_denominators: b_den,
            c_numerators: c_num,
            c_denominators: c_den,
            m: t.m,
            n: t.n,
            solve_algorithm: t.solve_algorithm,
            variable_select_type: t.variable_select_type,
            error: t.error,
            error_message: t.error_message,
            reduced_cost_numerators: reduced_cost_num,
            reduced_cost_denominators: reduced_cost_den,
            obj_numerator: *t.obj.numer(),
            obj_denominator: *t.obj.denom(),
            basis_indecies: t.basis_indecies,
            solved: t.solved,
        }
    }
}

impl Tableau {
    fn new(t: TableauData) -> Tableau {
        let mut a = vec![Vec::with_capacity(t.m);t.n];
        let zipped_a = t.A_numerators.into_iter().zip(t.A_denominators.into_iter());
        for (col, zipped_col) in a.iter_mut().zip(zipped_a.into_iter()) {
            for (num, den) in zipped_col.0.into_iter().zip(zipped_col.1.into_iter()) {
                col.push(Ratio::new(num, den));
            }
        }
        let mut b = Vec::with_capacity(t.m);
        let zipped_b = t.b_numerators.into_iter().zip(t.b_denominators.into_iter());
        for (num, den) in zipped_b {
            b.push(Ratio::new(num, den));
        }
        let mut c = Vec::with_capacity(t.n);
        let zipped_c = t.c_numerators.into_iter().zip(t.c_denominators.into_iter());
        for (num, den) in zipped_c {
            c.push(Ratio::new(num, den));
        }
        Tableau {
            A: a,
            b: b,
            c: c,
            m: t.m,
            n: t.n,
            obj: Ratio::from_integer(-1),
            basis_indecies: vec![t.m;t.m],
            reduced_cost: Vec::with_capacity(t.n),
            has_artificial_vars: false,
            variable_select_type: t.variable_select_type,
            solve_algorithm: t.solve_algorithm,
            solved: false,
            error: false,
            error_message: String::from(""),
            entering_var_index: None,
            leaving_var_index: None,
        }
    }

    fn find_basis_matrix(&mut self) {
        let mut I = vec![Ratio::new(0,1);self.m];
        I[0] = Ratio::new(1,1);
        for i in 0..self.m {
            let res = self.A.iter_mut().position(|row| row == &I);
            match res {
                Some(index) => {
                    self.basis_indecies[i] = index;
                },
                None => {
                    self.A.push(I.clone());
                    self.c.push(Ratio::from_integer(i64::MIN));
                    self.basis_indecies[i] = self.n;
                    self.n += 1;
                    self.has_artificial_vars = true;
                }
            }
            I.rotate_right(1);
        }
    }

    fn calc_reduced_cost(&mut self) {
        match self.has_artificial_vars {
            false => {
                let mut basis_cost= Vec::with_capacity(self.m);
                for i in 0..self.m {
                    basis_cost.push(self.c[self.basis_indecies[i]]);
                }
                let mut sums: Vec<Ratio<i64>> = Vec::with_capacity(self.n);
                for row in &self.A {
                    sums.push(row
                                .iter()
                                .zip(basis_cost.iter())
                                .map(|(r, c)| r*c )
                                .sum());
                }
                self.reduced_cost = sums.iter()
                    .zip(self.c.iter())
                    .map(|(sum, c)| sum - c)
                    .collect();

                self.obj = basis_cost.iter()
                                    .zip(self.b.iter())
                                    .map(|(c, b)| c*b)
                                    .sum();
            }, 
            true => {
            }
        }
    }

    fn select_entering_var(&mut self) {
        match self.variable_select_type.as_str() {
            "standard" => {
                match self.solve_algorithm.as_str() {
                    "standard" => {
                        let min_value = self.reduced_cost.iter().min();
                        match min_value {
                            Some(min_v) if min_v < &Ratio::from_integer(0i64) => {
                                let min_index = self.reduced_cost.iter()
                                                                 .position(|value| value == min_v)
                                                                 .unwrap();
                                self.entering_var_index = Some(min_index);
                                ()
                            },
                            Some(_) => {
                                self.solved = true;
                                self.entering_var_index = None;
                                ()
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
                                ()
                            }
                        }
                    }, 
                    "dual" => {
                        self.entering_var_index = Some(self.n);
                        ()
                    },
                    _ => {
                        self.error = true;
                        self.error_message = String::from("Invalid type for solve algorithm selection");
                        self.entering_var_index = None;
                        ()
                    }
                }
            },
            "bland" => {
                self.entering_var_index = Some(self.n);
                ()
            },
            _ => {
                self.error = true;
                self.error_message = String::from("Invalid type for variable selection method");
                self.entering_var_index = None;
                ()
            }
        }
    }

    fn select_leaving_var(&mut self) {
        match self.variable_select_type.as_str() {
            "standard" => {
                match self.solve_algorithm.as_str() {
                    "standard" => {
                        if self.m <= 0 {
                            self.error = true;
                            self.error_message = String::from("Coefficient matrix is empty. Cannot solve an empty coefficient matrix.");
                            self.leaving_var_index = None;
                            ()
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
                        let ratios: Vec<Ratio<i64>> = self.A[entering_index].iter()
                                                           .zip(self.b.iter())
                                                           .map(|(&a, &b)| b/a)
                                                           .collect();
                        let min_ratio = ratios.iter()
                                              .filter(|&&el| el > Ratio::from_integer(0i64))
                                              .min()
                                              .unwrap();
                        let min_index = ratios.iter()
                                              .position(|el| el == min_ratio)
                                              .unwrap();
                        self.leaving_var_index = Some(min_index);
                        ()
                    },
                    "dual" => {
                        self.leaving_var_index = None;
                        ()
                    },
                    _ => {
                        self.error = true;
                        self.error_message = String::from("Invalid type for solve algorithm selection");
                        self.leaving_var_index = None;
                        ()
                    }
                }
            },
            "bland" => {
                self.leaving_var_index = Some(self.n);
                ()
            },
            _ => {
                self.error = true;
                self.error_message = String::from("Invalid type for variable selection method");
                self.leaving_var_index = None;
                ()
            }
        }
    }

    fn pivot(&mut self) {

    }

    fn iterate(&mut self) {
        match self.solve_algorithm.as_str() {
            "standard" => {
                self.select_entering_var(); 
                self.select_leaving_var(); 
                self.pivot();
            },
            "dual" => {
                self.select_leaving_var(); 
                self.select_entering_var(); 
                self.pivot();
            },
            _ => { 
                self.error = true;
                self.error_message = String::from("Invalid solve algorithm selection.");
            }
        }
    }

    fn print_table(&self) {
        for row in 0..self.m {
            print!("[\t");
            for col in 0..self.n {
                print!("{}\t", self.A[col][row]);
            }
            print!("|\t{}\t", self.b[row]);
            println!("]");
        }
        for _ in 0..(self.n+3) {
            print!("________");
        } 
        print!("\n[\t");
        for col in 0..self.n {
            if self.reduced_cost[col] == Ratio::from_integer(i64::MIN) {
                print!("-M\t");
            } else {
                print!("{}\t", self.reduced_cost[col]);
            }
        }
        print!("|\t{}\t", self.obj);
        println!("]\n");
    }

    fn get_basis(&self) -> (usize, usize, usize) {
        (self.basis_indecies[0], self.basis_indecies[1], self.basis_indecies[2])
    }
}


#[post("/setup", format = "json", data = "<tableau>")]
fn setup(tableau: Json<TableauData>) -> Json<TableauData> {
    let mut t = Tableau::new(tableau.0);
    t.find_basis_matrix();
    t.calc_reduced_cost();
    t.select_entering_var();
    t.select_leaving_var();
    t.print_table();
    println!("Entering variable index: {}\nLeaving variable index: {}", t.entering_var_index.unwrap(), t.leaving_var_index.unwrap());
    let basis_indecies = t.get_basis();
    Json(TableauData::new(t))
}

fn main() {
    rocket::ignite().mount("/", routes![setup]).launch();
}
