//! This crate only implements the features necessary for the current quest. Must not be used outside.

#[derive(Debug, Clone, Copy)]
pub struct Complex(i64, i64);

impl Complex {
    pub fn new(real: i64, imag: i64) -> Self {
        Complex(real, imag)
    }

    pub fn real(&self) -> i64 {
        self.0
    }

    pub fn imag(&self) -> i64 {
        self.1
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex::new(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex::new(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + rhs.0 * self.1,
        )
    }
}

impl std::ops::MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Complex {
        Complex::new(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl std::ops::DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}
