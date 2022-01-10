use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };

fn gcd(a: i64, b: i64) -> i64 {
    // Basic Euclidean Algorithm
    // taken from: https://www.geeksforgeeks.org/euclidean-algorithms-basic-and-extended/
    let a = a.abs();
    let b = b.abs();            // take the abs so % corresponds to the actual modulo operator
    if a == 0  { b }
    else  { gcd(b%a, a) }
}

#[derive(Debug, Copy, Clone)]
pub struct Fraction {
    n: i64,
    d: i64
}

impl Fraction {
    fn reduce(&mut self) -> &Self {
        let mut divisor = gcd(self.n, self.d);
        if self.d.is_negative() { divisor *= -1; }
        self.n /= divisor;
        self.d /= divisor;
        self
    }

    fn new(n: i64, d: i64) -> Fraction {
        if d == 0 { panic!("Cannot have a denominator of 0"); }
        *Fraction {
            n,
            d
        }.reduce()
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.d == rhs.d { 
            *Fraction {
                n: self.n + rhs.n,
                d: self.d
            }.reduce()
        } else {
            *Fraction {
                n: self.n*rhs.d + rhs.n*self.d,
                d: self.d*rhs.d
            }.reduce()
        }
    }
}

impl Add<i64> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: i64) -> Fraction {
        *Fraction {
            n: self.n + rhs*self.d,
            d: self.d
        }.reduce()
    }
}

impl Add<f64> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: f64) -> Fraction {
        let rhs = Fraction::from(rhs);
        self + rhs
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.d == rhs.d {
            *Fraction {
                n: self.n - rhs.n,
                d: self.d
            }.reduce()
        } else {
            *Fraction {
                n: self.n*rhs.d - rhs.n*self.d,
                d: self.d*rhs.d
            }.reduce()
        }
    }
}

impl Sub<i64> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: i64) -> Fraction {
        *Fraction {
            n: self.n - rhs*self.d,
            d: self.d
        }.reduce()
    }
}

impl Sub<f64> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: f64) -> Fraction {
        self - Fraction::from(rhs)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let divisor = gcd(self.n, rhs.d);
        let self_n = self.n/divisor;
        let rhs_d = rhs.d/divisor;
        let divisor = gcd(self.d, rhs.n);
        let rhs_n = rhs.n/divisor;
        let self_d = self.d/divisor;
        *Fraction {
            n: self_n * rhs_n,
            d: self_d * rhs_d
        }.reduce()
    }
}

impl Mul<i64> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: i64) -> Fraction {
        let divisor = gcd(self.d, rhs);
        let self_d = self.d/divisor;
        let rhs = rhs/divisor;
        *Fraction {
            n: self.n * rhs,
            d: self_d
        }.reduce()
    }
}

impl Mul<f64> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: f64) -> Fraction {
        self * Fraction::from(rhs)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let divisor = gcd(self.n, rhs.n);       // factor out any common divisors first so that we reduce the likelyhood of overflow
        let self_n = self.n/divisor;
        let rhs_n = rhs.n/divisor;
        let divisor = gcd(self.d, rhs.d);
        let self_d = self.d/divisor;
        let rhs_d = rhs.d/divisor;
        *Fraction {
            n: self_n * rhs_d,
            d: self_d * rhs_n
        }.reduce()
    }
}

impl Div<i64> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: i64) -> Fraction {
        self/Fraction::from(rhs)
    }
}

impl Div<f64> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: f64) -> Fraction {
        self/Fraction::from(rhs)
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl AddAssign<i64> for Fraction {
    fn add_assign(&mut self, other: i64) {
        self.n += other*self.d;
        self.reduce();
    }
}

impl AddAssign<f64> for Fraction {
    fn add_assign(&mut self, other: f64) {
        *self = *self + other;
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl SubAssign<i64> for Fraction {
    fn sub_assign(&mut self, other: i64) {
        self.n -= self.d*other;
        self.reduce();
    }
}

impl SubAssign<f64> for Fraction {
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other;
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<i64> for Fraction {
    fn mul_assign(&mut self, other: i64) {
        *self = *self * other;
    }
}

impl MulAssign<f64> for Fraction {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl DivAssign<i64> for Fraction {
    fn div_assign(&mut self, other: i64) {
        *self = *self / other;
    }
}

impl DivAssign<f64> for Fraction {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl PartialOrd<i64> for Fraction {
    fn partial_cmp(&self, rhs: &i64) -> Option<Ordering> {
        Some(self.cmp(&Fraction::from(*rhs)))
    }
}

impl PartialOrd<Fraction> for i64 {
    fn partial_cmp(&self, rhs: &Fraction) -> Option<Ordering> {
        Some(Fraction::from(*self).cmp(rhs))
    }
}

impl PartialOrd<f64> for Fraction {
    fn partial_cmp(&self, rhs: &f64) -> Option<Ordering> {
        Some(self.cmp(&Fraction::from(*rhs)))
    }
}

impl PartialOrd<Fraction> for f64 {
    fn partial_cmp(&self, rhs: &Fraction) -> Option<Ordering> {
        Some(Fraction::from(*self).cmp(rhs))
    }
}

impl Ord for Fraction {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let divisor = gcd(self.n, rhs.d);           // we divide by any common factors to try and avoid overflow 
        let self_n = self.n / divisor;
        let rhs_d = rhs.d / divisor;
        let divisor = gcd(self.d, rhs.n);
        let self_d = self.d / divisor;
        let rhs_n = rhs.n / divisor;
        let lhs = self_n*rhs_d;
        let rhs = rhs_n*self_d;
        lhs.cmp(&rhs)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, rhs: &Self) -> bool {
        self.n == rhs.n && self.d == rhs.d
    }
}

impl PartialEq<i64> for Fraction {
    fn eq(&self, rhs: &i64) -> bool {
        self.n == rhs*self.d
    }
}

impl PartialEq<Fraction> for i64 {
    fn eq(&self, rhs: &Fraction) -> bool {
        self*rhs.d == rhs.n
    }
}

impl PartialEq<f64> for Fraction {
    fn eq(&self, rhs: &f64) -> bool {
        let diff: f64 = (*rhs*(self.d as f64) - self.n as f64).abs();
        diff < 1e-3         // this should require that the user enters decimals with 3 figs after the decimal
    }
}

impl PartialEq<Fraction> for f64 {
    fn eq(&self, rhs: &Fraction) -> bool {
        let diff: f64 = (*self*(rhs.d as f64) - rhs.n as f64).abs();
        diff < 1e-3         // this should require that the user enters decimals with 3 figs after the decimal
    }
}

impl Eq for Fraction {}

impl From<i64> for Fraction {
    fn from(n: i64) -> Fraction {
        *Fraction {
            n: n as i64,
            d: 1
        }.reduce()
    }
}

impl From<f64> for Fraction {
    fn from(f: f64) -> Fraction {
        let mut n = f;
        let mut d = 1i64;
        for _ in 0..18 {
            if n.fract().abs() < 1e-6 { break; }
            n *= 10f64;
            d *= 10;
        }
        *Fraction {
            n: n as i64,
            d 
        }.reduce()
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn greatest_common_denominator() {
        // works for either order of the arguments
        assert_eq!(gcd(10,2), 2);
        assert_eq!(gcd(2,10), 2);
        // making sure that we are not defaulting to a gcd of 1
        assert_ne!(gcd(10,2), 1);
        // making sure that gcd(a,b) == gcd(abs(a), abs(b))
        assert_eq!(gcd(-10,2), 2);
        assert_eq!(gcd(-10,-2), 2);
        // making sure that it works when the numbers do not share a common divisor
        assert_eq!(gcd(101,13), 1);
        assert_eq!(gcd(11,19), 1);
    }

    #[test]
    fn set_up() {
        // check for the from operators
        // ints
        assert_eq!(Fraction::from(3), Fraction::new(3,1));
        assert_eq!(Fraction::from(-2), Fraction::new(-2,1));
        // floats
        assert_eq!(Fraction::from(1.5), Fraction::new(3,2));
        assert_eq!(Fraction::from(-2.5), Fraction::new(-5,2));
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
        assert_eq!(Fraction::from(-2.5) - Fraction::from(2.5), Fraction::from(-5));
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
        assert_eq!(Fraction::from(3) * Fraction::from(-4), Fraction::from(-12));
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
        let mut test_int = Fraction::from(1);
        let mut test_float = Fraction::from(0.5);
        // Fraction + Fraction 
        test_int += Fraction::from(1);
        assert_eq!(test_int, Fraction::new(2,1));
        test_int += Fraction::from(1.5);
        assert_eq!(test_int, Fraction::new(7,2));
        test_float += Fraction::from(1);
        assert_eq!(test_float, Fraction::new(3,2));
        test_float += Fraction::from(1.25);
        assert_eq!(test_float, Fraction::new(11,4));
        test_int = Fraction::from(1);
        test_float = Fraction::from(0.5);
        // Fraction + int
        test_int += 1;
        assert_eq!(test_int, Fraction::from(2));
        test_float += 1;
        assert_eq!(test_float, Fraction::from(1.5));
        // Fraction + float 
        test_int += 1.5;
        assert_eq!(test_int, Fraction::from(3.5));
        test_float += 0.75;
        assert_eq!(test_float, Fraction::from(2.25));
    }

    #[test]
    fn sub_equals() {
        let mut test_int = Fraction::from(50);
        let mut test_float = Fraction::from(45.5);
        // Fraction - Fraction 
        test_int -= Fraction::from(9);
        test_float -= Fraction::from(5.25);
        assert_eq!(test_int, Fraction::from(41));
        assert_eq!(test_float, Fraction::from(40.25));
        test_int -= Fraction::from(50.75);
        test_float -= Fraction::from(50.25);
        assert_eq!(test_int, Fraction::from(-9.75));
        assert_eq!(test_float, Fraction::from(-10));
        // Fraction - int 
        test_int = Fraction::from(50);
        test_float = Fraction::from(45.5);
        test_int -= 7;
        test_float -= 9;
        assert_eq!(test_int, Fraction::from(43));
        assert_eq!(test_float, Fraction::from(36.5));
        // Fraction - float
        test_int -= 5.5;
        test_float -= 0.75;
        assert_eq!(test_int, Fraction::from(37.5));
        assert_eq!(test_float, Fraction::from(35.75));
    }

    #[test]
    fn times_equals() {
        let mut test_int = Fraction::from(3);
        let mut test_float = Fraction::new(1,3);
        // Fraction * Fraction
        test_int *= Fraction::from(8);
        test_float *= Fraction::from(3);
        assert_eq!(test_int, Fraction::from(24));
        assert_eq!(test_float, Fraction::from(1));
        test_int *= Fraction::from(0.125);
        test_float *= Fraction::from(0.5);
        assert_eq!(test_int, Fraction::from(3));
        assert_eq!(test_float, Fraction::from(0.5));
        // Fraction * int
        test_int = Fraction::from(3);
        test_float = Fraction::from(0.25);
        test_int *= 4;
        test_float *= 4;
        assert_eq!(test_int, Fraction::from(12));
        assert_eq!(test_float, Fraction::from(1));
        // Fraction * float
        test_int *= 0.5;
        test_float *= 4;
        test_float *= 1.25;
        assert_eq!(test_int, Fraction::from(6));
        assert_eq!(test_float, Fraction::from(5));
    }

    #[test]
    fn div_equals() {
        let mut test_int = Fraction::from(3);
        let mut test_float = Fraction::from(5.5);
        // Fraction / Fraction
        test_int /= 3;
        test_float /= 2;
        assert_eq!(test_int, Fraction::from(1));
        assert_eq!(test_float, Fraction::from(2.75));
        test_int /= Fraction::from(1.5);
        test_float /= Fraction::from(0.25);
        assert_eq!(test_int, Fraction::new(2,3));
        assert_eq!(test_float, Fraction::from(11));
        // Fraction / int
        test_int = Fraction::from(25);
        test_float = Fraction::from(12.5);
        test_int /= 5;
        test_float /= 5;
        assert_eq!(test_int, Fraction::from(5));
        assert_eq!(test_float, Fraction::from(2.5));
        // Fraction / float
        test_int /= 2.5;
        test_float /= 2.5;
        assert_eq!(test_int, Fraction::from(2));
        assert_eq!(test_float, Fraction::from(1));
    }
}