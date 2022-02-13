use crate::tableau::Tableau;
use crate::m::M;

use num::rational::Ratio;

impl Tableau {
    pub fn find_basis_indecies(&mut self) {
        let mut a_cols = vec![Vec::with_capacity(self.m);self.n];
        for row in 0..self.m {
            for col in 0..self.n {
                a_cols[col].push(self.A[row][col]);
            }
        }
        let mut I = vec![Ratio::new(0i64,1);self.m];
        I[0] = Ratio::new(1i64,1);
        match self.solve_algorithm.as_str() {
            "standard" => {
                for i in 0..self.m {
                    let res = a_cols.iter().position(|col| col == &I);
                    match res {
                        Some(index) => {
                            self.basis_indecies[i] = index;
                        },
                        None => {
                            self.add_col(i, M::new(Ratio::new(-1i64,1), Ratio::new(0i64,1)));
                        }
                    }
                    I.rotate_right(1);
                }
            },
            "dual" => {
                let mut neg_I = I.clone();
                neg_I[0] = -neg_I[0];
                for i in 0..self.m {
                    let res = a_cols.iter().position(|col| col == &I || col == &neg_I);
                    match res {
                        Some(index) => {
                            if self.A[i][index] == Ratio::new(-1i64,1) {
                                for el in self.A[i].iter_mut() {
                                    *el *= -1;
                                }
                                self.b[i] *= -1;
                            }
                            self.basis_indecies[i] = index;
                        }, 
                        None => {
                            self.basis_indecies[i] = self.n;
                        }
                    }
                    I.rotate_right(1);
                    neg_I.rotate_right(1);
                }
            },
            _ => {
                self.error = true;
                self.error_message = String::from("Unknown selection for solve algorithm.");
                return;
            }
        }
    }
}