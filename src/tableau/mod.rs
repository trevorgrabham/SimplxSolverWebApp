#![allow(non_snake_case)]

mod add_artificial_var;
mod calc_reduced_cost;
mod find_basis_indecies;
mod get_solution;
mod iterate;
mod pivot;
mod remove_col;
mod remove_row;
mod select_entering_var;
mod select_leaving_var;
mod setup;
mod solve;
mod two_phase_second_phase;

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

    pub fn print_table(&self) {
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
}