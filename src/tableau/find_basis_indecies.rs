use crate::tableau::Tableau;

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
}