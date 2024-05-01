use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug)]
pub struct Imaginary {
    pub real: f64,
    pub i: f64,
}

impl Imaginary {
    pub fn conjugate(&self) -> Imaginary {
        Imaginary {
            real: self.real,
            i: -self.i,
        }
    }
    pub fn absolute(&self) -> f64 {
        // https://www.quora.com/How-do-I-find-the-magnitude-of-a-complex-number-for-example-5-5i
        let magnitude = &self.conjugate() * self;
        assert!(magnitude.i == 0.0);
        magnitude.real.sqrt()
    }
}

impl Add for &Imaginary {
    type Output = Imaginary;
    fn add(self, rhs: Self) -> Self::Output {
        Imaginary {
            real: self.real + rhs.real,
            i: self.i + rhs.i,
        }
    }
}

impl Sub for &Imaginary {
    type Output = Imaginary;
    fn sub(self, rhs: Self) -> Self::Output {
        Imaginary {
            real: self.real - rhs.real,
            i: self.i - rhs.i,
        }
    }
}

impl Mul for &Imaginary {
    type Output = Imaginary;
    fn mul(self, rhs: Self) -> Self::Output {
        // https://www.cuemath.com/numbers/multiplying-complex-numbers/
        Imaginary {
            real: self.real * rhs.real - self.i * rhs.i,
            i: self.real * rhs.i + self.i * rhs.real,
        }
    }
}

impl Div for &Imaginary {
    type Output = Imaginary;
    fn div(self, rhs: Self) -> Self::Output {
        // https://www.cuemath.com/numbers/division-of-complex-numbers/
        let denominator = rhs.real * rhs.real + rhs.i * rhs.i;
        Imaginary {
            real: (self.real * rhs.real + self.i * rhs.i) / denominator,
            i: (self.i * rhs.real - self.real * rhs.i) / denominator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let num1 = Imaginary { real: 3.0, i: 2.0 };
        let num2 = Imaginary { real: 1.0, i: 7.0 };
        assert_eq!(&num1 + &num2, Imaginary { real: 4.0, i: 9.0 })
    }

    #[test]
    fn test_subtraction() {
        let num1 = Imaginary { real: 3.0, i: 2.0 };
        let num2 = Imaginary { real: 1.0, i: 7.0 };
        assert_eq!(&num1 - &num2, Imaginary { real: 2.0, i: -5.0 })
    }

    #[test]
    fn test_multiplication() {
        let num1 = Imaginary { real: 3.0, i: 2.0 };
        let num2 = Imaginary { real: 1.0, i: 7.0 };
        assert_eq!(
            &num1 * &num2,
            Imaginary {
                real: -11.0,
                i: 23.0
            }
        )
    }

    #[test]
    fn test_division() {
        let num1 = Imaginary {
            real: -11.0,
            i: 23.0,
        };
        let num2 = Imaginary { real: 1.0, i: 7.0 };
        assert_eq!(&num1 / &num2, Imaginary { real: 3.0, i: 2.0 })
    }
}
