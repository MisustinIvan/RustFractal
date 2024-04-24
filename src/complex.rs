use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub r: f64,
    pub i: f64,
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Self {
        return Complex { r, i };
    }

    pub fn mag(&self) -> f64 {
        return (self.r * self.r + self.i * self.i).sqrt();
    }

    pub fn abs(&self) -> Complex {
        return Complex {
            r: self.r.abs(),
            i: self.i.abs(),
        };
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        return Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        };
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        return Complex {
            r: self.r - rhs.r,
            i: self.i - rhs.i,
        };
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.i -= rhs.i;
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        return Complex {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        };
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        self.r = self.r * rhs.r - self.i * rhs.i;
        self.i = self.r * rhs.i + self.i * rhs.r;
    }
}
