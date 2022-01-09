use num::Num;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};

fn gcd(a: i64, b: i64) -> i64 {
    // Basic Euclidean Algorithm
    // taken from: https://www.geeksforgeeks.org/euclidean-algorithms-basic-and-extended/
    if a == 0  { b }
    else  { gcd(b%a, a) }
}

#[derive(Debug, Copy, Clone)]
pub struct Fraction {
    n: i64,
    d: i64
}

impl Fraction {
    fn reduce(&mut self) -> () {
        let divisor = gcd(self.n, self.d);
        self.n /= divisor;
        self.d /= divisor;
    }

    fn new(n: i64, d: i64) -> Fraction {
        let mut new_frac = Fraction {
            n,
            d
        };
        new_frac.reduce();
        new_frac
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Fraction::new(1,0)
    }
}

impl Add<i64> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: i64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl Add<f64> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: f64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Fraction::new(1,0)
    }
}

impl Sub<i64> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: i64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl Sub<f64> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: f64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Fraction::new(1,0)
    }
}

impl Mul<i64> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: i64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl Mul<f64> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: f64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Fraction::new(1,0)
    }
}

impl Div<i64> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: i64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl Div<f64> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: f64) -> Fraction {
        Fraction::new(1,0)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        None
    }
}

impl PartialOrd<i64> for Fraction {
    fn partial_cmp(&self, rhs: &i64) -> Option<Ordering> {
        None
    }
}

impl PartialOrd<Fraction> for i64 {
    fn partial_cmp(&self, rhs: &Fraction) -> Option<Ordering> {
        None
    }
}

impl PartialOrd<f64> for Fraction {
    fn partial_cmp(&self, rhs: &f64) -> Option<Ordering> {
        None
    }
}

impl PartialOrd<Fraction> for f64 {
    fn partial_cmp(&self, rhs: &Fraction) -> Option<Ordering> {
        None
    }
}

impl Ord for Fraction {
    fn cmp(&self, rhs: &Self) -> Ordering {
        1i64.cmp(&1i64)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, rhs: &Self) -> bool {
        false
    }
}

impl PartialEq<i64> for Fraction {
    fn eq(&self, rhs: &i64) -> bool {
        false
    }
}

impl PartialEq<Fraction> for i64 {
    fn eq(&self, rhs: &Fraction) -> bool {
        false
    }
}

impl PartialEq<f64> for Fraction {
    fn eq(&self, rhs: &f64) -> bool {
        false
    }
}

impl PartialEq<Fraction> for f64 {
    fn eq(&self, rhs: &Fraction) -> bool {
        false
    }
}

impl Eq for Fraction {}

impl From<i64> for Fraction {
    fn from(n: i64) -> Fraction {
        let mut new_frac = Fraction {
            n: n as i64,
            d: 1
        };
        new_frac.reduce();
        new_frac
    }
}

impl From<f64> for Fraction {
    fn from(f: f64) -> Fraction {
        let mut n = f;
        let mut d = 1i64;
        while n.fract() > 1e-10 {
            n *= 10f64;
            d *= 10;
        }
        let mut new_frac = Fraction {
            n: n as i64,
            d 
        };
        new_frac.reduce();
        new_frac
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn greatest_common_denominator() {
        // works for either order of the arguments
        assert_eq!(gcd(10,2), 5);
        assert_eq!(gcd(2,10), 5);
        // making sure that we are not defaulting to a gcd of 1
        assert_ne!(gcd(10,2), 1);
        // making sure that gcd(a,b) == gcd(abs(a), abs(b))
        assert_eq!(gcd(-10,2), 5);
        assert_eq!(gcd(-10,-2), 5);
        // making sure that it works when the numbers do not share a common divisor
        assert_eq!(gcd(101,13), 1);
        assert_eq!(gcd(11,19), 1);
    }

    #[test]
    fn set_up() {
        // check for the from operators
        // ints
        assert_eq!(Fraction::from(3), Fraction::new(3,1));
        // floats
        assert_eq!(Fraction::from(1.5), Fraction::new(3,2));
        // check that we are reducing our fractions
        assert_eq!(Fraction::from(3), Fraction::new(6,2));
        assert_eq!(Fraction::from(1.5), Fraction::new(6,4));
        // check that we are dealing with our signs properly 
        assert_eq!(Fraction::new(-2,1), Fraction::new(2,-1));
        assert_eq!(Fraction::new(-2,-1), Fraction::new(2,1));
        // fraction of zero
        assert_eq!(Fraction::new(0,1), Fraction::from(0));
    }

    #[test]
    #[should_panic]
    fn zero_denom() {
        Fraction::new(1,0);
    }

    #[test]
    fn addition() {
        // fraction + fraction
        assert_eq!(Fraction::from(1) + Fraction::from(2), Fraction::from(3));
        assert_eq!(Fraction::from(1.25) + Fraction::from(2.5), Fraction::from(3.75));
        // fraction + numeric
        assert_eq!(Fraction::from(2) + 1, Fraction::from(3));
        assert_eq!(Fraction::from(2) + 1.5, Fraction::from(3.5));
        // fraction + -fraction
        assert_eq!(Fraction::from(5) + Fraction::from(-5), Fraction::from(0));
        assert_eq!(Fraction::from(-2.5) + Fraction::from(2.5), Fraction::from(0));
        // fraction from int + fraction from float 
        assert_eq!(Fraction::from(2) + Fraction::from(2.5), Fraction::new(9,2));
    }

    #[test]
    fn subtraction() {
        // fraction - fraction
        assert_eq!(Fraction::from(1) - Fraction::from(2), Fraction::from(-1));
        assert_eq!(Fraction::from(1.25) - Fraction::from(1.25), Fraction::from(0));
        // fraction - numeric
        assert_eq!(Fraction::from(2) - 1, Fraction::from(1));
        assert_eq!(Fraction::from(2) - 1.5, Fraction::from(0.5));
        // fraction - -fraction
        assert_eq!(Fraction::from(5) - Fraction::from(-5), Fraction::from(10));
        assert_eq!(Fraction::from(-2.5) - Fraction::from(2.5), Fraction::from(5));
        // fraction from int - fraction from float 
        assert_eq!(Fraction::from(2) - Fraction::from(2.5), Fraction::new(-1,2));
    }

    #[test]
    fn multiplication() {
        // fraction * fraction
        assert_eq!(Fraction::from(1) * Fraction::from(1), Fraction::from(1));
        assert_eq!(Fraction::from(2.5) * Fraction::from(0.4), Fraction::from(1));
        // fraction * numeric
        assert_eq!(Fraction::from(2.5) * 2, Fraction::from(5));
        assert_eq!(Fraction::from(5) * 1.2, Fraction::from(6));
        // proper signs
        assert_eq!(Fraction::from(-2) * Fraction::from(1), Fraction::new(2,-1));
        assert_eq!(Fraction::from(3) * Fraction::from(-4), Fraction::from(12));
        assert_eq!(Fraction::from(-0.4) * Fraction::from(-0.5), Fraction::from(0.2));
        // fraction from int * fraction from float
        assert_eq!(Fraction::from(0.4) * Fraction::from(3), Fraction::from(1.2));
    }

    #[test]
    fn division() {
        // fraction / fraction
        assert_eq!(Fraction::from(1) / Fraction::from(1), Fraction::from(1));
        assert_eq!(Fraction::from(2.5) / Fraction::from(0.4), Fraction::new(25,4));
        // fraction / numeric
        assert_eq!(Fraction::from(2.5) / 2, Fraction::from(1.25));
        assert_eq!(Fraction::from(5) / 1.2, Fraction::new(25,6));
        // proper signs
        assert_eq!(Fraction::from(-2) / Fraction::from(1), Fraction::new(2,-1));
        assert_eq!(Fraction::from(3) / Fraction::from(-4), Fraction::new(3,-4));
        assert_eq!(Fraction::from(-0.4) / Fraction::from(-0.5), Fraction::new(4,5));
        // fraction from int / fraction from float
        assert_eq!(Fraction::from(0.4) / Fraction::from(3), Fraction::new(4,30));
    }

    #[test]
    fn comparisons() {
        // fraction and fraction
        assert!(Fraction::from(-1) < Fraction::from(1));
        assert!(Fraction::from(1.25) > Fraction::from(-1.25));
        assert!(Fraction::from(2) > Fraction::from(0));
        assert!(Fraction::from(1.75) > Fraction::from(0.75));
        // fraction and 1
        assert!(Fraction::from(1.1) > Fraction::from(1));
        assert!(Fraction::from(0.99) < Fraction::from(1));
        // fraction and numeric
        assert!(Fraction::from(2) < 2.1);
        assert!(1.5 < Fraction::from(3));
    }

    #[test]
    fn plus_equals() {

    }

    #[test]
    fn sub_equals() {

    }

    #[test]
    fn times_equals() {

    }

    #[test]
    fn div_equals() {

    }
}