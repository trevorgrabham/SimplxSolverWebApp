#![allow(non_snake_case)]

use crate::Tableau;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TableauData {
    pub A_numerators: Vec<Vec<i64>>,
    pub A_denominators: Vec<Vec<i64>>,
    pub b_numerators: Vec<i64>,
    pub b_denominators: Vec<i64>,
    b_m_numerators: Vec<i64>,
    b_m_denominators: Vec<i64>,
    pub c_numerators: Vec<i64>,
    pub c_denominators: Vec<i64>,
    pub c_m_numerators: Vec<i64>,
    pub c_m_denominators: Vec<i64>,
    pub m: usize,
    pub n: usize,
    pub solve_algorithm: String,
    pub variable_select_type: String,
    pub big_M_solve_algorithm: String,
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
    solution_m_numerators: Vec<i64>,
    solution_m_denominators: Vec<i64>,
}

impl TableauData {
    pub fn new(t: Tableau) -> TableauData {
        let (a_num, a_den): (Vec<Vec<i64>>, Vec<Vec<i64>>) = t.A.into_iter()
                                                                .map(|row| row.into_iter()
                                                                              .map(|el| (*el.numer(), *el.denom()))
                                                                              .unzip())
                                                                .unzip();
        let ((b_num, b_den), (b_m_num, b_m_den)): ((Vec<i64>, Vec<i64>), (Vec<i64>, Vec<i64>)) = t.b.into_iter()
                                                                                                    .map(|el| ((*el.constant.numer(), *el.constant.denom()), (*el.M.numer(), *el.M.denom())))
                                                                                                    .unzip();
        let ((c_num, c_den), (c_m_num, c_m_den)): ((Vec<i64>, Vec<i64>),(Vec<i64>, Vec<i64>)) = t.c.into_iter()
                                                                                                    .map(|el| ((*el.constant.numer(), *el.constant.denom()), (*el.M.numer(), *el.M.denom())))
                                                                                                    .unzip();
        let ((reduced_cost_num, reduced_cost_den), (reduced_cost_m_num, reduced_cost_m_den)): ((Vec<i64>, Vec<i64>), (Vec<i64>, Vec<i64>)) = t.reduced_cost.into_iter()
                                                                                                                               .map(|el| ((*el.constant.numer(), *el.constant.denom()), (*el.M.numer(), *el.M.denom())))
                                                                                                                               .unzip();
        let ((sol_num, sol_den), (sol_m_num, sol_m_den)): ((Vec<i64>, Vec<i64>), (Vec<i64>, Vec<i64>)) = t.solution.into_iter()
                                                                                                                   .map(|el| ((*el.constant.numer(), *el.constant.denom()), (*el.M.numer(), *el.M.denom())))
                                                                                                                   .unzip();
        TableauData {
            A_numerators: a_num,
            A_denominators: a_den,
            b_numerators: b_num,
            b_denominators: b_den,
            b_m_numerators: b_m_num,
            b_m_denominators: b_m_den,
            c_numerators: c_num,
            c_denominators: c_den,
            c_m_numerators: c_m_num,
            c_m_denominators: c_m_den,
            m: t.m,
            n: t.n,
            solve_algorithm: String::from(""),
            variable_select_type: String::from(""),
            big_M_solve_algorithm: String::from(""),
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
            solution_m_numerators: sol_m_num,
            solution_m_denominators: sol_m_den,
        }
    }
}
