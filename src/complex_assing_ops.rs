use crate::Complex;
use num_traits::{Num, NumAssign};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

impl<T: NumAssign> AddAssign<Complex<T>> for Complex<T> {
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}
impl<T: NumAssign> SubAssign<Complex<T>> for Complex<T> {
    fn sub_assign(&mut self, rhs: Complex<T>) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}
impl<T: Num + Clone> MulAssign<Complex<T>> for Complex<T> {
    fn mul_assign(&mut self, rhs: Complex<T>) {
        let tmp_real = self.real.clone();
        self.real = self.real.clone() * rhs.real.clone() - self.imag.clone() * rhs.imag.clone();
        self.imag = tmp_real * rhs.imag + self.imag.clone() * rhs.real;
    }
}

impl<T: Num + Clone> DivAssign<Complex<T>> for Complex<T> {
    fn div_assign(&mut self, rhs: Complex<T>) {
        let tmp_real = self.real.clone();
        let sqs = rhs.real.clone() * rhs.real.clone() + rhs.imag.clone() * rhs.imag.clone();
        self.real = (self.real.clone() * rhs.real.clone() + self.imag.clone() * rhs.imag.clone())
            / sqs.clone();
        self.imag = (self.imag.clone() * rhs.real - tmp_real * rhs.imag) / sqs;
    }
}

impl<T: NumAssign + Clone> RemAssign<Complex<T>> for Complex<T> {
    fn rem_assign(&mut self, rhs: Complex<T>) {
        let gausian_int = self.gausian_integer(rhs.clone());
        *self -= rhs * gausian_int
    }
}

impl<T: NumAssign> AddAssign<T> for Complex<T> {
    fn add_assign(&mut self, rhs: T) {
        self.real += rhs;
    }
}
impl<T: NumAssign> SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.real -= rhs;
    }
}
impl<T: NumAssign + Clone> MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.real *= rhs.clone();
        self.imag *= rhs;
    }
}
impl<T: NumAssign + Clone> DivAssign<T> for Complex<T> {
    fn div_assign(&mut self, rhs: T) {
        self.real /= rhs.clone();
        self.imag /= rhs;
    }
}

impl<T: NumAssign + Clone> RemAssign<T> for Complex<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.real %= rhs.clone();
        self.imag %= rhs;
    }
}
