#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use num::rational::Ratio;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Eq, Debug, Clone)]
struct M {
    M: Ratio<i64>,
    constant: Ratio<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TableauData {
    A_numerators: Vec<Vec<i64>>,
    A_denominators: Vec<Vec<i64>>,
    b_numerators: Vec<i64>,
    b_denominators: Vec<i64>,
    c_numerators: Vec<i64>,
    c_denominators: Vec<i64>,
    c_m_numerators: Vec<i64>,
    c_m_denominators: Vec<i64>,
    m: usize,
    n: usize,
    solve_algorithm: String,
    variable_select_type: String,
    error: bool,
    error_message: String,
    reduced_cost_numerators: Vec<i64>,
    reduced_cost_denominators: Vec<i64>,
    reduced_cost_m_numerators: Vec<i64>,
    reduced_cost_m_denominators: Vec<i64>,
    basis_indecies: Vec<usize>,
    obj_numerator: i64,
    obj_denominator: i64,
    obj_m_numerator: i64,
    obj_m_denominator: i64,
    solved: bool,
    solution_numerators: Vec<i64>,
    solution_denominators: Vec<i64>,
}

#[derive(Debug)]
struct Tableau {
    DEBUG: bool,
    A: Vec<Vec<Ratio<i64>>>,
    b: Vec<Ratio<i64>>,
    c: Vec<M>,
    m: usize,
    n: usize,
    obj: M,
    basis_indecies: Vec<usize>,
    reduced_cost: Vec<M>,
    has_artificial_vars: bool,
    variable_select_type: String,
    solve_algorithm: String,
    solved: bool,
    error: bool, 
    error_message: String,
    entering_var_index: Option<usize>,
    leaving_var_index: Option<usize>,
    solution: Vec<Ratio<i64>>,
}

impl M {
    fn new(m: Ratio<i64>, constant: Ratio<i64>) -> M {
        M {
            M: m,
            constant: constant,
        }
    }
}

impl std::ops::Mul<M> for Ratio<i64> {
    type Output = M;

    fn mul(self, rhs: M) -> M {
        M {
            M: rhs.M * self,
            constant: rhs.constant * self,
        }
    }
}

impl std::ops::Mul<&M> for Ratio<i64> {
    type Output = M;

    fn mul(self, rhs: &M) -> M {
        M {
            M: rhs.M * self,
            constant: rhs.constant * self,
        }
    }
}

impl std::ops::Add for M {
    type Output = Self;

    fn add(self, rhs: Self) -> Self { 
        M {
            M: self.M + rhs.M,
            constant: self.constant + rhs.constant,
        }
    }
}

impl std::ops::Sub for M {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        M {
            M: self.M - rhs.M,
            constant: self.constant - rhs.constant,
        }
    }
}

impl<'a> std::iter::Sum<Self> for M {
    fn sum<I>(iter: I) -> Self 
    where
        I: Iterator<Item = Self>,
        {
            iter.fold(Self { M: Ratio::from_integer(0i64), constant: Ratio::from_integer(0i64), }, |a ,b| Self { M: a.M + b.M, constant: a.constant + b.constant, })
        }
}

impl std::ops::Div<Ratio<i64>> for M {
    type Output = Self;

    fn div(self, rhs: Ratio<i64>) -> Self {
        M {
            M: self.M / rhs,
            constant: self.constant / rhs,
        }
    }
}

impl std::ops::AddAssign for M {
    fn add_assign(&mut self, rhs: Self) {
        self.M += rhs.M;
        self.constant += rhs.constant;
    }
}

impl std::ops::SubAssign<&M> for M {
    fn sub_assign(&mut self, rhs: &Self) {
        self.M -= rhs.M;
        self.constant -= rhs.constant;
    }
}

impl PartialOrd for M {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.M.partial_cmp(&other.M) {
            Some(std::cmp::Ordering::Equal) => { 
                self.constant.partial_cmp(&other.constant)
            },
            Some(ord) => {
                Some(ord)
            }
            None => {
                None
            }
        }
    }
}

impl Ord for M {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.M.cmp(&other.M) {
            std::cmp::Ordering::Equal => {
                self.constant.cmp(&other.constant)
            },
            ord => {
                ord
            }
        }
    }
}

impl PartialEq for M {
    fn eq(&self, other: &Self) -> bool {
        self.M == other.M && self.constant == other.constant
    }
}

impl std::fmt::Display for M {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.M == Ratio::from_integer(0i64) {
            true => {
                write!(f, "{}", self.constant)
            }, 
            false => {
                match self.constant {
                    c if c > Ratio::from_integer(0i64) => {
                        write!(f, "{}M+{}", self.M, self.constant)
                    },
                    c if c < Ratio::from_integer(0i64) => {
                        write!(f, "{}M{}", self.M, self.constant)
                    },
                    _ => {
                        write!(f, "{}M", self.M)
                    },
                }
            },
        }
    }
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
        let ((c_num, c_den), (c_m_num, c_m_den)): ((Vec<i64>, Vec<i64>),(Vec<i64>, Vec<i64>)) = t.c.into_iter()
                                                      .map(|el| ((*el.constant.numer(), *el.constant.denom()), (*el.M.numer(), *el.M.denom())))
                                                      .unzip();
        let ((reduced_cost_num, reduced_cost_den), (reduced_cost_m_num, reduced_cost_m_den)): ((Vec<i64>, Vec<i64>), (Vec<i64>, Vec<i64>)) = t.reduced_cost.into_iter()
                                                                                                                               .map(|el| ((*el.constant.numer(), *el.constant.denom()), (*el.M.numer(), *el.M.denom())))
                                                                                                                               .unzip();
        let (sol_num, sol_den): (Vec<i64>, Vec<i64>) = t.solution.into_iter()
                                                                 .map(|el| (*el.numer(), *el.denom()))
                                                                 .unzip();
        TableauData {
            A_numerators: a_num,
            A_denominators: a_den,
            b_numerators: b_num,
            b_denominators: b_den,
            c_numerators: c_num,
            c_denominators: c_den,
            c_m_numerators: c_m_num,
            c_m_denominators: c_m_den,
            m: t.m,
            n: t.n,
            solve_algorithm: t.solve_algorithm,
            variable_select_type: t.variable_select_type,
            error: t.error,
            error_message: t.error_message,
            reduced_cost_numerators: reduced_cost_num,
            reduced_cost_denominators: reduced_cost_den,
            reduced_cost_m_numerators: reduced_cost_m_num,
            reduced_cost_m_denominators: reduced_cost_m_den,
            obj_numerator: *t.obj.constant.numer(),
            obj_denominator: *t.obj.constant.denom(),
            obj_m_numerator: *t.obj.M.numer(),
            obj_m_denominator: *t.obj.M.denom(),
            basis_indecies: t.basis_indecies,
            solved: t.solved,
            solution_numerators: sol_num,
            solution_denominators: sol_den,
        }
    }
}

impl Tableau {
    fn new(t: TableauData) -> Tableau {
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
        let c: Vec<M> = t.c_numerators.into_iter()
                                      .zip(t.c_denominators.into_iter())
                                      .zip(t.c_m_numerators.into_iter().zip(t.c_m_denominators.into_iter()))
                                      .map(|((const_num, const_den), (m_num, m_den))| M::new(Ratio::new(m_num, m_den), Ratio::new(const_num, const_den)))
                                      .collect();
        Tableau {
            DEBUG: true,
            A: a,
            b: b,
            c: c,
            m: t.m,
            n: t.n,
            obj: M::new(Ratio::from_integer(0i64), Ratio::from_integer(-1i64)),
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
            solution: vec![Ratio::from_integer(0i64);t.n],
        }

    }

    fn error_tableau(t: Tableau) -> Tableau {
        Tableau {
            DEBUG: true,
            A: vec![Vec::with_capacity(0);1],
            b: Vec::with_capacity(0),
            c: Vec::with_capacity(0),
            m: 0,
            n: 0,
            reduced_cost: Vec::with_capacity(0),
            obj: M::new(Ratio::from_integer(0i64), Ratio::from_integer(-164)),
            basis_indecies: Vec::with_capacity(0),
            has_artificial_vars: false,
            variable_select_type: String::from(""),
            solve_algorithm: String::from(""),
            solved: false,
            error: t.error,
            error_message: t.error_message,
            entering_var_index: None,
            leaving_var_index: None,
            solution: Vec::with_capacity(0),
        }
    }

    fn find_basis_matrix(&mut self) {
        let mut a_cols = vec![Vec::with_capacity(self.m);self.n];
        for row in 0..self.m {
            for col in 0..self.n {
                a_cols[col].push(self.A[row][col]);
            }
        }
        let mut I = vec![Ratio::new(0,1);self.m];
        I[0] = Ratio::new(1,1);
        for i in 0..self.m {
            let res = a_cols.iter().position(|col| col == &I);
            match res {
                Some(index) => {
                    self.basis_indecies[i] = index;
                },
                None => {
                    for row in &mut self.A {
                        row.push(Ratio::new(0i64,1));
                    }
                    self.A[i][self.n] = Ratio::new(1i64,1);
                    self.c.push(M::new(Ratio::from_integer(-1i64), Ratio::from_integer(0)));
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
            false | true => {
                let basis_cost:Vec<&M> = self.basis_indecies.iter()
                                                                    .map(|&index| &self.c[index])
                                                                    .collect();
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
                        self.reduced_cost[col] += self.A[row][col] * basis_cost[row];
                    }
                    self.reduced_cost[col] -= &self.c[col];
                }
                self.obj = basis_cost.iter().cloned()
                                    .zip(self.b.iter().cloned())
                                    .map(|(c, b)| b*c )
                                    .sum();
            }, 
            _ => {
            // true => {
            }
        }
    }

    fn select_entering_var(&mut self) {
        match self.variable_select_type.as_str() {
            "standard" => {
                match self.solve_algorithm.as_str() {
                    "standard" => {
                        let min_value = self.reduced_cost.iter().min();
                        // match on 3 cases: we have a negative value for our min reduced cost, we have a positive value for our min reduced cost, or we have an empty reduced cost 
                        match min_value {
                            Some(min_v) if min_v < &M::new(Ratio::from_integer(0i64), Ratio::from_integer(0i64)) => {
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
                        let ratios: Vec<Ratio<i64>> = self.A.iter()
                                                            .map(|row| row[entering_index])
                                                            .zip(self.b.iter())
                                                            .map(|(a, b)| if a != Ratio::from_integer(0i64) { b/a } else { Ratio::new(i64::MAX, 1) })
                                                            .collect();
                        let min_ratio = ratios.iter()
                                              .filter(|&&el| el >= Ratio::from_integer(0i64))
                                              .min();
                        let min_ratio = match min_ratio {
                            Some(ratio) => ratio,
                            None => {
                                self.error = true;
                                self.error_message = String::from("Problem is unbounded.");
                                self.leaving_var_index = None;
                                return;
                            }
                        };
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
        match (self.entering_var_index, self.leaving_var_index) {
            (Some(entering_index), Some(leaving_index)) => {
                let leaving_row = &self.A[leaving_index];
                let entering_col: Vec<Ratio<i64>> = self.A.iter()
                                         .map(|row| row.iter().cloned().nth(entering_index).unwrap())
                                         .collect();
                let new_a: Vec<Vec<Ratio<i64>>> = self.A.iter()
                                                        .zip(entering_col.iter())
                                                        .map(|(row, &entering_el)| row.iter()
                                                                                        .zip(leaving_row.iter())
                                                                                        .map(|(row_el, &leaving_el)| if row == leaving_row { row_el / entering_el } else { row_el - leaving_el * entering_el / self.A[leaving_index][entering_index]})
                                                                                        .collect::<Vec<Ratio<i64>>>())
                                                        .collect();
                let new_reduced_cost: Vec<M> = self.reduced_cost.iter().cloned()
                                                                         .zip(leaving_row.iter())
                                                                         .map(|(reduced_el, &leaving_el)| reduced_el - leaving_el * &self.reduced_cost[entering_index] / self.A[leaving_index][entering_index])
                                                                         .collect();
                let new_b: Vec<Ratio<i64>> = self.b.iter()
                                                   .zip(entering_col.iter())
                                                   .zip(0..self.m)
                                                   .map(|((b_el, entering_el), index)| if index == leaving_index { b_el / self.A[leaving_index][entering_index] } else { b_el - entering_el * self.b[leaving_index] / self.A[leaving_index][entering_index] })
                                                   .collect();
                self.obj -= &(self.b[leaving_index] * &self.reduced_cost[entering_index] / self.A[leaving_index][entering_index]);
                self.A = new_a;
                self.reduced_cost = new_reduced_cost;
                self.b = new_b;
                self.basis_indecies[leaving_index] = entering_index;
                ()
            },
            (Some(_), None) => {
                self.error = true;
                self.error_message = String::from("No leaving variable has been identified.");
                ()
            },
            (None, Some(_)) => {
                self.error = true;
                self.error_message = String::from("No entering variable has been identified.");
                ()
            }, 
            (None, None) => {
                self.error = true;
                self.error_message = String::from("No entering or leaving variables have been identified.");
                ()
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

    fn setup(&mut self) {
        self.find_basis_matrix();
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

    fn get_solution(&mut self) {
        for (i, &basis_index) in self.basis_indecies.iter().enumerate() {
            self.solution[basis_index] = self.b[i];
        }
    }
}


#[post("/solve", format = "json", data = "<tableau>")]
fn solve(tableau: Json<TableauData>) -> Json<TableauData> {
    let mut t = Tableau::new(tableau.0);
    t.setup();
    for _ in 0..t.n + 1 {
        if t.solved || t.error { break; }
        t.print_table();
        t.iterate();
    }
    if t.solved {
        t.get_solution();
        Json(TableauData::new(t))
    } else if t.error {
        Json(TableauData::new(Tableau::error_tableau(t)))
    } else {
        t.error = true; 
        t.error_message = String::from("Stuck in a cycle, terminted solution process.");
        Json(TableauData::new(t))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![solve]).launch();
}
