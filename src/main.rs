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
    solve_algorithm: String,
    error: bool, 
    error_message: String,
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
            solve_algorithm: t.solve_algorithm,
            error: false,
            error_message: String::from(""),
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

    fn select_entering_var(&self) -> usize {
        0
    }

    fn select_leaving_var(&self) -> usize {
        0
    }

    fn pivot(&mut self, entering: usize, leaving: usize) {

    }

    fn iterate(&mut self) {
        match self.solve_algorithm.as_str() {
            "standard" => {
                let leaving_var = self.select_leaving_var();
                let entering_var = self.select_entering_var();
                self.pivot(entering_var, leaving_var);
            },
            "dual" => {
                let entering_var = self.select_entering_var();
                let leaving_var = self.select_leaving_var();
                self.pivot(entering_var, leaving_var);
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
fn setup(tableau: Json<TableauData>) -> String {
    let mut t = Tableau::new(tableau.0);
    t.find_basis_matrix();
    t.calc_reduced_cost();
    t.print_table();
    let basis_indecies = t.get_basis();
    format!("Basis indecies: [{}, {}, {}]\n", basis_indecies.0, basis_indecies.1, basis_indecies.2)
}

fn main() {
    rocket::ignite().mount("/", routes![setup]).launch();
}
