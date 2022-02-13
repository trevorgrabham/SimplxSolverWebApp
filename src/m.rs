#![allow(non_snake_case)]

use num::rational::Ratio;

#[derive(Eq, Debug, Clone)]
pub struct M {
    pub M: Ratio<i64>,
    pub constant: Ratio<i64>,
}

impl M {
    pub fn new(M: Ratio<i64>, constant: Ratio<i64>) -> M {
        M {
            M,
            constant,
        }
    }
}

impl std::ops::Mul<&M> for &Ratio<i64> {
    type Output = M;

    fn mul(self, rhs: &M) -> M {
        M {
            M: rhs.M * self,
            constant: rhs.constant * self,
        }
    }
}

impl std::ops::Mul<&M> for &M {
    type Output = M;

    fn mul(self, rhs: &M) -> M {
        if self.M == Ratio::new(0i64,1) {
            M {
                M: rhs.M * self.constant,
                constant: rhs.constant * self.constant,
            }
        } else if rhs.M == Ratio::new(0i64,1) {
            M {
                M: self.M * rhs.constant,
                constant: self.constant * rhs.constant,
            }
        } else {
            M {
                M: Ratio::new(0i64,1),
                constant: Ratio::new(0i64,1),
            }
        }
    }
}

impl std::ops::Sub<M> for &M {
    type Output = M;

    fn sub(self, rhs: M) -> M {
        M {
            M: self.M - rhs.M,
            constant: self.constant - rhs.constant,
        }
    }
}

impl std::ops::Neg for &M {
    type Output = M;

    fn neg(self) -> M {
        M {
            M: -self.M,
            constant: -self.constant,
        }
    }
}

impl std::iter::Sum<Self> for M {
    fn sum<I>(iter: I) -> Self 
    where
        I: Iterator<Item = Self>,
        {
            iter.fold(Self { M: Ratio::from_integer(0i64), constant: Ratio::from_integer(0i64), }, |a ,b| Self { M: a.M + b.M, constant: a.constant + b.constant, })
        }
}

impl std::ops::Div<&Ratio<i64>> for &M {
    type Output = M;

    fn div(self, rhs: &Ratio<i64>) -> M {
        M {
            M: self.M / rhs,
            constant: self.constant / rhs,
        }
    }
}

impl std::ops::Div<&Ratio<i64>> for M {
    type Output = Self;

    fn div(self, rhs: &Ratio<i64>) -> Self {
        M {
            M: self.M / rhs,
            constant: self.constant / rhs,
        }
    }
}

impl std::ops::AddAssign<&M> for M {
    fn add_assign(&mut self, rhs: &Self) {
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

impl std::ops::MulAssign<i64> for M {
    fn mul_assign(&mut self, rhs: i64) {
        self.M *= rhs;
        self.constant *= rhs;
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
                        if self.M == Ratio::from_integer(1i64) {
                            write!(f, "M+{}", self.constant)
                        } else if self.M == Ratio::from_integer(-1i64) {
                            write!(f, "-M+{}", self.constant)
                        } else {
                            write!(f, "{}M+{}", self.M, self.constant)
                        }
                    },
                    c if c < Ratio::from_integer(0i64) => {
                        if self.M == Ratio::from_integer(1i64) {
                            write!(f, "M{}", self.constant)
                        } else if self.M == Ratio::from_integer(-1i64) {
                            write!(f, "-M{}", self.M)
                        } else {
                            write!(f, "{}M{}", self.M, self.constant)
                        }
                    },
                    _ => {
                        if self.M == Ratio::from_integer(1i64) {
                            write!(f, "M")
                        } else if self.M == Ratio::from_integer(-1i64) {
                            write!(f, "-M")
                        } else {
                            write!(f, "{}M", self.M)
                        }
                    },
                }
            },
        }
    }
}